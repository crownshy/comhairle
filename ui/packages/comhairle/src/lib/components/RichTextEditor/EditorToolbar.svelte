<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import type { ActiveStates } from './types';
	import UrlInputPopover from '$lib/components/RichTextEditor/UrlInputPopover/UrlInputPopover.svelte';
	import DocumentPickerPopover from './DocumentPickerPopover.svelte';
	import TableControlsPopover from './TableControlsPopover.svelte';
	import { insertDefaultTable } from './tableActions';
	import type { ComhairleDocument } from '@crownshy/api-client/api';
	import { cn } from '$lib/utils';

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
		Table as TableIcon,
		ChevronDown,
		MoreHorizontal,
		Music,
		type Icon
	} from 'lucide-svelte';
	import type { ComponentType, Snippet } from 'svelte';

	type Props = {
		editor: Editor | undefined;
		activeStates: ActiveStates;
		documents: ComhairleDocument[];
		disabled?: boolean;
		/** Extra controls pinned to the right end of the toolbar, such as the translate button. They stay usable when the toolbar is disabled. */
		actions?: Snippet;
	};

	let { editor, activeStates, documents, disabled = false, actions }: Props = $props();

	const overflowId = $props.id();
	let overflowOpen = $state(false);

	// TODO: Upgrade lucide icons library to be able to use svelte 5 syntax as ComponentType is deprecated
	type ButtonProps = {
		title: string;
		active: boolean;
		onclick?: (event: MouseEvent) => void;
		classes?: string;
	} & ({ text: string; Icon?: undefined } | { text?: undefined; Icon: ComponentType<Icon> });

	const TOOLBAR_BUTTON_CLASS =
		'text-muted-foreground hover:bg-accent hover:text-accent-foreground flex size-8 shrink-0 cursor-pointer items-center justify-center rounded-md text-sm leading-none transition-colors disabled:cursor-not-allowed disabled:opacity-40';
	const ACTIVE_BUTTON_CLASS =
		'bg-primary text-primary-foreground hover:bg-primary hover:text-primary-foreground font-semibold';
	const GROUP_CLASS = 'flex items-center gap-0.5';
</script>

<!--
	As the toolbar narrows, groups move from the right into the "More" row. The breakpoints
	are container queries on the toolbar, not the viewport, because the same editor sits in
	full-width pages, side-by-side translation panes and dialogs.
-->

{#snippet divider()}
	<div class="bg-border mx-1 h-5 w-px shrink-0" aria-hidden="true"></div>
{/snippet}

{#snippet button({ title, active, onclick, classes, text, Icon }: ButtonProps)}
	<button
		type="button"
		{onclick}
		{title}
		aria-label={title}
		aria-pressed={active}
		class={cn(TOOLBAR_BUTTON_CLASS, classes, active && ACTIVE_BUTTON_CLASS)}
	>
		{#if text}
			{text}
		{/if}
		{#if Icon}
			<Icon size={16} />
		{/if}
	</button>
{/snippet}

{#snippet lists()}
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
{/snippet}

{#snippet quoteAndLink()}
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
{/snippet}

{#snippet alignment()}
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
{/snippet}

{#snippet media()}
	<UrlInputPopover
		type="image"
		allowLocalSelection
		onSubmit={(url) => {
			editor?.chain().focus().setImage({ src: url }).run();
		}}
	>
		{@render button({ title: 'Add Image', active: false, Icon: ImageIcon })}
	</UrlInputPopover>
	<UrlInputPopover
		type="video"
		allowLocalSelection
		onSubmit={(url) => {
			editor?.chain().focus().setIframe({ src: url }).run();
		}}
	>
		{@render button({ title: 'Add Video', active: false, Icon: Video })}
	</UrlInputPopover>
	<UrlInputPopover
		type="audio"
		allowLocalSelection
		onSubmit={(url) => {
			editor?.chain().focus().setAudio({ src: url, autoplay: false, controls: true }).run();
		}}
	>
		{@render button({ title: 'Add Audio', active: false, Icon: Music })}
	</UrlInputPopover>
	<DocumentPickerPopover
		{documents}
		onSelect={(documentId) => {
			editor?.chain().focus().setSourceDocument({ documentId }).run();
		}}
	>
		{@render button({ title: 'Insert Source Document', active: false, Icon: FileText })}
	</DocumentPickerPopover>
{/snippet}

{#snippet table()}
	<!-- Outside a table the click inserts one; inside a table it opens the manage menu. -->
	<TableControlsPopover {editor}>
		{@render button({
			title: activeStates.table ? 'Table options' : 'Insert Table',
			active: activeStates.table,
			onclick: (event) => {
				if (!activeStates.table && editor) {
					event.stopPropagation();
					insertDefaultTable(editor);
				}
			},
			Icon: TableIcon
		})}
	</TableControlsPopover>
{/snippet}

<div class="border-border bg-muted @container rounded-t-xl border border-b-0">
	<div class="flex min-h-12 items-center px-2 py-1.5">
		<fieldset {disabled} class="contents">
			<div class="relative w-28 min-w-20 shrink">
				<select
					class="text-muted-foreground hover:bg-accent hover:text-accent-foreground h-8 w-full cursor-pointer appearance-none truncate rounded-md border-0 bg-transparent pr-6 pl-2 text-sm"
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
					class="text-muted-foreground pointer-events-none absolute top-1/2 right-2 -translate-y-1/2"
				/>
			</div>

			{@render divider()}

			<div class={GROUP_CLASS}>
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

			<div class="hidden items-center @md:flex">
				{@render divider()}
				<div class={GROUP_CLASS}>{@render lists()}</div>
			</div>
			<div class="hidden items-center @lg:flex">
				{@render divider()}
				<div class={GROUP_CLASS}>{@render quoteAndLink()}</div>
			</div>
			<div class="hidden items-center @2xl:flex">
				{@render divider()}
				<div class={GROUP_CLASS}>{@render alignment()}</div>
			</div>
			<div class="hidden items-center @3xl:flex">
				{@render divider()}
				<div class={GROUP_CLASS}>{@render media()}</div>
				{@render divider()}
				<div class={GROUP_CLASS}>{@render table()}</div>
			</div>

			<button
				type="button"
				class={cn(TOOLBAR_BUTTON_CLASS, 'ml-1 @3xl:hidden', overflowOpen && 'bg-accent')}
				title="More formatting"
				aria-label="More formatting"
				aria-expanded={overflowOpen}
				aria-controls={overflowId}
				onclick={() => (overflowOpen = !overflowOpen)}
			>
				<MoreHorizontal size={16} />
			</button>
		</fieldset>

		{#if actions}
			<div class="ml-auto flex items-center pl-1">{@render actions()}</div>
		{/if}
	</div>

	{#if overflowOpen}
		<fieldset
			{disabled}
			id={overflowId}
			class="border-border flex min-w-0 flex-wrap items-center gap-x-3 gap-y-1 border-t px-2 py-1.5 @3xl:hidden"
		>
			<div class={cn(GROUP_CLASS, '@md:hidden')}>{@render lists()}</div>
			<div class={cn(GROUP_CLASS, '@lg:hidden')}>{@render quoteAndLink()}</div>
			<div class={cn(GROUP_CLASS, '@2xl:hidden')}>{@render alignment()}</div>
			<div class={GROUP_CLASS}>{@render media()}</div>
			<div class={GROUP_CLASS}>{@render table()}</div>
		</fieldset>
	{/if}
</div>
