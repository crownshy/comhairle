<script lang="ts">
	import type { ComhairleDocument } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { invalidate } from '$app/navigation';
	import { CircleCheck, LoaderCircle, TriangleAlert } from 'lucide-svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { key } from '$lib/utils/invalidationKey';

	type Props = {
		document: ComhairleDocument | undefined;
		conversationId: string;
	};

	let { document, conversationId }: Props = $props();

	// Writable derived: tracks the loaded document but polling can refresh it in place
	// while parsing is in flight, without a full page reload.
	let doc = $derived(document);

	const isParsing = $derived(
		!!doc &&
			doc.parse_status !== 'DONE' &&
			doc.parse_status !== 'FAIL' &&
			doc.parse_progress < 1
	);
	const isDone = $derived(!!doc && (doc.parse_status === 'DONE' || doc.parse_progress >= 1));
	const isFailed = $derived(doc?.parse_status === 'FAIL');
	const percent = $derived(doc ? Math.round(doc.parse_progress * 100) : 0);

	async function poll() {
		if (!doc) {
			return;
		}
		const documentId = doc.id;
		const res = await tryCatchAsync(() =>
			apiClient.GetDocument({
				params: { document_id: documentId, conversation_id: conversationId }
			})
		);
		if (res.err !== null) {
			console.error(res.err);
		} else {
			doc = res.ok;
		}
		// Once parsing settles, refresh the page data so the rest of the page (e.g. the
		// "needs parsed docs" gate) reflects the finished sync.
		if (!isParsing) {
			await invalidate(key('knowledge-base/documents'));
		}
	}

	// While the synced document is parsing, poll for progress every 10s. The effect
	// reschedules itself whenever `doc` changes and stops once parsing settles.
	$effect(() => {
		if (!isParsing) {
			return;
		}
		const timeout = setTimeout(poll, 10_000);
		return () => clearTimeout(timeout);
	});
</script>

{#if isFailed}
	<div class="text-destructive flex items-center gap-2 text-base">
		<TriangleAlert class="size-4" />
		<span>Sync failed. Try syncing again.</span>
	</div>
{:else if isParsing}
	<div class="text-muted-foreground flex items-center gap-2 text-base">
		<LoaderCircle class="size-4 animate-spin" />
		<span>Processing synced learn content... {percent}%</span>
	</div>
{:else if isDone}
	<div class="text-muted-foreground flex items-center gap-2 text-base">
		<CircleCheck class="text-brand size-4" />
		<span>Learn content synced and ready</span>
	</div>
{:else}
	<div class="text-muted-foreground flex items-center gap-2 text-base">
		<span>Not synced yet</span>
	</div>
{/if}
