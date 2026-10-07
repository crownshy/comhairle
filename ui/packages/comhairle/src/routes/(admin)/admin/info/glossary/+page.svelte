<script lang="ts">
	import { ADMIN_GLOSSARY, glossaryTermId } from '$lib/admin_guides';
	import GuideRichText from '$lib/components/AdminGuide/GuideRichText.svelte';

	const ALPHABET = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'.split('');

	// Terms are already sorted A–Z; group them under their first letter.
	const groups: [string, typeof ADMIN_GLOSSARY.terms][] = [];
	for (const entry of ADMIN_GLOSSARY.terms) {
		const letter = entry.term.charAt(0).toUpperCase();
		const last = groups.at(-1);
		if (last?.[0] === letter) last[1].push(entry);
		else groups.push([letter, [entry]]);
	}
	const usedLetters = new Set(groups.map(([letter]) => letter));
</script>

<svelte:head>
	<title>{ADMIN_GLOSSARY.title} - Comhairle Admin Guide</title>
</svelte:head>

<main class="w-full max-w-6xl min-w-0 rounded-xl p-6 md:p-10">
	<article class="max-w-3xl">
		<p class="text-muted-foreground text-sm font-medium">Reference</p>
		<h1 class="text-foreground mt-1 text-2xl font-bold">{ADMIN_GLOSSARY.title}</h1>
		<p class="text-muted-foreground mt-3 text-lg leading-7">{ADMIN_GLOSSARY.summary}</p>

		<!-- A–Z jump bar: letters with terms link to their group, the rest are greyed out. -->
		<nav aria-label="Jump to letter" class="mt-6 flex flex-wrap gap-1">
			{#each ALPHABET as letter (letter)}
				{#if usedLetters.has(letter)}
					<a
						href={`#letter-${letter}`}
						class="bg-accent text-accent-foreground hover:bg-primary hover:text-primary-foreground flex size-8 items-center justify-center rounded-md text-sm font-semibold transition-colors"
						>{letter}</a
					>
				{:else}
					<span
						class="text-muted-foreground/50 flex size-8 items-center justify-center text-sm"
						aria-hidden="true">{letter}</span
					>
				{/if}
			{/each}
		</nav>

		<div class="mt-8 flex flex-col gap-8">
			{#each groups as [letter, terms] (letter)}
				<section
					id={`letter-${letter}`}
					class="scroll-mt-6"
					aria-labelledby={`heading-${letter}`}
				>
					<h2
						id={`heading-${letter}`}
						class="text-primary border-border border-b pb-2 text-xl font-bold"
					>
						{letter}
					</h2>
					<!-- Each card has an id (e.g. #term-seed-statement) so other pages can link
					     straight to a term; the linked card gets a highlight ring. -->
					<dl class="mt-4 grid gap-3 sm:grid-cols-2">
						{#each terms as { term, definition } (term)}
							<div
								id={glossaryTermId(term)}
								class="border-border bg-card hover:border-primary/40 target:border-primary target:ring-primary/20 scroll-mt-6 rounded-xl border p-4 transition-colors target:ring-2"
							>
								<dt class="text-foreground font-semibold">{term}</dt>
								<dd class="text-muted-foreground mt-2 text-sm leading-6">
									<GuideRichText text={definition} />
								</dd>
							</div>
						{/each}
					</dl>
				</section>
			{/each}
		</div>
	</article>
</main>
