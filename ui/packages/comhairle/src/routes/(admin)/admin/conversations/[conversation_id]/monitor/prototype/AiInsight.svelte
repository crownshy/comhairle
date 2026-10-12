<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Sparkles } from 'lucide-svelte';
	import { cn } from '$lib/utils';

	type Props = {
		children: Snippet;
		/** 'dark' when placed on the bg-sidebar surface. */
		tone?: 'light' | 'dark';
		class?: string;
	};

	let { children, tone = 'light', class: className }: Props = $props();
</script>

<!--
	Marks text that only exists in AI mode, i.e. an interpretation a model would have to
	write rather than a number or a fixed sentence filled in from data.
-->
<div class={cn('flex flex-col gap-1.5', className)}>
	<span
		class={cn(
			'inline-flex items-center gap-1.5 self-start text-sm font-semibold',
			tone === 'dark' ? 'text-sidebar-secondary' : 'text-primary'
		)}
	>
		<Sparkles class="size-4" aria-hidden="true" />
		AI summary
	</span>
	{@render children()}
</div>
