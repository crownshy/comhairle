<script lang="ts">
	import { enhance } from '$app/forms';
	import { invalidate } from '$app/navigation';
	import { key, type InputKey } from '$lib/utils/invalidationKey';
	import type { Snippet } from 'svelte';
	import type { HTMLFormAttributes } from 'svelte/elements';

	export type OnSubmit = (formData: FormData) => Promise<void>;

	type SnippetArgs = {
		isSubmitting: boolean;
	};

	type Props = Omit<HTMLFormAttributes, 'children' | 'onsubmit'> & {
		children: Snippet<[SnippetArgs]>;
		ref?: HTMLFormElement | null;
		onsubmit: OnSubmit;
		closeOnSubmit?: string;
		noReset?: boolean;
		// Additional reset function if a form reset doesn't completely reset it
		additionalReset?: () => void;
		invalidate?: InputKey;
	};

	let {
		children,
		ref = $bindable(),
		onsubmit,
		closeOnSubmit,
		noReset = false,
		additionalReset,
		invalidate: ik,
		...props
	}: Props = $props();

	let isSubmitting = $state<boolean>(false);
	let Form = $state<HTMLFormElement | null>(null);

	$effect(() => {
		ref = Form;
	});
</script>

<form
	method="POST"
	{...props}
	bind:this={Form}
	use:enhance={async ({ cancel, formData }) => {
		isSubmitting = true;

		cancel();
		await onsubmit(formData);

		if (!noReset) {
			ref?.reset();
			additionalReset?.();
		}

		if (closeOnSubmit) {
			const dialog = document.getElementById(closeOnSubmit) as HTMLDialogElement | null;
			if (dialog) {
				dialog.close();
			}
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
