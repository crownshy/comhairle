import * as pdfjsLib from 'pdfjs-dist';
import type { PDFDocumentProxy, PDFPageProxy } from 'pdfjs-dist';
import type { PDFSource, PDFPageInfo } from './types.js';

/**
 * Load a PDF document from various sources
 */
export async function loadPDFDocument(source: PDFSource): Promise<PDFDocumentProxy> {
	let loadingTask;

	if (source instanceof File) {
		const arrayBuffer = await source.arrayBuffer();
		loadingTask = pdfjsLib.getDocument({ data: arrayBuffer });
	} else if (source instanceof ArrayBuffer) {
		loadingTask = pdfjsLib.getDocument({ data: source });
	} else {
		loadingTask = pdfjsLib.getDocument(source);
	}

	return await loadingTask.promise;
}

/**
 * Get page information including dimensions and scale
 */
export function getPageInfo(page: PDFPageProxy, scale: number): PDFPageInfo {
	const viewport = page.getViewport({ scale });

	return {
		pageNumber: page.pageNumber,
		width: viewport.width,
		height: viewport.height,
		scale
	};
}

/**
 * Calculate optimal scale for fit modes
 */
export function calculateFitScale(
	page: PDFPageProxy,
	containerWidth: number,
	containerHeight: number,
	mode: 'width' | 'page' | 'auto',
	currentScale: number = 1.0
): number {
	const viewport = page.getViewport({ scale: 1.0 });

	switch (mode) {
		case 'width':
			return containerWidth / viewport.width;
		case 'page':
			return Math.min(containerWidth / viewport.width, containerHeight / viewport.height);
		case 'auto':
			return Math.min(
				containerWidth / viewport.width,
				containerHeight / viewport.height,
				currentScale
			);
		default:
			return currentScale;
	}
}

/**
 * Render a PDF page to a canvas
 */
export async function renderPageToCanvas(
	page: PDFPageProxy,
	canvas: HTMLCanvasElement,
	scale: number
): Promise<void> {
	const context = canvas.getContext('2d');
	if (!context) {
		throw new Error('Unable to get canvas context');
	}

	const viewport = page.getViewport({ scale });
	canvas.width = viewport.width;
	canvas.height = viewport.height;

	const renderContext = {
		canvasContext: context,
		viewport: viewport
	};

	await page.render(renderContext).promise;
}

/**
 * Extract text content from a PDF page
 */
export async function extractTextFromPage(page: PDFPageProxy): Promise<string> {
	const textContent = await page.getTextContent();
	return textContent.items
		.filter((item): item is any => 'str' in item)
		.map((item) => item.str)
		.join(' ');
}

/**
 * Get text selection coordinates on a page
 */
export function getTextSelectionCoordinates(
	page: PDFPageProxy,
	selectionStart: number,
	selectionEnd: number,
	scale: number
): Promise<{ x: number; y: number; width: number; height: number }> {
	// This is a simplified implementation
	// In a real implementation, you'd need to map text positions to coordinates
	return Promise.resolve({
		x: 0,
		y: 0,
		width: 0,
		height: 0
	});
}

/**
 * Convert PDF coordinates to canvas coordinates
 */
export function pdfToCanvasCoordinates(
	x: number,
	y: number,
	viewport: any
): { x: number; y: number } {
	const [canvasX, canvasY] = viewport.convertToViewportPoint(x, y);
	return { x: canvasX, y: canvasY };
}

/**
 * Convert canvas coordinates to PDF coordinates
 */
export function canvasToPDFCoordinates(
	x: number,
	y: number,
	viewport: any
): { x: number; y: number } {
	const [pdfX, pdfY] = viewport.convertToPdfPoint(x, y);
	return { x: pdfX, y: pdfY };
}

/**
 * Clamp a value between min and max
 */
export function clamp(value: number, min: number, max: number): number {
	return Math.min(Math.max(value, min), max);
}

/**
 * Debounce function for performance optimization
 */
export function debounce<T extends (...args: any[]) => any>(
	func: T,
	wait: number
): (...args: Parameters<T>) => void {
	let timeout: number;
	return (...args: Parameters<T>) => {
		clearTimeout(timeout);
		timeout = setTimeout(() => func(...args), wait);
	};
}

/**
 * Throttle function for performance optimization
 */
export function throttle<T extends (...args: any[]) => any>(
	func: T,
	wait: number
): (...args: Parameters<T>) => void {
	let inThrottle: boolean;
	return (...args: Parameters<T>) => {
		if (!inThrottle) {
			func(...args);
			inThrottle = true;
			setTimeout(() => (inThrottle = false), wait);
		}
	};
}

/**
 * Check if a PDF source is valid
 */
export function isValidPDFSource(source: any): source is PDFSource {
	return typeof source === 'string' || source instanceof File || source instanceof ArrayBuffer;
}

/**
 * Format file size for display
 */
export function formatFileSize(bytes: number): string {
	if (bytes === 0) return '0 B';

	const k = 1024;
	const sizes = ['B', 'KB', 'MB', 'GB'];
	const i = Math.floor(Math.log(bytes) / Math.log(k));

	return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

/**
 * Get file info from various sources
 */
export async function getFileInfo(source: PDFSource): Promise<{
	name?: string;
	size?: number;
	type?: string;
}> {
	if (source instanceof File) {
		return {
			name: source.name,
			size: source.size,
			type: source.type
		};
	} else if (source instanceof ArrayBuffer) {
		return {
			size: source.byteLength,
			type: 'application/pdf'
		};
	} else {
		// URL string
		try {
			const url = new URL(source);
			return {
				name: url.pathname.split('/').pop() || 'document.pdf',
				type: 'application/pdf'
			};
		} catch {
			return {};
		}
	}
}
