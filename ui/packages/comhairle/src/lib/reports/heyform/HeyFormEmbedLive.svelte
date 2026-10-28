<script lang="ts">
	import type { Component } from 'svelte';
	import type { HeyFormPublicQuestion } from '@crownshy/api-client/api';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import EmbedState from '../EmbedState.svelte';
	import type { ReportEmbedProps } from '../embeds';
	import { loadHeyFormQuestion } from './embedData';

	type QuestionChart = Component<{ question: HeyFormPublicQuestion }>;

	// The editor imports every tool's embed, so the chart library loads only once an embed
	// shows. It also reads matchMedia on import, which unit tests of the editor don't have.
	const loadChart = async (): Promise<QuestionChart> =>
		(await import('./HeyFormQuestion.svelte')).default;

	/** A HeyForm question's results in the report, loaded from its public or preview endpoint. */
	let { toolStepId, item, preview = false }: ReportEmbedProps = $props();

	let question = $state<HeyFormPublicQuestion | null>(null);
	let Chart = $state<QuestionChart | null>(null);
	let loading = $state(true);

	$effect(() => {
		const stepId = toolStepId;
		const questionId = item;
		const fromPreview = preview;
		let cancelled = false;
		loading = true;
		question = null;
		if (!questionId) {
			loading = false;
			return;
		}

		(async () => {
			const result = await tryCatchAsync(async () => {
				const [loaded, chart] = await Promise.all([
					loadHeyFormQuestion(stepId, questionId, fromPreview),
					loadChart()
				]);
				return { loaded, chart };
			});
			if (cancelled) return;
			if (result.err === null) {
				question = result.ok.loaded;
				Chart = result.ok.chart;
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
{:else if !question || !Chart}
	<EmbedState state="unavailable" />
{:else}
	<Chart {question} />
{/if}
