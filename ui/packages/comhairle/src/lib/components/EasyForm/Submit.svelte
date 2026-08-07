<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import type { ComponentProps } from 'svelte';
	import { isSubmitting } from './form.svelte';
	import { Spinner } from '../ui/spinner';
	interface Props extends Omit<ComponentProps<typeof Button>, 'children'> {
		text: string;
	}
	let { text, ...props }: Props = $props();
</script>

<Button type="submit" {...props} disabled={$isSubmitting}>
	{#if $isSubmitting}
		<Spinner />
	{/if}
	{text}
</Button>
