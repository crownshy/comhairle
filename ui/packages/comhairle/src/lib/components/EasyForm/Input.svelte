<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import type { ComponentProps } from 'svelte';

	type Props = ComponentProps<typeof Input> & { label?: string };
	let { onblur, label, value = $bindable(), ...props }: Props = $props();

	let ref: HTMLInputElement | null = $state(null);
	let errorMessage = $state<string>('');
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
			if (ref?.checkValidity()) {
				errorMessage = ref.validationMessage;
			}
			onblur?.(e);
		}}
		oninvalid={() => {
			if (ref?.validity.valid === false) {
				errorMessage = ref.validationMessage;
			}
		}}
	/>
	{#if errorMessage}
		<p aria-live="polite" class="text-destructive text-xs">{errorMessage}</p>
	{/if}
</div>
