<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import type { PrioritisationStore } from './store.svelte';

	let { store }: { store: PrioritisationStore } = $props();

	let idx = $state(0);
	let pages = $derived(store.poll.report.pages);
	let page = $derived(pages[idx]);
</script>

<Card.Root>
	<Card.Header>
		<Card.Title>{store.poll.title || 'Poll'} — Shareback</Card.Title>
		<Card.Description>
			Page {idx + 1} of {pages.length || 1}
		</Card.Description>
	</Card.Header>
	<Card.Content>
		{#if !page}
			<p class="text-muted-foreground text-sm">
				No shareback pages yet — add some in the Edit view.
			</p>
		{:else}
			<ContentRenderer content={page.content} />
		{/if}
	</Card.Content>
	<Card.Footer class="flex justify-between">
		<Button variant="outline" onclick={() => (idx = Math.max(0, idx - 1))} disabled={idx === 0}>
			Previous
		</Button>
		<Button
			variant="outline"
			onclick={() => (idx = Math.min(pages.length - 1, idx + 1))}
			disabled={idx >= pages.length - 1}
		>
			Next
		</Button>
	</Card.Footer>
</Card.Root>
