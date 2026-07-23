// @vitest-environment jsdom
import { describe, it, expect } from 'vitest';
import { generateJSON, type JSONContent } from '@tiptap/core';
import { getBaseExtensions } from './editorConfig';

/**
 * Pasting a table from Word / Google Docs / a web page puts real HTML `<table>`
 * markup on the clipboard. This exercises that parse path (which needs a DOM,
 * hence the jsdom environment) to confirm the table extension turns it into
 * table/row/cell nodes, and that out-of-schema junk like per-cell background
 * colours is dropped rather than carried into stored content.
 */
describe('pasting an HTML table', () => {
	const extensions = getBaseExtensions({ mode: 'editor' });

	function findNode(node: JSONContent, type: string): JSONContent | undefined {
		if (node.type === type) return node;
		for (const child of node.content ?? []) {
			const found = findNode(child, type);
			if (found) return found;
		}
		return undefined;
	}

	function collectText(node: JSONContent): string {
		if (node.type === 'text') return node.text ?? '';
		return (node.content ?? []).map(collectText).join(' ');
	}

	// Mimics clipboard HTML from a pasted doc: inline styles, a coloured cell,
	// a header row, and a spanned cell.
	const pastedHtml = `
		<table style="border-collapse:collapse">
			<tbody>
				<tr>
					<th style="background-color:#c9d7f0"><p>Success</p></th>
					<th colspan="2"><p>What we'll do</p></th>
				</tr>
				<tr>
					<td style="background-color:#f7c9f0;width:220px"><p>Vision</p></td>
					<td><p>Build it</p></td>
					<td><p>Why it matters</p></td>
				</tr>
			</tbody>
		</table>`;

	it('parses into table / row / cell nodes', () => {
		const doc = generateJSON(pastedHtml, extensions);

		const table = findNode(doc, 'table');
		expect(table).toBeDefined();
		expect(findNode(doc, 'tableRow')).toBeDefined();
		expect(findNode(doc, 'tableHeader')).toBeDefined();
		expect(findNode(doc, 'tableCell')).toBeDefined();
	});

	it('preserves cell text and structure', () => {
		const doc = generateJSON(pastedHtml, extensions);
		const text = collectText(doc);

		expect(text).toContain('Success');
		expect(text).toContain('Vision');
		expect(text).toContain('Build it');

		// colspan is part of the schema, so it survives the round-trip
		const header = findNode(doc, 'tableHeader');
		const spanned = findNode(doc, 'table')?.content?.[0]?.content?.find(
			(c) => c.attrs?.colspan === 2
		);
		expect(header).toBeDefined();
		expect(spanned).toBeDefined();
	});

	it('drops out-of-schema cell background colours', () => {
		const doc = generateJSON(pastedHtml, extensions);
		const serialised = JSON.stringify(doc);

		expect(serialised).not.toContain('c9d7f0');
		expect(serialised).not.toContain('f7c9f0');
		expect(serialised).not.toContain('background-color');
	});
});
