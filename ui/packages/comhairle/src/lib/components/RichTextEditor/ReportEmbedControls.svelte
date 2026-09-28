<script lang="ts">
	import type { Editor } from '@tiptap/core';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import ChartNoAxesColumn from 'lucide-svelte/icons/chart-no-axes-column';
	import ChevronRight from 'lucide-svelte/icons/chevron-right';
	import Globe from 'lucide-svelte/icons/globe';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { reportWidgetsForTool, type ReportWidgetMeta } from '$lib/reports/embeds';

	/** A report-capable Step offered in stage 1 of the picker. */
	export type EmbeddableStep = {
		id: string;
		conversationId: string;
		workflowId: string;
		name: string;
		toolType: string;
		reportDataPublic: boolean;
	};

	let { editor, steps }: { editor: Editor | undefined; steps: EmbeddableStep[] } = $props();

	let open = $state(false);
	let selectedStep = $state<EmbeddableStep | null>(null);

	const componentsForStep = $derived<ReportWidgetMeta[]>(
		reportWidgetsForTool(selectedStep?.toolType)
	);

	// Steps made public from this dialog. Tracked locally rather than invalidating the workflow
	// key, which would rerun the report page load under an in-progress edit.
	let madePublic = $state<Set<string>>(new Set());
	let makingPublic = $state(false);
	const selectedStepPublic = $derived(
		!!selectedStep && (selectedStep.reportDataPublic || madePublic.has(selectedStep.id))
	);

	// Opting in is always an explicit admin action (ADR-0046): embedding never flips it.
	async function makeSelectedStepPublic() {
		if (!selectedStep) return;
		const step = selectedStep;
		makingPublic = true;
		const result = await tryCatchAsync(() =>
			apiClient.UpdateConversationWorkflowStep(
				{ report_data_public: true },
				{
					params: {
						conversation_id: step.conversationId,
						workflow_id: step.workflowId,
						workflow_step_id: step.id
					}
				}
			)
		);
		makingPublic = false;
		if (result.err !== null) {
			notifications.send({
				message: "Couldn't make this step's results public",
				priority: 'ERROR'
			});
			return;
		}
		madePublic = new Set([...madePublic, step.id]);
	}

	function reset() {
		selectedStep = null;
	}

	function pickStep(step: EmbeddableStep) {
		selectedStep = step;
	}

	// Insert stores only the reference (ADR-0012); the embedded component loads its own data
	// live, so this is instant — no freeze step.
	function pickComponent(meta: ReportWidgetMeta) {
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
			{#if !selectedStepPublic}
				<div class="bg-muted flex flex-col gap-3 rounded-lg p-4">
					<p class="text-base">
						Only admins can see these results. People reading the published report will
						see a placeholder until you make this step's results public.
					</p>
					<p class="text-muted-foreground text-sm">
						Public results are anonymised totals and moderated content only. You can
						turn this off again in the step's settings.
					</p>
					<div>
						<Button
							variant="outline"
							size="sm"
							disabled={makingPublic}
							onclick={makeSelectedStepPublic}
						>
							<Globe class="size-4" />
							Make results public
						</Button>
					</div>
				</div>
			{/if}
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
