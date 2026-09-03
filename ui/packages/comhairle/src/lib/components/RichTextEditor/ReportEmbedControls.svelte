<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import ChartNoAxesColumn from 'lucide-svelte/icons/chart-no-axes-column';
	import ChevronRight from 'lucide-svelte/icons/chevron-right';
	import {
		POLIS_EMBEDDABLE_COMPONENTS,
		type EmbeddableComponentMeta
	} from '$lib/reports/polis/embeddableComponents';

	/** A report-capable Step offered in stage 1 of the picker. */
	export type EmbeddableStep = { id: string; name: string; toolType: string };

	let { editor, steps }: { editor: Editor | undefined; steps: EmbeddableStep[] } = $props();

	let open = $state(false);
	let selectedStep = $state<EmbeddableStep | null>(null);

	// MVP: Polis only. When more tools land, switch the allow-list on selectedStep.toolType.
	const componentsForStep = $derived<EmbeddableComponentMeta[]>(
		selectedStep?.toolType === 'polis' ? POLIS_EMBEDDABLE_COMPONENTS : []
	);

	function reset() {
		selectedStep = null;
	}

	function pickStep(step: EmbeddableStep) {
		selectedStep = step;
	}

	// Insert stores only the reference (ADR-0012); the embedded component loads its own data
	// live, so this is instant — no freeze step.
	function pickComponent(meta: EmbeddableComponentMeta) {
		if (!editor || !selectedStep) return;
		editor
			.chain()
			.focus()
			.setReportComponentEmbed({ toolStepId: selectedStep.id, componentType: meta.type })
			.run();
		open = false;
		reset();
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
					? `Pick which part of "${selectedStep.name}" to embed. It's added where your cursor is.`
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
						class="hover:bg-accent flex w-full items-center justify-between gap-3 rounded-lg border p-3 text-left"
						onclick={() => pickComponent(meta)}
					>
						<span class="flex flex-col">
							<span class="text-base font-medium">{meta.label}</span>
							<span class="text-muted-foreground text-sm">{meta.description}</span>
						</span>
						<ChevronRight class="text-muted-foreground size-4" />
					</button>
				{/each}
			</div>
			<Dialog.Footer class="sm:justify-start">
				<Button variant="ghost" size="sm" onclick={() => (selectedStep = null)}>Back</Button
				>
			</Dialog.Footer>
		{/if}
	</Dialog.Content>
</Dialog.Root>
