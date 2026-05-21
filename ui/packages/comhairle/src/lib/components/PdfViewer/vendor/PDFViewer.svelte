<script lang="ts">
	import * as pdfjsLib from 'pdfjs-dist';
	import type { PDFDocumentProxy, PDFPageProxy } from 'pdfjs-dist';
	import { TextLayer } from 'pdfjs-dist';
	import 'pdfjs-dist/web/pdf_viewer.css';
	import { untrack } from 'svelte';
	import { textSelection, zoom, navigation, annotation } from './actions.js';
	import type {
		PDFSource,
		PDFDocumentState,
		PDFPageInfo,
		DisplayMode,
		FitMode,
		Theme,
		TextSelection,
		TextSelectionActionOptions,
		ZoomActionOptions,
		NavigationActionOptions,
		AnnotationActionOptions,
		PDFAnnotation,
		AnnotationInteractionEvent,
		PDFAnnotationSummary,
		AnnotationsLoadEvent
	} from './types.js';
	import type { RenderTask } from 'pdfjs-dist/types/src/display/api.js';
	import './PDFViewer.css';
	// Local patch (comhairle): use the app's shadcn Button in the toolbar.
	import { Button } from '$lib/components/ui/button';
	import ChevronLeftIcon from '@lucide/svelte/icons/chevron-left';
	import ChevronRightIcon from '@lucide/svelte/icons/chevron-right';
	import MinusIcon from '@lucide/svelte/icons/minus';
	import PlusIcon from '@lucide/svelte/icons/plus';

	// Configure PDF.js worker
	pdfjsLib.GlobalWorkerOptions.workerSrc = new URL(
		'pdfjs-dist/build/pdf.worker.min.mjs',
		import.meta.url
	).toString();

	interface Props {
		src: PDFSource;
		page?: number;
		scale?: number;
		fitMode?: FitMode;
		displayMode?: DisplayMode;
		showNavigation?: boolean;
		enableTextSelection?: boolean;
		theme?: Theme;
		containerClass?: string;
		canvasClass?: string;
		width?: number;
		height?: number;
		maxScale?: number;
		minScale?: number;
		textSelectionOptions?: TextSelectionActionOptions;
		zoomOptions?: ZoomActionOptions;
		navigationOptions?: NavigationActionOptions;
		annotationOptions?: AnnotationActionOptions;
		// Callback props to replace deprecated createEventDispatcher
		onLoad?: (event: { numPages: number; fingerprint: string }) => void;
		onError?: (event: { error: string }) => void;
		onPageChange?: (event: { page: number; total: number }) => void;
		onScaleChange?: (event: { scale: number }) => void;
		onTextSelect?: (selection: TextSelection) => void;
		onAnnotationsLoad?: (event: AnnotationsLoadEvent) => void;
	}

	let {
		src,
		page = $bindable(1),
		scale = $bindable(1.0),
		fitMode = 'none',
		displayMode = 'single',
		showNavigation = true,
		enableTextSelection = true,
		theme = 'light',
		containerClass = '',
		canvasClass = '',
		width,
		height,
		maxScale = 3.0,
		minScale = 0.1,
		textSelectionOptions = {},
		zoomOptions = {},
		navigationOptions = {},
		annotationOptions = {},
		onLoad,
		onError,
		onPageChange,
		onScaleChange,
		onTextSelect,
		onAnnotationsLoad
	}: Props = $props();

	// State management with Svelte 5 runes
	let document: PDFDocumentProxy | null = $state(null);
	let currentPageProxy: PDFPageProxy | null = $state(null);
	let allPageProxies: PDFPageProxy[] = $state([]);
	let canvas: HTMLCanvasElement | null = $state(null);
	let canvasElements: HTMLCanvasElement[] = $state([]);
	let textLayerElement: HTMLDivElement | null = $state(null);
	let textLayerElements: HTMLDivElement[] = $state([]);
	let annotationLayerElement: HTMLDivElement | null = $state(null);
	let annotationLayerElements: HTMLDivElement[] = $state([]);
	let containerElement: HTMLDivElement | null = $state(null);
	let scrollContainer: HTMLDivElement | null = $state(null);
	let renderTasks: RenderTask[] = $state([]);
	let isRendering = $state(false);

	// Local patch (comhairle): reactive signal bumped on container resize so the
	// fit-to-width/page scale recomputes. See vendor/VENDORED.md.
	let resizeTick = $state(0);

	// Local patch (comhairle): user zoom multiplier applied on top of the
	// fit-to-width/page scale, so the +/- buttons work while a fitMode is active.
	let userZoom = $state(1.0);

	// Lazy loading state - removed unused variables

	// Non-reactive variables to prevent circular dependencies
	let lastRenderedScale = 0;
	let lastRenderedPage = 0;
	let pendingRender = false;

	let documentState: PDFDocumentState = $state({
		numPages: 0,
		fingerprint: '',
		isLoading: false,
		isLoaded: false,
		error: null
	});

	let pageInfo: PDFPageInfo = $state({
		pageNumber: 1,
		width: 0,
		height: 0,
		scale: 1.0
	});

	// Derived state
	let totalPages = $derived(documentState.numPages);
	let isLoading = $derived(documentState.isLoading);
	let error = $derived(documentState.error);
	let canGoNext = $derived(page < totalPages);
	let canGoPrev = $derived(page > 1);

	// Cached computed values to prevent constant recalculation
	// Removed validCanvasCount derived - use direct checks where needed

	// Calculate scale based on fit mode - simplified to prevent circular dependencies
	let calculatedScale = $derived.by(() => {
		// Local patch (comhairle): track resizeTick so fit scale recomputes on resize.
		void resizeTick;
		// Local patch (comhairle): track userZoom so fit scale recomputes on zoom.
		// Must be read outside untrack() below to register as a dependency.
		void userZoom;
		// For 'none' fitMode, always return the current scale
		if (fitMode === 'none') return scale;

		// Local patch (comhairle): continuous mode never sets currentPageProxy,
		// so fall back to the first loaded page for the fit calculation.
		const fitPageProxy = currentPageProxy ?? allPageProxies[0];
		// Local patch (comhairle): in continuous mode measure the scroll
		// container — its clientWidth excludes the vertical scrollbar, so pages
		// fit exactly instead of leaving a stray horizontal scrollbar.
		const measureEl = displayMode === 'continuous' ? scrollContainer : containerElement;
		if (!fitPageProxy || !measureEl) return scale;

		try {
			const viewport = fitPageProxy.getViewport({ scale: 1.0 });

			// Use untrack to read DOM properties without creating dependencies
			return untrack(() => {
				const containerWidth = measureEl!.clientWidth;
				const containerHeight = measureEl!.clientHeight;

				// Avoid division by zero
				if (containerWidth <= 0 || containerHeight <= 0) return scale;

				let newScale;
				switch (fitMode) {
					case 'width':
						// Local patch (comhairle): apply userZoom on top of fit scale.
						newScale = (containerWidth / viewport.width) * userZoom;
						break;
					case 'page':
						// Local patch (comhairle): apply userZoom on top of fit scale.
						newScale =
							Math.min(
								containerWidth / viewport.width,
								containerHeight / viewport.height
							) * userZoom;
						break;
					case 'auto':
						newScale = Math.min(
							containerWidth / viewport.width,
							containerHeight / viewport.height,
							scale
						);
						break;
					default:
						newScale = scale;
						break;
				}

				return newScale;
			});
		} catch (err) {
			console.warn(err);
			return scale;
		}
	});

	// Load PDF document
	async function loadPDF(source: PDFSource) {
		try {
			documentState.isLoading = true;
			documentState.error = null;

			let loadingTask;
			if (source instanceof File) {
				const arrayBuffer = await source.arrayBuffer();
				loadingTask = pdfjsLib.getDocument({ data: arrayBuffer });
			} else if (source instanceof ArrayBuffer) {
				loadingTask = pdfjsLib.getDocument({ data: source });
			} else {
				loadingTask = pdfjsLib.getDocument(source);
			}

			document = await loadingTask.promise;

			documentState.numPages = document.numPages;
			documentState.fingerprint = document.fingerprints?.[0] || '';
			documentState.isLoaded = true;
			documentState.isLoading = false;

			onLoad?.({
				numPages: documentState.numPages,
				fingerprint: documentState.fingerprint
			});

			// Load annotations from all pages and call callback
			if (onAnnotationsLoad) {
				await loadAllAnnotations();
			}

			// Load pages based on display mode
			if (displayMode === 'continuous') {
				await loadAllPages();
			} else {
				await loadPage(page);
			}
		} catch (err) {
			const errorMessage = err instanceof Error ? err.message : 'Failed to load PDF';
			documentState.error = errorMessage;
			documentState.isLoading = false;
			onError?.({ error: errorMessage });
		}
	}

	// Load specific page (for single mode)
	async function loadPage(pageNumber: number) {
		if (!document || pageNumber < 1 || pageNumber > documentState.numPages) return;

		try {
			currentPageProxy = await document.getPage(pageNumber);
			pageInfo.pageNumber = pageNumber;

			// Trigger render after page load
			if (displayMode === 'single') {
				triggerRender();
			}
		} catch (err) {
			const errorMessage = err instanceof Error ? err.message : 'Failed to load page';
			documentState.error = errorMessage;
			onError?.({ error: errorMessage });
		}
	}

	// Cancel all pending render tasks
	function cancelAllRenderTasks() {
		renderTasks.forEach((task) => {
			if (task && typeof task.cancel === 'function') {
				try {
					task.cancel();
				} catch (err) {
					console.warn('Error cancelling render task:', err);
				}
			}
		});
		renderTasks = [];
	}

	// Helper functions for render coordination
	function isScaleChanged() {
		return Math.abs(calculatedScale - lastRenderedScale) > 0.001;
	}

	function isPageChanged() {
		return displayMode === 'single' && pageInfo.pageNumber !== lastRenderedPage;
	}

	function isContinuousReady() {
		return (
			displayMode === 'continuous' &&
			allPageProxies.length > 0 &&
			canvasElements.filter(Boolean).length === allPageProxies.length &&
			lastRenderedScale === 0
		);
	}

	function canRenderSinglePage() {
		return displayMode === 'single' && currentPageProxy;
	}

	function canRenderContinuous() {
		return (
			displayMode === 'continuous' &&
			allPageProxies.length > 0 &&
			canvasElements.filter(Boolean).length === allPageProxies.length
		);
	}

	// Simple render trigger - no state mutations to prevent circular dependencies
	function triggerRender() {
		if (isRendering || pendingRender) return;

		pendingRender = true;

		setTimeout(() => {
			pendingRender = false;
			if (isRendering) return;

			// Check what changed
			const scaleChanged = isScaleChanged();
			const pageChanged = isPageChanged();
			const continuousReady = isContinuousReady();

			const shouldRender = scaleChanged || pageChanged || continuousReady;

			if (shouldRender) {
				// Execute the appropriate render method
				if (canRenderSinglePage()) {
					renderPage();
				} else if (canRenderContinuous()) {
					renderAllPages();
				}

				// Update tracking variables
				lastRenderedScale = calculatedScale;
				lastRenderedPage = pageInfo.pageNumber;

				// Notify scale change callback
				if (scaleChanged && onScaleChange) {
					onScaleChange({ scale: calculatedScale });
				}
			}
		}, 16); // ~60fps debounce
	}

	// Comprehensive cleanup function
	function cleanup() {
		// Cancel renders
		cancelAllRenderTasks();
		isRendering = false;

		// Reset non-reactive variables
		pendingRender = false;
		lastRenderedScale = 0;
		lastRenderedPage = 0;

		// Clean up observers
		if (intersectionObserver) {
			intersectionObserver.disconnect();
			intersectionObserver = null;
		}

		// Reset state
		currentPageProxy = null;
		allPageProxies = [];
		canvasElements = [];
		textLayerElements = [];
		annotationLayerElements = [];

		// Reset document state
		documentState.isLoading = false;
		documentState.error = null;
	}

	// Load all pages (for continuous mode) - simplified version
	async function loadAllPages() {
		if (!document) return;

		try {
			// Cancel any existing render tasks
			cancelAllRenderTasks();

			allPageProxies = [];
			canvasElements = [];
			textLayerElements = [];
			annotationLayerElements = [];

			for (let i = 1; i <= documentState.numPages; i++) {
				const pageProxy = await document.getPage(i);
				allPageProxies.push(pageProxy);
			}

			// Wait for DOM to update with new canvas elements - with retry logic
			await waitForCanvasElements();

			triggerRender();
		} catch (err) {
			const errorMessage = err instanceof Error ? err.message : 'Failed to load pages';
			documentState.error = errorMessage;
			onError?.({ error: errorMessage });
		}
	}

	// Wait for DOM to create all canvas elements
	async function waitForCanvasElements() {
		const maxRetries = 10;
		let retries = 0;

		while (retries < maxRetries) {
			const validCanvases = canvasElements.filter(Boolean);

			if (validCanvases.length === allPageProxies.length) {
				return;
			}

			// Wait before retrying
			await new Promise((resolve) => setTimeout(resolve, 50));
			retries++;
		}
	}

	// Render a single page in continuous mode when it comes into view
	async function renderSinglePageInContinuous(pageNumber: number) {
		if (!allPageProxies.length || isRendering) {
			return;
		}

		const pageProxy = allPageProxies[pageNumber - 1];
		const canvas = canvasElements[pageNumber - 1];
		const textLayerDiv = textLayerElements[pageNumber - 1];

		if (!pageProxy || !canvas) {
			return;
		}

		const context = canvas.getContext('2d');
		if (!context) {
			return;
		}

		try {
			const viewport = pageProxy.getViewport({ scale: calculatedScale });

			// Clear the canvas first
			context.clearRect(0, 0, canvas.width, canvas.height);

			canvas.width = viewport.width;
			canvas.height = viewport.height;

			// Set CSS dimensions for proper display in continuous mode
			canvas.style.width = `${viewport.width}px`;
			canvas.style.height = `${viewport.height}px`;

			const renderContext = {
				canvasContext: context,
				viewport: viewport
			};

			await pageProxy.render(renderContext).promise;

			// Render text layer if enabled
			if (enableTextSelection && textLayerDiv) {
				await renderTextLayer(pageProxy, textLayerDiv, canvas);
			}

			// Render annotation layer
			const annotationLayerDiv = annotationLayerElements[pageNumber - 1];
			if (annotationLayerDiv) {
				await renderAnnotationLayer(pageProxy, annotationLayerDiv, canvas);
			}
		} catch (err) {
			console.error(err);
		}
	}

	// Render page to canvas (for single mode) - pure function, no reactive state mutations
	async function renderPage() {
		if (!currentPageProxy || !canvas || isRendering) return;

		const context = canvas.getContext('2d');
		if (!context) return;

		try {
			isRendering = true;

			const viewport = currentPageProxy.getViewport({ scale: calculatedScale });

			// Only update dimensions, not scale (to prevent circular dependency)
			pageInfo.width = viewport.width;
			pageInfo.height = viewport.height;
			// DO NOT update pageInfo.scale here - it causes circular dependency

			// Round viewport dimensions to integers to ensure canvas buffer and display align perfectly
			const roundedWidth = Math.round(viewport.width);
			const roundedHeight = Math.round(viewport.height);

			canvas.width = roundedWidth;
			canvas.height = roundedHeight;

			// Set CSS dimensions to match the rounded buffer size exactly
			const canvasWidth = width || roundedWidth;
			const canvasHeight = height || roundedHeight;
			canvas.style.width = `${canvasWidth}px`;
			canvas.style.height = `${canvasHeight}px`;

			const renderContext = {
				canvasContext: context,
				viewport: viewport
			};

			await currentPageProxy.render(renderContext).promise;

			// Always render text layer if text selection is enabled and text layer element exists
			if (enableTextSelection && textLayerElement) {
				await renderTextLayer(currentPageProxy, textLayerElement, canvas);
			}

			// Always render annotation layer if annotation layer element exists
			if (annotationLayerElement) {
				await renderAnnotationLayer(currentPageProxy, annotationLayerElement, canvas);
			}

			isRendering = false;
		} catch (err) {
			const errorMessage = err instanceof Error ? err.message : 'Failed to render page';
			documentState.error = errorMessage;
			onError?.({ error: errorMessage });
			isRendering = false;
		}
	}

	// Render all pages to canvases (for continuous mode)
	async function renderAllPages() {
		if (!allPageProxies.length || isRendering) {
			return;
		}

		// Cancel any existing render tasks first
		cancelAllRenderTasks();

		try {
			isRendering = true;

			// Check that all canvas elements are bound and ready
			const validCanvases = canvasElements.filter(Boolean);

			if (validCanvases.length !== allPageProxies.length) {
				isRendering = false;

				// Retry after a short delay
				setTimeout(() => {
					renderAllPages();
				}, 100);
				return;
			}

			// Mark canvases as initialized
			canvasElements.forEach((canvas) => {
				if (canvas && canvas.dataset.initialized === 'false') {
					canvas.dataset.initialized = 'true';
				}
			});

			const renderPromises = [];

			for (let i = 0; i < allPageProxies.length; i++) {
				const pageProxy = allPageProxies[i];
				const canvas = canvasElements[i];

				if (!pageProxy || !canvas) {
					continue;
				}

				const context = canvas.getContext('2d');
				if (!context) {
					continue;
				}

				const viewport = pageProxy.getViewport({ scale: calculatedScale });

				// Clear the canvas first
				context.clearRect(0, 0, canvas.width, canvas.height);

				canvas.width = viewport.width;
				canvas.height = viewport.height;

				// Set CSS dimensions for proper display in continuous mode
				canvas.style.width = `${viewport.width}px`;
				canvas.style.height = `${viewport.height}px`;

				const renderContext = {
					canvasContext: context,
					viewport: viewport
				};

				// Start the render task and store it
				const renderTask = pageProxy.render(renderContext);
				renderTasks.push(renderTask);

				renderPromises.push(renderTask.promise);
			}

			// Wait for all renders to complete
			await Promise.all(renderPromises);

			// Always render text layers if text selection is enabled (handles scale changes)
			if (enableTextSelection) {
				const textLayerPromises = [];
				for (let i = 0; i < allPageProxies.length; i++) {
					const pageProxy = allPageProxies[i];
					const textLayerDiv = textLayerElements[i];
					const canvas = canvasElements[i];

					if (pageProxy && textLayerDiv && canvas) {
						textLayerPromises.push(renderTextLayer(pageProxy, textLayerDiv, canvas));
					}
				}

				if (textLayerPromises.length > 0) {
					await Promise.all(textLayerPromises);
				}
			}

			// Always render annotation layers
			const annotationLayerPromises = [];
			for (let i = 0; i < allPageProxies.length; i++) {
				const pageProxy = allPageProxies[i];
				const annotationLayerDiv = annotationLayerElements[i];
				const canvas = canvasElements[i];

				if (pageProxy && annotationLayerDiv && canvas) {
					annotationLayerPromises.push(
						renderAnnotationLayer(pageProxy, annotationLayerDiv, canvas)
					);
				}
			}

			if (annotationLayerPromises.length > 0) {
				await Promise.all(annotationLayerPromises);
			}

			// Clear completed tasks and reset state
			renderTasks = [];
			isRendering = false;
		} catch (err) {
			const errorMessage = err instanceof Error ? err.message : 'Failed to render pages';
			documentState.error = errorMessage;
			onError?.({ error: errorMessage });
			isRendering = false;
		}
	}

	// Render text layer for a specific page
	async function renderTextLayer(
		pageProxy: PDFPageProxy,
		textLayerDiv: HTMLDivElement,
		canvasElement: HTMLCanvasElement
	) {
		try {
			// Clear existing text layer content
			textLayerDiv.innerHTML = '';

			// Create a scaled viewport that matches the current display scale
			// This ensures text layer positioning matches the canvas exactly
			const scaledViewport = pageProxy.getViewport({ scale: calculatedScale });

			// Use canvas buffer dimensions to ensure perfect alignment
			const canvasBufferWidth = canvasElement.width;
			const canvasBufferHeight = canvasElement.height;

			// Set text layer to PDF viewport dimensions initially
			textLayerDiv.style.width = `${scaledViewport.width}px`;
			textLayerDiv.style.height = `${scaledViewport.height}px`;

			// Set PDF.js scale factor CSS variable for proper text scaling
			textLayerDiv.style.setProperty('--scale-factor', calculatedScale.toString());

			// Calculate transform to scale text layer to match canvas buffer exactly
			const scaleX = canvasBufferWidth / scaledViewport.width;
			const scaleY = canvasBufferHeight / scaledViewport.height;
			textLayerDiv.style.transform = `scale(${scaleX}, ${scaleY})`;
			textLayerDiv.style.transformOrigin = '0 0'; // Scale from top-left corner

			// Get text content from PDF
			const textContent = await pageProxy.getTextContent();

			// Render text layer using PDF.js with the natural viewport (CSS transform handles scaling)
			const textLayer = new TextLayer({
				textContentSource: textContent,
				container: textLayerDiv,
				viewport: scaledViewport
			});

			// Wait for text layer to be ready
			await textLayer.render();

			// Ensure text layer is selectable and has proper cursor
			textLayerDiv.style.userSelect = 'text';
			textLayerDiv.style.pointerEvents = 'auto';
			textLayerDiv.style.cursor = 'text';

			// Let PDF.js handle all text layer styling - don't override

			// Handle canvas pointer events for text selection
			if (enableTextSelection) {
				canvasElement.style.pointerEvents = 'none';
			} else {
				canvasElement.style.pointerEvents = 'auto';
			}
		} catch (err) {
			console.error(err);
		}
	}

	// Render annotation layer for a specific page
	async function renderAnnotationLayer(
		pageProxy: PDFPageProxy,
		annotationLayerDiv: HTMLDivElement,
		canvasElement: HTMLCanvasElement
	) {
		try {
			// Clear existing annotation layer content
			annotationLayerDiv.innerHTML = '';

			// Get annotations from the page
			const annotations = await pageProxy.getAnnotations();
			if (!annotations || annotations.length === 0) {
				return;
			}

			// Create a scaled viewport that matches the current display scale
			const scaledViewport = pageProxy.getViewport({ scale: calculatedScale });

			// Use canvas buffer dimensions to ensure perfect alignment
			const canvasBufferWidth = canvasElement.width;
			const canvasBufferHeight = canvasElement.height;

			// Set annotation layer to PDF viewport dimensions initially
			annotationLayerDiv.style.width = `${scaledViewport.width}px`;
			annotationLayerDiv.style.height = `${scaledViewport.height}px`;

			// Calculate transform to scale annotation layer to match canvas buffer exactly
			const scaleX = canvasBufferWidth / scaledViewport.width;
			const scaleY = canvasBufferHeight / scaledViewport.height;
			annotationLayerDiv.style.transform = `scale(${scaleX}, ${scaleY})`;
			annotationLayerDiv.style.transformOrigin = '0 0'; // Scale from top-left corner

			// Process each annotation
			annotations.forEach((annotation: PDFAnnotation, index: number) => {
				const annotationElement = createAnnotationElement(
					annotation,
					scaledViewport,
					index
				);
				if (annotationElement) {
					annotationLayerDiv.appendChild(annotationElement);
				}
			});

			// Ensure annotation layer has proper stacking
			annotationLayerDiv.style.pointerEvents = 'auto';
		} catch (err) {
			console.error('Error rendering annotation layer:', err);
		}
	}

	// Create DOM element for a specific annotation
	function createAnnotationElement(
		annotation: PDFAnnotation,
		viewport: any,
		index: number
	): HTMLElement | null {
		if (!annotation.rect || annotation.rect.length !== 4) {
			return null;
		}

		const [x1, y1, x2, y2] = annotation.rect;

		// Transform annotation coordinates to viewport coordinates
		const rect = viewport.convertToViewportRectangle([x1, y1, x2, y2]);
		const left = Math.min(rect[0], rect[2]);
		const top = Math.min(rect[1], rect[3]);
		const width = Math.abs(rect[2] - rect[0]);
		const height = Math.abs(rect[3] - rect[1]);

		// Create annotation element
		const element = globalThis.document.createElement('div');
		element.className = `pdf-annotation pdf-annotation-${annotation.subtype?.toLowerCase() || 'unknown'}`;
		element.style.position = 'absolute';
		element.style.left = `${left}px`;
		element.style.top = `${top}px`;
		element.style.width = `${width}px`;
		element.style.height = `${height}px`;
		element.style.cursor = 'pointer';
		element.style.boxSizing = 'border-box';

		// Set annotation data attributes
		element.setAttribute('data-annotation-id', annotation.id);
		element.setAttribute('data-annotation-type', annotation.subtype || 'unknown');
		element.setAttribute('data-annotation-index', index.toString());

		// Handle different annotation types
		switch (annotation.subtype) {
			case 'Link':
				element.style.backgroundColor = 'transparent';
				element.style.border = '1px solid transparent';
				element.style.borderRadius = '2px';
				element.title = annotation.url || 'Link';

				// Add hover effect for links
				element.addEventListener('mouseenter', () => {
					element.style.backgroundColor = 'rgba(0, 123, 255, 0.1)';
					element.style.border = '1px solid rgba(0, 123, 255, 0.3)';
				});
				element.addEventListener('mouseleave', () => {
					element.style.backgroundColor = 'transparent';
					element.style.border = '1px solid transparent';
				});
				break;

			case 'Text':
			case 'Note':
				element.style.backgroundColor = 'rgba(255, 255, 0, 0.3)';
				element.style.border = '1px solid rgba(255, 255, 0, 0.8)';
				element.style.borderRadius = '3px';
				if (annotation.contents) {
					element.title = annotation.contents;
				}
				break;

			case 'Highlight':
				element.style.backgroundColor = 'rgba(255, 255, 0, 0.4)';
				element.style.border = 'none';
				element.style.borderRadius = '2px';
				if (annotation.contents) {
					element.title = annotation.contents;
				}
				break;

			case 'Underline':
				element.style.backgroundColor = 'transparent';
				element.style.borderBottom = '2px solid rgba(255, 0, 0, 0.8)';
				element.style.height = '2px';
				element.style.top = `${top + height - 2}px`;
				break;

			case 'StrikeOut':
				element.style.backgroundColor = 'transparent';
				element.style.borderTop = '2px solid rgba(255, 0, 0, 0.8)';
				element.style.height = '2px';
				element.style.top = `${top + height / 2}px`;
				break;

			default:
				// Generic annotation styling
				element.style.backgroundColor = 'rgba(200, 200, 200, 0.3)';
				element.style.border = '1px solid rgba(200, 200, 200, 0.6)';
				element.style.borderRadius = '2px';
				break;
		}

		// Add click handler
		element.addEventListener('click', (event: Event) => {
			handleAnnotationClick(annotation, element, event);
		});

		// Add hover handler for tooltip-like behavior
		element.addEventListener('mouseenter', (event: Event) => {
			handleAnnotationHover(annotation, element, event);
		});

		return element;
	}

	// Handle annotation click events
	function handleAnnotationClick(annotation: PDFAnnotation, element: HTMLElement, event: Event) {
		event.preventDefault();
		event.stopPropagation();

		const pageNumber = parseInt(
			element.closest('[data-page]')?.getAttribute('data-page') || '1',
			10
		);

		// Handle link annotations
		if (annotation.subtype === 'Link' && annotation.url) {
			// External URL
			if (annotation.url.startsWith('http://') || annotation.url.startsWith('https://')) {
				window.open(annotation.url, '_blank', 'noopener,noreferrer');
			} else if (annotation.url.startsWith('mailto:')) {
				window.location.href = annotation.url;
			}
		}

		// Call user-defined annotation click handler
		if (annotationOptions.onAnnotationClick) {
			const interactionEvent: AnnotationInteractionEvent = {
				annotation,
				pageNumber,
				target: element,
				originalEvent: event
			};
			annotationOptions.onAnnotationClick(interactionEvent);
		}
	}

	// Handle annotation hover events
	function handleAnnotationHover(annotation: PDFAnnotation, element: HTMLElement, event: Event) {
		const pageNumber = parseInt(
			element.closest('[data-page]')?.getAttribute('data-page') || '1',
			10
		);

		// Call user-defined annotation hover handler
		if (annotationOptions.onAnnotationHover) {
			const interactionEvent: AnnotationInteractionEvent = {
				annotation,
				pageNumber,
				target: element,
				originalEvent: event
			};
			annotationOptions.onAnnotationHover(interactionEvent);
		}
	}

	// Load all annotations from the document
	async function loadAllAnnotations() {
		if (!document || !onAnnotationsLoad) return;

		try {
			const annotationsByPage: PDFAnnotationSummary[] = [];
			const annotationsByType: Record<string, PDFAnnotation[]> = {};
			let totalAnnotations = 0;

			// Load annotations from each page
			for (let pageNum = 1; pageNum <= documentState.numPages; pageNum++) {
				const pageProxy = await document.getPage(pageNum);
				const annotations = await pageProxy.getAnnotations();

				if (annotations && annotations.length > 0) {
					// Process annotations and add page number
					const processedAnnotations = annotations.map((annotation: PDFAnnotation) => ({
						...annotation,
						page: pageNum
					}));

					annotationsByPage.push({
						pageNumber: pageNum,
						annotations: processedAnnotations
					});

					// Group by type
					processedAnnotations.forEach((annotation: PDFAnnotation) => {
						const type = annotation.subtype || 'unknown';
						if (!annotationsByType[type]) {
							annotationsByType[type] = [];
						}
						annotationsByType[type].push(annotation);
						totalAnnotations++;
					});
				}
			}

			// Call the callback with annotation summary
			onAnnotationsLoad({
				totalAnnotations,
				annotationsByPage,
				annotationsByType
			});
		} catch (err) {
			console.error('Error loading annotations:', err);
		}
	}

	// Navigation functions
	function nextPage() {
		if (canGoNext) {
			const newPage = Math.min(page + 1, totalPages);
			goToPage(newPage);
		}
	}

	function prevPage() {
		if (canGoPrev) {
			const newPage = Math.max(page - 1, 1);
			goToPage(newPage);
		}
	}

	function goToPage(pageNumber: number) {
		if (pageNumber >= 1 && pageNumber <= totalPages) {
			page = pageNumber;

			if (displayMode === 'continuous' && scrollContainer) {
				// Scroll to the target page in continuous mode
				scrollToPage(pageNumber);
			}
		}
	}

	// Scroll to specific page in continuous mode
	function scrollToPage(pageNumber: number) {
		if (!scrollContainer || displayMode !== 'continuous') return;

		const targetCanvas = canvasElements[pageNumber - 1];
		if (targetCanvas) {
			targetCanvas.scrollIntoView({
				behavior: 'smooth',
				block: 'start'
			});
		}
	}

	// Intersection Observer for tracking visible pages in continuous mode
	let intersectionObserver: IntersectionObserver | null = null;

	function setupIntersectionObserver() {
		if (!scrollContainer || displayMode !== 'continuous') {
			return;
		}

		// Clean up existing observer
		if (intersectionObserver) {
			intersectionObserver.disconnect();
		}

		intersectionObserver = new IntersectionObserver(
			(entries) => {
				// Find the entry with the largest intersection ratio
				let mostVisibleEntry: IntersectionObserverEntry | null = null;
				let maxRatio = 0;

				entries.forEach((entry) => {
					// Check if this page needs rendering when it becomes visible
					if (entry.intersectionRatio > 0.1) {
						const canvas = entry.target as HTMLCanvasElement;
						const pageNumber = parseInt(canvas.dataset.page || '1', 10);

						// Check if canvas is unrendered (still default size)
						if (canvas.width === 300 && canvas.height === 150) {
							renderSinglePageInContinuous(pageNumber);
						}
					}

					if (entry.intersectionRatio > maxRatio) {
						maxRatio = entry.intersectionRatio;
						mostVisibleEntry = entry;
					}
				});

				if (
					mostVisibleEntry &&
					(mostVisibleEntry as IntersectionObserverEntry).intersectionRatio > 0.3
				) {
					const canvas = (mostVisibleEntry as IntersectionObserverEntry)
						.target as HTMLCanvasElement;
					const pageNumber = parseInt(canvas.dataset.page || '1', 10);

					if (page !== pageNumber) {
						page = pageNumber;
						onPageChange?.({ page, total: totalPages });
					}
				}
			},
			{
				root: scrollContainer,
				rootMargin: '-20% 0px -20% 0px', // Only consider pages that are well within view
				threshold: [0.1, 0.3, 0.5, 0.7, 0.9]
			}
		);

		// Observe all canvas elements
		const validCanvases = canvasElements.filter(Boolean);

		validCanvases.forEach((canvas) => {
			if (canvas) {
				intersectionObserver?.observe(canvas);
			}
		});
	}

	// Zoom functions
	// Local patch (comhairle): in fit modes (width/page) the rendered scale is
	// derived from the container, not `scale`, so +/- adjust the userZoom
	// multiplier instead. In 'none' mode they adjust the absolute `scale`.
	function zoomIn() {
		if (fitMode === 'none') {
			scale = Math.min(scale * 1.2, maxScale);
		} else {
			userZoom = Math.min(userZoom * 1.2, maxScale);
			triggerRender();
		}
	}

	function zoomOut() {
		if (fitMode === 'none') {
			scale = Math.max(scale / 1.2, minScale);
		} else {
			userZoom = Math.max(userZoom / 1.2, minScale);
			triggerRender();
		}
	}

	function resetZoom() {
		if (fitMode === 'none') {
			scale = 1.0;
		} else {
			userZoom = 1.0;
			triggerRender();
		}
	}

	// Throttled render functions replaced by centralized requestRender coordination

	// Effects
	$effect(() => {
		if (src) {
			// Use untrack to prevent reactive dependencies from cleanup() and loadPDF()
			untrack(() => {
				cleanup();
				loadPDF(src);
			});
		}

		// Cleanup when component unmounts or src changes
		return () => {
			untrack(() => {
				cleanup();
			});
		};
	});

	$effect(() => {
		if (document && documentState.isLoaded && displayMode === 'single' && !isRendering) {
			// Always load the page when it changes, regardless of pageInfo.pageNumber
			loadPage(page);
			onPageChange?.({ page, total: totalPages });
		}
	});

	// Effect to trigger render when scale or fitMode changes
	$effect(() => {
		if (scale && fitMode && (!document || !documentState.isLoaded)) return;

		triggerRender();
	});

	// Handle display mode changes - track displayMode separately to prevent loops
	let previousDisplayMode = displayMode;
	$effect(() => {
		if (
			document &&
			documentState.isLoaded &&
			!isRendering &&
			displayMode !== previousDisplayMode
		) {
			// Cancel any pending renders when mode changes
			cancelAllRenderTasks();

			// Reset to page 1 and default zoom when switching modes
			page = 1;
			scale = 1.0;
			// Reset lastRenderedScale to 0 to trigger re-rendering in new mode
			lastRenderedScale = 0;
			lastRenderedPage = 0;

			if (displayMode === 'continuous') {
				loadAllPages();
			} else {
				loadPage(page);
			}

			// Update the previous mode to prevent loops
			previousDisplayMode = displayMode;
		}

		// Cleanup when effect re-runs
		return () => {
			cancelAllRenderTasks();
		};
	});

	// Setup Intersection Observer for continuous mode
	$effect(() => {
		if (displayMode === 'continuous' && allPageProxies.length > 0 && !isRendering) {
			// Wait a bit for DOM to settle and rendering to complete
			setTimeout(() => {
				if (!isRendering && displayMode === 'continuous') {
					setupIntersectionObserver();
				}
			}, 500);
		}

		return () => {
			if (intersectionObserver) {
				intersectionObserver.disconnect();
				intersectionObserver = null;
			}
		};
	});

	// Reactive container resize
	$effect(() => {
		if (!containerElement) return;

		const resizeObserver = new ResizeObserver(() => {
			if (fitMode !== 'none') {
				// Local patch (comhairle): bump resizeTick so calculatedScale recomputes.
				resizeTick++;
				if (!isRendering) triggerRender();
			}
		});

		resizeObserver.observe(containerElement);

		return () => {
			resizeObserver.disconnect();
		};
	});
</script>

<div
	bind:this={containerElement}
	class="pdf-viewer {theme} {containerClass}"
	class:loading={isLoading}
	class:error={!!error}
>
	{#if showNavigation}
		<!-- Local patch (comhairle): compact icon-button toolbar; the two groups
		     wrap onto a second row on narrow screens (see PDFViewer.css). -->
		<div class="pdf-navigation">
			<div class="pdf-nav-group">
				<Button
					variant="outline"
					size="sm"
					class="pdf-page-button"
					aria-label="Previous page"
					title="Previous page"
					onclick={prevPage}
					disabled={!canGoPrev || isLoading}
				>
					<ChevronLeftIcon class="size-4" />
				</Button>

				<span class="page-info">
					Page {page} of {totalPages}
				</span>

				<Button
					variant="outline"
					size="sm"
					class="pdf-page-button"
					aria-label="Next page"
					title="Next page"
					onclick={nextPage}
					disabled={!canGoNext || isLoading}
				>
					<ChevronRightIcon class="size-4" />
				</Button>
			</div>

			<div class="pdf-nav-group">
				<Button
					variant="outline"
					size="sm"
					aria-label="Zoom out"
					title="Zoom out"
					onclick={zoomOut}
					disabled={(fitMode === 'none' ? scale : userZoom) <= minScale}
				>
					<MinusIcon class="size-4" />
				</Button>
				<span class="zoom-level">{Math.round(calculatedScale * 100)}%</span>
				<Button
					variant="outline"
					size="sm"
					aria-label="Zoom in"
					title="Zoom in"
					onclick={zoomIn}
					disabled={(fitMode === 'none' ? scale : userZoom) >= maxScale}
				>
					<PlusIcon class="size-4" />
				</Button>
			</div>
		</div>
	{/if}

	<div class="pdf-content" class:continuous-mode={displayMode === 'continuous'}>
		{#if isLoading}
			<div class="loading-state">
				<div class="loading-spinner"></div>
				<p>Loading PDF...</p>
			</div>
		{:else if error}
			<div class="error-state">
				<p class="error-message">Error: {error}</p>
			</div>
		{:else if document}
			{#if displayMode === 'continuous'}
				<div bind:this={scrollContainer} class="scroll-container">
					{#each allPageProxies as pageProxy, index (pageProxy.pageNumber)}
						<div class="page-container">
							<div class="page-content">
								<canvas
									bind:this={canvasElements[index]}
									class="pdf-canvas {canvasClass}"
									data-page={index + 1}
									data-initialized="false"
									use:textSelection={{
										...textSelectionOptions,
										enabled: enableTextSelection,
										onSelect: onTextSelect
									}}
									use:zoom={zoomOptions}
									use:navigation={navigationOptions}
									use:annotation={annotationOptions}
								></canvas>
								<div
									bind:this={textLayerElements[index]}
									class="textLayer"
									data-page={index + 1}
								></div>
								<div
									bind:this={annotationLayerElements[index]}
									class="annotationLayer"
									data-page={index + 1}
								></div>
							</div>
						</div>
					{/each}
				</div>
			{:else}
				<div class="page-content">
					<canvas
						bind:this={canvas}
						class="pdf-canvas {canvasClass}"
						use:textSelection={{
							...textSelectionOptions,
							enabled: enableTextSelection,
							onSelect: onTextSelect
						}}
						use:zoom={zoomOptions}
						use:navigation={navigationOptions}
						use:annotation={annotationOptions}
					></canvas>
					<div bind:this={textLayerElement} class="textLayer" data-page={page}></div>
					<div
						bind:this={annotationLayerElement}
						class="annotationLayer"
						data-page={page}
					></div>
				</div>
			{/if}
		{/if}
	</div>
</div>
