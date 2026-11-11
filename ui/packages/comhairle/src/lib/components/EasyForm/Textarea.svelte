<script lang="ts">
	import { Label } from '$lib/components/ui/label';
	import { Textarea } from '../ui/textarea';
	import type { ComponentProps } from 'svelte';

	type Props = ComponentProps<typeof Textarea> & {
		ref?: HTMLTextAreaElement | null;
		label?: string;
		files?: undefined;
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

	const id = crypto.randomUUID();
</script>

<div class="flex w-full flex-col">
	{#if label}
		<Label class="mb-1 text-sm font-semibold lg:w-75 lg:shrink-0 lg:pt-2" for={id}>
			{label}
		</Label>
	{/if}
	<Textarea
		{...props}
		{id}
		{ref}
		onblur={(e) => {
			onblur?.(e);
			if (!ref) {
				return;
			}
			if (ref.checkValidity()) {
				clearError();
			} else {
				setError(ref.validationMessage);
			}
		}}
		oninvalid={() => {
			if (ref?.validity.valid === false) {
				errorMessage = ref.validationMessage;
			}
		}}
	></Textarea>
	{#if errorMessage}
		<p aria-live="polite" class="text-destructive text-xs">{errorMessage}</p>
	{/if}
</div>
