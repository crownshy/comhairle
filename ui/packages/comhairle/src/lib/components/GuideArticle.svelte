<script lang="ts">
	import type { ToolGuide } from '$lib/tool_guides';
	import { ImageIcon } from 'lucide-svelte';

	let { guide }: { guide: ToolGuide } = $props();
</script>

<div class="max-w-3xl">
	<h1 class="text-foreground text-4xl font-bold">{guide.title}</h1>

	<div class="mt-12 flex flex-col gap-5">
		{#each guide.sections as section, i (i)}
			{#if section.image}
				<div
					class="border-border bg-muted flex h-64 w-full max-w-md items-center justify-center rounded-md border"
				>
					<ImageIcon class="text-muted-foreground/50 size-14" />
				</div>
			{:else}
				{#if section.heading}
					<h2 class="text-foreground text-3xl font-semibold">{section.heading}</h2>
				{/if}
				{#if section.html}
					<!-- Trusted static copy from tool_guides.ts -->
					<div
						class="text-foreground prose prose-sm max-w-none text-lg leading-8 [&_a]:underline [&_li]:my-1 [&_p]:my-0 [&_ul]:list-disc [&_ul]:pl-6"
					>
						{@html section.html}
					</div>
				{/if}
			{/if}
		{/each}
	</div>
</div>
