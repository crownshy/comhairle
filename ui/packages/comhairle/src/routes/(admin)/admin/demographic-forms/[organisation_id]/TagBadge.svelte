<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { cn } from '$lib/utils';
	import type { TagName } from '../demographicModel';

	type Props = { tag: TagName; label?: string; class?: string; children?: Snippet };

	let { tag, label, class: className, children }: Props = $props();

	function stylesForTag(tag: TagName): string {
		switch (tag) {
			case 'Age':
				return 'bg-blue-100 text-blue-900 outline-blue-300 dark:bg-blue-950 dark:text-blue-100';
			case 'Gender':
				return 'bg-purple-100 text-purple-900 outline-purple-300 dark:bg-purple-950 dark:text-purple-100';
			case 'Ethnicity':
				return 'bg-amber-100 text-amber-900 outline-amber-300 dark:bg-amber-950 dark:text-amber-100';
			case 'Postcode':
				return 'bg-green-100 text-green-900 outline-green-300 dark:bg-green-950 dark:text-green-100';
			case 'Special category':
				return 'bg-red-100 text-red-900 outline-red-300 dark:bg-red-950 dark:text-red-100';
			case 'Custom':
			default:
				return 'bg-slate-100 text-slate-800 outline-slate-300 dark:bg-slate-800 dark:text-slate-100';
		}
	}
</script>

<Badge variant="secondary" class={cn(stylesForTag(tag), className)}>
	{#if children}
		{@render children()}
	{:else}
		{label ?? tag}
	{/if}
</Badge>
