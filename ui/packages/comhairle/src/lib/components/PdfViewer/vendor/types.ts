export type PDFSource = string | File | ArrayBuffer;

export interface PDFPageInfo {
	pageNumber: number;
	width: number;
	height: number;
	scale: number;
}

export interface PDFDocumentState {
	numPages: number;
	fingerprint: string;
	isLoading: boolean;
	isLoaded: boolean;
	error: string | null;
}

export type DisplayMode = 'single' | 'continuous';
export type FitMode = 'none' | 'width' | 'page' | 'auto';
export type Theme = 'light' | 'dark';

export interface TextSelection {
	text: string;
	pageNumber: number;
	x: number;
	y: number;
	width: number;
	height: number;
}

export interface PDFViewerProps {
	src: PDFSource;
	page?: number;
	scale?: number;
	fitMode?: FitMode;
	displayMode?: DisplayMode;
	showNavigation?: boolean;
	enableTextSelection?: boolean;
	theme?: Theme;
	loading?: boolean;
	error?: string | null;
	containerClass?: string;
	canvasClass?: string;
	width?: number;
	height?: number;
	maxScale?: number;
	minScale?: number;
	// Callback props to replace deprecated createEventDispatcher
	onLoad?: (event: { numPages: number; fingerprint: string }) => void;
	onError?: (event: { error: string }) => void;
	onPageChange?: (event: { page: number; total: number }) => void;
	onScaleChange?: (event: { scale: number }) => void;
	onTextSelect?: (selection: TextSelection) => void;
}

// Deprecated: PDFViewerEvents interface removed in favor of callback props
// Use callback props (onLoad, onError, onPageChange, etc.) instead

export interface TextSelectionActionOptions {
	onSelect?: (selection: TextSelection) => void;
	enabled?: boolean;
}

export interface ZoomActionOptions {
	onZoom?: (scale: number) => void;
	minScale?: number;
	maxScale?: number;
	step?: number;
}

export interface NavigationActionOptions {
	onPageChange?: (page: number) => void;
	enableKeyboard?: boolean;
	enableMouseWheel?: boolean;
}

// PDF.js annotation interfaces
export interface PDFAnnotation {
	id: string;
	annotationType: number;
	rect: [number, number, number, number];
	subtype: string;
	title?: string;
	contents?: string;
	url?: string;
	dest?: any;
	hasPopup?: boolean;
	color?: [number, number, number] | null;
	borderStyle?: {
		width: number;
		style: number;
		dashArray: number[];
	};
	quadPoints?: number[];
	page?: number;
	// Additional properties for different annotation types
	[key: string]: any;
}

export interface AnnotationInteractionEvent {
	annotation: PDFAnnotation;
	pageNumber: number;
	target: HTMLElement;
	originalEvent: Event;
}

export interface PDFAnnotationSummary {
	pageNumber: number;
	annotations: PDFAnnotation[];
}

export interface AnnotationsLoadEvent {
	totalAnnotations: number;
	annotationsByPage: PDFAnnotationSummary[];
	annotationsByType: Record<string, PDFAnnotation[]>;
}

export interface AnnotationActionOptions {
	onAnnotationClick?: (event: AnnotationInteractionEvent) => void;
	onAnnotationHover?: (event: AnnotationInteractionEvent) => void;
	enableLinks?: boolean;
	enableTextAnnotations?: boolean;
	enableHighlights?: boolean;
	highlightOpacity?: number;
	linkColor?: string;
	onAnnotate?: (annotation: any) => void;
	color?: string;
	thickness?: number;
}
