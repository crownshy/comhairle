<script lang="ts">
	import { enhance } from '$app/forms';
	import type { HTMLFormAttributes } from 'svelte/elements';
	import { isSubmitting } from './form-state';

	interface Props extends HTMLFormAttributes {
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
</script>

<form
	{method}
	{...props}
	bind:this={ref}
	use:enhance={async ({ cancel, formData }) => {
		$isSubmitting = true;
		if (handleSubmission) {
			cancel();
			await handleSubmission(formData);
			$isSubmitting = false;
		}
		return async ({ update }) => {
			await update();
			$isSubmitting = false;
		};
	}}
>
	{@render children?.()}
</form>
