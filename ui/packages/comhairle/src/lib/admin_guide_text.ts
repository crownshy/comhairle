/**
 * Splits admin guide text into pieces the page can render, so the content in
 * admin_guides.ts can use two bits of inline formatting without {@html}:
 *
 *   **Launch**                bold
 *   [Sign up](/auth/signup)   link
 */
export type GuideTextPart =
	| { kind: 'text'; text: string }
	| { kind: 'bold'; text: string }
	| { kind: 'link'; text: string; href: string };

const TOKEN = /\*\*(.+?)\*\*|\[([^\]]+)\]\(([^)\s]+)\)/g;

export function parseGuideText(source: string): GuideTextPart[] {
	const parts: GuideTextPart[] = [];
	let last = 0;
	for (const match of source.matchAll(TOKEN)) {
		const index = match.index ?? 0;
		if (index > last) parts.push({ kind: 'text', text: source.slice(last, index) });
		if (match[1] !== undefined) {
			parts.push({ kind: 'bold', text: match[1] });
		} else {
			parts.push({ kind: 'link', text: match[2], href: match[3] });
		}
		last = index + match[0].length;
	}
	if (last < source.length) parts.push({ kind: 'text', text: source.slice(last) });
	return parts;
}

/** Links to other sites (and mailto:) open outside the guide. */
export function isExternalHref(href: string) {
	return /^(https?:|mailto:)/.test(href);
}
