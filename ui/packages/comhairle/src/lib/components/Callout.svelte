<script lang="ts">
	import { cn } from '$lib/utils';
	import type { Snippet } from 'svelte';

	type Props = {
		children: Snippet;
		variant?: 'primary' | 'success' | 'warning' | 'danger' | 'default';
		class?: string;
	};

	let { children, variant = 'default', class: className }: Props = $props();

	function getColour(variant: Props['variant']): string {
		switch (variant) {
			case 'primary':
				// Blue
				return 'oklch(62.3% 0.214 259.815)';
			case 'warning':
				// Amber
				return 'oklch(76.9% 0.188 70.08)';
			case 'success':
				// Green
				return 'oklch(72.3% 0.219 149.579)';
			case 'danger':
				// Red
				return 'oklch(63.7% 0.237 25.331)';
			case 'default':
			default:
				return 'oklch(55.4% 0.046 257.417)';
		}
	}
</script>

<div
	data-purpose="callout"
	class={cn('callout mb-5 rounded-md border px-5 py-3', className)}
	style="--_color: var(--color, {getColour(variant)})"
>
	{@render children()}
</div>

<style>
	.callout {
		--_color: var(--color, oklch(50% 0 0));
		--_border-color: oklch(from var(--_color) l c h / 0.5);
		border-color: var(--_border-color);
		background-color: oklch(from var(--_color) l c h / 0.1);
		color: color-mix(var(--color-foreground) 85%, var(--_color));
	}
</style>
