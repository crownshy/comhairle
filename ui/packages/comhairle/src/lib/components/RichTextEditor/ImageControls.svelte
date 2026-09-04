<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import { NodeSelection } from '@tiptap/pm/state';
	import { AlignLeft, AlignCenter, AlignRight } from 'lucide-svelte';
	import {
		IMAGE_ALIGN_OPTIONS,
		IMAGE_WIDTH_OPTIONS,
		type ImageAlign,
		type ImageWidth
	} from './extensions/image';

	type Props = {
		editor: Editor;
	};

	let { editor }: Props = $props();

	// A small bar floating just inside the top edge of the selected image. Positioned
	// with `fixed` from the image's live rect (same approach as TableInsertControls)
	// so it tracks the image through scrolling inside the editor's own scroll
	// container. Inside rather than above: the editor scrolls the selected image to
	// the top of its box, where a bar above it would sit on the toolbar.
	type Bar = {
		x: number;
		y: number;
		align: ImageAlign | null;
		width: ImageWidth | null;
	};

	let bar = $state<Bar | null>(null);

	const alignIcons = { left: AlignLeft, center: AlignCenter, right: AlignRight } as const;

	function selectedImagePos(): number | null {
		const selection = editor.state.selection;
		if (!(selection instanceof NodeSelection)) return null;
		return selection.node.type.name === 'image' ? selection.from : null;
	}

	function update() {
		if (!editor.isEditable) {
			bar = null;
			return;
		}
		const pos = selectedImagePos();
		if (pos === null) {
			bar = null;
			return;
		}
		const dom = editor.view.nodeDOM(pos);
		if (!(dom instanceof HTMLElement)) {
			bar = null;
			return;
		}
		const rect = dom.getBoundingClientRect();
		const attrs = editor.getAttributes('image');
		bar = {
			x: rect.left + rect.width / 2,
			y: rect.top,
			align: (attrs.align as ImageAlign | null) ?? null,
			width: (attrs.width as ImageWidth | null) ?? null
		};
	}

	function setAlign(align: ImageAlign) {
		// Clicking the active alignment clears it back to the document default.
		const next = bar?.align === align ? null : align;
		editor.chain().focus().updateAttributes('image', { align: next }).run();
	}

	function setWidth(width: ImageWidth) {
		const next = bar?.width === width ? null : width;
		editor.chain().focus().updateAttributes('image', { width: next }).run();
	}

	$effect(() => {
		if (!editor) return;

		// `transaction` covers both selection moves and attribute updates; scroll and
		// resize just move the image on screen, so re-measure.
		editor.on('transaction', update);
		window.addEventListener('scroll', update, true);
		window.addEventListener('resize', update);
		// Clicking an already-selected image dispatches nothing, so measure once up front.
		update();

		return () => {
			editor.off('transaction', update);
			window.removeEventListener('scroll', update, true);
			window.removeEventListener('resize', update);
		};
	});
</script>

{#if bar}
	<div
		class="image-controls"
		style="left: {bar.x}px; top: {bar.y}px;"
		role="toolbar"
		aria-label="Image layout"
	>
		{#each IMAGE_ALIGN_OPTIONS as option (option.key)}
			{@const Icon = alignIcons[option.key]}
			<button
				type="button"
				class="image-controls__btn"
				class:image-controls__btn--active={bar.align === option.key}
				title={option.label}
				aria-label={option.label}
				aria-pressed={bar.align === option.key}
				onclick={() => setAlign(option.key)}
			>
				<Icon size={14} />
			</button>
		{/each}

		<div class="image-controls__divider"></div>

		{#each IMAGE_WIDTH_OPTIONS as option (option.key)}
			<button
				type="button"
				class="image-controls__btn"
				class:image-controls__btn--active={bar.width === option.key}
				title="Width {option.label}"
				aria-label="Width {option.label}"
				aria-pressed={bar.width === option.key}
				onclick={() => setWidth(option.key)}
			>
				{option.label}
			</button>
		{/each}
	</div>
{/if}

<style>
	.image-controls {
		position: fixed;
		z-index: 40;
		display: inline-flex;
		align-items: center;
		gap: 0.125rem;
		padding: 0.25rem;
		border: 1px solid var(--color-border);
		border-radius: 0.5rem;
		background: var(--color-popover);
		color: var(--color-popover-foreground);
		box-shadow: 0 4px 16px rgb(0 0 0 / 0.18);
		/* Anchored at the image's top-centre; centre it and nudge it down inside. */
		transform: translate(-50%, 8px);
	}

	.image-controls__btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		height: 1.75rem;
		min-width: 1.75rem;
		padding: 0 0.375rem;
		border: 0;
		border-radius: 0.25rem;
		background: transparent;
		color: var(--color-muted-foreground);
		font-size: 0.75rem;
		line-height: 1;
		cursor: pointer;
	}

	.image-controls__btn:hover {
		background: var(--color-accent);
		color: var(--color-accent-foreground);
	}

	.image-controls__btn--active {
		background: var(--color-primary);
		color: var(--color-primary-foreground);
	}

	.image-controls__divider {
		width: 1px;
		height: 1rem;
		margin: 0 0.125rem;
		background: var(--color-border);
	}
</style>
