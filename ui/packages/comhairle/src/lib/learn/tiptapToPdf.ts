import type {
	Content,
	Margins,
	StyleDictionary,
	TableCell,
	TDocumentDefinitions
} from 'pdfmake/interfaces';

/**
 * Convert a conversation's learn-step content into a pdfmake document definition.
 *
 * The output builds a *text-bearing* PDF (real text runs and real table cells), which is the
 * hard requirement for the Learning Assistant: RAGFlow parses it natively for retrieval and
 * returns per-chunk highlight positions, and the existing PDF viewer displays it. See the
 * "learn content as PDF" spec.
 *
 * The mapping is deliberately small and defensive - author content varies, so unknown nodes
 * recurse into their children, missing fields are skipped, and nothing throws on unexpected
 * shapes (a malformed page falls back to plain text rather than failing the whole sync).
 *
 * Note on `is_rich`: it is NOT trusted for parsing. Learn pages are often stored as "legacy"
 * markdown entries (`is_rich = false`) whose `content` is nonetheless TipTap ProseMirror JSON.
 * So every page is parsed as ProseMirror first regardless of the flag, and only content that
 * is genuinely not a document node is treated as markdown/plain text.
 */

/** A learn page as returned by `GET /documents/learn_content`. */
export type LearnContentPage = {
	/** Raw page content: usually TipTap ProseMirror JSON, occasionally plain markdown. */
	content: string;
	/** Backend hint; unreliable (see module note), so used only as a weak signal, never trusted. */
	is_rich: boolean;
};

export type LearnContentSection = {
	heading: string;
	pages: LearnContentPage[];
};

/** src -> embeddable data URL, for images resolved by the caller (pdfmake needs embedded data). */
export type ImageMap = Record<string, string>;

/** Minimal shape of a ProseMirror/TipTap node - only the fields we read. */
type ProseMirrorMark = { type?: string };
type ProseMirrorNode = {
	type?: string;
	text?: string;
	attrs?: Record<string, unknown>;
	marks?: ProseMirrorMark[];
	content?: ProseMirrorNode[];
};

const HEADING_MARGIN: Margins = [0, 10, 0, 4];
const BODY_MARGIN: Margins = [0, 0, 0, 8];
/** Content width of an A4 page with the 40pt margins set below (595.28 - 2*40 ≈ 515). */
const MAX_IMAGE_WIDTH = 500;

const styles: StyleDictionary = {
	h1: { fontSize: 20, bold: true, margin: [0, 14, 0, 6] },
	h2: { fontSize: 16, bold: true, margin: HEADING_MARGIN },
	h3: { fontSize: 14, bold: true, margin: HEADING_MARGIN },
	h4: { fontSize: 12, bold: true, margin: HEADING_MARGIN },
	h5: { fontSize: 11, bold: true, margin: HEADING_MARGIN },
	h6: { fontSize: 11, bold: true, italics: true, margin: HEADING_MARGIN },
	normal: { fontSize: 11, margin: BODY_MARGIN }
};

/**
 * Images resolved by the caller for the current build. Set synchronously at the start of
 * `sectionsToPdfDefinition` and read during its (synchronous) walk, so there is no interleaving
 * - this keeps the recursive node mappers from having to thread the map through every call.
 */
let imageMap: ImageMap = {};

/** Clamp a heading level to 1..6 and map it to a `styles` key (`h1`..`h6`). */
function headingStyle(level: unknown): string {
	const n = typeof level === 'number' && Number.isFinite(level) ? Math.trunc(level) : 1;
	return `h${Math.min(6, Math.max(1, n))}`;
}

/** True when the node carries a mark of the given type (bold / italic / underline / strike). */
function hasMark(node: ProseMirrorNode, type: string): boolean {
	return Array.isArray(node.marks) && node.marks.some((mark) => mark?.type === type);
}

/**
 * Flatten a block node's inline children (text runs and hard breaks) into a pdfmake `text`
 * value. Bold / italic / underline / strike marks become the matching run properties. Returns
 * an empty string for an empty paragraph so it still occupies a line.
 */
function inlineContent(node: ProseMirrorNode): Content {
	const children = node.content;
	if (!Array.isArray(children) || children.length === 0) return '';

	const runs: Content[] = [];
	for (const child of children) {
		if (child?.type === 'hardBreak') {
			runs.push('\n');
			continue;
		}
		if (child?.type === 'text' && typeof child.text === 'string') {
			const decoration: ('underline' | 'lineThrough')[] = [];
			if (hasMark(child, 'underline')) decoration.push('underline');
			if (hasMark(child, 'strike')) decoration.push('lineThrough');
			runs.push({
				text: child.text,
				bold: hasMark(child, 'bold') || undefined,
				italics: hasMark(child, 'italic') || hasMark(child, 'italics') || undefined,
				decoration: decoration.length > 0 ? decoration : undefined
			});
			continue;
		}
		// Unknown inline node with its own children (e.g. a link wrapper): recurse.
		if (Array.isArray(child?.content)) {
			runs.push(inlineContent(child));
		}
	}
	return runs;
}

/** Convert a `listItem`'s block content into a single pdfmake list entry. */
function listItem(node: ProseMirrorNode): Content {
	const blocks = blocksFromChildren(node);
	if (blocks.length === 0) return '';
	return blocks.length === 1 ? blocks[0] : { stack: blocks };
}

/** Convert a table cell's block content into a pdfmake `TableCell`. */
function tableCell(node: ProseMirrorNode, isHeader: boolean): TableCell {
	const blocks = blocksFromChildren(node);
	// A header cell is wrapped in a `stack` so it can carry the `bold` style whatever its
	// block content is (a stack is a single known pdfmake type, which keeps the union happy).
	if (isHeader) return { stack: blocks.length > 0 ? blocks : [''], bold: true };
	if (blocks.length === 0) return '';
	return blocks.length === 1 ? blocks[0] : { stack: blocks };
}

/** Map an `image` node to an embedded pdfmake image, or `null` when it can't be embedded. */
function imageNode(node: ProseMirrorNode): Content | null {
	const src = typeof node.attrs?.src === 'string' ? node.attrs.src : null;
	const data = src ? imageMap[src] : undefined;
	// No embeddable data (unresolved src, or the fetch/encode failed, e.g. CORS): skip it
	// rather than dump a raw URL into the text layer.
	if (!data) return null;
	const authoredWidth = typeof node.attrs?.width === 'number' ? node.attrs.width : undefined;
	const width = Math.min(authoredWidth ?? MAX_IMAGE_WIDTH, MAX_IMAGE_WIDTH);
	return { image: data, width, margin: BODY_MARGIN };
}

/** Map a single block node to pdfmake content, or `null` when it produces nothing. */
function blockNode(node: ProseMirrorNode): Content | null {
	switch (node.type) {
		case 'heading':
			return { text: inlineContent(node), style: headingStyle(node.attrs?.level) };
		case 'paragraph':
			return { text: inlineContent(node), style: 'normal' };
		case 'bulletList':
			return { ul: (node.content ?? []).map(listItem), margin: BODY_MARGIN };
		case 'orderedList':
			return { ol: (node.content ?? []).map(listItem), margin: BODY_MARGIN };
		case 'table':
			return tableNode(node);
		case 'image':
			return imageNode(node);
		default: {
			// Unknown block: recurse into children so their text is not lost.
			const blocks = blocksFromChildren(node);
			if (blocks.length === 0) return null;
			return blocks.length === 1 ? blocks[0] : { stack: blocks };
		}
	}
}

/** Convert a `table` node into a pdfmake table with light horizontal rules. */
function tableNode(node: ProseMirrorNode): Content {
	const body: TableCell[][] = (node.content ?? [])
		.filter((row) => row?.type === 'tableRow')
		.map((row) =>
			(row.content ?? []).map((cell) => tableCell(cell, cell?.type === 'tableHeader'))
		)
		.filter((row) => row.length > 0);

	if (body.length === 0) return { text: '', style: 'normal' };
	return { table: { body }, layout: 'lightHorizontalLines', margin: BODY_MARGIN };
}

/** Convert every child of a node into a flat list of block-level pdfmake content. */
function blocksFromChildren(node: ProseMirrorNode): Content[] {
	const children = node.content;
	if (!Array.isArray(children)) return [];
	const out: Content[] = [];
	for (const child of children) {
		const block = blockNode(child);
		if (block !== null) out.push(block);
	}
	return out;
}

/**
 * Parse a page's raw content as a ProseMirror document node, or `null` if it is not one.
 *
 * Ignores `is_rich` on purpose (see module note): legacy-flagged pages routinely carry TipTap
 * JSON. Requires the parsed value to be an object with a `type` field so genuine markdown /
 * plain text (which never parses to such a node) falls through to the markdown path.
 */
function parseProseMirror(raw: string): ProseMirrorNode | null {
	try {
		const value = JSON.parse(raw) as unknown;
		if (
			value &&
			typeof value === 'object' &&
			typeof (value as ProseMirrorNode).type === 'string'
		) {
			return value as ProseMirrorNode;
		}
	} catch {
		// Not JSON at all - handled as markdown by the caller.
	}
	return null;
}

/**
 * Minimal markdown -> pdfmake for the rare page that is genuinely markdown (not TipTap JSON),
 * e.g. placeholder "# Page 1" pages. Handles ATX headings and `-`/`*` bullets; everything else
 * is a paragraph. Intentionally tiny - real authored content comes through as ProseMirror.
 */
function markdownToBlocks(md: string): Content[] {
	const blocks: Content[] = [];
	let bullets: Content[] = [];
	const flushBullets = () => {
		if (bullets.length > 0) {
			blocks.push({ ul: bullets, margin: BODY_MARGIN });
			bullets = [];
		}
	};
	for (const rawLine of md.split('\n')) {
		const line = rawLine.trim();
		if (!line) {
			flushBullets();
			continue;
		}
		const heading = /^(#{1,6})\s+(.*)$/.exec(line);
		if (heading) {
			flushBullets();
			blocks.push({ text: heading[2], style: `h${heading[1].length}` });
			continue;
		}
		const bullet = /^[-*]\s+(.*)$/.exec(line);
		if (bullet) {
			bullets.push(bullet[1]);
			continue;
		}
		flushBullets();
		blocks.push({ text: line, style: 'normal' });
	}
	flushBullets();
	return blocks;
}

/** Convert one page's raw content into block-level pdfmake content. */
function pageContent(page: LearnContentPage): Content[] {
	const raw = page.content?.trim();
	if (!raw) return [];
	const doc = parseProseMirror(page.content);
	return doc ? blocksFromChildren(doc) : markdownToBlocks(page.content);
}

/** Walk a ProseMirror node collecting every image `src`. */
function collectImageSrcs(node: ProseMirrorNode, out: Set<string>) {
	if (node.type === 'image' && typeof node.attrs?.src === 'string') {
		out.add(node.attrs.src);
	}
	if (Array.isArray(node.content)) {
		for (const child of node.content) collectImageSrcs(child, out);
	}
}

/**
 * Every image `src` referenced across the learn content. The caller fetches and encodes these
 * into an {@link ImageMap} (pdfmake can only embed image data, not remote URLs) and passes it
 * back to {@link sectionsToPdfDefinition}.
 */
export function collectImageSources(sections: LearnContentSection[]): string[] {
	const srcs = new Set<string>();
	for (const section of sections) {
		for (const page of section.pages ?? []) {
			const doc = parseProseMirror(page.content);
			if (doc) collectImageSrcs(doc, srcs);
		}
	}
	return [...srcs];
}

/**
 * Build the full pdfmake document definition from the learn-content sections. Each step becomes
 * an `h1` heading followed by its pages, as one flowing PDF (RAGFlow chunks by content, so a
 * single document is fine). Pass `images` (src -> data URL) to embed images; any src missing
 * from the map is skipped.
 */
export function sectionsToPdfDefinition(
	sections: LearnContentSection[],
	images: ImageMap = {}
): TDocumentDefinitions {
	imageMap = images;
	const content: Content[] = [];
	for (const section of sections) {
		if (section.heading?.trim()) {
			content.push({ text: section.heading, style: 'h1' });
		}
		for (const page of section.pages ?? []) {
			content.push(...pageContent(page));
		}
	}
	imageMap = {};

	return {
		content,
		styles,
		defaultStyle: { fontSize: 11 },
		pageMargins: [40, 40, 40, 40]
	};
}
