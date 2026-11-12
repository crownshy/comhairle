<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import { Editor, type Extensions } from '@tiptap/core';
	import { Placeholder } from '@tiptap/extensions';
	import { getBaseExtensions, getEditorProps } from '$lib/components/RichTextEditor/editorConfig';
	import { MarkdownTablePaste } from '$lib/components/RichTextEditor/extensions/markdownTablePaste';
	import { CONTENT_TYPES } from '$lib/components/RichTextEditor/types';
	import { detectContentType } from '$lib/utils/contentDetection';
	import '$lib/components/RichTextEditor/editor-content.css';

	type Props = {
		/** Read once when the editor mounts; later edits come from the editor itself. */
		initialValue: string;
		editable: boolean;
		onChange: (json: string) => void;
		onFocus?: (editor: Editor) => void;
		editor?: Editor;
		label: string;
		/** Shown inside the empty editor and never saved. */
		placeholder: string;
		/** Added to the shared rich text extensions, such as a rule about what this editor takes. */
		extensions?: Extensions;
	};

	let {
		initialValue,
		editable,
		onChange,
		onFocus,
		editor = $bindable(),
		label,
		placeholder,
		extensions = []
	}: Props = $props();

	const EMPTY_DOCUMENT = { type: 'doc', content: [{ type: 'paragraph' }] };

	let element = $state<HTMLElement>();
	let lastReported: string | undefined;

	onMount(() => {
		// Empty markdown loads as a document with no paragraph, which leaves no line to type on
		// and nothing for the placeholder to sit in.
		const detected = initialValue.trim()
			? detectContentType(initialValue)
			: { content: EMPTY_DOCUMENT, type: CONTENT_TYPES.JSON };
		const base = getEditorProps();
		editor = new Editor({
			element,
			extensions: [
				...getBaseExtensions({ mode: 'editor' }),
				MarkdownTablePaste,
				Placeholder.configure({ placeholder }),
				...extensions
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
					style: 'outline: none; box-shadow: none'
				}
			},
			// TipTap normalises the loaded content while it mounts and reports that as an update.
			// Only edits made after mounting count, so opening the page never saves anything.
			// Comparing with the last version sent, not the loaded one, still sends an undo that
			// lands back on the loaded document.
			onCreate: ({ editor: created }) => {
				setTimeout(() => {
					lastReported = JSON.stringify(created.getJSON());
				}, 0);
			},
			onUpdate: ({ editor: updated }) => {
				if (lastReported === undefined) return;
				const json = JSON.stringify(updated.getJSON());
				if (json === lastReported) return;
				lastReported = json;
				onChange(json);
			},
			onFocus: ({ editor: focused }) => onFocus?.(focused)
		});
	});

	$effect(() => {
		editor?.setEditable(editable);
	});

	onDestroy(() => editor?.destroy());
</script>

<div
	bind:this={element}
	class="[&_.is-editor-empty:first-child]:before:text-muted-foreground [&_.is-editor-empty:first-child]:before:pointer-events-none [&_.is-editor-empty:first-child]:before:float-start [&_.is-editor-empty:first-child]:before:h-0 [&_.is-editor-empty:first-child]:before:content-[attr(data-placeholder)]"
></div>
