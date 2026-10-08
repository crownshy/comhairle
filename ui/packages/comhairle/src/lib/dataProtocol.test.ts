import { describe, expect, it } from 'vitest';
import * as m from '$lib/paraglide/messages';
import {
	defaultDataProtocol,
	defaultDataProtocolHtml,
	defaultDataProtocolParagraphs,
	hasDataProtocolText,
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

	it('wraps the default in a paragraph for the rich editor', () => {
		expect(defaultDataProtocolHtml('polis')).toBe(`<p>${m.data_protocol_default_polis()}</p>`);
	});

	it('splits a default with a blank line into paragraphs', () => {
		const paragraphs = defaultDataProtocolParagraphs('learn');
		expect(paragraphs).toHaveLength(2);
		expect(defaultDataProtocolHtml('learn')).toBe(
			paragraphs.map((p) => `<p>${p}</p>`).join('')
		);
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
