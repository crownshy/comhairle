export { default as PDFViewer } from './PDFViewer.svelte';

export type {
	PDFSource,
	PDFPageInfo,
	PDFDocumentState,
	DisplayMode,
	FitMode,
	Theme,
	TextSelection,
	PDFViewerProps,
	TextSelectionActionOptions,
	ZoomActionOptions,
	NavigationActionOptions,
	AnnotationActionOptions
} from './types.js';

export { textSelection, zoom, navigation, annotation } from './actions.js';

export {
	loadPDFDocument,
	getPageInfo,
	calculateFitScale,
	renderPageToCanvas,
	extractTextFromPage,
	getTextSelectionCoordinates,
	pdfToCanvasCoordinates,
	canvasToPDFCoordinates,
	clamp,
	debounce,
	throttle,
	isValidPDFSource,
	formatFileSize,
	getFileInfo
} from './utils.js';
