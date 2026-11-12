<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import type { ComponentType } from 'svelte';
	import type { Icon as LucideIcon } from 'lucide-svelte';
	import ImageIcon from 'lucide-svelte/icons/image';
	import Video from 'lucide-svelte/icons/video';
	import Music from 'lucide-svelte/icons/music';
	import TableIcon from 'lucide-svelte/icons/table';
	import LinkIcon from 'lucide-svelte/icons/link';
	import Unlink from 'lucide-svelte/icons/unlink';
	import ChartNoAxesColumn from 'lucide-svelte/icons/chart-no-axes-column';
	import UrlInputPopover from '$lib/components/RichTextEditor/UrlInputPopover/UrlInputPopover.svelte';
	import TableControlsPopover from '$lib/components/RichTextEditor/TableControlsPopover.svelte';
	import { insertDefaultTable } from '$lib/components/RichTextEditor/tableActions';
	import { isVideoFileUrl } from '$lib/utils/videoUrl';
	import { cn } from '$lib/utils';
	import { trackEditorTransactions } from './editorTransactions.svelte';
	import { HISTORY_TOOLS, PAGE_BREAK_TOOL, TEXT_TOOL_GROUPS, type Tool } from './toolbarTools';

	type Props = {
		/** The editor the cursor is in: the summary or the body. */
		editor: Editor | undefined;
		/** Charts and page breaks only go in the body, not the summary. */
		inBody: boolean;
		onAddChart: () => void;
	};
	let { editor, inBody, onAddChart }: Props = $props();

	const BUTTON_CLASS =
		'text-muted-foreground hover:bg-accent hover:text-foreground flex size-9 shrink-0 items-center justify-center rounded-full transition-colors disabled:pointer-events-none disabled:opacity-40';
	const ACTIVE_CLASS = 'bg-accent text-foreground';
	const ONLY_IN_BODY = 'Goes in the report body, not the summary';

	const transactions = trackEditorTransactions(() => editor);
	const inTable = $derived(transactions.isActive((current) => current.isActive('table')));
	const inLink = $derived(transactions.isActive((current) => current.isActive('link')));

	function insertVideo(from: 'library' | 'external-url', url: string) {
		if (from === 'library' || isVideoFileUrl(url)) {
			editor?.chain().focus().setVideo({ src: url }).run();
		} else {
			editor?.chain().focus().setIframe({ src: url }).run();
		}
	}
</script>

{#snippet divider()}
	<div class="bg-border mx-1 h-5 w-px shrink-0" aria-hidden="true"></div>
{/snippet}

{#snippet toolButton(tool: Tool, disabled = false)}
	{@const active = transactions.isActive(tool.isActive)}
	{@const nothingToDo = transactions.isActive((current) => tool.isDisabled?.(current) ?? false)}
	<button
		type="button"
		class={cn(BUTTON_CLASS, active && ACTIVE_CLASS)}
		title={disabled ? ONLY_IN_BODY : tool.label}
		aria-label={tool.label}
		aria-pressed={active}
		disabled={!editor || disabled || nothingToDo}
		onclick={() => editor && tool.run(editor)}
	>
		<tool.Icon class="size-5" />
	</button>
{/snippet}

<!-- The popovers wrap this in their own button, which takes its name from the hidden text. -->
{#snippet popoverTrigger(label: string, Icon: ComponentType<LucideIcon>, active = false)}
	<span class={cn(BUTTON_CLASS, active && ACTIVE_CLASS)} title={label}
		><Icon class="size-5" aria-hidden="true" /><span class="sr-only">{label}</span></span
	>
{/snippet}

<!-- Pressing a tool keeps the editor's selection, so each tool acts on it. -->
<div
	role="toolbar"
	aria-label="Editing tools"
	tabindex="-1"
	class="flex items-center gap-0.5 overflow-x-auto [scrollbar-width:none]"
	onmousedown={(event) => {
		if (!(event.target instanceof HTMLInputElement)) event.preventDefault();
	}}
>
	<button
		type="button"
		class="bg-accent text-primary hover:bg-accent/70 flex h-9 shrink-0 items-center gap-2 rounded-full px-3 text-base font-medium transition-colors disabled:opacity-40"
		title={inBody ? 'Add a chart' : ONLY_IN_BODY}
		disabled={!inBody}
		onclick={onAddChart}
	>
		<ChartNoAxesColumn class="size-5" />Chart
	</button>

	{@render divider()}
	{#each HISTORY_TOOLS as tool (tool.label)}
		{@render toolButton(tool)}
	{/each}

	{#each TEXT_TOOL_GROUPS as group, groupIndex (groupIndex)}
		{@render divider()}
		{#each group as tool (tool.label)}
			{@render toolButton(tool)}
		{/each}
	{/each}

	{@render divider()}
	{#if inLink}
		<button
			type="button"
			class={cn(BUTTON_CLASS, ACTIVE_CLASS)}
			title="Remove link"
			aria-label="Remove link"
			onclick={() => editor?.chain().focus().extendMarkRange('link').unsetLink().run()}
		>
			<Unlink class="size-5" />
		</button>
	{:else}
		<UrlInputPopover
			type="link"
			onSubmit={({ url }) =>
				editor?.chain().focus().extendMarkRange('link').setLink({ href: url }).run()}
		>
			{@render popoverTrigger('Link', LinkIcon)}
		</UrlInputPopover>
	{/if}

	{@render divider()}
	{@render toolButton(PAGE_BREAK_TOOL, !inBody)}
	<UrlInputPopover
		type="image"
		allowLocalSelection
		onSubmit={({ url }) => editor?.chain().focus().setImage({ src: url }).run()}
	>
		{@render popoverTrigger('Image', ImageIcon)}
	</UrlInputPopover>
	<UrlInputPopover
		type="video"
		allowLocalSelection
		onSubmit={({ from, url }) => insertVideo(from, url)}
	>
		{@render popoverTrigger('Video', Video)}
	</UrlInputPopover>
	<UrlInputPopover
		type="audio"
		allowLocalSelection
		onSubmit={({ url }) =>
			editor?.chain().focus().setAudio({ src: url, autoplay: false, controls: true }).run()}
	>
		{@render popoverTrigger('Audio', Music)}
	</UrlInputPopover>
	<!-- Outside a table the click inserts one; inside a table it opens the table menu. -->
	<TableControlsPopover {editor}>
		<span
			role="presentation"
			class={cn(BUTTON_CLASS, inTable && ACTIVE_CLASS)}
			title={inTable ? 'Table options' : 'Table'}
			onclick={(event) => {
				if (!inTable && editor) {
					event.stopPropagation();
					insertDefaultTable(editor);
				}
			}}
		>
			<TableIcon class="size-5" aria-hidden="true" />
			<span class="sr-only">{inTable ? 'Table options' : 'Table'}</span>
		</span>
	</TableControlsPopover>
</div>
