import { describe, expect, it } from 'vitest';
import {
	createGlossaryMatcher,
	markGlossaryTerms,
	planGlossaryTerms,
	pluralise
} from './admin_guide_glossary';
import { parseGuideText } from './admin_guide_text';
import { ADMIN_GLOSSARY, ADMIN_GUIDES } from './admin_guides';
import { TOOL_GUIDES } from './tool_guides';

const matcher = createGlossaryMatcher([
	{ term: 'Step', definition: 'A stage.' },
	{ term: 'Statement', definition: 'A point.' },
	{ term: 'Seed statement', definition: 'A starter point.' },
	{ term: 'Launch', definition: 'Go live.', aliases: ['launched'] },
	{ term: 'Moderation policy', definition: 'Reasons.' }
]);

const all = () => new Set(matcher.entries.keys());
const mark = (text: string, available = all()) =>
	markGlossaryTerms(parseGuideText(text), available, matcher);
const terms = (text: string) =>
	mark(text).flatMap((part) => (part.kind === 'term' ? [`${part.term}:${part.text}`] : []));

describe('pluralise', () => {
	it('handles common endings', () => {
		expect(pluralise('step')).toBe('steps');
		expect(pluralise('Moderation policy')).toBe('Moderation policies');
		expect(pluralise('process')).toBe('processes');
	});
});

describe('markGlossaryTerms', () => {
	it('marks plurals, aliases and keeps the original casing', () => {
		expect(terms('Add Steps once it has launched.')).toEqual(['Step:Steps', 'Launch:launched']);
	});

	it('prefers the longest term', () => {
		expect(terms('Write a seed statement.')).toEqual(['Seed statement:seed statement']);
	});

	it('only matches whole words', () => {
		expect(terms('A footstep or step-by-step guide.')).toEqual([]);
	});

	it('marks only the first mention', () => {
		expect(terms('One step, then another step.')).toEqual(['Step:step']);
	});

	it('leaves bold UI names alone', () => {
		expect(terms('Click **Launch** to launch it.')).toEqual(['Launch:launch']);
	});

	it('turns a glossary link into the tooltip', () => {
		const parts = mark('Arrange your [steps](/admin/info/glossary#term-step).');
		expect(parts).toContainEqual({ kind: 'term', text: 'steps', term: 'Step' });
	});

	it('keeps the text around a match', () => {
		const parts = mark('Each step counts.');
		expect(parts.map((p) => ('text' in p ? p.text : '')).join('')).toBe('Each step counts.');
	});
});

describe('planGlossaryTerms', () => {
	it('marks each term once across the whole page', () => {
		const plan = planGlossaryTerms(
			['Add a step.', 'Then another step, and a statement.'],
			matcher
		);
		expect(plan.get('Add a step.')).toEqual(new Set(['Step']));
		expect(plan.get('Then another step, and a statement.')).toEqual(new Set(['Statement']));
	});
});

describe('ADMIN_GLOSSARY learnMore links', () => {
	const pages = new Set([
		'/admin/info/tools',
		...Object.keys(TOOL_GUIDES).map((key) => `/admin/info/tools/${key}`),
		...Object.values(ADMIN_GUIDES).flatMap((guide) =>
			guide.topics.map((topic) => `/admin/info/how-to/${guide.key}/${topic.key}`)
		)
	]);

	it.each(ADMIN_GLOSSARY.terms.filter((entry) => entry.learnMore))(
		'$term links to a handbook page that exists',
		(entry) => {
			expect(pages).toContain(entry.learnMore!.href);
		}
	);
});
