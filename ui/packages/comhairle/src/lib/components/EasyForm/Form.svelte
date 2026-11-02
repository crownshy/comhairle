<script lang="ts">
	import { enhance } from '$app/forms';
	import type { Snippet } from 'svelte';
	import type { HTMLFormAttributes } from 'svelte/elements';

	export type OnSubmit = (formData: FormData) => Promise<void>;

	type SnippetArgs = {
		isSubmitting: boolean;
	};

	type Props = Omit<HTMLFormAttributes, 'children' | 'onSubmit'> & {
		children: Snippet<[SnippetArgs]>;
		ref?: HTMLFormElement | null;
		onSubmit: OnSubmit;
		resetOnSubmit?: boolean;
	};

	let { children, ref = $bindable(), onSubmit, resetOnSubmit = true, ...props }: Props = $props();

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
		await onSubmit(formData);
		isSubmitting = false;

		return async ({ update }) => {
			await update();
			isSubmitting = false;
			if (resetOnSubmit) {
				Form?.reset();
			}
		};
	}}
>
	{@render children({ isSubmitting })}
</form>
