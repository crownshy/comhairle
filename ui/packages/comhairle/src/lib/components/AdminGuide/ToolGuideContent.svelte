<script lang="ts">
	import Clock from '@lucide/svelte/icons/clock';
	import Target from '@lucide/svelte/icons/target';
	import Timer from '@lucide/svelte/icons/timer';
	import type { ToolGuide } from '$lib/tool_guides';
	import { toSlug } from '$lib/utils/casingUtils';
	import { cn } from '$lib/utils';

	// The body of an engagement tool page: title, introduction, at-a-glance facts, screenshot
	// and sections. Used by the tool page and, with `embedded`, by the "learn more"
	// dialog/panel opened from a glossary tooltip (no "On this page" jump links there).
	// "What you need to know" is the tool's introduction, so it is shown under the title.
	const INTRO_HEADING = 'What you need to know';

	let { guide, embedded = false }: { guide: ToolGuide; embedded?: boolean } = $props();

	let image = $derived(guide.sections.find((section) => section.image)?.image);
	let contentSections = $derived(
		guide.sections.filter((section) => !section.image && section.heading !== INTRO_HEADING)
	);
	let intro = $derived(guide.sections.find((section) => section.heading === INTRO_HEADING));
	let jumpSections = $derived(contentSections.filter((section) => section.heading));

	// Shared styling for the trusted HTML content in tool_guides.ts.
	const PROSE =
		'text-foreground mt-2 leading-7 [&_a]:text-primary [&_a]:font-medium [&_a]:underline [&_a]:underline-offset-2 [&_li]:my-1 [&_ol]:list-decimal [&_ol]:pl-5 [&_p+p]:mt-3 [&_strong]:font-semibold [&_ul]:list-disc [&_ul]:pl-5';
</script>

<a href="/admin/info/tools" class="text-muted-foreground hover:text-primary text-sm font-medium"
	>Engagement tools</a
>
<h1 class="text-foreground mt-1 text-2xl font-bold">{guide.title}</h1>

{#if intro?.html}
	<div class={cn(PROSE, 'text-muted-foreground mt-3 text-lg')}>
		<!-- Trusted, static HTML written in tool_guides.ts -->
		<!-- eslint-disable-next-line svelte/no-at-html-tags -->
		{@html intro.html}
	</div>
{/if}

{#if guide.atAGlance}
	<dl class="mt-5 flex flex-wrap gap-2 text-sm">
		<div
			class="bg-accent text-accent-foreground flex items-center gap-2 rounded-full px-3 py-1.5"
		>
			<Target class="size-4 shrink-0" aria-hidden="true" />
			<dt class="sr-only">Best for</dt>
			<dd class="font-medium">
				Best for: {guide.atAGlance.bestFor.toLowerCase()}
			</dd>
		</div>
		<div class="bg-muted text-foreground flex items-center gap-2 rounded-full px-3 py-1.5">
			<Clock class="size-4 shrink-0" aria-hidden="true" />
			<dt class="sr-only">Participant time</dt>
			<dd>{guide.atAGlance.participantTime} for participants</dd>
		</div>
		<div class="bg-muted text-foreground flex items-center gap-2 rounded-full px-3 py-1.5">
			<Timer class="size-4 shrink-0" aria-hidden="true" />
			<dt class="sr-only">Set-up time</dt>
			<dd>Set-up: {guide.atAGlance.setupTime}</dd>
		</div>
	</dl>
{/if}

{#if image}
	<figure
		class="border-border bg-muted mt-6 w-full overflow-hidden rounded-xl border p-2 shadow-sm"
	>
		<img src={image.src} alt={image.alt ?? ''} class="block h-auto w-full rounded-xl" />
	</figure>
{/if}

{#if !embedded && jumpSections.length > 1}
	<nav class="mt-6" aria-label="On this page">
		<h2 class="text-muted-foreground text-sm font-medium">On this page</h2>
		<ul class="mt-2 flex flex-wrap gap-2">
			{#each jumpSections as section (section.heading)}
				<li>
					<a
						href={`#${toSlug(section.heading ?? '')}`}
						class="border-border text-foreground hover:border-primary/40 hover:bg-muted/60 inline-flex rounded-full border px-3 py-1 text-sm font-medium transition-colors"
					>
						{section.heading}
					</a>
				</li>
			{/each}
		</ul>
	</nav>
{/if}

{#each contentSections as section, index (index)}
	<section id={section.heading ? toSlug(section.heading) : undefined} class="mt-8 scroll-mt-6">
		{#if section.heading}
			<h2 class="text-foreground text-lg font-semibold">{section.heading}</h2>
		{/if}
		{#if section.html}
			<!-- Trusted, static HTML written in tool_guides.ts -->
			<!-- eslint-disable-next-line svelte/no-at-html-tags -->
			<div class={PROSE}>{@html section.html}</div>
		{/if}
	</section>
{/each}
