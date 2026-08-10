<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import { apiClient } from '@crownshy/api-client/client';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { notifications } from '$lib/notifications.svelte';
	import ChartNoAxesColumn from 'lucide-svelte/icons/chart-no-axes-column';
	import ChevronRight from 'lucide-svelte/icons/chevron-right';
	import Loader from 'lucide-svelte/icons/loader-circle';
	import { polisInsightsLoader } from '$lib/reports/polis/insights-loader';
	import {
		POLIS_EMBEDDABLE_COMPONENTS,
		type EmbeddableComponentMeta,
		type PolisEmbeddableComponentType
	} from '$lib/reports/polis/embeddableComponents';
	import { freezePolisComponent } from './reportEmbed/freezeReportComponent';

	/** A report-capable Step offered in stage 1 of the picker. */
	export type EmbeddableStep = { id: string; name: string; toolType: string };

	let { editor, steps }: { editor: Editor | undefined; steps: EmbeddableStep[] } = $props();

	let open = $state(false);
	let selectedStep = $state<EmbeddableStep | null>(null);
	let inserting = $state(false);

	// MVP: Polis only. When more tools land, switch the allow-list on selectedStep.toolType.
	const componentsForStep = $derived<EmbeddableComponentMeta[]>(
		selectedStep?.toolType === 'polis' ? POLIS_EMBEDDABLE_COMPONENTS : []
	);

	function reset() {
		selectedStep = null;
		inserting = false;
	}

	function pickStep(step: EmbeddableStep) {
		selectedStep = step;
	}

	async function pickComponent(meta: EmbeddableComponentMeta) {
		if (!editor || !selectedStep || inserting) return;
		inserting = true;
		try {
			const { polis } = await polisInsightsLoader(apiClient, selectedStep.id);
			const frozenHtml = freezePolisComponent(
				meta.type as PolisEmbeddableComponentType,
				polis.reportData,
				polis.statementAux
			);
			editor
				.chain()
				.focus()
				.setReportComponentEmbed({
					toolStepId: selectedStep.id,
					componentType: meta.type,
					frozenHtml
				})
				.run();
			open = false;
			reset();
		} catch {
			notifications.send({
				message: 'Could not embed that component. Please try again.',
				priority: 'ERROR'
			});
			inserting = false;
		}
	}
</script>

<div class="flex flex-wrap items-center gap-2 px-1 py-2">
	<Button
		variant="outline"
		size="sm"
		disabled={!editor || steps.length === 0}
		onclick={() => {
			reset();
			open = true;
		}}
	>
		<ChartNoAxesColumn class="size-4" />
		Embed report component
	</Button>
</div>

<Dialog.Root
	bind:open
	onOpenChange={(v) => {
		if (!v) reset();
	}}
>
	<Dialog.Content class="sm:max-w-[560px]">
		<Dialog.Header>
			<Dialog.Title>
				{selectedStep ? 'Choose a component' : 'Choose a step'}
			</Dialog.Title>
			<Dialog.Description>
				{selectedStep
					? `Pick which part of "${selectedStep.name}" to embed. It's added as a snapshot where your cursor is.`
					: 'Embed results from a step in this conversation into the report.'}
			</Dialog.Description>
		</Dialog.Header>

		{#if !selectedStep}
			<!-- Stage 1: pick the step -->
			<div class="flex flex-col gap-2">
				{#each steps as step (step.id)}
					<button
						type="button"
						class="hover:bg-accent flex w-full items-center justify-between rounded-lg border p-3 text-left"
						onclick={() => pickStep(step)}
					>
						<span class="text-base font-medium">{step.name}</span>
						<ChevronRight class="text-muted-foreground size-4" />
					</button>
				{/each}
				{#if steps.length === 0}
					<p class="text-muted-foreground p-3 text-base">
						No report-capable steps in this conversation yet.
					</p>
				{/if}
			</div>
		{:else}
			<!-- Stage 2: pick the component -->
			<div class="flex flex-col gap-2">
				{#each componentsForStep as meta (meta.type)}
					<button
						type="button"
						disabled={inserting}
						class="hover:bg-accent flex w-full items-center justify-between gap-3 rounded-lg border p-3 text-left disabled:opacity-60"
						onclick={() => pickComponent(meta)}
					>
						<span class="flex flex-col">
							<span class="text-base font-medium">{meta.label}</span>
							<span class="text-muted-foreground text-sm">{meta.description}</span>
						</span>
						{#if inserting}
							<Loader class="text-muted-foreground size-4 animate-spin" />
						{:else}
							<ChevronRight class="text-muted-foreground size-4" />
						{/if}
					</button>
				{/each}
			</div>
			<Dialog.Footer class="sm:justify-start">
				<Button
					variant="ghost"
					size="sm"
					disabled={inserting}
					onclick={() => (selectedStep = null)}
				>
					Back
				</Button>
			</Dialog.Footer>
		{/if}
	</Dialog.Content>
</Dialog.Root>
