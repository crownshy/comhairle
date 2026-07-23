<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import type { ComponentProps } from 'svelte';

	type Props = ComponentProps<typeof Input> & { label?: string };
	const { onblur, label, ...props }: Props = $props();

	let ref: HTMLInputElement | null = $state(null);
</script>

<div class="flex flex-col">
	{#if label}
		<Label class="mb-1 text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">{label}</Label>
	{/if}
	<Input
		bind:ref
		{...props}
		onblur={(e) => {
			ref?.checkValidity();
			onblur?.(e);
		}}
	/>
	<p aria-live="polite" class="text-destructive text-xs">{ref?.validationMessage}</p>
</div>
