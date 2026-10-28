<script lang="ts">
	import ArrowLeft from 'lucide-svelte/icons/arrow-left';
	import Check from 'lucide-svelte/icons/check';
	import ChevronRight from 'lucide-svelte/icons/chevron-right';
	import Globe from 'lucide-svelte/icons/globe';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import ReportEmbedLive from '$lib/reports/ReportEmbedLive.svelte';
	import { sectionBlocksForTool, type SectionBlockMeta } from '$lib/reports/embeds';
	import type { EmbeddableStep } from '$lib/components/RichTextEditor/ReportEmbedControls.svelte';
	import { cn } from '$lib/utils';

	type Props = {
		open: boolean;
		steps: EmbeddableStep[];
		/** `stepId:sectionBlock` for every chart the report already embeds. */
		embeddedKeys: Set<string>;
		published: boolean;
		onPick: (step: EmbeddableStep, block: SectionBlockMeta) => void;
	};
	let { open = $bindable(), steps, embeddedKeys, published, onPick }: Props = $props();

	let selectedStep = $state<EmbeddableStep | null>(null);
	const blocks = $derived(sectionBlocksForTool(selectedStep?.toolType));

	function chartsInReport(step: EmbeddableStep) {
		return sectionBlocksForTool(step.toolType).filter((block) =>
			embeddedKeys.has(`${step.id}:${block.type}`)
		).length;
	}

	function pick(block: SectionBlockMeta) {
		if (!selectedStep) return;
		onPick(selectedStep, block);
		open = false;
		selectedStep = null;
	}
</script>

<Dialog.Root
	bind:open
	onOpenChange={(isOpen) => {
		if (!isOpen) selectedStep = null;
	}}
>
	<Dialog.Content class={cn(selectedStep ? 'sm:max-w-4xl' : 'sm:max-w-lg')}>
		<Dialog.Header>
			<Dialog.Title>
				{selectedStep ? `Charts from ${selectedStep.name}` : 'Add a chart'}
			</Dialog.Title>
			<Dialog.Description class="text-base">
				{selectedStep
					? 'Pick one to add where your cursor is.'
					: 'Which step should the chart come from?'}
			</Dialog.Description>
		</Dialog.Header>

		{#if !selectedStep}
			<div class="flex flex-col gap-2">
				{#each steps as step (step.id)}
					{@const inReport = chartsInReport(step)}
					<button
						type="button"
						class="hover:bg-accent flex w-full items-center justify-between gap-3 rounded-xl border p-4 text-left"
						onclick={() => (selectedStep = step)}
					>
						<span class="flex flex-col">
							<span class="text-base font-medium">{step.name}</span>
							<span class="text-muted-foreground text-sm">
								{sectionBlocksForTool(step.toolType).length} charts{inReport > 0
									? `, ${inReport} in the report`
									: ''}
							</span>
						</span>
						<ChevronRight class="text-muted-foreground size-5" />
					</button>
				{/each}
				{#if steps.length === 0}
					<p class="text-muted-foreground text-base">
						No steps in this conversation have charts yet.
					</p>
				{/if}
			</div>
		{:else}
			<div class="grid max-h-[60vh] gap-3 overflow-y-auto sm:grid-cols-2">
				{#each blocks as block (block.type)}
					{@const inReport = embeddedKeys.has(`${selectedStep.id}:${block.type}`)}
					<button
						type="button"
						class={cn(
							'hover:border-primary flex flex-col gap-2 rounded-xl border p-2 text-left transition-colors',
							inReport && 'border-primary/60'
						)}
						onclick={() => pick(block)}
					>
						<!-- A live preview at half size, so the chart is picked by sight. -->
						<div
							class="bg-background pointer-events-none relative h-40 overflow-hidden rounded-lg"
							aria-hidden="true"
						>
							<div
								class="absolute top-0 left-0 w-[200%] origin-top-left scale-50 p-4"
							>
								<ReportEmbedLive
									toolStepId={selectedStep.id}
									componentType={block.type}
								/>
							</div>
						</div>
						<span class="flex flex-col px-1">
							<span class="text-base font-medium">{block.label}</span>
							<span class="text-muted-foreground text-sm">{block.description}</span>
						</span>
						{#if inReport}
							<span
								class="text-primary flex items-center gap-1.5 px-1 text-sm font-medium"
							>
								<Check class="size-4" />Already in the report
							</span>
						{/if}
					</button>
				{/each}
			</div>
		{/if}

		<Dialog.Footer class="items-center sm:justify-between">
			<p class="text-muted-foreground flex items-center gap-2 text-sm">
				<Globe class="size-4 shrink-0" />
				{published
					? 'The report is published, so a chart you add is public straight away.'
					: 'Charts you add become public when the report is published.'}
			</p>
			{#if selectedStep}
				<Button variant="ghost" onclick={() => (selectedStep = null)}>
					<ArrowLeft class="size-4" />Steps
				</Button>
			{/if}
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
