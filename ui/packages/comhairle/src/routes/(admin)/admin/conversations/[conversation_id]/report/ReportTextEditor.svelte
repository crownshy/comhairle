<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { Editor } from '@tiptap/core';
	import { TextSelection } from '@tiptap/pm/state';
	import { BubbleMenu } from '@tiptap/extension-bubble-menu';
	import { getBaseExtensions, getEditorProps } from '$lib/components/RichTextEditor/editorConfig';
	import { MarkdownTablePaste } from '$lib/components/RichTextEditor/extensions/markdownTablePaste';
	import { detectContentType } from '$lib/utils/contentDetection';
	import '$lib/components/RichTextEditor/editor-content.css';
	import SelectionBubble from './SelectionBubble.svelte';

	type Props = {
		/** Read once when the editor mounts; later edits come from the editor itself. */
		initialValue: string;
		editable: boolean;
		onChange: (json: string) => void;
		editor?: Editor;
		label: string;
	};

	let { initialValue, editable, onChange, editor = $bindable(), label }: Props = $props();

	let element = $state<HTMLElement>();
	let bubble = $state<HTMLElement>();
	let loaded: string | undefined;

	onMount(() => {
		const detected = detectContentType(initialValue);
		const base = getEditorProps();
		editor = new Editor({
			element,
			extensions: [
				...getBaseExtensions({ mode: 'editor' }),
				MarkdownTablePaste,
				BubbleMenu.configure({
					element: bubble,
					// Text only: a selected chart or page break has nothing to format.
					shouldShow: ({ editor: current, state }) =>
						current.isEditable &&
						state.selection instanceof TextSelection &&
						!state.selection.empty,
					// Starting at the selection keeps the bubble clear of the rail on the left.
					options: { placement: 'top-start' }
				})
			],
			content: detected.content,
			contentType: detected.type,
			editable,
			editorProps: {
				attributes: {
					...base.attributes,
					'aria-label': label,
					// The editor is the page here, so the app's focus ring would frame the whole
					// document. Inline because that ring is unlayered and beats utility classes.
					style: 'outline: none; box-shadow: none; min-height: 3rem'
				}
			},
			// TipTap normalises the loaded content while it mounts and reports that as an update.
			// Only edits made after mounting count, so opening the page never saves anything.
			onCreate: ({ editor: created }) => {
				setTimeout(() => {
					loaded = JSON.stringify(created.getJSON());
				}, 0);
			},
			onUpdate: ({ editor: updated }) => {
				if (loaded === undefined) return;
				const json = JSON.stringify(updated.getJSON());
				if (json !== loaded) onChange(json);
			}
		});
	});

	$effect(() => {
		editor?.setEditable(editable);
	});

	onDestroy(() => editor?.destroy());
</script>

<div bind:this={element}></div>
<!-- Hidden until TipTap positions it for a selection; it sets visibility inline from then on. -->
<div bind:this={bubble} class="invisible absolute z-20">
	<SelectionBubble {editor} />
</div>
