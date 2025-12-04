<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { Editor } from '@tiptap/core';
	import { detectContentType } from '$lib/utils/contentDetection';
	import { getBaseExtensions, getEditorProps } from '../editorConfig';

	type Props = {
		content?: string;
		class?: string;
	};

	let { content = '', class: className = '' }: Props = $props();

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
				editorProps: getEditorProps()
			});
		} catch (error) {
			//how should we log things? UX: how should we handle it? 
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

<div class="content-renderer {className}" bind:this={editorElement}>
	<!-- Tiptap editor renders here -->
</div>

<style>
	.content-renderer {
		width: 100%;
	}

	:global(.content-renderer .iframe-wrapper) {
		position: relative;
		padding-bottom: 56.25%; /* 16:9 aspect ratio */
		height: 0;
		overflow: hidden;
		width: 100%;
		margin: 1rem 0;
		border-radius: 0.5rem;
	}

	:global(.content-renderer .iframe-wrapper iframe) {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		border-radius: 0.5rem;
	}

	:global(.content-renderer .iframe-wrapper.iframe-blocked) {
		background: red;
		border: 2px dashed #d1d5db;
	}
</style>
