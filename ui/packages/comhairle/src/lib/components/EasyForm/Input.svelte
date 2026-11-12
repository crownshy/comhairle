<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Eye, EyeOff } from 'lucide-svelte';
	import type { ComponentProps } from 'svelte';
	import type { HTMLInputTypeAttribute } from 'svelte/elements';

	type Props = Exclude<ComponentProps<typeof Input>, 'value' | 'type' | 'files'> & {
		type?: Exclude<HTMLInputTypeAttribute, 'file'>;
		ref?: HTMLInputElement | null;
		label?: string;
		files?: undefined;
	};
	let { onblur, ref = $bindable(), label, type, ...props }: Props = $props();

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

	// For password inputs
	let visible = $state<boolean>(false);
</script>

<div class="flex w-full flex-col">
	{#if label}
		<Label class="mb-1 text-sm font-semibold lg:w-75 lg:shrink-0 lg:pt-2">{label}</Label>
	{/if}
	<span class:pile={type === 'password'}>
		<Input
			{...props}
			bind:ref
			type={type === 'password' && visible ? 'text' : type}
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
		{#if type === 'password'}
			<button
				class="z-10 mr-3 justify-self-end"
				type="button"
				title="{visible ? 'Hide' : 'Show'} password"
				aria-label="{visible ? 'Hide' : 'Show'} password"
				onclick={() => (visible = !visible)}
			>
				{#if visible}
					<EyeOff class="h-4 w-4" />
				{:else}
					<Eye class="h-4 w-4" />
				{/if}
			</button>
		{/if}
	</span>
	{#if errorMessage}
		<p aria-live="polite" class="text-destructive text-xs">{errorMessage}</p>
	{/if}
</div>
