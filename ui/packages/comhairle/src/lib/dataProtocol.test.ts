import { describe, expect, it } from 'vitest';
import { m } from '$lib/paraglide/messages';
import {
	defaultDataProtocol,
	defaultDataProtocolDoc,
	defaultDataProtocolParagraphs,
	hasDataProtocolText,
	isDefaultDataProtocol,
	toolUsesAI
} from './dataProtocol';

describe('defaultDataProtocol', () => {
	it('returns the default for the tool', () => {
		expect(defaultDataProtocol('polis')).toBe(m.data_protocol_default_polis());
	});

	it('falls back to the generic default for an unknown or missing tool', () => {
		expect(defaultDataProtocol('videocall')).toBe(m.data_protocol_default_generic());
		expect(defaultDataProtocol(undefined)).toBe(m.data_protocol_default_generic());
	});

	it('splits a default with a blank line into paragraphs', () => {
		const paragraphs = defaultDataProtocolParagraphs('learn');
		expect(paragraphs).toHaveLength(2);
		const doc = JSON.parse(defaultDataProtocolDoc('learn'));
		expect(
			doc.content.map((node: { content: { text: string }[] }) => node.content[0].text)
		).toEqual(paragraphs);
	});
});

describe('isDefaultDataProtocol', () => {
	it('treats blank and the untouched default as the default', () => {
		expect(isDefaultDataProtocol(null, 'learn')).toBe(true);
		expect(
			isDefaultDataProtocol('{"type":"doc","content":[{"type":"paragraph"}]}', 'learn')
		).toBe(true);
		expect(isDefaultDataProtocol(defaultDataProtocolDoc('learn'), 'learn')).toBe(true);
	});

	it('recognises the default saved as HTML by the old Start from the default button', () => {
		const html = defaultDataProtocolParagraphs('polis')
			.map((p) => `<p>${p}</p>`)
			.join('');
		expect(isDefaultDataProtocol(html, 'polis')).toBe(true);
	});

	it('sees an edit as custom text', () => {
		const edited = defaultDataProtocolDoc('learn').replace('reading', 'skimming');
		expect(isDefaultDataProtocol(edited, 'learn')).toBe(false);
		expect(isDefaultDataProtocol(defaultDataProtocolDoc('learn'), 'polis')).toBe(false);
	});
});

describe('hasDataProtocolText', () => {
	it('treats a missing or cleared field as blank', () => {
		expect(hasDataProtocolText(null)).toBe(false);
		expect(hasDataProtocolText(undefined)).toBe(false);
		expect(hasDataProtocolText('<p></p>')).toBe(false);
		expect(hasDataProtocolText('<p>  </p>')).toBe(false);
		expect(hasDataProtocolText('<p>&nbsp;</p>')).toBe(false);
	});

	it('sees text inside markup', () => {
		expect(hasDataProtocolText('<p>Your votes are anonymous.</p>')).toBe(true);
	});
});

describe('toolUsesAI', () => {
	it('flags only the tools that use AI', () => {
		expect(toolUsesAI('thinkingspace')).toBe(true);
		expect(toolUsesAI('learn')).toBe(true);
		expect(toolUsesAI('polis')).toBe(false);
		expect(toolUsesAI(undefined)).toBe(false);
	});
});
