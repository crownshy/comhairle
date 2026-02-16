<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { Editor } from '@tiptap/core';
	import { Color } from '@tiptap/extension-color';
	import { ListItem } from '@tiptap/extension-list-item';
	import { TextStyle } from '@tiptap/extension-text-style';
	import { Underline } from '@tiptap/extension-underline';
	import { TextAlign } from '@tiptap/extension-text-align';
	import EditorToolbar from './EditorToolbar.svelte';
	import { CONTENT_TYPES, type ContentType, type ActiveStates } from '$lib/components/RichTextEditor/types';
	import { detectContentType } from '$lib/utils/contentDetection';
	import { getBaseExtensions, getEditorProps } from './editorConfig';
	import './editor-content.css';

	type Props = {
		value?: string;
		placeholder?: string;
		editable?: boolean;
		class?: string;
		minHeight?: string;
		maxHeight?: string;
		width?: string;
		onChange?: (json: string) => void;
	};

	let {
		value = '',
		placeholder = 'Start typing...',
		editable = true,
		class: className = '',
		minHeight = '200px',
		maxHeight,
		width,
		onChange
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
	
	let isCompact = $derived(containerWidth < 600);
	let resizeObserver: ResizeObserver;
	
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

	onMount(() => {
		// Set up resize observer to track container width
		resizeObserver = new ResizeObserver((entries) => {
			for (const entry of entries) {
				containerWidth = entry.contentRect.width;
			}
		});
		
		if (containerElement) {
			resizeObserver.observe(containerElement);
		}

		if (!editorElement) return;

		const detected = detectContentType(value);

		editor = new Editor({
			element: editorElement,
			extensions: [
				// Editor-specific extensions
				Color.configure({ types: ['textStyle', 'listItem'] }),
				TextStyle,
				ListItem,
				Underline,
				TextAlign.configure({
					types: ['heading', 'paragraph']
				}),
				// Shared base extensions
				...getBaseExtensions({ mode: 'editor' })
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
			heading: editor.isActive('heading', { level: 1 }) ? '1' :
					 editor.isActive('heading', { level: 2 }) ? '2' :
					 editor.isActive('heading', { level: 3 }) ? '3' : 'p',
			textAlign: editor.isActive({ textAlign: 'center' }) ? 'center' :
					   editor.isActive({ textAlign: 'right' }) ? 'right' :
					   editor.isActive({ textAlign: 'justify' }) ? 'justify' : 'left'
		};
	}


	$effect(() => {
		if (editor && !isInitializing && value !== undefined && value !== previousValue) {
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
		resizeObserver?.disconnect();
	});

</script>

<div bind:this={containerElement} class={width ? 'overflow-hidden' : ''} style={width ? `width: ${width}` : ''}>
	{#if editor}
		<EditorToolbar
			{editor}
			{activeStates}
			bind:showLinkPopover
			bind:showImagePopover
			bind:showVideoPopover
			{menuExpanded}
			compact={isCompact}
			onToggleMenu={() => menuExpanded = !menuExpanded}
			onLinkPopoverChange={(open) => showLinkPopover = open}
			onImagePopoverChange={(open) => showImagePopover = open}
			onVideoPopoverChange={(open) => showVideoPopover = open}
		/>
	{/if}

	<div class="bg-white border border-gray-300 rounded-b-[12px] md:rounded-b-[12px] md:border-t {className}">
		{#if maxHeight}
			<div class="editor-scroll-container" style="max-height: {maxHeight}; overflow-y: auto;">
				<div bind:this={editorElement} class="p-4" style="min-height: {minHeight}"></div>
			</div>
		{:else}
			<div bind:this={editorElement} class="p-4" style="min-height: {minHeight}"></div>
		{/if}
	</div>
</div>
