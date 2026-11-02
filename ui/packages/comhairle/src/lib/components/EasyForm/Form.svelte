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
	};

	let { children, ref = $bindable(), onSubmit, ...props }: Props = $props();

	let isSubmitting = $state<boolean>(false);
</script>

<form
	method="POST"
	{...props}
	bind:this={ref}
	use:enhance={async ({ cancel, formData }) => {
		isSubmitting = true;

		cancel();
		await onSubmit(formData);
		isSubmitting = false;

		return async ({ update }) => {
			await update();
			isSubmitting = false;
		};
	}}
>
	{@render children({ isSubmitting })}
</form>
