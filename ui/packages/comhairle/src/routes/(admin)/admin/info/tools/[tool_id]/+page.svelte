<script lang="ts">
	import { page } from '$app/state';
	import { TOOL_GUIDES } from '$lib/tool_guides';
	import { ImageIcon } from 'lucide-svelte';
	import { toSlug } from '$lib/utils/casingUtils';

	// The tool's key is the `[tool_id]` route param, which matches the TOOL_GUIDES keys.
	let guide = $derived(TOOL_GUIDES[page.params.tool_id ?? '']);
	let contentSections = $derived(guide?.sections.filter((section) => !section.image) ?? []);
	let imageSections = $derived(guide?.sections.filter((section) => section.image) ?? []);
	let jumpSections = $derived(
		contentSections.filter(
			(section) => section.heading && section.heading !== 'What you need to know'
		)
	);

	// function sectionId(heading: string): string {
	// 	return heading
	// 		.toLowerCase()
	// 		.replace(/[^a-z0-9]+/g, '-')
	// 		.replace(/^-|-$/g, '');
	// }
</script>

<svelte:head>
	<title>{guide?.title ?? 'Tools'} - Comhairle Tools Guide</title>
</svelte:head>

{#if guide}
	<div class="max-w-6xl">
		<h1 class="text-primary text-4xl font-bold">{guide.title}</h1>

		{#if guide.atAGlance}
			<dl class="mt-6 grid gap-3 sm:grid-cols-3">
				<div class="border-border bg-card rounded-lg border p-4">
					<dt class="text-muted-foreground text-sm font-medium">Best for</dt>
					<dd class="text-foreground mt-1 text-base font-semibold">
						{guide.atAGlance.bestFor}
					</dd>
				</div>
				<div class="border-border bg-card rounded-lg border p-4">
					<dt class="text-muted-foreground text-sm font-medium">Participant time</dt>
					<dd class="text-foreground mt-1 text-base font-semibold">
						{guide.atAGlance.participantTime}
					</dd>
				</div>
				<div class="border-border bg-card rounded-lg border p-4">
					<dt class="text-muted-foreground text-sm font-medium">Setup time</dt>
					<dd class="text-foreground mt-1 text-base font-semibold">
						{guide.atAGlance.setupTime}
					</dd>
				</div>
			</dl>
		{/if}

		<div class="mt-10 grid gap-10 lg:grid-cols-[minmax(0,1fr)_minmax(18rem,0.7fr)]">
			<div class="flex min-w-0 flex-col gap-10">
				{#each contentSections as section, index (index)}
					<section id={section.heading ? toSlug(section.heading) : undefined}>
						{#if section.heading}
							<h2 class="text-foreground text-3xl font-semibold">
								{section.heading}
							</h2>
						{/if}
						{#if section.html}
							<div
								class="text-foreground prose prose-sm mt-3 max-w-none text-lg leading-8 [&_a]:underline [&_li]:my-1 [&_p]:my-0 [&_ul]:list-disc [&_ul]:pl-6"
							>
								{@html section.html}
							</div>
						{/if}
					</section>

					{#if index === 0 && jumpSections.length > 0}
						<nav class="border-border border-t pt-6" aria-label="On this page">
							<h2 class="text-foreground text-lg font-semibold">On this page</h2>
							<div class="mt-3 flex flex-wrap gap-2">
								{#each jumpSections as section (section.heading)}
									{#if section.heading}
										<a
											href={`#${toSlug(section.heading)}`}
											class="bg-muted text-foreground hover:bg-accent inline-flex rounded-full px-3 py-1.5 text-base font-medium transition-colors"
										>
											{section.heading}
										</a>
									{/if}
								{/each}
							</div>
						</nav>
					{/if}
				{/each}
			</div>

			{#if imageSections.length > 0}
				<aside
					class="space-y-5 lg:sticky lg:top-6 lg:self-start"
					aria-label={`${guide.title} images`}
				>
					{#each imageSections as section, index (index)}
						{#if section.image?.src}
							<img
								class="border-border h-auto w-full rounded-lg border"
								src={section.image.src}
								alt={section.image.alt ?? ''}
							/>
						{:else}
							<div
								class="border-border bg-muted flex aspect-[4/3] w-full items-center justify-center rounded-lg border"
							>
								<ImageIcon class="text-muted-foreground/50 size-14" />
							</div>
						{/if}
					{/each}
				</aside>
			{/if}
		</div>
	</div>
{:else}
	<p class="text-muted-foreground">Guide not found.</p>
{/if}
