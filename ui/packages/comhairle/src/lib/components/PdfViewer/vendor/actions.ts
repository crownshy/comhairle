import type {
	TextSelectionActionOptions,
	ZoomActionOptions,
	NavigationActionOptions,
	AnnotationActionOptions,
	TextSelection
} from './types.js';
import { throttle } from './utils.js';

/**
 * Text selection action for PDF canvas - now works with text layer
 */
export function textSelection(canvas: HTMLCanvasElement, options: TextSelectionActionOptions = {}) {
	const { onSelect, enabled = true } = options;

	let textLayerElement: HTMLDivElement | null = null;

	function initialize() {
		if (!enabled) return;

		// Find the text layer element (should be sibling to canvas)
		const pageContent = canvas.parentElement;
		if (pageContent) {
			textLayerElement = pageContent.querySelector('.textLayer') as HTMLDivElement;
		}

		if (!textLayerElement) {
			return;
		}

		// Enable text selection on the text layer
		textLayerElement.style.userSelect = 'text';
		textLayerElement.style.pointerEvents = 'auto';
		textLayerElement.style.zIndex = '1001'; // Above canvas

		// Listen for mouseup events to detect completed selections
		document.addEventListener('mouseup', handleSelectionChange);
	}

	function handleSelectionChange() {
		if (!enabled || !textLayerElement || !onSelect) return;

		const selection = window.getSelection();
		if (!selection || selection.rangeCount === 0) return;

		const range = selection.getRangeAt(0);
		const selectedText = selection.toString().trim();

		// Check if selection is within our text layer
		const textLayerContainsSelection =
			textLayerElement.contains(range.commonAncestorContainer) ||
			textLayerElement.contains(range.startContainer) ||
			textLayerElement.contains(range.endContainer);

		if (!textLayerContainsSelection || !selectedText) return;

		// Get selection bounds relative to text layer
		const rangeRect = range.getBoundingClientRect();
		const textLayerRect = textLayerElement.getBoundingClientRect();

		const textSelection: TextSelection = {
			text: selectedText,
			pageNumber: parseInt(textLayerElement.dataset.page || '1', 10),
			x: rangeRect.left - textLayerRect.left,
			y: rangeRect.top - textLayerRect.top,
			width: rangeRect.width,
			height: rangeRect.height
		};

		onSelect(textSelection);
	}

	// Initialize when action is applied
	initialize();

	// Keep canvas non-selectable (text selection happens on text layer)
	canvas.style.userSelect = 'none';

	// Disable pointer events on canvas when text selection is enabled
	if (enabled) {
		canvas.style.pointerEvents = 'none';
	}

	return {
		update(newOptions: TextSelectionActionOptions) {
			Object.assign(options, newOptions);

			// Re-initialize if enabled state changed
			if (newOptions.enabled !== undefined) {
				if (newOptions.enabled) {
					initialize();
					canvas.style.pointerEvents = 'none';
				} else {
					// Disable text selection
					if (textLayerElement) {
						textLayerElement.style.userSelect = 'none';
						textLayerElement.style.pointerEvents = 'none';
					}
					canvas.style.pointerEvents = 'auto';
				}
			}
		},
		destroy() {
			document.removeEventListener('mouseup', handleSelectionChange);
		}
	};
}

/**
 * Zoom action for PDF canvas
 */
export function zoom(canvas: HTMLCanvasElement, options: ZoomActionOptions = {}) {
	const { onZoom, minScale = 0.1, maxScale = 3.0, step = 0.1 } = options;

	const throttledZoom = throttle((delta: number) => {
		if (onZoom) {
			const currentScale = parseFloat(
				canvas.style.transform?.match(/scale\(([^)]+)\)/)?.[1] || '1'
			);
			const newScale = Math.min(Math.max(currentScale + delta, minScale), maxScale);
			onZoom(newScale);
		}
	}, 50);

	function handleWheel(event: WheelEvent) {
		if (!event.ctrlKey && !event.metaKey) return;

		event.preventDefault();
		const delta = event.deltaY > 0 ? -step : step;
		throttledZoom(delta);
	}

	function handleKeyDown(event: KeyboardEvent) {
		if (!event.ctrlKey && !event.metaKey) return;

		switch (event.key) {
			case '+':
			case '=':
				event.preventDefault();
				throttledZoom(step);
				break;
			case '-':
				event.preventDefault();
				throttledZoom(-step);
				break;
			case '0':
				event.preventDefault();
				if (onZoom) onZoom(1.0);
				break;
		}
	}

	canvas.addEventListener('wheel', handleWheel, { passive: false });
	document.addEventListener('keydown', handleKeyDown);

	return {
		update(newOptions: ZoomActionOptions) {
			Object.assign(options, newOptions);
		},
		destroy() {
			canvas.removeEventListener('wheel', handleWheel);
			document.removeEventListener('keydown', handleKeyDown);
		}
	};
}

/**
 * Navigation action for PDF canvas
 */
export function navigation(canvas: HTMLCanvasElement, options: NavigationActionOptions = {}) {
	const { onPageChange, enableKeyboard = true, enableMouseWheel = false } = options;

	function handleKeyDown(event: KeyboardEvent) {
		if (!enableKeyboard || !onPageChange) return;

		switch (event.key) {
			case 'ArrowLeft':
			case 'PageUp':
				event.preventDefault();
				// This would need to get current page from component state
				onPageChange(-1); // Relative change
				break;
			case 'ArrowRight':
			case 'PageDown':
			case ' ':
				event.preventDefault();
				onPageChange(1); // Relative change
				break;
			case 'Home':
				event.preventDefault();
				onPageChange(1); // Absolute page
				break;
			case 'End':
				event.preventDefault();
				onPageChange(-1); // Go to last page (component would handle this)
				break;
		}
	}

	function handleWheel(event: WheelEvent) {
		if (!enableMouseWheel || !onPageChange) return;
		if (event.ctrlKey || event.metaKey) return; // Don't interfere with zoom

		const delta = event.deltaY > 0 ? 1 : -1;
		onPageChange(delta);
	}

	if (enableKeyboard) {
		document.addEventListener('keydown', handleKeyDown);
	}

	if (enableMouseWheel) {
		canvas.addEventListener('wheel', handleWheel);
	}

	return {
		update(newOptions: NavigationActionOptions) {
			Object.assign(options, newOptions);
		},
		destroy() {
			document.removeEventListener('keydown', handleKeyDown);
			canvas.removeEventListener('wheel', handleWheel);
		}
	};
}

/**
 * Annotation action for PDF canvas - handles PDF.js annotations and optional drawing overlay
 */
export function annotation(canvas: HTMLCanvasElement, options: AnnotationActionOptions = {}) {
	const {
		onAnnotationClick,
		onAnnotationHover,
		enableLinks = true,
		enableTextAnnotations = true,
		enableHighlights = true,
		// Legacy drawing support (backward compatibility)
		onAnnotate,
		color = '#ff0000',
		thickness = 2
	} = options;

	let annotationLayerElement: HTMLDivElement | null = null;
	let overlayCanvas: HTMLCanvasElement | null = null;
	let ctx: CanvasRenderingContext2D | null = null;
	let isDrawing = false;
	let lastX = 0;
	let lastY = 0;
	let annotations: any[] = [];

	function initialize() {
		// Find the annotation layer element (should be sibling to canvas)
		const pageContent = canvas.parentElement;
		if (pageContent) {
			annotationLayerElement = pageContent.querySelector(
				'.annotationLayer'
			) as HTMLDivElement;
		}

		// Set up annotation layer interaction if it exists
		if (annotationLayerElement) {
			setupAnnotationInteraction();
		}

		// Set up legacy drawing overlay if onAnnotate callback exists
		if (onAnnotate) {
			setupDrawingOverlay();
		}
	}

	function setupAnnotationInteraction() {
		if (!annotationLayerElement) return;

		// Annotation interactions are handled by the PDF viewer component itself
		// This action just ensures the annotation layer has proper event handling
		annotationLayerElement.style.pointerEvents = 'auto';
		annotationLayerElement.style.zIndex = '3'; // Above text layer (z-index 2)

		// The actual click and hover handling is done by the PDFViewer component
		// in the createAnnotationElement function, so we don't need to duplicate it here
	}

	function setupDrawingOverlay() {
		// Create overlay canvas for legacy drawing functionality
		overlayCanvas = document.createElement('canvas');
		overlayCanvas.style.position = 'absolute';
		overlayCanvas.style.top = '0';
		overlayCanvas.style.left = '0';
		overlayCanvas.style.pointerEvents = 'none';
		overlayCanvas.style.zIndex = '999'; // Above everything

		canvas.parentElement?.appendChild(overlayCanvas);

		ctx = overlayCanvas.getContext('2d');
		if (ctx) {
			ctx.strokeStyle = color;
			ctx.lineWidth = thickness;
			ctx.lineCap = 'round';
			ctx.lineJoin = 'round';
		}

		updateCanvasSize();
		canvas.addEventListener('mousedown', handleMouseDown);
		canvas.addEventListener('mousemove', handleMouseMove);
		canvas.addEventListener('mouseup', handleMouseUp);

		// Update overlay canvas size when main canvas changes
		const resizeObserver = new ResizeObserver(updateCanvasSize);
		resizeObserver.observe(canvas);

		// Store resize observer for cleanup
		(overlayCanvas as any)._resizeObserver = resizeObserver;
	}

	function updateCanvasSize() {
		if (!overlayCanvas) return;

		const rect = canvas.getBoundingClientRect();
		overlayCanvas.width = canvas.width;
		overlayCanvas.height = canvas.height;
		overlayCanvas.style.width = `${rect.width}px`;
		overlayCanvas.style.height = `${rect.height}px`;
	}

	function handleMouseDown(event: MouseEvent) {
		if (!event.shiftKey || !ctx) return; // Only draw when shift is held

		isDrawing = true;
		const rect = canvas.getBoundingClientRect();
		lastX = event.clientX - rect.left;
		lastY = event.clientY - rect.top;

		// Scale coordinates to canvas size
		const scaleX = canvas.width / rect.width;
		const scaleY = canvas.height / rect.height;
		lastX *= scaleX;
		lastY *= scaleY;
	}

	function handleMouseMove(event: MouseEvent) {
		if (!isDrawing || !ctx) return;

		const rect = canvas.getBoundingClientRect();
		const currentX = (event.clientX - rect.left) * (canvas.width / rect.width);
		const currentY = (event.clientY - rect.top) * (canvas.height / rect.height);

		ctx.beginPath();
		ctx.moveTo(lastX, lastY);
		ctx.lineTo(currentX, currentY);
		ctx.stroke();

		lastX = currentX;
		lastY = currentY;
	}

	function handleMouseUp() {
		if (!isDrawing) return;

		isDrawing = false;

		// Save legacy drawing annotation
		const annotation = {
			type: 'drawing',
			color,
			thickness,
			timestamp: Date.now()
		};

		annotations.push(annotation);

		if (onAnnotate) {
			onAnnotate(annotation);
		}
	}

	// Initialize when action is applied
	initialize();

	return {
		update(newOptions: AnnotationActionOptions) {
			Object.assign(options, newOptions);

			// Update drawing overlay if it exists
			if (ctx) {
				ctx.strokeStyle = newOptions.color || color;
				ctx.lineWidth = newOptions.thickness || thickness;
			}

			// Re-initialize if drawing overlay needs to be added/removed
			if (newOptions.onAnnotate && !overlayCanvas) {
				setupDrawingOverlay();
			} else if (!newOptions.onAnnotate && overlayCanvas) {
				cleanupDrawingOverlay();
			}
		},
		destroy() {
			cleanupDrawingOverlay();
		}
	};

	function cleanupDrawingOverlay() {
		if (overlayCanvas) {
			canvas.removeEventListener('mousedown', handleMouseDown);
			canvas.removeEventListener('mousemove', handleMouseMove);
			canvas.removeEventListener('mouseup', handleMouseUp);

			const resizeObserver = (overlayCanvas as any)._resizeObserver;
			if (resizeObserver) {
				resizeObserver.disconnect();
			}

			overlayCanvas.remove();
			overlayCanvas = null;
			ctx = null;
		}
	}
}
