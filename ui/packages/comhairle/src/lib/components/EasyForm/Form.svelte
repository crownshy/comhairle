<script lang="ts">
	import { enhance } from '$app/forms';
	import type { Snippet } from 'svelte';
	import type { HTMLFormAttributes } from 'svelte/elements';

	interface Props extends Omit<HTMLFormAttributes, 'children'> {
		children: Snippet<[isSubmitting: boolean]>;
		ref?: HTMLFormElement | null;
		handleSubmission?: (formData: FormData) => Promise<void>;
	}

	let {
		children,
		ref = $bindable(),
		handleSubmission,
		method = 'POST',
		...props
	}: Props = $props();

	let isSubmitting = $state<boolean>(false);
</script>

<form
	{method}
	{...props}
	bind:this={ref}
	use:enhance={async ({ cancel, formData }) => {
		isSubmitting = true;
		if (handleSubmission) {
			cancel();
			await handleSubmission(formData);
			isSubmitting = false;
		}
		return async ({ update }) => {
			await update();
			isSubmitting = false;
		};
	}}
>
	{@render children(isSubmitting)}
</form>
