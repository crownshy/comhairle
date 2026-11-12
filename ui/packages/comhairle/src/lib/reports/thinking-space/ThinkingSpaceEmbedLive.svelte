<script lang="ts">
	import EmbedState from '../EmbedState.svelte';
	import { loadEmbed } from '../embedLoad.svelte';
	import type { ReportEmbedProps } from '../embeds';
	import { loadThinkingSpaceEmbedData } from './embedData';
	import type { ThinkingSpaceEmbeddableComponentType } from './embeddableComponents';

	/**
	 * A live Thinking Space section block in the report (ADR-0012). Loads its data from the
	 * block's public or preview endpoint (ADR-0061).
	 */
	let { toolStepId, componentType, preview = false }: ReportEmbedProps = $props();

	const embed = loadEmbed(() => [
		loadThinkingSpaceEmbedData(
			toolStepId,
			componentType as ThinkingSpaceEmbeddableComponentType,
			preview
		),
		import('./ThinkingSpaceReportSection.svelte').then((module) => module.default)
	]);
</script>

{#if embed.loading}
	<EmbedState state="loading" />
{:else if !embed.loaded}
	<EmbedState state="unavailable" />
{:else}
	<embed.loaded.Section data={embed.loaded.data} />
{/if}
