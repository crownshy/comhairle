<script lang="ts">
	import { enhance } from '$app/forms';
	import type { HTMLFormAttributes } from 'svelte/elements';
	import { isSubmitting } from './form.svelte';

	interface Props extends HTMLFormAttributes {
		ref: HTMLFormElement | null;
	}

	let { children, ref = $bindable(), ...props }: Props = $props();
</script>

<form
	{...props}
	bind:this={ref}
	use:enhance={() => {
		$isSubmitting = true;
		return async ({ update }) => {
			await update();
			$isSubmitting = false;
		};
	}}
>
	{@render children?.()}
</form>
