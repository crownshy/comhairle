<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import type { ComponentProps } from 'svelte';

	type Props = ComponentProps<typeof Input> & { label?: string };
	let { onblur, label, value = $bindable(), ...props }: Props = $props();

	let ref: HTMLInputElement | null = $state(null);
	let isError: boolean = $state(false);
</script>

<div class="flex flex-col">
	{#if label}
		<Label class="mb-1 text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">{label}</Label>
	{/if}
	<Input
		bind:ref
		{...props}
		bind:value
		onblur={(e) => {
			isError = !(ref?.checkValidity() ?? true);
			onblur?.(e);
		}}
		oninvalid={() => (isError = true)}
	/>
	{#if isError}
		<p aria-live="polite" class="text-destructive text-xs">{ref?.validationMessage}</p>
	{/if}
</div>
