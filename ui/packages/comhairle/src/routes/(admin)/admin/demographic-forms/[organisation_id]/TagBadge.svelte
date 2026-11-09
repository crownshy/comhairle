<script lang="ts">
	import type { TagName } from '../demographicModel';
	import { cn } from '$lib/utils';

	type Props = { tag: TagName; selected?: boolean; class?: string };

	let { tag, selected, class: className }: Props = $props();

	function getColour(tag: TagName): string {
		switch (tag) {
			case 'Age':
				// Blue
				return 'oklch(62.3% 0.214 259.815)';
			case 'Gender':
				// Purple
				return 'oklch(62.7% 0.265 303.9)';
			case 'Ethnicity':
				// Amber
				return 'oklch(76.9% 0.188 70.08)';
			case 'Postcode':
				// Green
				return 'oklch(72.3% 0.219 149.579)';
			case 'Special category':
				// Red
				return 'oklch(63.7% 0.237 25.331)';
			case 'Custom':
			default:
				return 'oklch(55.4% 0.046 257.417)';
		}
	}
</script>

<span
	class={cn(
		'badge focus-visible:ring-ring/50 h-full rounded-full border px-2.5 py-0.5 text-xs font-medium shadow-xs transition-[color,box-shadow] focus-visible:ring-[3px]',
		selected ? 'selected' : '',
		className
	)}
	style="--color: {getColour(tag)}"
>
	{tag}
</span>

<style>
	.badge {
		--_color: var(--color, oklch(50% 0 0));
		--_border-color: oklch(from var(--_color) l c h / 0.8);
		border-color: var(--_border-color);
		background-color: oklch(from var(--_color) l c h / 0.2);
		color: color-mix(var(--color-foreground) 85%, var(--_color));

		&.selected {
			--_new_bg: oklch(from var(--_color) 0.8 calc(c - 0.05) calc(h - 5));
			background-color: var(--_new_bg);
			border-color: var(--_new_bg);
			color: contrast-color(var(--_new_bg));
		}
	}
</style>
