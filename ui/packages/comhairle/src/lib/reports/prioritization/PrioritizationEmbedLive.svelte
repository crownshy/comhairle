<script lang="ts">
	import EmbedState from '../EmbedState.svelte';
	import { loadEmbed } from '../embedLoad.svelte';
	import type { ReportEmbedProps } from '../embeds';
	import { loadPrioritizationEmbedData } from './embedData';
	import type { PrioritizationEmbeddableComponentType } from './embeddableComponents';

	/**
	 * A live Prioritization section block in the report (ADR-0012). Loads its data from the
	 * block's public or preview endpoint (ADR-0061).
	 */
	let { toolStepId, componentType, item, preview = false }: ReportEmbedProps = $props();

	const embed = loadEmbed(() => [
		loadPrioritizationEmbedData(
			toolStepId,
			componentType as PrioritizationEmbeddableComponentType,
			item,
			preview
		),
		import('./PrioritizationReportSection.svelte').then((module) => module.default)
	]);
</script>

{#if embed.loading}
	<EmbedState state="loading" />
{:else if !embed.loaded}
	<EmbedState state="unavailable" />
{:else}
	<embed.loaded.Section data={embed.loaded.data} />
{/if}
