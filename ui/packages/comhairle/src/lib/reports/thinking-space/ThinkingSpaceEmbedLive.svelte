<script lang="ts">
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import EmbedState from '../EmbedState.svelte';
	import type { ReportEmbedProps } from '../embeds';
	import ThinkingSpaceReportSection from './ThinkingSpaceReportSection.svelte';
	import { loadThinkingSpaceEmbedData, type ThinkingSpaceEmbedData } from './embedData';
	import type { ThinkingSpaceEmbeddableComponentType } from './embeddableComponents';

	/**
	 * A live Thinking Space section block in the report (ADR-0012). Loads its data from the
	 * block's public or preview endpoint (ADR-0061).
	 */
	let { toolStepId, componentType, preview = false }: ReportEmbedProps = $props();

	let data = $state<ThinkingSpaceEmbedData | null>(null);
	let loading = $state(true);

	$effect(() => {
		const stepId = toolStepId;
		const blockType = componentType as ThinkingSpaceEmbeddableComponentType;
		const fromPreview = preview;
		let cancelled = false;
		loading = true;
		data = null;

		(async () => {
			const result = await tryCatchAsync(() =>
				loadThinkingSpaceEmbedData(stepId, blockType, fromPreview)
			);
			if (cancelled) return;
			if (result.err === null) data = result.ok;
			loading = false;
		})();

		return () => {
			cancelled = true;
		};
	});
</script>

{#if loading}
	<EmbedState state="loading" />
{:else if !data}
	<EmbedState state="unavailable" />
{:else}
	<ThinkingSpaceReportSection {data} />
{/if}
