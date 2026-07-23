// @vitest-environment jsdom
import { describe, it, expect } from 'vitest';
import { Editor } from '@tiptap/core';
import { getBaseExtensions } from './editorConfig';
import { containsMarkdownTable } from './markdownTables';

describe('markdown table detection', () => {
	it('detects a table inside pasted text', () => {
		const text = ['intro', '| A | B |', '| --- | --- |', '| 1 | 2 |', 'outro'].join('\n');
		expect(containsMarkdownTable(text)).toBe(true);
	});

	it('detects a table with alignment colons', () => {
		expect(containsMarkdownTable('| A | B |\n| :--- | ---: |\n| 1 | 2 |')).toBe(true);
	});

	it('does not flag ordinary text with a stray pipe', () => {
		expect(containsMarkdownTable('a | b is a choice\nsecond line')).toBe(false);
	});

	it('does not flag a horizontal rule', () => {
		expect(containsMarkdownTable('some text\n\n---\n\nmore text')).toBe(false);
	});
});

describe('pasted markdown table insertion', () => {
	it('inserts a pasted GFM table as real table nodes', () => {
		const editor = new Editor({ extensions: getBaseExtensions({ mode: 'editor' }) });
		const pasted = ['| A | B |', '| --- | --- |', '| 1 | 2 |'].join('\n');

		expect(containsMarkdownTable(pasted)).toBe(true);
		editor.commands.insertContent(pasted, { contentType: 'markdown' });

		const html = editor.getHTML();
		expect(html).toContain('<table');
		expect(html).toContain('<th');
		expect(html).toContain('<td');
		editor.destroy();
	});
});
