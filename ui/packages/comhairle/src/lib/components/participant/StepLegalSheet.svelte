<script lang="ts">
	import * as Sheet from '$lib/components/ui/sheet';
	import LegalDocView from '$lib/components/rights/LegalDocView.svelte';
	import { legalDocs, type LegalDocId } from '$lib/components/rights/legalDocs';
	import * as m from '$lib/paraglide/messages';
	import { tick } from 'svelte';

	let {
		doc = $bindable()
	}: {
		/** The document on screen, or null when the reader is closed. */
		doc: LegalDocId | null;
	} = $props();

	const docs = legalDocs();

	// A participant mid-step may have unsaved tool state (a Polis vote, a half-filled form), so
	// Your Rights reads in place rather than navigating away. See ADR-0019.
	let open = $derived(doc !== null);

	let body = $state<HTMLElement | null>(null);

	async function select(next: LegalDocId) {
		doc = next;
		// Every document starts at its own title. `instant` because the reader is switching
		// documents, not moving within one, so there is no position to carry the eye between.
		await tick();
		body?.scrollTo({ top: 0, behavior: 'instant' });
	}
</script>

<Sheet.Root
	{open}
	onOpenChange={(next) => {
		if (!next) doc = null;
	}}
>
	<Sheet.Content side="right" class="w-full gap-0 sm:max-w-xl">
		<!-- A panel label, not a display heading: each document already opens with its own title,
			and two stacked headings cost more than they earn on a phone. -->
		<Sheet.Header class="pr-12 pb-2">
			<Sheet.Title class="text-muted-foreground text-base font-semibold">
				{m.your_rights()}
			</Sheet.Title>
		</Sheet.Header>

		<!-- Scrolls rather than wraps: the three labels do not fit one row at mobile widths, and a
			second row pushes the document off screen. Matches SubTabStrip's tab language. -->
		<nav
			class="border-border scrollbar-none flex w-full shrink-0 gap-1.5 overflow-x-auto border-b px-4"
			aria-label={m.your_rights()}
		>
			{#each docs as entry (entry.id)}
				{@const active = doc === entry.id}
				<button
					type="button"
					class="text-foreground inline-flex h-10 shrink-0 items-center px-1.5 text-sm font-medium whitespace-nowrap transition-opacity"
					class:text-primary={active}
					class:opacity-70={!active}
					class:hover:opacity-100={!active}
					aria-current={active ? 'page' : undefined}
					onclick={() => select(entry.id)}
				>
					{entry.label}
				</button>
			{/each}
		</nav>

		<div bind:this={body} class="min-h-0 flex-1 overflow-y-auto px-4 py-6">
			{#if doc}
				<LegalDocView {doc} onSelect={select} />
			{/if}
		</div>
	</Sheet.Content>
</Sheet.Root>
