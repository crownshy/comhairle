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
		readImageLayout,
		type ImageLayout
	} from './extensions/image';
	import { ACTIVE_BUTTON_CLASS, TOOLBAR_BUTTON_CLASS } from './styles';

	type Props = { editor: Editor | undefined };
	let { editor }: Props = $props();

	type LayoutButtonProps = {
		label: string;
		active: boolean;
		onclick: () => void;
		Icon?: typeof AlignLeft;
		text?: string;
	};

	type SelectedImage = { x: number; y: number; layout: ImageLayout };

	let selectedImage = $state<SelectedImage | null>(null);

	const ALIGN_ICONS = { left: AlignLeft, center: AlignCenter, right: AlignRight } as const;
	const MIN_VISIBLE_IMAGE_PX = 48;

	function selectedImagePosition(current: Editor): number | null {
		const selection = current.state.selection;
		if (!(selection instanceof NodeSelection)) return null;
		return selection.node.type.name === 'image' ? selection.from : null;
	}

	// The top edge of the area the image is visible in, so the bar never floats above a scroll box.
	function visibleTop(element: HTMLElement): number {
		let top = 0;
		for (let parent = element.parentElement; parent; parent = parent.parentElement) {
			const overflow = getComputedStyle(parent).overflowY;
			if (overflow === 'auto' || overflow === 'scroll') {
				top = Math.max(top, parent.getBoundingClientRect().top);
			}
		}
		return top;
	}

	function readSelectedImage(current: Editor): SelectedImage | null {
		const position = current.isEditable ? selectedImagePosition(current) : null;
		const dom = position === null ? null : current.view.nodeDOM(position);
		if (!(dom instanceof HTMLElement)) return null;
		const rect = dom.getBoundingClientRect();
		const y = Math.max(rect.top, visibleTop(dom));
		if (rect.bottom - y < MIN_VISIBLE_IMAGE_PX) return null;
		return {
			x: rect.left + rect.width / 2,
			y,
			layout: readImageLayout(current.getAttributes('image'))
		};
	}

	// Pressing the option that is already on goes back to the default layout.
	function setLayout<Name extends keyof ImageLayout>(name: Name, value: ImageLayout[Name]) {
		const next = selectedImage?.layout[name] === value ? null : value;
		editor
			?.chain()
			.focus()
			.updateAttributes('image', { [name]: next })
			.run();
	}

	$effect(() => {
		const current = editor;
		if (!current) return;
		const update = () => (selectedImage = readSelectedImage(current));
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

{#snippet layoutButton({ label, active, onclick, Icon, text }: LayoutButtonProps)}
	<button
		type="button"
		class={cn(TOOLBAR_BUTTON_CLASS, 'px-1.5', active && ACTIVE_BUTTON_CLASS)}
		title={label}
		aria-label={label}
		aria-pressed={active}
		{onclick}
	>
		{#if Icon}
			<Icon class="size-4" />
		{:else}
			{text}
		{/if}
	</button>
{/snippet}

{#if selectedImage}
	{@const layout = selectedImage.layout}
	<div
		role="toolbar"
		aria-label="Image layout"
		class="bg-popover text-popover-foreground fixed z-40 flex -translate-x-1/2 translate-y-2 items-center gap-0.5 rounded-lg border p-1 shadow-md"
		style:left="{selectedImage.x}px"
		style:top="{selectedImage.y}px"
	>
		{#each IMAGE_ALIGN_OPTIONS as option (option.key)}
			{@render layoutButton({
				label: option.label,
				active: layout.align === option.key,
				onclick: () => setLayout('align', option.key),
				Icon: ALIGN_ICONS[option.key]
			})}
		{/each}
		<div class="bg-border mx-0.5 h-4 w-px" aria-hidden="true"></div>
		{#each IMAGE_WIDTH_OPTIONS as option (option.key)}
			{@render layoutButton({
				label: `Width ${option.label}`,
				active: layout.width === option.key,
				onclick: () => setLayout('width', option.key),
				text: option.label
			})}
		{/each}
	</div>
{/if}
