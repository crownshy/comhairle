<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import type { ComponentProps } from 'svelte';
	import { Spinner } from '../ui/spinner';

	interface Props extends Omit<ComponentProps<typeof Button>, 'children'> {
		text: string;
		ref?: HTMLButtonElement | null;
		isSubmitting?: boolean;
	}

	let { text, ref = $bindable(), isSubmitting, ...props }: Props = $props();
</script>

<Button type="submit" bind:ref {...props} disabled={isSubmitting}>
	{#if isSubmitting}
		<Spinner />
	{/if}
	{text}
</Button>
