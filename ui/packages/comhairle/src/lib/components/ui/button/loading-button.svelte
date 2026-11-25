<script lang="ts">
	import Button, { type ButtonProps } from './button.svelte';
	import { Spinner } from '$lib/components/ui/spinner';
	import type { Snippet } from 'svelte';
	import { cn } from '$lib/utils';

	type Props = Omit<ButtonProps, 'children'> & {
		loading?: boolean;
		children: Snippet;
	};

	let { loading = false, children, disabled, class: classes, ...restProps }: Props = $props();
</script>

<Button
	disabled={loading || disabled}
	class={cn('flex items-center gap-x-2', classes)}
	{...restProps}
>
	{#if loading}
		<Spinner />
	{/if}
	<div>
		{@render children()}
	</div>
</Button>
