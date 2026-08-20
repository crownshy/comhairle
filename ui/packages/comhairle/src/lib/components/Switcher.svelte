<!--
	Component for being able to switch between options, basically fancy radio buttons
-->
<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Button } from './ui/button';

	type Id = string;
	interface Option {
		id: Id;
		content: Snippet;
		aria?: string;
	}

	interface Props {
		options: Option[];
		onswitch: (optionId: Id) => void;
		initiallySelected?: Id;
	}

	let { options, onswitch, initiallySelected }: Props = $props();

	let selectedId = $derived<Id>(initiallySelected ?? options[0].id);
</script>

<section class="bg-muted flex flex-row flex-wrap items-center gap-2 rounded-md p-2">
	{#each options as option (option.id)}
		<Button
			size="sm"
			variant={option.id === selectedId ? 'default' : 'secondary'}
			title={option.aria}
			aria-label={option.aria}
			onclick={() => {
				selectedId = option.id;
				onswitch(option.id);
			}}
			class="rounded-md border border-transparent px-4 py-4"
		>
			{@render option.content()}
		</Button>
	{/each}
</section>
