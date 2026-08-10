<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import type { ComponentProps } from 'svelte';

	type Props = Exclude<ComponentProps<typeof Input>, 'value'> & {
		ref?: HTMLInputElement | null;
		label?: string;
	};
	let { onblur, ref = $bindable(), label, ...props }: Props = $props();

	let errorMessage = $state<string>('');

	export function clearError() {
		ref?.setCustomValidity('');
		errorMessage = '';
	}
	export function setError(error: string) {
		ref?.setCustomValidity(error);
		errorMessage = error;
	}
	export function setValue(value: string) {
		if (ref) {
			ref.value = value;
			ref.checkValidity();
			setError(ref.validationMessage);
		}
	}
	export function getValue(): string | undefined {
		return ref?.value;
	}
</script>

<div class="flex flex-col">
	{#if label}
		<Label class="mb-1 text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">{label}</Label>
	{/if}
	<Input
		bind:ref
		{...props}
		onblur={(e) => {
			onblur?.(e);
			if (ref?.checkValidity()) {
				errorMessage = ref.validationMessage;
			}
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
