<script lang="ts">
	import Badge from '$lib/components/ui/badge/badge.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import { Plus, X } from 'lucide-svelte';
	import { cn } from '$lib/utils.js';
	import type { HTMLInputAttributes } from 'svelte/elements';

	type Props = HTMLInputAttributes & {
		badges: { id: string; value: string }[];
		onAddBadge: (value: string) => void;
		onDeleteBadge: (id: string) => void;
		class: string;
	};

	let { badges, onAddBadge, onDeleteBadge, class: className, ...restProps }: Props = $props();

	let value = $state('');
</script>

<div class={cn('flex w-full flex-col items-start gap-8', className)}>
	<div class="flex w-full gap-2">
		<Input
			bind:value
			class="grow"
			onkeydown={(e) => {
				if (e.key === 'Enter') {
					onAddBadge(value);
					value = '';
				}
			}}
			{...restProps}
		/>
		<Button
			onclick={() => {
				onAddBadge(value);
				value = '';
			}}><Plus /></Button
		>
	</div>
	{#each badges as badge (badge.id)}
		<Badge class="px-3 py-2">
			{badge.value}
			<button type="button" onclick={() => onDeleteBadge(badge.id)}
				><X class="h-4 w-4" /></button
			>
		</Badge>
	{/each}
</div>
