<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import type { ActiveStates } from './types';
	import { validateUrl, validateIframeUrl, DEFAULT_ALLOWED_DOMAINS } from '$lib/utils/urlValidation';
	import UrlInputPopover from '$lib/components/UrlInputPopover/UrlInputPopover.svelte';
	
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
		ChevronDown,
		Code2,
		MoreHorizontal,
		X
	} from 'lucide-svelte';

	type Props = {
		editor: Editor | undefined;
		activeStates: ActiveStates;
		showLinkPopover: boolean;
		showImagePopover: boolean;
		showVideoPopover: boolean;
		menuExpanded: boolean;
		compact?: boolean;
		onToggleMenu: () => void;
		onLinkPopoverChange: (open: boolean) => void;
		onImagePopoverChange: (open: boolean) => void;
		onVideoPopoverChange: (open: boolean) => void;
	};

	let {
		editor,
		activeStates,
		showLinkPopover = $bindable(),
		showImagePopover = $bindable(),
		showVideoPopover = $bindable(),
		menuExpanded,
		compact = false,
		onToggleMenu,
		onLinkPopoverChange,
		onImagePopoverChange,
		onVideoPopoverChange
	}: Props = $props();

	const btnBase = 'py-1 px-1.5 border-0 rounded bg-transparent text-gray-600 cursor-pointer text-sm leading-none transition-all duration-150 flex items-center justify-center min-w-[1.75rem] h-7 flex-shrink-0 hover:bg-gray-200 hover:text-gray-900 disabled:opacity-40 disabled:cursor-not-allowed';
</script>

<div class="relative flex items-center gap-1 px-3 border border-gray-300 border-b-0 rounded-t-[12px] bg-gray-50 min-h-[3rem] xl:p-2 overflow-x-auto">
	<!-- Always visible on mobile: Heading + BISU -->
	<div class="flex items-center gap-1 flex-1 xl:flex-none ">
		<!-- Heading selector -->
		<div class="flex items-center gap-0.5">
			<div class="relative inline-block xl:flex-1">
				<select
					class="py-1 px-1.5 pr-6 border-0 rounded bg-transparent text-gray-600 cursor-pointer text-sm appearance-none min-w-[6rem] flex-shrink-0 hover:bg-gray-200 xl:w-full"
					value={activeStates.heading}
					aria-label="Text style"
					onchange={(e) => {
						const value = e.currentTarget.value;
						if (value === 'p') {
							editor?.chain().focus().setParagraph().run();
						} else {
							const level = parseInt(value);
							if ([1, 2, 3].includes(level)) {
								editor?.chain().focus().toggleHeading({ level: level as 1 | 2 | 3 }).run();
							}
						}
					}}
				>
					<option value="p">Paragraph</option>
					<option value="1">Heading 1</option>
					<option value="2">Heading 2</option>
					<option value="3">Heading 3</option>
				</select>
				<ChevronDown size={12} class="absolute right-1.5 top-1/2 -translate-y-1/2 pointer-events-none text-gray-500" />
			</div>
		</div>

		<div class="w-px h-5 bg-gray-300 mx-1 flex-shrink-0"></div>

		<!-- BISU (always visible) -->
		<div class="flex items-center gap-0.5">
			<button
				type="button"
				onclick={() => editor?.chain().focus().toggleBold().run()}
				title="Bold"
				aria-label="Bold"
				class="{btnBase} font-bold"
				class:!bg-blue-600={activeStates.bold}
				class:!text-white={activeStates.bold}
				class:!font-semibold={activeStates.bold}
			>
				B
			</button>
			<button
				type="button"
				onclick={() => editor?.chain().focus().toggleItalic().run()}
				title="Italic"
				aria-label="Italic"
				class="{btnBase} italic"
				class:!bg-blue-600={activeStates.italic}
				class:!text-white={activeStates.italic}
				class:!font-semibold={activeStates.italic}
			>
				I
			</button>
			<button
				type="button"
				onclick={() => editor?.chain().focus().toggleStrike().run()}
				title="Strikethrough"
				aria-label="Strikethrough"
				class="{btnBase} line-through"
				class:!bg-blue-600={activeStates.strike}
				class:!text-white={activeStates.strike}
				class:!font-semibold={activeStates.strike}
			>
				S
			</button>
			<button
				type="button"
				onclick={() => editor?.chain().focus().toggleUnderline().run()}
				title="Underline"
				aria-label="Underline"
				class="{btnBase} underline"
				class:!bg-blue-600={activeStates.underline}
				class:!text-white={activeStates.underline}
				class:!font-semibold={activeStates.underline}
			>
				U
			</button>
		</div>

		<!-- Mobile/Compact "more" toggle -->
		{#if !compact}
			<button
				type="button"
				class="flex ml-auto p-1.5 border-0 bg-transparent cursor-pointer rounded text-gray-600 flex-shrink-0 items-center justify-center hover:bg-gray-200 xl:hidden"
				onclick={onToggleMenu}
				aria-label="More options"
			>
				<MoreHorizontal size={18} />
			</button>
		{/if}
	</div>

	<!-- Desktop toolbar content / Mobile expandable content (hidden in compact mode) -->
	{#if !compact}
		<div class="hidden xl:flex xl:items-center xl:gap-1 absolute top-full left-0 right-0 flex-col gap-2 p-3 bg-white border border-gray-300 border-t-0 shadow-lg z-10 xl:static xl:w-auto xl:flex-row xl:p-0 xl:bg-transparent xl:border-0 xl:shadow-none" class:flex={menuExpanded} class:hidden={!menuExpanded}>
		<div class="hidden xl:block w-px h-5 bg-gray-300 mx-1 flex-shrink-0"></div>

		<!-- Lists -->
		<div class="flex items-center gap-0.5">
			<button
				type="button"
				onclick={() => editor?.chain().focus().toggleBulletList().run()}
				title="Bullet List"
				aria-label="Bullet List"
				class="{btnBase}"
				class:!bg-blue-600={activeStates.bulletList}
				class:!text-white={activeStates.bulletList}
				class:!font-semibold={activeStates.bulletList}
			>
				<List size={16} />
			</button>
			<button
				type="button"
				onclick={() => editor?.chain().focus().toggleOrderedList().run()}
				title="Numbered List"
				aria-label="Numbered List"
				class="{btnBase}"
				class:!bg-blue-600={activeStates.orderedList}
				class:!text-white={activeStates.orderedList}
				class:!font-semibold={activeStates.orderedList}
			>
				<ListOrdered size={16} />
			</button>
		</div>

		<div class="hidden xl:block w-px h-5 bg-gray-300 mx-1 flex-shrink-0"></div>

		<!-- Text Alignment -->
		<div class="flex items-center gap-0.5">
			<button
				type="button"
				onclick={() => editor?.chain().focus().setTextAlign('left').run()}
				title="Align Left"
				aria-label="Align Left"
				class="{btnBase}"
				class:!bg-blue-600={activeStates.textAlign === 'left'}
				class:!text-white={activeStates.textAlign === 'left'}
				class:!font-semibold={activeStates.textAlign === 'left'}
			>
				<AlignLeft size={16} />
			</button>
			<button
				type="button"
				onclick={() => editor?.chain().focus().setTextAlign('center').run()}
				title="Align Center"
				aria-label="Align Center"
				class="{btnBase}"
				class:!bg-blue-600={activeStates.textAlign === 'center'}
				class:!text-white={activeStates.textAlign === 'center'}
				class:!font-semibold={activeStates.textAlign === 'center'}
			>
				<AlignCenter size={16} />
			</button>
			<button
				type="button"
				onclick={() => editor?.chain().focus().setTextAlign('right').run()}
				title="Align Right"
				aria-label="Align Right"
				class="{btnBase}"
				class:!bg-blue-600={activeStates.textAlign === 'right'}
				class:!text-white={activeStates.textAlign === 'right'}
				class:!font-semibold={activeStates.textAlign === 'right'}
			>
				<AlignRight size={16} />
			</button>
			<button
				type="button"
				onclick={() => editor?.chain().focus().setTextAlign('justify').run()}
				title="Justify"
				aria-label="Justify"
				class="{btnBase}"
				class:!bg-blue-600={activeStates.textAlign === 'justify'}
				class:!text-white={activeStates.textAlign === 'justify'}
				class:!font-semibold={activeStates.textAlign === 'justify'}
			>
				<AlignJustify size={16} />
			</button>
		</div>

		<div class="hidden xl:block w-px h-5 bg-gray-300 mx-1 flex-shrink-0"></div>

		<!-- Blockquote -->
		<div class="flex items-center gap-0.5">
			<button
				type="button"
				onclick={() => editor?.chain().focus().toggleBlockquote().run()}
				title="Blockquote"
				aria-label="Blockquote"
				class="{btnBase}"
				class:!bg-blue-600={activeStates.blockquote}
				class:!text-white={activeStates.blockquote}
				class:!font-semibold={activeStates.blockquote}
			>
				<Quote size={16} />
			</button>
		</div>

		<div class="hidden xl:block w-px h-5 bg-gray-300 mx-1 flex-shrink-0"></div>

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
					class="{btnBase}"
					class:!bg-blue-600={activeStates.link}
					class:!text-white={activeStates.link}
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
				<button
					type="button"
					title="Add Image"
					aria-label="Add Image"
					class="{btnBase}"
				>
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
				<button
					type="button"
					title="Add Video"
					aria-label="Add Video"
					class="{btnBase}"
				>
					<Video size={16} />
				</button>
			</UrlInputPopover>
		</div>


		</div>
	{/if}
</div>
