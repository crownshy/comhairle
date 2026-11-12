<script lang="ts">
	import type { Component } from 'svelte';
	import EmbedState from './EmbedState.svelte';
	import HeyFormEmbedLive from './heyform/HeyFormEmbedLive.svelte';
	import PolisEmbedLive from './polis/PolisEmbedLive.svelte';
	import { toolTypeForSectionBlock, type ReportEmbedProps } from './embeds';
	import { getReportPreview } from './reportPreview';

	/**
	 * A live section block embedded in the report (ADR-0012). Hands the stored reference to the
	 * owning tool's embed component, which loads its own data. Shared by the editor node view
	 * and the published report page.
	 */
	let { toolStepId, componentType, item, preview = false }: ReportEmbedProps = $props();

	const onPreviewPage = getReportPreview();
	const loadsPreview = $derived(preview || onPreviewPage());

	const EMBED_BY_TOOL: Record<string, Component<ReportEmbedProps>> = {
		polis: PolisEmbedLive,
		heyform: HeyFormEmbedLive
	};

	const ToolEmbed = $derived(EMBED_BY_TOOL[toolTypeForSectionBlock(componentType) ?? '']);
</script>

{#if ToolEmbed}
	<ToolEmbed {toolStepId} {componentType} {item} preview={loadsPreview} />
{:else}
	<EmbedState state="unavailable" />
{/if}
