<script lang="ts">
	import EmbedState from '../EmbedState.svelte';
	import { loadEmbed } from '../embedLoad.svelte';
	import type { ReportEmbedProps } from '../embeds';
	import { loadHeyFormQuestion } from './embedData';

	/**
	 * A HeyForm question's results in the report, loaded from its public or preview endpoint
	 * (ADR-0061). Shared by the editor node view and the published report page.
	 */
	let { toolStepId, item, preview = false }: ReportEmbedProps = $props();

	const embed = loadEmbed(() => [
		item
			? loadHeyFormQuestion(toolStepId, item, preview)
			: Promise.reject(new Error('A question embed needs a question')),
		import('./HeyFormQuestion.svelte').then((module) => module.default)
	]);
</script>

{#if embed.loading}
	<EmbedState state="loading" />
{:else if !embed.loaded}
	<EmbedState state="unavailable" />
{:else}
	<embed.loaded.Section question={embed.loaded.data} />
{/if}
