// @vitest-environment jsdom
import { afterEach, describe, expect, it } from 'vitest';
import { Editor, type JSONContent } from '@tiptap/core';
import { getBaseExtensions } from '$lib/components/RichTextEditor/editorConfig';
import { KeepPageBreaksOut, countPageBreaks } from './pageBreaks';

const paragraph = (text: string) => ({ type: 'paragraph', content: [{ type: 'text', text }] });
const rule = { type: 'horizontalRule' };

let editor: Editor | undefined;
afterEach(() => editor?.destroy());

function createEditor(content: JSONContent[], guarded = true) {
	editor = new Editor({
		extensions: [
			...getBaseExtensions({ mode: 'editor' }),
			...(guarded ? [KeepPageBreaksOut] : [])
		],
		content: { type: 'doc', content }
	});
	return editor;
}

describe('KeepPageBreaksOut', () => {
	it('leaves the body, which has no guard, free to take a page break', () => {
		const body = createEditor([paragraph('Findings')], false);
		body.chain().focus('end').setHorizontalRule().run();
		expect(countPageBreaks(body.state.doc)).toBe(1);
	});

	it('turns away a page break in the summary', () => {
		const summary = createEditor([paragraph('Findings')]);
		summary.chain().focus('end').setHorizontalRule().run();
		expect(countPageBreaks(summary.state.doc)).toBe(0);
	});

	it('lets a chart into the summary', () => {
		const summary = createEditor([paragraph('Findings')]);
		summary
			.chain()
			.focus('end')
			.setReportComponentEmbed({ toolStepId: 'step', componentType: 'map' })
			.run();
		expect(JSON.stringify(summary.getJSON())).toContain('reportComponentEmbed');
	});

	it('still takes ordinary typing', () => {
		const summary = createEditor([paragraph('Findings')]);
		summary.chain().focus('end').insertContent(' so far').run();
		expect(summary.getText()).toBe('Findings so far');
	});

	it('lets one that is already there be deleted', () => {
		const summary = createEditor([paragraph('One'), rule, paragraph('Two')]);
		summary.commands.setContent({ type: 'doc', content: [paragraph('One')] });
		expect(countPageBreaks(summary.state.doc)).toBe(0);
	});
});
