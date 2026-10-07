/**
 * Splits admin guide text into pieces the page can render, so the content in
 * admin_guides.ts can use two bits of inline formatting without {@html}:
 *
 *   **Launch**                bold
 *   [Sign up](/auth/signup)   link
 *   :accept: :reject: :split: the moderation action buttons, drawn as icons
 */
export type GuideTextPart =
	| { kind: 'text'; text: string }
	| { kind: 'bold'; text: string }
	| { kind: 'link'; text: string; href: string }
	| { kind: 'icon'; name: GuideIconName };

export type GuideIconName = 'accept' | 'reject' | 'split';

const TOKEN = /\*\*(.+?)\*\*|\[([^\]]+)\]\(([^)\s]+)\)|:(accept|reject|split):/g;

export function parseGuideText(source: string): GuideTextPart[] {
	const parts: GuideTextPart[] = [];
	let last = 0;
	for (const match of source.matchAll(TOKEN)) {
		const index = match.index ?? 0;
		if (index > last) parts.push({ kind: 'text', text: source.slice(last, index) });
		if (match[1] !== undefined) {
			parts.push({ kind: 'bold', text: match[1] });
		} else if (match[4] !== undefined) {
			parts.push({ kind: 'icon', name: match[4] as GuideIconName });
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
