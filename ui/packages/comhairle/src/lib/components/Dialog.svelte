<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { X } from 'lucide-svelte';
	import { cn } from '$lib/utils';

	type Action = {
		label: string;
		onclick?: () => void;
	};

	type Props = {
		id: string;
		ref?: HTMLDialogElement | null;
		title?: string;
		class?: string;
		children?: Snippet<[]>;
		actions?: {
			primary?: Action;
			secondary?: Action;
		};
		HeaderActions?: Snippet<[]>;
		PrimaryAction?: Snippet<[]>;
		SecondaryAction?: Snippet<[]>;
		// If true, clicking outside of the Dialog won't dismiss it
		onlyHardDismiss?: boolean;
	};

	let {
		id,
		ref = $bindable(),
		title,
		class: classNames,
		actions,
		children,
		HeaderActions,
		PrimaryAction,
		SecondaryAction,
		onlyHardDismiss
	}: Props = $props();

	export const open = () => ref?.showModal();
	export const close = () => ref?.close();
</script>

<dialog
	{id}
	bind:this={ref}
	class="bg-background border-border relative m-auto overflow-y-auto rounded-2xl border p-3 pt-0"
	style="max-width: var(--max-width, none); min-width: var(--min-width, none); max-height: var(--max-height, none); min-height: var(--min-height,  '40vh');"
	closedby={onlyHardDismiss ? 'closerequest' : 'any'}
>
	<Button
		class="absolute top-3 right-3 h-4 w-4 rounded-sm px-0! py-0 text-sm font-medium hover:opacity-80 focus-visible:opacity-80"
		plain
		command={`close-dialog:${id}`}
		icon={X}
		title="close"
		aria-label="Close dialog"
	/>
	<div class={cn('grid grid-rows-[auto_1fr_auto] p-3 pt-5 pr-5', classNames)}>
		{#if title || HeaderActions}
			<header
				class="border-border bg-background sticky top-0 z-2 mb-5 flex flex-col-reverse items-center justify-between border-b-2 pt-4 pb-3 md:flex-row"
			>
				<span class="shrink-0 grow text-3xl font-medium">{title}</span>
				<span class="flex shrink grow-0 flex-row items-center gap-x-2">
					{#if HeaderActions}
						{@render HeaderActions()}
					{/if}
				</span>
			</header>
		{:else}
			<header></header>
		{/if}
		<section>
			{@render children?.()}
		</section>
		{#if PrimaryAction || actions?.primary}
			<footer class="flex flex-row justify-end gap-x-2">
				{#if SecondaryAction}
					{@render SecondaryAction()}
				{:else if actions?.secondary}
					<Button
						onclick={() => {
							actions?.secondary?.onclick?.();
						}}
						variant="outline"
						command={`close-dialog:${id}`}
					>
						{actions.secondary.label}
					</Button>
				{/if}
				{#if PrimaryAction}
					{@render PrimaryAction()}
				{:else if actions?.primary}
					<Button
						onclick={() => {
							actions?.primary?.onclick?.();
						}}
						command={`close-dialog:${id}`}
					>
						{actions.primary.label}
					</Button>
				{/if}
			</footer>
		{/if}
	</div>
</dialog>

<style>
	dialog {
		--_distance: 15px;
		--_duration: 300ms;
		--_shrink: 0.95;

		opacity: 0;
		translate: 0 0;
		scale: var(--_shrink);
		transition: opacity, display, overlay, translate, scale;
		transition-behavior: allow-discrete;
		transition-duration: var(--_duration);

		&::backdrop {
			opacity: 0;
			transition: opacity var(--_duration);
			background-color: oklch(from var(--background) l c h / 0.7);
			backdrop-filter: blur(5px);
		}

		&[open] {
			opacity: 1;
			translate: 0 0;
			scale: 1;

			&::backdrop {
				opacity: 1;
			}

			@starting-style {
				opacity: 0;
				translate: 0 var(--_distance);
				scale: 1;

				&::backdrop {
					opacity: 0;
				}
			}
		}
	}
</style>
