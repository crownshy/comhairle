<script lang="ts">
	import { onMount } from 'svelte';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Sparkles } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';

	const messages = [
		m.thinking_space_follow_loading_1(),
		m.thinking_space_follow_loading_2(),
		m.thinking_space_follow_loading_3(),
		m.thinking_space_follow_loading_4()
	];

	// One line-width pair per card the picker will show, varied so the skeleton doesn't look
	// like a flat block.
	const lineLayouts: Array<{ first: string; second: string | null }> = [
		{ first: 'w-full', second: 'w-2/3' },
		{ first: 'w-11/12', second: null },
		{ first: 'w-full', second: 'w-1/2' }
	];

	let index = $state(0);
	let fading = $state(false);

	onMount(() => {
		const interval = setInterval(() => {
			fading = true;
			setTimeout(() => {
				index = (index + 1) % messages.length;
				fading = false;
			}, 300);
		}, 4000);
		return () => clearInterval(interval);
	});
</script>

<!-- Same three-card frame the picker lands in, stretched over the same height, so the wait reads
     as the questions arriving rather than as a separate screen. -->
<section class="flex min-h-0 flex-1 flex-col gap-6">
	<header>
		<h2 class="text-foreground text-xl leading-snug font-semibold">
			{m.thinking_space_generating_heading()}
		</h2>
		<p class="text-muted-foreground mt-2 text-base leading-relaxed">
			{m.thinking_space_generating_desc()}
		</p>
	</header>

	<div class="flex min-h-0 flex-1 flex-col gap-3" aria-hidden="true">
		{#each lineLayouts as layout, i (i)}
			<div
				class="border-border flex flex-1 flex-col justify-center gap-2 rounded-xl border px-4 py-4"
			>
				<Skeleton class="h-4 {layout.first}" />
				{#if layout.second}
					<Skeleton class="h-4 {layout.second}" />
				{/if}
			</div>
		{/each}
	</div>

	<div class="flex items-start gap-2">
		<Sparkles class="text-primary mt-0.5 size-4 shrink-0 animate-pulse" />
		<p
			class="text-muted-foreground text-base leading-relaxed transition-opacity duration-300"
			class:opacity-0={fading}
			class:opacity-100={!fading}
			aria-live="polite"
		>
			{messages[index]}
		</p>
	</div>
</section>
