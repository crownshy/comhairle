<script lang="ts">
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { loadPolisEmbedData, type PolisEmbedData } from './embedData';
	import PolisReportSection from './PolisReportSection.svelte';
	import type { PolisEmbeddableComponentType } from './embeddableComponents';
	import Loader from 'lucide-svelte/icons/loader-circle';

	/**
	 * A live, interactive report component embedded in the report (ADR-0012). Given only a
	 * reference (`toolStepId` + `componentType`) it loads the data itself and renders the real
	 * section component — the same code the Insights tab uses — so expand/hover/filter all work.
	 * Shared by the editor node view and the published report page.
	 */
	let {
		toolStepId,
		componentType
	}: {
		toolStepId: string;
		componentType: string;
	} = $props();

	let data = $state<PolisEmbedData | null>(null);
	let loading = $state(true);
	let failed = $state(false);

	// Reload whenever the reference changes. Client-side only (needs the API + DOM); on the
	// SSR'd public page this runs on hydration and the component pops in.
	$effect(() => {
		const stepId = toolStepId;
		let cancelled = false;
		loading = true;
		failed = false;
		data = null;

		(async () => {
			const result = await tryCatchAsync(() => loadPolisEmbedData(stepId));
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
	<div
		class="border-border text-muted-foreground flex items-center justify-center gap-2 rounded-xl border border-dashed p-8 text-base"
	>
		<Loader class="size-4 animate-spin" />
		Loading component…
	</div>
{:else if failed || !data}
	<div
		class="border-border bg-card text-muted-foreground rounded-xl border border-dashed p-8 text-center text-base"
	>
		This component's data is no longer available.
	</div>
{:else}
	<PolisReportSection
		componentType={componentType as PolisEmbeddableComponentType}
		reportData={data.reportData}
		statementAux={data.statementAux}
		frozen={false}
	/>
{/if}
