<script lang="ts">
	import type { Component } from 'svelte';
	import EmbedState from './EmbedState.svelte';
	import PolisEmbedLive from './polis/PolisEmbedLive.svelte';
	import { toolTypeForWidget, type ReportEmbedProps } from './embeds';

	/**
	 * A live report widget embedded in the report (ADR-0012). Hands the stored reference to the
	 * owning tool's embed component, which loads its own data. Shared by the editor node view
	 * and the published report page.
	 */
	let { toolStepId, componentType }: ReportEmbedProps = $props();

	const EMBED_BY_TOOL: Record<string, Component<ReportEmbedProps>> = {
		polis: PolisEmbedLive
	};

	const ToolEmbed = $derived(EMBED_BY_TOOL[toolTypeForWidget(componentType) ?? '']);
</script>

{#if ToolEmbed}
	<ToolEmbed {toolStepId} {componentType} />
{:else}
	<EmbedState state="unavailable" />
{/if}
