<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { Plus, Trash2 } from 'lucide-svelte';
	import Shareback from './Shareback.svelte';
	import type { PrioritisationStore } from './store.svelte';

	let { store }: { store: PrioritisationStore } = $props();

	let mode = $state<'edit' | 'preview'>('edit');
</script>

<div class="flex flex-col gap-4">
	<div class="flex items-center justify-between">
		<h3 class="text-lg font-semibold">Report / Shareback</h3>
		<div class="flex gap-2">
			<Button
				variant={mode === 'edit' ? 'default' : 'outline'}
				onclick={() => (mode = 'edit')}>Edit</Button
			>
			<Button
				variant={mode === 'preview' ? 'default' : 'outline'}
				onclick={() => (mode = 'preview')}>Preview</Button
			>
			<Button onclick={() => store.publishReport()}>Publish report</Button>
		</div>
	</div>

	{#if mode === 'edit'}
		<div class="flex flex-col gap-3">
			{#each store.poll.report.pages as page (page.id)}
				<Card.Root>
					<Card.Header class="flex flex-row items-center justify-between">
						<Card.Title>Page {page.order}</Card.Title>
						<Button
							variant="ghost"
							size="icon"
							onclick={() => store.removeReportPage(page.id)}
							aria-label="Remove page"
						>
							<Trash2 class="size-4" />
						</Button>
					</Card.Header>
					<Card.Content>
						<RichTextEditor
							value={page.content || null}
							placeholder="Write the shareback content for this page…"
							minHeight="200px"
							onChange={(json) => store.updateReportPage(page.id, json)}
						/>
					</Card.Content>
				</Card.Root>
			{/each}
			<Button variant="outline" onclick={() => store.addReportPage()}>
				<Plus class="mr-1 size-4" /> Add page
			</Button>
			{#if store.poll.report.pages.length === 0}
				<p class="text-muted-foreground text-sm">
					No report pages yet. Add a page to start drafting your shareback narrative.
				</p>
			{/if}
		</div>
	{:else}
		<Shareback {store} />
	{/if}
</div>
