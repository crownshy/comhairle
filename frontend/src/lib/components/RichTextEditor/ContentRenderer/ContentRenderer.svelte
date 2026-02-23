<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { Editor } from '@tiptap/core';
	import { detectContentType } from '$lib/utils/contentDetection';
	import { getBaseExtensions, getEditorProps } from '../editorConfig';

	type Props = {
		content?: string;
		class?: string;
		minimal?: boolean;
	};

	let { content = '', class: className = '', minimal = false }: Props = $props();

	let editorElement = $state<HTMLElement>();
	let editor = $state<Editor>();

	onMount(() => {
		if (!editorElement) return;

		try {
			const detected = detectContentType(content);

			editor = new Editor({
				element: editorElement,
				extensions: getBaseExtensions({ mode: 'renderer' }),
				content: detected.content,
				contentType: detected.type,
				editable: false,
				editorProps: minimal ? {} : getEditorProps()
			});
		} catch (error) {
			console.error('[ContentRenderer] Failed to initialize:', error);
		}
	});

	$effect(() => {
		if (editor && content !== undefined) {
			try {
				const detected = detectContentType(content);

				editor.commands.setContent(detected.content, {
					contentType: detected.type,
					emitUpdate: false
				});
			} catch (error) {
				console.error('[ContentRenderer] Failed to update content:', error);
			}
		}
	});

	onDestroy(() => {
		if (editor) {
			editor.destroy();
		}
	});
</script>

<div class="content-renderer {className}" class:content-renderer--minimal={minimal} bind:this={editorElement}>
	<!-- Tiptap editor renders here -->
</div>

<style>
	.content-renderer {
		width: 100%;
	}

	:global(.content-renderer .tiptap) {
		min-height: unset;
	}
</style>
