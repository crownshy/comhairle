<script lang="ts">
	import { loadPolisEmbedData } from './embedData';
	import PolisReportSection from './PolisReportSection.svelte';
	import type { PolisEmbeddableComponentType } from './embeddableComponents';
	import EmbedState from '../EmbedState.svelte';
	import { loadEmbed } from '../embedLoad.svelte';
	import type { ReportEmbedProps } from '../embeds';

	/**
	 * A live Polis section block in the report (ADR-0012). Loads its data from the block's
	 * public or preview endpoint (ADR-0061) and renders the same section component the step's
	 * Insights page uses. Shared by the editor node view and the published report page.
	 */
	let { toolStepId, componentType, preview = false }: ReportEmbedProps = $props();

	// Client-side only (needs the API and DOM): on the SSR'd public page this runs on
	// hydration and the component pops in.
	const embed = loadEmbed(() => [
		loadPolisEmbedData(toolStepId, componentType as PolisEmbeddableComponentType, preview),
		Promise.resolve(PolisReportSection)
	]);
</script>

{#if embed.loading}
	<EmbedState state="loading" />
{:else if !embed.loaded}
	<EmbedState state="unavailable" />
{:else}
	<embed.loaded.Section data={embed.loaded.data} />
{/if}
