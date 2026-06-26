<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import type { ActiveStates } from './types';
	import {
		validateUrl,
		validateIframeUrl,
		DEFAULT_ALLOWED_DOMAINS
	} from '$lib/utils/urlValidation';
	import UrlInputPopover from '$lib/components/UrlInputPopover/UrlInputPopover.svelte';
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
		Music
	} from 'lucide-svelte';

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
		onLinkPopoverChange,
		onImagePopoverChange,
		onVideoPopoverChange,
		onAudioPopoverChange,
		onDocumentPopoverChange
	}: Props = $props();
</script>

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

		<div class="bg-border mx-1 h-5 w-px shrink-0"></div>

		<!-- BISU (always visible) -->
		<div class="flex items-center gap-0.5">
			<button
				type="button"
				onclick={() => editor?.chain().focus().toggleBold().run()}
				title="Bold"
				aria-label="Bold"
				class="btn font-bold"
				class:!bg-primary={activeStates.bold}
				class:!text-primary-foreground={activeStates.bold}
				class:!font-semibold={activeStates.bold}
			>
				B
			</button>
			<button
				type="button"
				onclick={() => editor?.chain().focus().toggleItalic().run()}
				title="Italic"
				aria-label="Italic"
				class="btn italic"
				class:!bg-primary={activeStates.italic}
				class:!text-primary-foreground={activeStates.italic}
				class:!font-semibold={activeStates.italic}
			>
				I
			</button>
			<button
				type="button"
				onclick={() => editor?.chain().focus().toggleStrike().run()}
				title="Strikethrough"
				aria-label="Strikethrough"
				class="btn line-through"
				class:!bg-primary={activeStates.strike}
				class:!text-primary-foreground={activeStates.strike}
				class:!font-semibold={activeStates.strike}
			>
				S
			</button>
			<button
				type="button"
				onclick={() => editor?.chain().focus().toggleUnderline().run()}
				title="Underline"
				aria-label="Underline"
				class="btn underline"
				class:!bg-primary={activeStates.underline}
				class:!text-primary-foreground={activeStates.underline}
				class:!font-semibold={activeStates.underline}
			>
				U
			</button>
		</div>

		<!-- Mobile/Compact "more" toggle -->
		{#if !compact}
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
			<div class="bg-border mx-1 hidden h-5 w-px shrink-0 xl:block"></div>

			<!-- Lists -->
			<div class="flex items-center gap-0.5">
				<button
					type="button"
					onclick={() => editor?.chain().focus().toggleBulletList().run()}
					title="Bullet List"
					aria-label="Bullet List"
					class="btn"
					class:!bg-primary={activeStates.bulletList}
					class:!text-primary-foreground={activeStates.bulletList}
					class:!font-semibold={activeStates.bulletList}
				>
					<List size={16} />
				</button>
				<button
					type="button"
					onclick={() => editor?.chain().focus().toggleOrderedList().run()}
					title="Numbered List"
					aria-label="Numbered List"
					class="btn"
					class:!bg-primary={activeStates.orderedList}
					class:!text-primary-foreground={activeStates.orderedList}
					class:!font-semibold={activeStates.orderedList}
				>
					<ListOrdered size={16} />
				</button>
			</div>

			<div class="bg-border mx-1 hidden h-5 w-px shrink-0 xl:block"></div>

			<!-- Text Alignment -->
			<div class="flex items-center gap-0.5">
				<button
					type="button"
					onclick={() => editor?.chain().focus().setTextAlign('left').run()}
					title="Align Left"
					aria-label="Align Left"
					class="btn"
					class:!bg-primary={activeStates.textAlign === 'left'}
					class:!text-primary-foreground={activeStates.textAlign === 'left'}
					class:!font-semibold={activeStates.textAlign === 'left'}
				>
					<AlignLeft size={16} />
				</button>
				<button
					type="button"
					onclick={() => editor?.chain().focus().setTextAlign('center').run()}
					title="Align Center"
					aria-label="Align Center"
					class="btn"
					class:!bg-primary={activeStates.textAlign === 'center'}
					class:!text-primary-foreground={activeStates.textAlign === 'center'}
					class:!font-semibold={activeStates.textAlign === 'center'}
				>
					<AlignCenter size={16} />
				</button>
				<button
					type="button"
					onclick={() => editor?.chain().focus().setTextAlign('right').run()}
					title="Align Right"
					aria-label="Align Right"
					class="btn"
					class:!bg-primary={activeStates.textAlign === 'right'}
					class:!text-primary-foreground={activeStates.textAlign === 'right'}
					class:!font-semibold={activeStates.textAlign === 'right'}
				>
					<AlignRight size={16} />
				</button>
				<button
					type="button"
					onclick={() => editor?.chain().focus().setTextAlign('justify').run()}
					title="Justify"
					aria-label="Justify"
					class="btn"
					class:!bg-primary={activeStates.textAlign === 'justify'}
					class:!text-primary-foreground={activeStates.textAlign === 'justify'}
					class:!font-semibold={activeStates.textAlign === 'justify'}
				>
					<AlignJustify size={16} />
				</button>
			</div>

			<div class="bg-border mx-1 hidden h-5 w-px shrink-0 xl:block"></div>

			<!-- Blockquote -->
			<div class="flex items-center gap-0.5">
				<button
					type="button"
					onclick={() => editor?.chain().focus().toggleBlockquote().run()}
					title="Blockquote"
					aria-label="Blockquote"
					class="btn"
					class:!bg-primary={activeStates.blockquote}
					class:!text-primary-foreground={activeStates.blockquote}
					class:!font-semibold={activeStates.blockquote}
				>
					<Quote size={16} />
				</button>
			</div>

			<div class="bg-border mx-1 hidden h-5 w-px shrink-0 xl:block"></div>

			<!-- Link, Image & Video -->
			<div class="flex items-center gap-0.5">
				<UrlInputPopover
					bind:open={showLinkPopover}
					label="Insert Link"
					placeholder="https://example.com"
					onSubmit={(url) => {
						editor?.chain().focus().setLink({ href: url }).run();
					}}
					onOpenChange={onLinkPopoverChange}
					validateFn={(url) => {
						if (!validateUrl(url)) {
							return 'Please enter a valid HTTPS URL';
						}
						return null;
					}}
				>
					<button
						type="button"
						title="Add Link"
						aria-label="Add Link"
						class="btn"
						class:!bg-primary={activeStates.link}
						class:!text-primary-foreground={activeStates.link}
						class:!font-semibold={activeStates.link}
					>
						<LinkIcon size={16} />
					</button>
				</UrlInputPopover>
				<UrlInputPopover
					bind:open={showImagePopover}
					label="Insert Image"
					placeholder="https://example.com/image.jpg"
					onSubmit={(url) => {
						editor?.chain().focus().setImage({ src: url }).run();
					}}
					onOpenChange={onImagePopoverChange}
					validateFn={(url) => {
						if (!validateUrl(url)) {
							return 'Please enter a valid HTTPS image URL';
						}
						return null;
					}}
				>
					<button type="button" title="Add Image" aria-label="Add Image" class="btn">
						<ImageIcon size={16} />
					</button>
				</UrlInputPopover>
				<UrlInputPopover
					bind:open={showVideoPopover}
					label="Insert Video"
					placeholder="https://youtube.com/embed/..."
					onSubmit={(url) => {
						editor?.chain().focus().setIframe({ src: url }).run();
					}}
					onOpenChange={onVideoPopoverChange}
					validateFn={(url) => {
						if (!validateIframeUrl(url, DEFAULT_ALLOWED_DOMAINS)) {
							return 'Please enter a valid video URL';
						}
						return null;
					}}
				>
					<button type="button" title="Add Video" aria-label="Add Video" class="btn">
						<Video size={16} />
					</button>
				</UrlInputPopover>
				<UrlInputPopover
					bind:open={showAudioPopover}
					label="Insert Audio"
					placeholder="https://example.com/file.mp3"
					onSubmit={(url) => {
						editor
							?.chain()
							.focus()
							.setAudio({ src: url, autoplay: false, controls: true })
							.run();
					}}
					onOpenChange={onAudioPopoverChange}
					validateFn={(url) => {
						if (!validateUrl(url)) {
							return 'Please enter a valid Audio URL';
						}
						return null;
					}}
				>
					<button type="button" title="Add Audio" aria-label="Add Audio" class="btn">
						<Music size={16} />
					</button>
				</UrlInputPopover>
				<DocumentPickerPopover
					bind:open={showDocumentPopover}
					{documents}
					onSelect={(docId) => {
						editor?.chain().focus().setSourceDocument({ documentId: docId }).run();
					}}
					onOpenChange={onDocumentPopoverChange}
				>
					<button
						type="button"
						title="Insert Source Document"
						aria-label="Insert Source Document"
						class="btn"
					>
						<FileText size={16} />
					</button>
				</DocumentPickerPopover>
			</div>
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
