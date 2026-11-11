<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import { NodeSelection } from '@tiptap/pm/state';
	import AlignLeft from 'lucide-svelte/icons/align-left';
	import AlignCenter from 'lucide-svelte/icons/align-center';
	import AlignRight from 'lucide-svelte/icons/align-right';
	import { cn } from '$lib/utils';
	import {
		IMAGE_ALIGN_OPTIONS,
		IMAGE_WIDTH_OPTIONS,
		type ImageAlign,
		type ImageWidth
	} from './extensions/image';

	type Props = { editor: Editor | undefined };
	let { editor }: Props = $props();

	type Bar = { x: number; y: number; align: ImageAlign | null; width: ImageWidth | null };

	let bar = $state<Bar | null>(null);

	const ALIGN_ICONS = { left: AlignLeft, center: AlignCenter, right: AlignRight } as const;
	const BUTTON_CLASS =
		'text-muted-foreground hover:bg-accent hover:text-foreground flex h-8 min-w-8 items-center justify-center rounded-md px-1.5 text-sm transition-colors';
	const ACTIVE_CLASS =
		'bg-primary text-primary-foreground hover:bg-primary/90 hover:text-primary-foreground';

	function selectedImagePosition(current: Editor): number | null {
		const selection = current.state.selection;
		if (!(selection instanceof NodeSelection)) return null;
		return selection.node.type.name === 'image' ? selection.from : null;
	}

	// The bar sits just inside the top of the selected image, placed from its live position.
	function measure(current: Editor) {
		const position = current.isEditable ? selectedImagePosition(current) : null;
		const dom = position === null ? null : current.view.nodeDOM(position);
		if (!(dom instanceof HTMLElement)) {
			bar = null;
			return;
		}
		const rect = dom.getBoundingClientRect();
		const attributes = current.getAttributes('image');
		bar = {
			x: rect.left + rect.width / 2,
			y: rect.top,
			align: (attributes.align as ImageAlign | null) ?? null,
			width: (attributes.width as ImageWidth | null) ?? null
		};
	}

	// Pressing the option that is already on goes back to the default layout.
	function setAttribute(name: 'align' | 'width', value: string) {
		const next = bar?.[name] === value ? null : value;
		editor
			?.chain()
			.focus()
			.updateAttributes('image', { [name]: next })
			.run();
	}

	$effect(() => {
		const current = editor;
		if (!current) return;
		const update = () => measure(current);
		current.on('transaction', update);
		window.addEventListener('scroll', update, true);
		window.addEventListener('resize', update);
		update();
		return () => {
			current.off('transaction', update);
			window.removeEventListener('scroll', update, true);
			window.removeEventListener('resize', update);
		};
	});
</script>

{#if bar}
	<div
		role="toolbar"
		aria-label="Image layout"
		class="bg-popover text-popover-foreground fixed z-40 flex -translate-x-1/2 translate-y-2 items-center gap-0.5 rounded-lg border p-1 shadow-md"
		style:left="{bar.x}px"
		style:top="{bar.y}px"
	>
		{#each IMAGE_ALIGN_OPTIONS as option (option.key)}
			{@const Icon = ALIGN_ICONS[option.key]}
			{@const active = bar.align === option.key}
			<button
				type="button"
				class={cn(BUTTON_CLASS, active && ACTIVE_CLASS)}
				title={option.label}
				aria-label={option.label}
				aria-pressed={active}
				onclick={() => setAttribute('align', option.key)}
			>
				<Icon class="size-4" />
			</button>
		{/each}
		<div class="bg-border mx-0.5 h-4 w-px" aria-hidden="true"></div>
		{#each IMAGE_WIDTH_OPTIONS as option (option.key)}
			{@const active = bar.width === option.key}
			<button
				type="button"
				class={cn(BUTTON_CLASS, active && ACTIVE_CLASS)}
				title="Width {option.label}"
				aria-label="Width {option.label}"
				aria-pressed={active}
				onclick={() => setAttribute('width', option.key)}
			>
				{option.label}
			</button>
		{/each}
	</div>
{/if}
