<script lang="ts">
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { loadPolisEmbedData, type PolisEmbedData } from './embedData';
	import PolisReportSection from './PolisReportSection.svelte';
	import type { PolisEmbeddableComponentType } from './embeddableComponents';
	import EmbedState from '../EmbedState.svelte';
	import type { ReportEmbedProps } from '../embeds';

	/**
	 * A live Polis section block in the report (ADR-0012). Loads its data from the block's
	 * public or preview endpoint (ADR-0061) and renders the same section component the Insights
	 * tab uses. Shared by the editor node view and the published report page.
	 */
	let { toolStepId, componentType, preview = false }: ReportEmbedProps = $props();

	let data = $state<PolisEmbedData | null>(null);
	let loading = $state(true);
	let failed = $state(false);

	// Reload whenever the reference changes. Client-side only (needs the API + DOM); on the
	// SSR'd public page this runs on hydration and the component pops in.
	$effect(() => {
		const stepId = toolStepId;
		const blockType = componentType as PolisEmbeddableComponentType;
		const fromPreview = preview;
		let cancelled = false;
		loading = true;
		failed = false;
		data = null;

		(async () => {
			const result = await tryCatchAsync(() =>
				loadPolisEmbedData(stepId, blockType, fromPreview)
			);
			if (cancelled) return;
			if (result.err !== null) {
				failed = true;
			} else {
				data = result.ok;
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
{:else if failed || !data}
	<EmbedState state="unavailable" />
{:else}
	<PolisReportSection {data} />
{/if}
