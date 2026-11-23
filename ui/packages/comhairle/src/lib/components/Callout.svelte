<script lang="ts">
	import { cn } from '$lib/utils';
	import type { Snippet } from 'svelte';
	import type { HTMLAttributes } from 'svelte/elements';

	type Props = HTMLAttributes<HTMLDivElement> & {
		children: Snippet;
		variant?: 'primary' | 'warning' | 'danger';
		class?: string;
	};

	let { children, variant = 'primary', class: className, ...rest }: Props = $props();

	function getColour(variant: Props['variant']): string {
		switch (variant) {
			case 'warning':
				return 'color-amber-400';
			case 'danger':
				return 'color-red-500';
			case 'primary':
			default:
				return 'color-primary';
		}
	}
</script>

<div
	{...rest}
	data-purpose="callout"
	class={cn('callout rounded-md border px-5 py-3', className)}
	style="--_color: var(--color, var(--{getColour(variant)}))"
>
	{@render children()}
</div>

<style>
	.callout {
		border-color: oklch(from var(--_color) l c h);
		background-color: oklch(from var(--_color) l c h / 0.3);
	}
</style>
