// @vitest-environment jsdom
import { afterEach, describe, expect, it } from 'vitest';
import { Editor, type JSONContent } from '@tiptap/core';
import { getBaseExtensions } from '$lib/components/RichTextEditor/editorConfig';
import { KeepBodyOnlyNodesOut, countBodyOnlyNodes } from './bodyOnlyNodes';

const paragraph = (text: string) => ({ type: 'paragraph', content: [{ type: 'text', text }] });
const rule = { type: 'horizontalRule' };

let editor: Editor | undefined;
afterEach(() => editor?.destroy());

function summaryEditor(content: JSONContent[], guarded = true) {
	editor = new Editor({
		extensions: [
			...getBaseExtensions({ mode: 'editor' }),
			...(guarded ? [KeepBodyOnlyNodesOut] : [])
		],
		content: { type: 'doc', content }
	});
	return editor;
}

describe('KeepBodyOnlyNodesOut', () => {
	it('leaves the body, which has no guard, free to take both', () => {
		const body = summaryEditor([paragraph('Findings')], false);
		body.chain()
			.focus('end')
			.setHorizontalRule()
			.setReportComponentEmbed({ toolStepId: 'step', componentType: 'map' })
			.run();
		expect(countBodyOnlyNodes(body.state.doc)).toBe(2);
	});

	it('turns away a page break', () => {
		const summary = summaryEditor([paragraph('Findings')]);
		summary.chain().focus('end').setHorizontalRule().run();
		expect(countBodyOnlyNodes(summary.state.doc)).toBe(0);
	});

	it('turns away a chart', () => {
		const summary = summaryEditor([paragraph('Findings')]);
		summary
			.chain()
			.focus('end')
			.setReportComponentEmbed({ toolStepId: 'step', componentType: 'map' })
			.run();
		expect(countBodyOnlyNodes(summary.state.doc)).toBe(0);
	});

	it('still takes ordinary typing', () => {
		const summary = summaryEditor([paragraph('Findings')]);
		summary.chain().focus('end').insertContent(' so far').run();
		expect(summary.getText()).toBe('Findings so far');
	});

	it('lets one that is already there be deleted', () => {
		const summary = summaryEditor([paragraph('One'), rule, paragraph('Two')]);
		summary.commands.setContent({ type: 'doc', content: [paragraph('One')] });
		expect(countBodyOnlyNodes(summary.state.doc)).toBe(0);
	});
});
