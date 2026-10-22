type ReportNode = {
	type?: string;
	attrs?: { level?: number } & Record<string, unknown>;
	content?: ReportNode[];
	text?: string;
};

export type ReportCard = {
	title: string;
	/** A TipTap document (or a legacy plain string) that `ReportBody` can render. */
	content: string;
};

export type ReportSection = {
	label: string;
	cards: ReportCard[];
};

/** One card in reading order, with where it sits so the pager can say "2 of 3". */
export type ReportPage = ReportCard & {
	sectionIndex: number;
	cardIndex: number;
	cardCount: number;
};

function parseDocumentNodes(content: string): ReportNode[] | undefined {
	let document: unknown;
	try {
		document = JSON.parse(content);
	} catch {
		return undefined;
	}
	if (
		!document ||
		typeof document !== 'object' ||
		(document as ReportNode).type !== 'doc' ||
		!Array.isArray((document as ReportNode).content)
	) {
		return undefined;
	}
	return (document as { content: ReportNode[] }).content;
}

function nodeText(node: ReportNode): string {
	return (node.text ?? '') + (node.content ?? []).map(nodeText).join('');
}

function isHeading(node: ReportNode, level: number) {
	return node.type === 'heading' && node.attrs?.level === level;
}

function toDocument(nodes: ReportNode[]) {
	return JSON.stringify({ type: 'doc', content: nodes });
}

/** True when a rich text value has any visible text or an embedded component. */
export function hasReportContent(content: string | null | undefined): boolean {
	if (!content) return false;
	const nodes = parseDocumentNodes(content);
	if (!nodes) return content.trim().length > 0;
	return nodes.some(
		(node) => node.type === 'reportComponentEmbed' || nodeText(node).trim().length > 0
	);
}

/**
 * Splits the report body into the sections and cards the paged report shows. An H2 starts a
 * section, an H3 starts a card titled by it, and a horizontal rule starts a card that keeps
 * the section title. Content before the first H2 goes in a section called `introLabel`.
 * Empty cards and sections are dropped. See ADR-0058.
 */
export function splitReportIntoSections(content: string, introLabel: string): ReportSection[] {
	const nodes = parseDocumentNodes(content);
	if (!nodes) {
		return hasReportContent(content)
			? [{ label: introLabel, cards: [{ title: introLabel, content }] }]
			: [];
	}

	const sections: { label: string; cards: { title: string; nodes: ReportNode[] }[] }[] = [];
	const currentSection = () => {
		if (sections.length === 0) {
			sections.push({ label: introLabel, cards: [{ title: introLabel, nodes: [] }] });
		}
		return sections[sections.length - 1];
	};

	for (const node of nodes) {
		if (isHeading(node, 2)) {
			const label = nodeText(node).trim() || introLabel;
			sections.push({ label, cards: [{ title: label, nodes: [] }] });
		} else if (isHeading(node, 3)) {
			const section = currentSection();
			section.cards.push({ title: nodeText(node).trim() || section.label, nodes: [] });
		} else if (node.type === 'horizontalRule') {
			const section = currentSection();
			section.cards.push({ title: section.label, nodes: [] });
		} else {
			const section = currentSection();
			section.cards[section.cards.length - 1].nodes.push(node);
		}
	}

	return sections
		.map((section) => ({
			label: section.label,
			cards: section.cards
				.filter((card) => hasReportContent(toDocument(card.nodes)))
				.map((card) => ({ title: card.title, content: toDocument(card.nodes) }))
		}))
		.filter((section) => section.cards.length > 0);
}

/** Lays the sections' cards out in reading order, one page per card. */
export function toReportPages(sections: ReportSection[]): ReportPage[] {
	return sections.flatMap((section, sectionIndex) =>
		section.cards.map((card, cardIndex) => ({
			...card,
			sectionIndex,
			cardIndex,
			cardCount: section.cards.length
		}))
	);
}

const CARD_PARAM = 'card';

/**
 * Which screen the URL asks for: undefined for the report's first screen, 1 to `pageCount`
 * for a card, and `pageCount + 1` for the closing thank-you. Anything else falls back to the
 * first screen, so a stale link still opens the report.
 */
export function readCardNumber(url: URL, pageCount: number): number | undefined {
	const card = Number(url.searchParams.get(CARD_PARAM));
	return Number.isInteger(card) && card >= 1 && card <= pageCount + 1 ? card : undefined;
}

/** A link to a card, or to the report's first screen when `card` is left out. */
export function cardHref(url: URL, card?: number): string {
	const target = new URL(url);
	if (card === undefined) target.searchParams.delete(CARD_PARAM);
	else target.searchParams.set(CARD_PARAM, String(card));
	return target.pathname + target.search;
}
