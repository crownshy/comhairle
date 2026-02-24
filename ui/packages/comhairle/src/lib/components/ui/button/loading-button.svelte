<script lang="ts">
	import Button, { type ButtonProps } from './button.svelte';
	import { Spinner } from '$lib/components/ui/spinner';
	import type { Snippet } from 'svelte';

	type Props = Omit<ButtonProps, 'children'> & {
		loading?: boolean;
		children: Snippet;
	};

	let { loading = false, children, disabled, ...restProps }: Props = $props();
</script>

<Button disabled={loading || disabled} {...restProps}>
	{#if loading}
		<Spinner />
	{/if}
	{@render children()}
</Button>
