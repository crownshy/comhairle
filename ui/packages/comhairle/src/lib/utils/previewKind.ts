/**
 * Picks which in-app document viewer (see `PdfDocumentDialog`) should render a file, based on
 * its name. Used wherever a source document is opened in-page rather than downloaded: the
 * Learning Assistant's cited sources and the rich-text source-document badges.
 */

/** How a document should be previewed in the shared viewer, keyed off its filename. */
export type PreviewKind = 'pdf' | 'image' | 'docx' | 'text';

const IMAGE_EXTENSIONS = ['.jpg', '.jpeg', '.png', '.gif', '.webp', '.bmp', '.svg', '.avif'];
const DOCX_EXTENSIONS = ['.doc', '.docx'];
const TEXT_EXTENSIONS = ['.md', '.markdown', '.txt'];

/**
 * Map a filename to the viewer that should render it, or `null` when we have no
 * in-page viewer for that type (the caller should fall back to a plain download).
 * Callers that always want a viewer (e.g. RAGFlow source chunks, which are only
 * ever PDFs or uploaded docs) can default an unknown type with `?? 'pdf'`.
 */
export function getPreviewKind(fileName: string): PreviewKind | null {
	const lower = fileName.toLowerCase();
	if (lower.endsWith('.pdf')) return 'pdf';
	if (IMAGE_EXTENSIONS.some((ext) => lower.endsWith(ext))) return 'image';
	if (DOCX_EXTENSIONS.some((ext) => lower.endsWith(ext))) return 'docx';
	if (TEXT_EXTENSIONS.some((ext) => lower.endsWith(ext))) return 'text';
	return null;
}
