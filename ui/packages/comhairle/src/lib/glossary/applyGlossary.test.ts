import { describe, it, expect } from 'vitest';
import type { JSONContent } from '@tiptap/core';
import { applyGlossary } from './applyGlossary';
import type { Glossary } from './types';

const doc = (text: string): JSONContent => ({
	type: 'doc',
	content: [{ type: 'paragraph', content: [{ type: 'text', text }] }]
});

const glossary: Glossary = [
	{ text: ['bus', 'autobus'], tooltip: 'A vehicle that carries multiple people' },
	{ text: ['referral'], tooltip: 'When your case is passed to another team' }
];

/** Collect the text of every node carrying the glossaryTerm mark. */
function markedTerms(node: JSONContent): string[] {
	const out: string[] = [];
	const walk = (n: JSONContent) => {
		if (n.type === 'text' && n.marks?.some((m) => m.type === 'glossaryTerm') && n.text) {
			out.push(n.text);
		}
		n.content?.forEach(walk);
	};
	walk(node);
	return out;
}

describe('applyGlossary', () => {
	it('marks a matching term with its tooltip', () => {
		const result = applyGlossary(doc('Take the bus home'), glossary);
		const para = result.content?.[0].content ?? [];
		expect(para.map((n) => n.text)).toEqual(['Take the ', 'bus', ' home']);
		const marked = para.find((n) => n.text === 'bus');
		expect(marked?.marks).toEqual([
			{ type: 'glossaryTerm', attrs: { tooltip: 'A vehicle that carries multiple people' } }
		]);
	});

	it('matches synonyms in the same entry', () => {
		expect(markedTerms(applyGlossary(doc('Catch the autobus'), glossary))).toEqual(['autobus']);
	});

	it('is case-insensitive but preserves the original casing', () => {
		expect(markedTerms(applyGlossary(doc('The Bus is late'), glossary))).toEqual(['Bus']);
	});

	it('only matches whole words, not substrings', () => {
		expect(markedTerms(applyGlossary(doc('Running a business'), glossary))).toEqual([]);
	});

	it('tooltips only the first occurrence by default', () => {
		expect(markedTerms(applyGlossary(doc('bus then another bus'), glossary))).toEqual(['bus']);
	});

	it('tooltips every occurrence when firstOccurrenceOnly is false', () => {
		const result = applyGlossary(doc('bus then another bus'), glossary, {
			firstOccurrenceOnly: false
		});
		expect(markedTerms(result)).toEqual(['bus', 'bus']);
	});

	it('handles multiple different terms', () => {
		const result = applyGlossary(doc('A referral onto the bus'), glossary);
		expect(markedTerms(result)).toEqual(['referral', 'bus']);
	});

	it('leaves content untouched when the glossary is empty', () => {
		const input = doc('Take the bus home');
		expect(applyGlossary(input, [])).toBe(input);
	});

	it('does not mutate the input document', () => {
		const input = doc('Take the bus home');
		const snapshot = JSON.stringify(input);
		applyGlossary(input, glossary);
		expect(JSON.stringify(input)).toBe(snapshot);
	});

	it('preserves existing marks on a matched term', () => {
		const input: JSONContent = {
			type: 'doc',
			content: [
				{
					type: 'paragraph',
					content: [{ type: 'text', text: 'bus', marks: [{ type: 'bold' }] }]
				}
			]
		};
		const marked = applyGlossary(input, glossary).content?.[0].content?.[0];
		expect(marked?.marks).toEqual([
			{ type: 'bold' },
			{ type: 'glossaryTerm', attrs: { tooltip: 'A vehicle that carries multiple people' } }
		]);
	});
});
