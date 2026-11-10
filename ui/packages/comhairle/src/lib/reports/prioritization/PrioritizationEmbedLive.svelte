<script lang="ts">
	import type { Component } from 'svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import EmbedState from '../EmbedState.svelte';
	import type { ReportEmbedProps } from '../embeds';
	import { loadPrioritizationEmbedData, type PrioritizationEmbedData } from './embedData';
	import type { PrioritizationEmbeddableComponentType } from './embeddableComponents';

	type Section = Component<{ data: PrioritizationEmbedData }>;

	// The editor imports every tool's embed, so the chart library loads only once an embed
	// shows. It also reads matchMedia on import, which unit tests of the editor don't have.
	const loadSection = async (): Promise<Section> =>
		(await import('./PrioritizationReportSection.svelte')).default;

	/**
	 * A live Prioritization section block in the report (ADR-0012). Loads its data from the
	 * block's public or preview endpoint (ADR-0061).
	 */
	let { toolStepId, componentType, item, preview = false }: ReportEmbedProps = $props();

	let data = $state<PrioritizationEmbedData | null>(null);
	let ReportSection = $state<Section | null>(null);
	let loading = $state(true);

	$effect(() => {
		const stepId = toolStepId;
		const blockType = componentType as PrioritizationEmbeddableComponentType;
		const proposalId = item;
		const fromPreview = preview;
		let cancelled = false;
		loading = true;
		data = null;

		(async () => {
			const result = await tryCatchAsync(async () => {
				const [loaded, section] = await Promise.all([
					loadPrioritizationEmbedData(stepId, blockType, proposalId, fromPreview),
					loadSection()
				]);
				return { loaded, section };
			});
			if (cancelled) return;
			if (result.err === null) {
				data = result.ok.loaded;
				ReportSection = result.ok.section;
			}
			loading = false;
		})();

		return () => {
			cancelled = true;
		};
	});
</script>

{#if loading}
	<EmbedState state="loading" />
{:else if !data || !ReportSection}
	<EmbedState state="unavailable" />
{:else}
	<ReportSection {data} />
{/if}
