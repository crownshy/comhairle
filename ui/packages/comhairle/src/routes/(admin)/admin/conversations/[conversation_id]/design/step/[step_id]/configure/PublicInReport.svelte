<script lang="ts">
	import { resolve } from '$app/paths';
	import ArrowRight from 'lucide-svelte/icons/arrow-right';
	import { Button } from '$lib/components/ui/button';
	import { embeddedSectionBlockLabel, type EmbeddedSectionBlockMeta } from '$lib/reports/embeds';

	type Props = {
		conversationId: string;
		/** This step's section blocks that the report embeds. */
		embedded: EmbeddedSectionBlockMeta[];
		reportIsPublic: boolean;
	};

	let { conversationId, embedded, reportIsPublic }: Props = $props();

	const labels = $derived(embedded.map(embeddedSectionBlockLabel).join(', '));
	const reportHref = $derived(
		resolve('/(admin)/admin/conversations/[conversation_id]/report', {
			conversation_id: conversationId
		})
	);
</script>

<div class="pt-4">
	<div class="flex flex-col gap-1">
		<span class="text-lg font-semibold">Public results</span>
		<p class="text-muted-foreground text-sm">
			Only results you embed in the report are public, and only once the report is published.
		</p>
	</div>
	<div class="flex flex-col items-start gap-2 pt-4">
		<p class="text-base">
			{#if embedded.length === 0}
				Nothing from this step is in the report yet.
			{:else if reportIsPublic}
				Public now: {labels}.
			{:else}
				In the report: {labels}. Public once you publish the report.
			{/if}
		</p>
		<Button variant="outline" href={reportHref}>
			{embedded.length === 0 ? 'Open report' : 'Edit in report'}
			<ArrowRight class="size-4" />
		</Button>
	</div>
</div>
