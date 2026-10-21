<script lang="ts">
	import { enhance } from '$app/forms';
	import { useDebounce } from 'runed';
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

	// Small debounce so that if it does the update very quickly we don't get a flash of the spinner
	const setIsSubmitting = useDebounce(
		(newIsSubmitting: boolean) => void (isSubmitting = newIsSubmitting),
		100
	);
</script>

<form
	{method}
	{...props}
	bind:this={ref}
	use:enhance={async ({ cancel, formData }) => {
		setIsSubmitting(true);
		if (handleSubmission) {
			cancel();
			await handleSubmission(formData);
			setIsSubmitting(false);
		}
		return async ({ update }) => {
			await update();
			setIsSubmitting(false);
		};
	}}
>
	{@render children(isSubmitting)}
</form>
