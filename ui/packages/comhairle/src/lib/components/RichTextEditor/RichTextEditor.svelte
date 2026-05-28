<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { useResizeObserver } from 'runed';
	import { Editor } from '@tiptap/core';
	import { Color } from '@tiptap/extension-color';
	import { ListItem } from '@tiptap/extension-list-item';
	import { TextStyle } from '@tiptap/extension-text-style';
	import { Underline } from '@tiptap/extension-underline';
	import EditorToolbar from './EditorToolbar.svelte';
	import { type ActiveStates } from '$lib/components/RichTextEditor/types';
	import { detectContentType } from '$lib/utils/contentDetection';
	import { getBaseExtensions, getEditorProps } from './editorConfig';
	import { SourceDocument } from './extensions/sourceDocument';
	import type { ComhairleDocument } from '@crownshy/api-client/api';
	import './editor-content.css';

	type Props = {
		value?: string | null;
		placeholder?: string;
		editable?: boolean;
		class?: string;
		minHeight?: string;
		maxHeight?: string;
		width?: string;
		onChange?: (json: string) => void;
		availableDocuments?: ComhairleDocument[];
		conversationId?: string;
	};

	let {
		value = null,
		placeholder = 'Start typing...',
		editable = true,
		class: className = '',
		minHeight = '200px',
		maxHeight = 'clamp(200px, calc(100dvh - 420px), 1200px)',
		width,
		onChange,
		availableDocuments = [],
		conversationId
	}: Props = $props();

	let editorElement = $state<HTMLElement>();
	let containerElement = $state<HTMLElement>();
	let editor = $state<Editor>();
	let containerWidth = $state(1000);

	let isInitializing = $state(true);
	let menuExpanded = $state(false);
	let previousValue = $state<string>();

	let showLinkPopover = $state(false);
	let showImagePopover = $state(false);
	let showVideoPopover = $state(false);
	let showDocumentPopover = $state(false);

	let isCompact = $derived(containerWidth < 600);

	let activeStates = $state<ActiveStates>({
		bold: false,
		italic: false,
		strike: false,
		code: false,
		underline: false,
		link: false,
		bulletList: false,
		orderedList: false,
		blockquote: false,
		heading: 'p' as 'p' | '1' | '2' | '3',
		textAlign: 'left' as 'left' | 'center' | 'right' | 'justify'
	});

	useResizeObserver(
		() => containerElement,
		(entries) => {
			const entry = entries[0];
			if (entry) {
				containerWidth = entry.contentRect.width;
			}
		}
	);

	function buildDocMap(docs: ComhairleDocument[]) {
		const map: Record<string, { name: string; size: number }> = {};
		for (const doc of docs) {
			map[doc.id] = { name: doc.name, size: doc.size };
		}
		return map;
	}

	let lastDocMapKey = $state('');

	function createEditor() {
		if (editor) {
			editor.destroy();
			editor = undefined;
		}
		if (!editorElement) return;

		isInitializing = true;
		const detected = detectContentType(value);
		const docMap = buildDocMap(availableDocuments);
		lastDocMapKey = JSON.stringify({ docMap, conversationId });

		editor = new Editor({
			element: editorElement,
			extensions: [
				// Shared base extensions (filter out default SourceDocument, add configured one)
				...getBaseExtensions({ mode: 'editor' }).filter(
					(ext) => ext.name !== 'sourceDocument'
				),
				SourceDocument.configure({ documents: docMap, conversationId, editable: true })
			],
			content: detected.content,
			contentType: detected.type,
			editable: editable,
			editorProps: getEditorProps(),
			onTransaction: () => {
				if (editor && !isInitializing) {
					updateActiveStates();

					const newValue = JSON.stringify(editor.getJSON());
					previousValue = newValue;
					onChange?.(newValue);
				}
			},
			onSelectionUpdate: () => {
				if (editor && !isInitializing) {
					updateActiveStates();
				}
			},
			onCreate: () => {
				setTimeout(() => {
					isInitializing = false;
					updateActiveStates();
					if (editor) {
						previousValue = JSON.stringify(editor.getJSON());
					}
				}, 0);
			}
		});
	}

	onMount(() => {
		createEditor();
	});

	$effect(() => {
		const docMap = buildDocMap(availableDocuments);
		const newKey = JSON.stringify({ docMap, conversationId });
		if (newKey !== lastDocMapKey && editorElement) {
			createEditor();
		}
	});

	function updateActiveStates() {
		if (!editor) return;

		activeStates = {
			bold: editor.isActive('bold'),
			italic: editor.isActive('italic'),
			strike: editor.isActive('strike'),
			code: editor.isActive('code'),
			underline: editor.isActive('underline'),
			link: editor.isActive('link'),
			bulletList: editor.isActive('bulletList'),
			orderedList: editor.isActive('orderedList'),
			blockquote: editor.isActive('blockquote'),
			heading: editor.isActive('heading', { level: 1 })
				? '1'
				: editor.isActive('heading', { level: 2 })
					? '2'
					: editor.isActive('heading', { level: 3 })
						? '3'
						: 'p',
			textAlign: editor.isActive({ textAlign: 'center' })
				? 'center'
				: editor.isActive({ textAlign: 'right' })
					? 'right'
					: editor.isActive({ textAlign: 'justify' })
						? 'justify'
						: 'left'
		};
	}

	$effect(() => {
		if (
			editor &&
			!isInitializing &&
			value !== undefined &&
			value !== null &&
			value !== previousValue
		) {
			const detected = detectContentType(value);

			editor.commands.setContent(detected.content, {
				contentType: detected.type,
				emitUpdate: false
			});

			previousValue = value;
		}
	});

	$effect(() => {
		if (editor) {
			editor.setEditable(editable);
		}
	});

	onDestroy(() => {
		if (editor) {
			editor.destroy();
		}
	});
</script>

<div
	bind:this={containerElement}
	class={width ? 'overflow-hidden' : ''}
	style={width ? `width: ${width}` : ''}
>
	{#if editor}
		<EditorToolbar
			{editor}
			{activeStates}
			bind:showLinkPopover
			bind:showImagePopover
			bind:showVideoPopover
			bind:showDocumentPopover
			documents={availableDocuments}
			{menuExpanded}
			compact={isCompact}
			onToggleMenu={() => (menuExpanded = !menuExpanded)}
			onLinkPopoverChange={(open) => (showLinkPopover = open)}
			onImagePopoverChange={(open) => (showImagePopover = open)}
			onVideoPopoverChange={(open) => (showVideoPopover = open)}
			onDocumentPopoverChange={(open) => (showDocumentPopover = open)}
		/>
	{/if}

	<div
		class="bg-background border-border rounded-b-xl border md:rounded-b-xl md:border-t {className}"
	>
		{#if maxHeight}
			<div class="editor-scroll-container" style="max-height: {maxHeight}; overflow-y: auto;">
				<div bind:this={editorElement} class="p-4" style="min-height: {minHeight}"></div>
			</div>
		{:else}
			<div bind:this={editorElement} class="p-4" style="min-height: {minHeight}"></div>
		{/if}
	</div>
</div>
