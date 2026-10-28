<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { Editor } from '@tiptap/core';
	import { getBaseExtensions, getEditorProps } from '$lib/components/RichTextEditor/editorConfig';
	import { MarkdownTablePaste } from '$lib/components/RichTextEditor/extensions/markdownTablePaste';
	import { detectContentType } from '$lib/utils/contentDetection';
	import '$lib/components/RichTextEditor/editor-content.css';

	type Props = {
		/** Read once when the editor mounts; later edits come from the editor itself. */
		initialValue: string;
		editable: boolean;
		/** Switches every embed's label from "Public when published" to "Public now". */
		published: boolean;
		onChange: (json: string) => void;
		onFocus?: (editor: Editor) => void;
		editor?: Editor;
		label: string;
	};

	let {
		initialValue,
		editable,
		published,
		onChange,
		onFocus,
		editor = $bindable(),
		label
	}: Props = $props();

	let element = $state<HTMLElement>();
	let loaded: string | undefined;

	onMount(() => {
		const detected = detectContentType(initialValue);
		const base = getEditorProps();
		editor = new Editor({
			element,
			extensions: [...getBaseExtensions({ mode: 'editor' }), MarkdownTablePaste],
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
			},
			onFocus: ({ editor: focused }) => onFocus?.(focused)
		});
	});

	$effect(() => {
		editor?.setEditable(editable);
	});

	$effect(() => {
		if (!editor) return;
		editor.view.dom.toggleAttribute('data-report-published', published);
	});

	onDestroy(() => editor?.destroy());
</script>

<div bind:this={element}></div>
