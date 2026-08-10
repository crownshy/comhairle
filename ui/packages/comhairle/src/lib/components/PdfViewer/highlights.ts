/**
 * A highlight rectangle over a PDF page, expressed in PDF point coordinates
 * (1pt units, origin top-left). `page` is 1-based to match the viewer's page
 * numbering.
 */
export type PdfHighlight = {
	page: number;
	left: number;
	top: number;
	width: number;
	height: number;
};

/**
 * Convert RAGFlow chunk `positions` into highlight rectangles.
 *
 * Each entry is `[page, x0, x1, top, bottom]` in PDF points with a top-left
 * origin, and corresponds to one line of the retrieved passage. Pages are
 * 1-based, matching {@link PdfHighlight.page} and the viewer. Malformed rows
 * (fewer than 5 numbers) are skipped rather than throwing.
 */
export function highlightsFromPositions(positions: number[][] | undefined | null): PdfHighlight[] {
	if (!positions?.length) return [];
	const out: PdfHighlight[] = [];
	for (const p of positions) {
		if (p.length < 5) continue;
		const [page, x0, x1, top, bottom] = p;
		out.push({
			page,
			left: x0,
			top,
			width: Math.max(x1 - x0, 0),
			height: Math.max(bottom - top, 0)
		});
	}
	return out;
}
