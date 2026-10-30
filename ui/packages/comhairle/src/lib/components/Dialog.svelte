<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { X } from 'lucide-svelte';

	type Action = {
		label: string;
		onclick: () => void;
	};

	type SnippetArgs = {
		close: () => void;
	};

	type Props = {
		id: string;
		title?: string;
		children?: Snippet;
		actions?: {
			primary?: Action;
			secondary?: Action;
		};
		HeaderAction?: Snippet<[SnippetArgs]>;
		PrimaryAction?: Snippet<[SnippetArgs]>;
		SecondaryAction?: Snippet<[SnippetArgs]>;
	};

	let { id, title, actions, children, HeaderAction, PrimaryAction, SecondaryAction }: Props =
		$props();

	let Dialog = $state<HTMLDialogElement | null>(null);

	const close = () => Dialog?.close();
</script>

<dialog
	{id}
	bind:this={Dialog}
	class="bg-background border-border relative m-auto min-h-40 rounded-2xl border p-3"
	style="max-width: var(--max-width, none); min-width: var(--min-width, none); max-height: var(--max-height, none); min-height: var(--min-height, none);"
	closedby="any"
>
	<!-- Would be ideal for this to just be a plain button but for some reason the invoker commands don't work with plain buttons, no idea why-->
	<Button
		class="absolute inset-e-3 h-4 w-4 rounded-sm px-0! py-0 text-sm font-medium hover:opacity-80 focus-visible:opacity-80"
		variant="ghost"
		dialogClose={id}
		icon={X}
		title="close"
		aria-label="Close dialog"
	/>
	<div class="grid grid-rows-[auto_1fr_auto] p-3 pt-5 pr-5">
		{#if title || HeaderAction}
			<header
				class="border-border mb-5 flex flex-row items-center justify-between border-b-2 pb-2"
			>
				<span class="shrink-0 grow text-3xl font-medium">{title}</span>
				<span class="shrink grow-0">
					{#if HeaderAction}
						{@render HeaderAction({ close })}
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
					{@render SecondaryAction({ close })}
				{:else if actions?.secondary}
					<Button onclick={actions.secondary.onclick} variant="outline">
						{actions.secondary.label}
					</Button>
				{/if}
				{#if PrimaryAction}
					{@render PrimaryAction({ close })}
				{:else if actions?.primary}
					<Button onclick={actions.primary.onclick}>{actions.primary.label}</Button>
				{/if}
			</footer>
		{/if}
	</div>
</dialog>

<style>
	dialog {
		--_distance: 25px;
		--_duration: 400ms;

		opacity: 0;
		translate: 0 var(--_distance);
		transition: opacity, display, overlay, translate;
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

			&::backdrop {
				opacity: 1;
			}

			@starting-style {
				opacity: 0;
				translate: 0 var(--_distance);

				&::backdrop {
					opacity: 0;
				}
			}
		}
	}
</style>
