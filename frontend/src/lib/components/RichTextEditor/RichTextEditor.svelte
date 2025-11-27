<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { Editor } from '@tiptap/core';
	import { StarterKit } from '@tiptap/starter-kit';
	import { Color } from '@tiptap/extension-color';
	import { ListItem } from '@tiptap/extension-list-item';
	import { TextStyle } from '@tiptap/extension-text-style';
	import { Link } from '@tiptap/extension-link';
	import { Image } from '@tiptap/extension-image';
	import { Underline } from '@tiptap/extension-underline';
	import { TextAlign } from '@tiptap/extension-text-align';
	import { Markdown } from '@tiptap/markdown';
	import { Iframe } from '$lib/components/extensions/iframe';
	import EditorToolbar from './EditorToolbar.svelte';
	import { CONTENT_TYPES, type ContentType, type ActiveStates } from '$lib/components/RichTextEditor/types';
	import { detectContentType } from '$lib/utils/contentDetection';
	import './editor-content.css';

	type Props = {
		value?: string;
		placeholder?: string;
		editable?: boolean;
		class?: string;
		onChange?: (json: string) => void;
	};

	let {
		value = $bindable(''),
		placeholder = 'Start typing...',
		editable = true,
		class: className = '',
		onChange
	}: Props = $props();

	let editorElement = $state<HTMLElement>();
	let editor = $state<Editor>();

		
	let isInitializing = $state(true);
	let menuExpanded = $state(false);
	let previousValue = $state<string>();
	
	let showLinkPopover = $state(false);
	let showImagePopover = $state(false);
	let showVideoPopover = $state(false);
	
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
		if (!editorElement) return;

		const detected = detectContentType(value);

		editor = new Editor({
			element: editorElement,
			extensions: [
				Color.configure({ types: ['textStyle', 'listItem'] }),
				TextStyle,
				ListItem,
				Link.configure({
					openOnClick: false,
					HTMLAttributes: {
						class: 'text-blue-600 underline hover:text-blue-800'
					}
				}),
				Image.configure({
					HTMLAttributes: {
						class: 'max-w-full h-auto rounded-lg'
					}
				}),
				Underline,
				TextAlign.configure({
					types: ['heading', 'paragraph']
				}),
				Iframe,
				StarterKit.configure({
					heading: {
						levels: [1, 2, 3, 4, 5, 6]
					}
				}),
				Markdown
			],
			content: detected.content,
			contentType: detected.type,
			editable: editable,
			editorProps: {
				attributes: {
					class: 'prose prose-sm max-w-none focus:outline-none'
				}
			},
			onTransaction: () => {
				if (editor && !isInitializing) {
					updateActiveStates();
					
					value = JSON.stringify(editor.getJSON());
					previousValue = value; 
					if (onChange) {
						onChange(value);
					}
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
	});

</script>

{#if editor}
	<EditorToolbar
		{editor}
		{activeStates}
		bind:showLinkPopover
		bind:showImagePopover
		bind:showVideoPopover
		{menuExpanded}
		onToggleMenu={() => menuExpanded = !menuExpanded}
		onLinkPopoverChange={(open) => showLinkPopover = open}
		onImagePopoverChange={(open) => showImagePopover = open}
		onVideoPopoverChange={(open) => showVideoPopover = open}
	/>
{/if}

<div class="bg-white border border-gray-300 rounded-b-lg md:rounded-b-lg md:border-t {className}">
	<div bind:this={editorElement} class="p-4 min-h-[200px]"></div>
</div>
