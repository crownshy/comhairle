<script lang="ts">
	// Only what a report needs; alignment, tables and media are left out to keep the rail short.
	import type { Editor } from '@tiptap/core';
	import type { ComponentType } from 'svelte';
	import type { Icon as LucideIcon } from 'lucide-svelte';
	import Pilcrow from 'lucide-svelte/icons/pilcrow';
	import Heading2 from 'lucide-svelte/icons/heading-2';
	import Heading3 from 'lucide-svelte/icons/heading-3';
	import Bold from 'lucide-svelte/icons/bold';
	import Italic from 'lucide-svelte/icons/italic';
	import List from 'lucide-svelte/icons/list';
	import ListOrdered from 'lucide-svelte/icons/list-ordered';
	import Quote from 'lucide-svelte/icons/quote';
	import LinkIcon from 'lucide-svelte/icons/link';
	import ChartNoAxesColumn from 'lucide-svelte/icons/chart-no-axes-column';
	import UrlInputPopover from '$lib/components/RichTextEditor/UrlInputPopover/UrlInputPopover.svelte';
	import { cn } from '$lib/utils';

	type Props = { editor: Editor | undefined; onAddChart: () => void };
	let { editor, onAddChart }: Props = $props();

	type Tool = {
		label: string;
		Icon: ComponentType<LucideIcon>;
		isActive: (current: Editor) => boolean;
		run: (current: Editor) => void;
	};

	const TOOL_GROUPS: Tool[][] = [
		[
			{
				label: 'Text',
				Icon: Pilcrow,
				isActive: (e) => e.isActive('paragraph'),
				run: (e) => e.chain().focus().setParagraph().run()
			},
			{
				label: 'Section (a new pill)',
				Icon: Heading2,
				isActive: (e) => e.isActive('heading', { level: 2 }),
				run: (e) => e.chain().focus().toggleHeading({ level: 2 }).run()
			},
			{
				label: 'Card (a new page)',
				Icon: Heading3,
				isActive: (e) => e.isActive('heading', { level: 3 }),
				run: (e) => e.chain().focus().toggleHeading({ level: 3 }).run()
			}
		],
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
		]
	];

	const RAIL_BUTTON_CLASS =
		'text-muted-foreground hover:bg-accent hover:text-foreground flex size-10 items-center justify-center rounded-lg transition-colors';
	const ACTIVE_CLASS = 'bg-accent text-foreground';

	// Bumped on every transaction so the active states re-read the editor.
	let revision = $state(0);
	$effect(() => {
		const current = editor;
		if (!current) return;
		const bump = () => queueMicrotask(() => (revision += 1));
		current.on('transaction', bump);
		return () => current.off('transaction', bump);
	});

	function active(tool: Tool) {
		void revision;
		return !!editor && !editor.isDestroyed && tool.isActive(editor);
	}
</script>

<nav
	aria-label="Editing tools"
	class="bg-card flex flex-col items-center gap-1 rounded-2xl border p-1.5 shadow-sm"
>
	<button
		type="button"
		class="bg-primary text-primary-foreground hover:bg-primary/90 flex size-10 items-center justify-center rounded-lg"
		title="Add a chart"
		aria-label="Add a chart"
		onclick={onAddChart}
	>
		<ChartNoAxesColumn class="size-5" />
	</button>
	{#each TOOL_GROUPS as group, groupIndex (groupIndex)}
		<div class="bg-border my-1 h-px w-6" aria-hidden="true"></div>
		{#each group as tool (tool.label)}
			<button
				type="button"
				class={cn(RAIL_BUTTON_CLASS, active(tool) && ACTIVE_CLASS)}
				title={tool.label}
				aria-label={tool.label}
				aria-pressed={active(tool)}
				disabled={!editor}
				onmousedown={(event) => event.preventDefault()}
				onclick={() => editor && tool.run(editor)}
			>
				<tool.Icon class="size-5" />
			</button>
		{/each}
	{/each}
	<UrlInputPopover
		type="link"
		onSubmit={(url) => editor?.chain().focus().setLink({ href: url }).run()}
	>
		<span class={RAIL_BUTTON_CLASS} title="Link" aria-label="Link"
			><LinkIcon class="size-5" /></span
		>
	</UrlInputPopover>
</nav>
