<script lang="ts">
	import type { ComponentProps } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Spinner } from '$lib/components/ui/spinner';

	interface Props extends Omit<ComponentProps<typeof Button>, 'children'> {
		text: string;
		ref?: HTMLButtonElement | null;
		isSubmitting?: boolean;
	}

	let { text, ref = $bindable(), isSubmitting, ...props }: Props = $props();

	let showSpinner = $state<boolean>(false);

	$effect(() => {
		let timeout: ReturnType<typeof setTimeout> | undefined;

		if (!isSubmitting) {
			showSpinner = false;
		} else {
			// Delay the spinner for a little bit as otherwise you can get a flash when it submits
			timeout = setTimeout(() => {
				showSpinner = true;
			}, 100);
		}

		return () => {
			clearTimeout(timeout);
		};
	});
</script>

<Button type="submit" bind:ref {...props} disabled={isSubmitting}>
	{#if showSpinner}
		<Spinner />
	{/if}
	{text}
</Button>
