<script lang="ts">
	import { toSlug } from '$lib/utils/casingUtils';
	import type { PageProps } from './$types';
	let { data }: PageProps = $props();
</script>

<svelte:head>
	<title>{data.guide.title} - Comhairle Admin Guide</title>
</svelte:head>

<main class="bg-muted w-full max-w-6xl min-w-0 rounded-xl p-6">
	<h1 class="text-primary text-4xl font-bold">{data.guide.title}</h1>
	<div class="mt-10 flex max-w-3xl flex-col gap-10">
		{#each data.guide.sections as section, index (index)}
			<section id={section.heading ? toSlug(section.heading) : undefined}>
				{#if section.heading}<h2 class="text-foreground text-3xl font-semibold">
						{section.heading}
					</h2>{/if}
				{#if section.content}
					<div
						class="text-foreground prose mt-3 max-w-none text-lg leading-8 [&_a]:underline [&_li]:my-1 [&_p]:my-0 [&_ul]:list-disc [&_ul]:pl-6"
					>
						{section.content}
					</div>
				{/if}
			</section>
		{/each}
	</div>
</main>
