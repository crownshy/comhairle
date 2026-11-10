<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import type { ComponentType } from 'svelte';
	import type { Icon as LucideIcon } from 'lucide-svelte';
	import Bold from 'lucide-svelte/icons/bold';
	import Italic from 'lucide-svelte/icons/italic';
	import Underline from 'lucide-svelte/icons/underline';
	import Strikethrough from 'lucide-svelte/icons/strikethrough';
	import List from 'lucide-svelte/icons/list';
	import ListOrdered from 'lucide-svelte/icons/list-ordered';
	import Quote from 'lucide-svelte/icons/quote';
	import AlignLeft from 'lucide-svelte/icons/align-left';
	import AlignCenter from 'lucide-svelte/icons/align-center';
	import AlignRight from 'lucide-svelte/icons/align-right';
	import AlignJustify from 'lucide-svelte/icons/align-justify';
	import LinkIcon from 'lucide-svelte/icons/link';
	import Check from 'lucide-svelte/icons/check';
	import { cn } from '$lib/utils';
	import { validateUrl } from '$lib/utils/urlValidation';
	import { trackEditorTransactions } from './editorTransactions.svelte';

	type Props = { editor: Editor | undefined };
	let { editor }: Props = $props();

	type Tool = {
		label: string;
		Icon: ComponentType<LucideIcon>;
		isActive: (current: Editor) => boolean;
		run: (current: Editor) => void;
	};

	const TOOL_GROUPS: Tool[][] = [
		[
			{
				label: 'Bold',
				Icon: Bold,
				isActive: (e) => e.isActive('bold'),
				run: (e) => e.chain().focus().toggleBold().run()
			},
			{
				label: 'Italic',
				Icon: Italic,
				isActive: (e) => e.isActive('italic'),
				run: (e) => e.chain().focus().toggleItalic().run()
			},
			{
				label: 'Underline',
				Icon: Underline,
				isActive: (e) => e.isActive('underline'),
				run: (e) => e.chain().focus().toggleUnderline().run()
			},
			{
				label: 'Strikethrough',
				Icon: Strikethrough,
				isActive: (e) => e.isActive('strike'),
				run: (e) => e.chain().focus().toggleStrike().run()
			}
		],
		[
			{
				label: 'Bullet list',
				Icon: List,
				isActive: (e) => e.isActive('bulletList'),
				run: (e) => e.chain().focus().toggleBulletList().run()
			},
			{
				label: 'Numbered list',
				Icon: ListOrdered,
				isActive: (e) => e.isActive('orderedList'),
				run: (e) => e.chain().focus().toggleOrderedList().run()
			},
			{
				label: 'Quote',
				Icon: Quote,
				isActive: (e) => e.isActive('blockquote'),
				run: (e) => e.chain().focus().toggleBlockquote().run()
			}
		],
		(
			[
				['left', 'Align left', AlignLeft],
				['center', 'Align centre', AlignCenter],
				['right', 'Align right', AlignRight],
				['justify', 'Justify', AlignJustify]
			] as const
		).map(([alignment, label, Icon]) => ({
			label,
			Icon,
			isActive: (e: Editor) => e.isActive({ textAlign: alignment }),
			run: (e: Editor) => e.chain().focus().setTextAlign(alignment).run()
		}))
	];

	const BUBBLE_BUTTON_CLASS =
		'text-muted-foreground hover:bg-accent hover:text-foreground flex size-8 items-center justify-center rounded-md transition-colors';
	const ACTIVE_CLASS = 'bg-accent text-foreground';

	const transactions = trackEditorTransactions(() => editor);

	let editingLink = $state(false);
	let linkUrl = $state('');
	let linkError = $state<string | null>(null);

	// A new selection gets the tools again, not a link field left open for an old one.
	$effect(() => {
		const current = editor;
		if (!current) return;
		const closeLinkField = () => (editingLink = false);
		current.on('selectionUpdate', closeLinkField);
		return () => current.off('selectionUpdate', closeLinkField);
	});

	function startLink() {
		if (!editor) return;
		if (editor.isActive('link')) {
			editor.chain().focus().extendMarkRange('link').unsetLink().run();
			return;
		}
		linkUrl = '';
		linkError = null;
		editingLink = true;
	}

	function applyLink() {
		const url = validateUrl(linkUrl);
		if (!url) {
			linkError = 'Use a full https:// link';
			return;
		}
		editor?.chain().focus().extendMarkRange('link').setLink({ href: url }).run();
		editingLink = false;
	}

	function onLinkKeydown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			event.preventDefault();
			applyLink();
		} else if (event.key === 'Escape') {
			editingLink = false;
			editor?.commands.focus();
		}
	}
</script>

<!-- The editor keeps its selection while a button is pressed, so each tool acts on it. -->
<div
	role="toolbar"
	aria-label="Format text"
	tabindex="-1"
	class="bg-popover flex items-center gap-0.5 rounded-xl border p-1 shadow-md"
	onmousedown={(event) => {
		if (!(event.target instanceof HTMLInputElement)) event.preventDefault();
	}}
>
	{#if editingLink}
		<input
			type="url"
			class="bg-background h-8 w-64 rounded-md border px-2 text-base"
			placeholder="https://example.com"
			aria-label="Link address"
			aria-invalid={linkError !== null}
			bind:value={linkUrl}
			onkeydown={onLinkKeydown}
			{@attach (input) => input.focus()}
		/>
		<button type="button" class={BUBBLE_BUTTON_CLASS} aria-label="Add link" onclick={applyLink}>
			<Check class="size-4" />
		</button>
		{#if linkError}
			<span class="text-destructive px-2 text-sm">{linkError}</span>
		{/if}
	{:else}
		{#each TOOL_GROUPS as group, groupIndex (groupIndex)}
			{#if groupIndex > 0}
				<div class="bg-border mx-1 h-5 w-px" aria-hidden="true"></div>
			{/if}
			{#each group as tool (tool.label)}
				{@const active = transactions.isActive(tool.isActive)}
				<button
					type="button"
					class={cn(BUBBLE_BUTTON_CLASS, active && ACTIVE_CLASS)}
					title={tool.label}
					aria-label={tool.label}
					aria-pressed={active}
					onclick={() => editor && tool.run(editor)}
				>
					<tool.Icon class="size-4" />
				</button>
			{/each}
		{/each}
		<div class="bg-border mx-1 h-5 w-px" aria-hidden="true"></div>
		{@const linkActive = transactions.isActive((e) => e.isActive('link'))}
		<button
			type="button"
			class={cn(BUBBLE_BUTTON_CLASS, linkActive && ACTIVE_CLASS)}
			title={linkActive ? 'Remove link' : 'Link'}
			aria-label={linkActive ? 'Remove link' : 'Link'}
			aria-pressed={linkActive}
			onclick={startLink}
		>
			<LinkIcon class="size-4" />
		</button>
	{/if}
</div>
