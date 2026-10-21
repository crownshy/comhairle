<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import type { ComponentProps } from 'svelte';
	import { Spinner } from '../ui/spinner';

	interface Props extends Omit<ComponentProps<typeof Button>, 'children'> {
		text: string;
		isSubmitting?: boolean;
	}

	let { text, isSubmitting, ...props }: Props = $props();
</script>

<Button type="submit" {...props} disabled={isSubmitting}>
	{#if isSubmitting}
		<Spinner />
	{/if}
	{text}
</Button>
