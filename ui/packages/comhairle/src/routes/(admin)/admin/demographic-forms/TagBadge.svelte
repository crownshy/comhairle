<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { cn } from '$lib/utils';
	import { tagColorClass } from './demographicPrototypeData';

	type Props = { tag: string; label?: string; class?: string; children?: Snippet };

	let { tag, label, class: className, children }: Props = $props();
</script>

<Badge variant="secondary" class={cn(tagColorClass(tag), className)}>
	{#if children}
		{@render children()}
	{:else}
		{label ?? tag}
	{/if}
</Badge>
