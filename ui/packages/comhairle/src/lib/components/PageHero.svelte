<script lang="ts">
	import type { Snippet } from 'svelte';
	import { cn } from '$lib/utils';

	type Props = {
		children: Snippet;
		/** Less room below the content, for pages whose hero is only a heading. */
		compact?: boolean;
	};
	let { children, compact = false }: Props = $props();
</script>

<!-- Pulled up by the NavBar's height so the gradient runs behind the see-through bar; the top
	padding adds that height back. The gradient comes from each theme's accent colour, with two
	see-through white shapes for texture. Text is dark, so it reads at WCAG AA throughout. -->
<section
	class="text-foreground relative isolate -mt-[5.5rem] overflow-hidden rounded-b-[2rem] bg-linear-to-b/srgb from-[color-mix(in_oklab,var(--sidebar-primary)_8%,var(--background))] to-[color-mix(in_oklab,var(--sidebar-primary)_50%,var(--background))] md:rounded-b-[3rem]"
>
	<div
		aria-hidden="true"
		class="pointer-events-none absolute inset-0 -z-10 bg-white/30 [clip-path:polygon(100%_25%,100%_100%,30%_100%)]"
	></div>
	<div
		aria-hidden="true"
		class="pointer-events-none absolute inset-0 -z-10 bg-white/20 [clip-path:polygon(72%_0%,100%_0%,100%_100%,60%_100%)]"
	></div>
	<div
		class={cn(
			'mx-auto w-full max-w-[1300px] px-4 md:px-8',
			compact
				? 'pt-[8.5rem] pb-12 md:pt-[10rem] md:pb-16'
				: 'pt-[9.5rem] pb-16 md:pt-[12.5rem] md:pb-28'
		)}
	>
		<div class="mx-auto flex w-full max-w-6xl flex-col items-start gap-8">
			{@render children()}
		</div>
	</div>
</section>
