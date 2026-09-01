import type { LocalizedConversationDto } from '@crownshy/api-client/api';
import type { JSONContent } from '@tiptap/core';
import { CONTENT_TYPES } from '$lib/components/RichTextEditor/types';
import { detectContentType } from '$lib/utils/contentDetection';
import { splitSlides } from '$lib/step-brief/splitSlides';
import * as m from '$lib/paraglide/messages';
import type { StepPreview } from './stepPreview';

/**
 * One screen of Before you start. The chips across the top are the pages, in order, and a
 * participant sees exactly one of them at a time (ADR-0024).
 */
export type BeforeYouStartPage = {
	id: string;
	/** The chip. Kept short: the strip has to stay usable on a narrow screen. */
	label: string;
	/**
	 * Rendered above the content. Omitted when the content opens with its own heading, which
	 * would otherwise be printed twice.
	 */
	heading?: string;
	content?: string;
	/** The step list, which is computed rather than authored and so carries no content. */
	kind?: 'steps';
};

/** A markdown ATX heading, with its trailing hashes optional. */
const MARKDOWN_HEADING = /^[ \t]{0,3}#{1,6}[ \t]+(.+?)[ \t]*#*$/;

function proseMirrorHeading(document: JSONContent): string | null {
	const first = (document.content ?? [])[0];
	if (first?.type !== 'heading') return null;
	const text = (first.content ?? [])
		.map((node) => node.text ?? '')
		.join('')
		.trim();
	return text || null;
}

/**
 * The page's own heading, when it opens with one. Only the first block counts: a heading
 * further down names a part of the page rather than the page.
 */
function leadingHeading(content: string | null | undefined): string | null {
	const detected = detectContentType(content);
	if (!detected.content) return null;

	if (detected.type === CONTENT_TYPES.JSON) {
		return proseMirrorHeading(detected.content as JSONContent);
	}

	const firstLine = String(detected.content)
		.split('\n')
		.find((line) => line.trim().length > 0);
	return firstLine?.match(MARKDOWN_HEADING)?.[1]?.trim() ?? null;
}

function contentPage(
	id: string,
	content: string,
	fallbackLabel: string,
	fallbackHeading?: string
): BeforeYouStartPage {
	const heading = leadingHeading(content);
	return {
		id,
		label: heading ?? fallbackLabel,
		heading: heading ? undefined : fallbackHeading,
		content
	};
}

/** Two chips reading the same word say nothing, so a repeat is numbered. */
function withUniqueLabels(pages: BeforeYouStartPage[]): BeforeYouStartPage[] {
	const seen = new Map<string, number>();
	return pages.map((page) => {
		const count = (seen.get(page.label) ?? 0) + 1;
		seen.set(page.label, count);
		return count === 1 ? page : { ...page, label: `${page.label} ${count}` };
	});
}

/**
 * The pages of Before you start, in the order the chips offer them.
 *
 * The description splits at horizontal rules, the same break the step brief uses for its
 * slides (ADR-0017), so an admin writes as many pages as the conversation needs without a
 * new field. The FAQs and the privacy policy stay one page each: they are separate fields
 * with their own pages elsewhere, and a rule inside a privacy policy is a divider rather
 * than a page break.
 *
 * The step list goes last because it is the only computed page. Everything an admin wrote
 * is read before the page that is generated for them.
 */
export function beforeYouStartPages(
	conversation: LocalizedConversationDto,
	steps: StepPreview[]
): BeforeYouStartPage[] {
	const pages: BeforeYouStartPage[] = splitSlides(conversation.description).map(
		(content, index) =>
			contentPage(
				`about-${index}`,
				content,
				index === 0 ? m.landing_nav_about() : m.landing_nav_more(),
				index === 0 ? m.landing_about_heading() : undefined
			)
	);

	if (conversation.faqs) {
		pages.push(
			contentPage(
				'questions',
				conversation.faqs,
				m.landing_questions_heading(),
				m.landing_questions_heading()
			)
		);
	}

	if (conversation.privacyPolicy) {
		pages.push(
			contentPage(
				'your-data',
				conversation.privacyPolicy,
				m.landing_your_data_heading(),
				m.landing_your_data_heading()
			)
		);
	}

	if (steps.length) {
		pages.push({
			id: 'steps',
			label: m.landing_nav_steps(),
			heading: m.landing_steps_heading(),
			kind: 'steps'
		});
	}

	return withUniqueLabels(pages);
}
