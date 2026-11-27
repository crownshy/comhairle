<script lang="ts" module>
	export type OnSubmit = (formData: ReturnType<typeof FormDataParser>) => Promise<void>;
</script>

<script lang="ts">
	import { enhance } from '$app/forms';
	import { invalidate } from '$app/navigation';
	import { key, type InputKey } from '$lib/utils/invalidationKey';
	import type { Snippet } from 'svelte';
	import type { HTMLFormAttributes } from 'svelte/elements';
	import FormDataParser from '$lib/interfaces/FormDataParser';

	type SnippetArgs = {
		isSubmitting: boolean;
	};

	type Props = Omit<HTMLFormAttributes, 'children' | 'onsubmit'> & {
		children: Snippet<[SnippetArgs]>;
		ref?: HTMLFormElement | null;
		onsubmit: OnSubmit;
		noReset?: boolean;
		// Additional reset function if a form reset doesn't completely reset it
		additionalReset?: () => void;
		invalidate?: InputKey;
	};

	let {
		children,
		ref = $bindable(),
		onsubmit,
		noReset = false,
		additionalReset,
		invalidate: ik,
		...props
	}: Props = $props();

	let isSubmitting = $state<boolean>(false);
</script>

<form
	method="POST"
	{...props}
	bind:this={ref}
	use:enhance={async ({ cancel, formData }) => {
		isSubmitting = true;

		cancel();
		await onsubmit(FormDataParser(formData));

		if (!noReset) {
			ref?.reset();
			additionalReset?.();
		}

		if (ik) {
			await invalidate(key(ik));
		}

		isSubmitting = false;

		return async ({ update }) => {
			await update();
		};
	}}
>
	{@render children({ isSubmitting })}
</form>
