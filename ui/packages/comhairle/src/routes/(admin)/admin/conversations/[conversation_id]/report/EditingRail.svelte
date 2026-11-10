<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import type { ComponentType } from 'svelte';
	import type { Icon as LucideIcon } from 'lucide-svelte';
	import Pilcrow from 'lucide-svelte/icons/pilcrow';
	import Heading2 from 'lucide-svelte/icons/heading-2';
	import Heading3 from 'lucide-svelte/icons/heading-3';
	import SeparatorHorizontal from 'lucide-svelte/icons/separator-horizontal';
	import ImageIcon from 'lucide-svelte/icons/image';
	import Video from 'lucide-svelte/icons/video';
	import Music from 'lucide-svelte/icons/music';
	import TableIcon from 'lucide-svelte/icons/table';
	import ChartNoAxesColumn from 'lucide-svelte/icons/chart-no-axes-column';
	import UrlInputPopover from '$lib/components/RichTextEditor/UrlInputPopover/UrlInputPopover.svelte';
	import TableControlsPopover from '$lib/components/RichTextEditor/TableControlsPopover.svelte';
	import { insertDefaultTable } from '$lib/components/RichTextEditor/tableActions';
	import { isVideoFileUrl } from '$lib/utils/videoUrl';
	import { cn } from '$lib/utils';
	import { trackEditorTransactions } from './editorTransactions.svelte';

	/** Block tools for the report body. Text formatting lives in the selection bubble. */
	type Props = { editor: Editor | undefined; onAddChart: () => void };
	let { editor, onAddChart }: Props = $props();

	type Tool = {
		label: string;
		Icon: ComponentType<LucideIcon>;
		isActive: (current: Editor) => boolean;
		run: (current: Editor) => void;
	};

	const STRUCTURE_TOOLS: Tool[] = [
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
		},
		{
			// A horizontal rule starts a new card that keeps the section title (ADR-0058).
			label: 'Page break',
			Icon: SeparatorHorizontal,
			isActive: () => false,
			run: (e) => e.chain().focus().setHorizontalRule().run()
		}
	];

	const RAIL_BUTTON_CLASS =
		'text-muted-foreground hover:bg-accent hover:text-foreground flex size-10 items-center justify-center rounded-lg transition-colors';
	const ACTIVE_CLASS = 'bg-accent text-foreground';

	const transactions = trackEditorTransactions(() => editor);
	const inTable = $derived(transactions.isActive((e) => e.isActive('table')));

	function insertVideo(from: 'library' | 'external-url', url: string) {
		if (from === 'library' || isVideoFileUrl(url)) {
			editor?.chain().focus().setVideo({ src: url }).run();
		} else {
			editor?.chain().focus().setIframe({ src: url }).run();
		}
	}
</script>

{#snippet divider()}
	<div class="bg-border my-1 h-px w-6" aria-hidden="true"></div>
{/snippet}

{#snippet railIcon(label: string, Icon: ComponentType<LucideIcon>)}
	<span class={RAIL_BUTTON_CLASS} title={label} aria-label={label}><Icon class="size-5" /></span>
{/snippet}

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

	{@render divider()}
	{#each STRUCTURE_TOOLS as tool (tool.label)}
		{@const active = transactions.isActive(tool.isActive)}
		<button
			type="button"
			class={cn(RAIL_BUTTON_CLASS, active && ACTIVE_CLASS)}
			title={tool.label}
			aria-label={tool.label}
			aria-pressed={active}
			disabled={!editor}
			onmousedown={(event) => event.preventDefault()}
			onclick={() => editor && tool.run(editor)}
		>
			<tool.Icon class="size-5" />
		</button>
	{/each}

	{@render divider()}
	<UrlInputPopover
		type="image"
		allowLocalSelection
		onSubmit={({ url }) => editor?.chain().focus().setImage({ src: url }).run()}
	>
		{@render railIcon('Image', ImageIcon)}
	</UrlInputPopover>
	<UrlInputPopover
		type="video"
		allowLocalSelection
		onSubmit={({ from, url }) => insertVideo(from, url)}
	>
		{@render railIcon('Video', Video)}
	</UrlInputPopover>
	<UrlInputPopover
		type="audio"
		allowLocalSelection
		onSubmit={({ url }) =>
			editor?.chain().focus().setAudio({ src: url, autoplay: false, controls: true }).run()}
	>
		{@render railIcon('Audio', Music)}
	</UrlInputPopover>
	<!-- Outside a table the click inserts one; inside a table it opens the table menu. -->
	<TableControlsPopover {editor}>
		<span
			role="presentation"
			class={cn(RAIL_BUTTON_CLASS, inTable && ACTIVE_CLASS)}
			title={inTable ? 'Table options' : 'Table'}
			aria-label={inTable ? 'Table options' : 'Table'}
			onclick={(event) => {
				if (!inTable && editor) {
					event.stopPropagation();
					insertDefaultTable(editor);
				}
			}}
		>
			<TableIcon class="size-5" />
		</span>
	</TableControlsPopover>
</nav>
