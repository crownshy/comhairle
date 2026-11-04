<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import ChartNoAxesColumn from 'lucide-svelte/icons/chart-no-axes-column';
	import ChevronRight from 'lucide-svelte/icons/chevron-right';
	import {
		sectionBlocksForTool,
		sectionBlocksInDocument,
		type SectionBlockMeta
	} from '$lib/reports/embeds';

	/** A report-capable Step offered in stage 1 of the picker. */
	export type EmbeddableStep = { id: string; name: string; toolType: string };

	let { editor, steps }: { editor: Editor | undefined; steps: EmbeddableStep[] } = $props();

	let open = $state(false);
	let selectedStep = $state<EmbeddableStep | null>(null);
	let blockAwaitingConfirm = $state<SectionBlockMeta | null>(null);

	const componentsForStep = $derived<SectionBlockMeta[]>(
		sectionBlocksForTool(selectedStep?.toolType)
	);

	const title = $derived.by(() => {
		if (blockAwaitingConfirm) return 'Make these results public?';
		if (selectedStep) return 'Choose a component';
		return 'Choose a step';
	});

	function reset() {
		selectedStep = null;
		blockAwaitingConfirm = null;
	}

	function reportAlreadyEmbeds(stepId: string, sectionBlock: string): boolean {
		if (!editor) return false;
		return sectionBlocksInDocument(JSON.stringify(editor.getJSON())).some(
			(block) => block.workflowStepId === stepId && block.sectionBlock === sectionBlock
		);
	}

	function pickStep(step: EmbeddableStep) {
		selectedStep = step;
	}

	// Insert stores only the reference (ADR-0012); the embedded component loads its own data
	// live, so this is instant, with no freeze step.
	function insertComponent(meta: SectionBlockMeta) {
		if (!editor || !selectedStep) return;
		editor
			.chain()
			.focus()
			.setReportComponentEmbed({ toolStepId: selectedStep.id, componentType: meta.type })
			.run();
		open = false;
		reset();
	}

	// Embedding a section block for the first time is what makes it public (ADR-0061), so ask first.
	function pickComponent(meta: SectionBlockMeta) {
		if (!selectedStep) return;
		if (reportAlreadyEmbeds(selectedStep.id, meta.type)) insertComponent(meta);
		else blockAwaitingConfirm = meta;
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
			<Dialog.Title>{title}</Dialog.Title>
			{#if blockAwaitingConfirm && selectedStep}
				<Dialog.Description>
					This makes the "{blockAwaitingConfirm.label}" results from "{selectedStep.name}"
					public once the report is published. Anyone with the report link can see these
					totals. Remove the embed or unpublish the report to make them private again.
				</Dialog.Description>
			{:else if selectedStep}
				<Dialog.Description>
					Pick which part of "{selectedStep.name}" to embed. It's added where your cursor
					is.
				</Dialog.Description>
			{:else}
				<Dialog.Description>
					Embed results from a step in this conversation into the report.
				</Dialog.Description>
			{/if}
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
		{:else if blockAwaitingConfirm}
			<!-- Stage 3: confirm a section block's first embed -->
			{@const block = blockAwaitingConfirm}
			<Dialog.Footer class="sm:justify-between">
				<Button variant="ghost" size="sm" onclick={() => (blockAwaitingConfirm = null)}
					>Back</Button
				>
				<Button onclick={() => insertComponent(block)}>Embed and make public</Button>
			</Dialog.Footer>
		{:else}
			<!-- Stage 2: pick the component -->
			<p class="text-muted-foreground text-base">
				Results you embed become public when the report is published.
			</p>
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
