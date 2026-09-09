<script lang="ts">
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import ReportEmbedLive from '$lib/reports/polis/ReportEmbedLive.svelte';
	import type { ComhairleDocument } from '@crownshy/api-client/api';

	/**
	 * Renders a report `summary` document with LIVE embedded components (ADR-0012): it walks
	 * the top-level nodes and splits them into runs of ordinary rich text (rendered by
	 * ContentRenderer, the safe schema-walking path) with a live `ReportEmbedLive` mounted at
	 * each `reportComponentEmbed` node. Embeds are top-level block atoms, so a top-level walk
	 * is enough. Falls back to rendering the whole thing as rich text when the content isn't a
	 * ProseMirror doc (e.g. a legacy markdown summary, which can't contain embeds anyway).
	 */
	let {
		content = '',
		conversationId = '',
		availableDocuments = []
	}: {
		content?: string;
		conversationId?: string;
		availableDocuments?: ComhairleDocument[];
	} = $props();

	type Segment =
		| { kind: 'text'; json: string }
		| { kind: 'embed'; toolStepId: string; componentType: string };

	const segments = $derived.by<Segment[]>(() => {
		let doc: unknown;
		try {
			doc = JSON.parse(content);
		} catch {
			return [{ kind: 'text', json: content }];
		}
		if (
			!doc ||
			typeof doc !== 'object' ||
			(doc as { type?: string }).type !== 'doc' ||
			!Array.isArray((doc as { content?: unknown[] }).content)
		) {
			return [{ kind: 'text', json: content }];
		}

		const nodes = (doc as { content: { type?: string; attrs?: Record<string, unknown> }[] })
			.content;
		const out: Segment[] = [];
		let run: unknown[] = [];
		const flush = () => {
			if (run.length > 0) {
				out.push({ kind: 'text', json: JSON.stringify({ type: 'doc', content: run }) });
				run = [];
			}
		};

		for (const node of nodes) {
			if (node.type === 'reportComponentEmbed') {
				flush();
				out.push({
					kind: 'embed',
					toolStepId: String(node.attrs?.toolStepId ?? ''),
					componentType: String(node.attrs?.componentType ?? '')
				});
			} else {
				run.push(node);
			}
		}
		flush();
		return out;
	});
</script>

{#each segments as segment, i (i)}
	{#if segment.kind === 'embed'}
		{#if segment.toolStepId && segment.componentType}
			<div class="report-embed my-4">
				<ReportEmbedLive
					toolStepId={segment.toolStepId}
					componentType={segment.componentType}
				/>
			</div>
		{/if}
	{:else}
		<ContentRenderer content={segment.json} {conversationId} {availableDocuments} />
	{/if}
{/each}
