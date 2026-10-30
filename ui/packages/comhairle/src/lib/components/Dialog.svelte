<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { X } from 'lucide-svelte';

	type Action = {
		label: string;
		onclick: () => void;
	};

	type Props = {
		id: string;
		title?: string;
		children?: Snippet;
		actions?: {
			primary?: Action;
			secondary?: Action;
		};
		HeaderAction?: Snippet<[]>;
		PrimaryAction?: Snippet<[]>;
		SecondaryAction?: Snippet<[]>;
	};

	let { id, title, actions, children, HeaderAction, PrimaryAction, SecondaryAction }: Props =
		$props();
</script>

<dialog {id} class="bg-background border-border m-auto rounded-2xl border p-2">
	<Button
		class="absolute inset-e-2 h-4 w-4 rounded-sm p-0 text-sm font-medium hover:opacity-80 focus-visible:opacity-80"
		variant="ghost"
		dialogClose={id}
		icon={X}
		title="close"
		aria-label="Close dialog"
	/>
	<div class="grid h-full w-full grid-rows-[auto_1fr_auto] p-5 pr-2">
		<header class="flex flex-row items-center justify-between">
			<span class="shrink-0 grow pt-4 text-3xl font-medium">{title}</span>
			<span class="shrink grow-0">
				{#if HeaderAction}
					{@render HeaderAction()}
				{/if}
			</span>
		</header>
		<section>
			{@render children?.()}
		</section>
		{#if PrimaryAction || actions?.primary}
			<footer class="flex flex-row justify-end gap-x-2 pr-3">
				{#if SecondaryAction}
					{@render SecondaryAction()}
				{:else if actions?.secondary}
					<Button onclick={actions.secondary.onclick} variant="outline">
						{actions.secondary.label}
					</Button>
				{/if}
				{#if PrimaryAction}
					{@render PrimaryAction()}
				{:else if actions?.primary}
					<Button onclick={actions.primary.onclick}>{actions.primary.label}</Button>
				{/if}
			</footer>
		{/if}
	</div>
</dialog>

<style>
</style>
