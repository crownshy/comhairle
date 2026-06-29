<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import type { ActiveStates } from './types';
	import UrlInputPopover from '$lib/components/RichTextEditor/UrlInputPopover/UrlInputPopover.svelte';
	import DocumentPickerPopover from './DocumentPickerPopover.svelte';
	import type { ComhairleDocument } from '@crownshy/api-client/api';

	import {
		List,
		ListOrdered,
		Link as LinkIcon,
		Quote,
		AlignLeft,
		AlignCenter,
		AlignRight,
		AlignJustify,
		Image as ImageIcon,
		Video,
		FileText,
		ChevronDown,
		MoreHorizontal,
		Music,
		type Icon
	} from 'lucide-svelte';
	import type { ComponentType } from 'svelte';

	type Props = {
		editor: Editor | undefined;
		activeStates: ActiveStates;
		showLinkPopover: boolean;
		showImagePopover: boolean;
		showVideoPopover: boolean;
		showAudioPopover: boolean;
		showDocumentPopover: boolean;
		documents: ComhairleDocument[];
		menuExpanded: boolean;
		compact?: boolean;
		onToggleMenu: () => void;
		onLinkPopoverChange: (open: boolean) => void;
		onImagePopoverChange: (open: boolean) => void;
		onVideoPopoverChange: (open: boolean) => void;
		onAudioPopoverChange: (open: boolean) => void;
		onDocumentPopoverChange: (open: boolean) => void;
	};

	let {
		editor,
		activeStates,
		showLinkPopover = $bindable(),
		showImagePopover = $bindable(),
		showVideoPopover = $bindable(),
		showAudioPopover = $bindable(),
		showDocumentPopover = $bindable(),
		documents,
		menuExpanded,
		compact = false,
		onToggleMenu,
		onDocumentPopoverChange
	}: Props = $props();

	// TODO: Upgrade lucide icons library to be able to use svelte 5 syntax as ComponentType is deprecated
	type ButtonProps = {
		title: string;
		active: boolean;
		onclick?: (event: MouseEvent) => void;
		classes?: string;
	} & ({ text: string; Icon?: undefined } | { text?: undefined; Icon: ComponentType<Icon> });
</script>

{#snippet divider()}
	<div class="bg-border mx-1 hidden h-5 w-px shrink-0 xl:block"></div>
{/snippet}

{#snippet button({ title, active, onclick, classes, text, Icon }: ButtonProps)}
	<button
		type="button"
		{onclick}
		{title}
		aria-label={title}
		class={`btn ${classes}`}
		class:!bg-primary={active}
		class:!text-primary-foreground={active}
		class:!font-semibold={active}
	>
		{#if text}
			{text}
		{/if}
		{#if Icon}
			<Icon size={16} />
		{/if}
	</button>
{/snippet}
<div
	class="border-border bg-muted relative flex min-h-12 items-center gap-1 overflow-x-auto rounded-t-xl border border-b-0 px-3 xl:p-2"
>
	<!-- Always visible on mobile: Heading + BISU -->
	<div class="flex flex-1 items-center gap-1 xl:flex-none">
		<!-- Heading selector -->
		<div class="flex items-center gap-0.5">
			<div class="relative inline-block xl:flex-1">
				<select
					class="text-muted-foreground hover:bg-accent hover:text-accent-foreground min-w-24 shrink-0 cursor-pointer appearance-none rounded border-0 bg-transparent px-1.5 py-1 pr-6 text-sm xl:w-full"
					value={activeStates.heading}
					aria-label="Text style"
					onchange={(e) => {
						const value = e.currentTarget.value;
						if (value === 'p') {
							editor?.chain().focus().setParagraph().run();
						} else {
							const level = parseInt(value);
							if ([1, 2, 3].includes(level)) {
								editor
									?.chain()
									.focus()
									.toggleHeading({ level: level as 1 | 2 | 3 })
									.run();
							}
						}
					}}
				>
					<option value="p">Paragraph</option>
					<option value="1">Heading 1</option>
					<option value="2">Heading 2</option>
					<option value="3">Heading 3</option>
				</select>
				<ChevronDown
					size={12}
					class="text-muted-foreground pointer-events-none absolute top-1/2 right-1.5 -translate-y-1/2"
				/>
			</div>
		</div>

		{@render divider()}

		<!-- BISU (always visible) -->
		<div class="flex items-center gap-0.5">
			{@render button({
				title: 'Bold',
				active: activeStates.bold,
				onclick: () => editor?.chain().focus().toggleBold().run(),
				text: 'B',
				classes: 'font-bold'
			})}
			{@render button({
				title: 'Italic',
				active: activeStates.italic,
				onclick: () => editor?.chain().focus().toggleItalic().run(),
				text: 'I',
				classes: 'italic'
			})}
			{@render button({
				title: 'Strikethrough',
				active: activeStates.strike,
				onclick: () => editor?.chain().focus().toggleStrike().run(),
				text: 'S',
				classes: 'line-through'
			})}
			{@render button({
				title: 'Underline',
				active: activeStates.underline,
				onclick: () => editor?.chain().focus().toggleUnderline().run(),
				text: 'U',
				classes: 'underline'
			})}
		</div>

		<!-- Mobile/Compact "more" toggle -->
		{#if compact}
			<button
				type="button"
				class="text-muted-foreground hover:bg-accent hover:text-accent-foreground ml-auto flex shrink-0 cursor-pointer items-center justify-center rounded border-0 bg-transparent p-1.5 xl:hidden"
				onclick={onToggleMenu}
				aria-label="More options"
			>
				<MoreHorizontal size={18} />
			</button>
		{/if}
	</div>

	<!-- Desktop toolbar content / Mobile expandable content (hidden in compact mode) -->
	{#if !compact}
		<div
			class="border-border bg-background absolute top-full right-0 left-0 z-10 hidden flex-col gap-2 border border-t-0 p-3 shadow-lg xl:static xl:flex xl:w-auto xl:flex-row xl:items-center xl:gap-1 xl:border-0 xl:bg-transparent xl:p-0 xl:shadow-none"
			class:flex={menuExpanded}
			class:hidden={!menuExpanded}
		>
			{@render divider()}

			<!-- Lists -->
			<div class="flex items-center gap-0.5">
				{@render button({
					title: 'Bullet List',
					active: activeStates.bulletList,
					onclick: () => editor?.chain().focus().toggleBulletList().run(),
					Icon: List
				})}
				{@render button({
					title: 'Numbered List',
					active: activeStates.orderedList,
					onclick: () => editor?.chain().focus().toggleOrderedList().run(),
					Icon: ListOrdered
				})}
			</div>

			{@render divider()}

			<!-- Text Alignment -->
			<div class="flex items-center gap-0.5">
				{@render button({
					title: 'Align Left',
					active: activeStates.textAlign === 'left',
					onclick: () => editor?.chain().focus().setTextAlign('left').run(),
					Icon: AlignLeft
				})}
				{@render button({
					title: 'Align Center',
					active: activeStates.textAlign === 'center',
					onclick: () => editor?.chain().focus().setTextAlign('center').run(),
					Icon: AlignCenter
				})}
				{@render button({
					title: 'Align Right',
					active: activeStates.textAlign === 'right',
					onclick: () => editor?.chain().focus().setTextAlign('right').run(),
					Icon: AlignRight
				})}
				{@render button({
					title: 'Justify',
					active: activeStates.textAlign === 'justify',
					onclick: () => editor?.chain().focus().setTextAlign('justify').run(),
					Icon: AlignJustify
				})}
			</div>

			{@render divider()}

			<!-- Blockquote & Link -->
			{@render button({
				title: 'Blockquote',
				active: activeStates.blockquote,
				onclick: () => editor?.chain().focus().toggleBlockquote().run(),
				Icon: Quote
			})}
			<UrlInputPopover
				type="link"
				onSubmit={(url) => {
					editor?.chain().focus().setLink({ href: url }).run();
				}}
			>
				{@render button({
					title: 'Add Link',
					active: activeStates.link,
					onclick: (event) => {
						if (activeStates.link) {
							event.stopPropagation();
							editor?.chain().focus().unsetLink().run();
						}
					},
					Icon: LinkIcon
				})}
			</UrlInputPopover>

			{@render divider()}

			<!-- Image, Video & Audio -->
			<UrlInputPopover
				type="image"
				allowLocalSelection
				onSubmit={(url) => {
					editor?.chain().focus().setImage({ src: url }).run();
				}}
			>
				{@render button({
					title: 'Add Image',
					active: false,
					Icon: ImageIcon
				})}
			</UrlInputPopover>
			<UrlInputPopover
				type="video"
				allowLocalSelection
				onSubmit={(url) => {
					editor?.chain().focus().setIframe({ src: url }).run();
				}}
			>
				{@render button({
					title: 'Add Video',
					active: false,
					Icon: Video
				})}
			</UrlInputPopover>
			<UrlInputPopover
				type="audio"
				allowLocalSelection
				onSubmit={(url) => {
					editor
						?.chain()
						.focus()
						.setAudio({ src: url, autoplay: false, controls: true })
						.run();
				}}
			>
				{@render button({
					title: 'Add Audio',
					active: false,
					Icon: Music
				})}
			</UrlInputPopover>
			<DocumentPickerPopover
				bind:open={showDocumentPopover}
				{documents}
				onSelect={(docId) => {
					editor?.chain().focus().setSourceDocument({ documentId: docId }).run();
				}}
				onOpenChange={onDocumentPopoverChange}
			>
				{@render button({
					title: 'Insert Source Document',
					active: false,
					Icon: FileText
				})}
			</DocumentPickerPopover>
		</div>
	{/if}
</div>

<style>
	@import 'tailwindcss';

	.btn {
		@apply flex h-7 min-w-7 shrink-0 cursor-pointer items-center justify-center rounded border-0 bg-transparent px-1.5 py-1 text-sm leading-none transition-all duration-150 disabled:cursor-not-allowed disabled:opacity-40;
		color: var(--color-muted-foreground);
	}
	.btn:hover {
		background-color: var(--color-accent);
		color: var(--color-accent-foreground);
	}
</style>
