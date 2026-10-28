<script lang="ts">
	import Globe from 'lucide-svelte/icons/globe';
	import Lock from 'lucide-svelte/icons/lock';
	import TriangleAlert from 'lucide-svelte/icons/triangle-alert';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import LoadingButton from '$lib/components/ui/button/loading-button.svelte';
	import {
		embeddedSectionBlockLabel,
		sectionBlocksForTool,
		type PublicStepResults
	} from '$lib/reports/embeds';
	import type { EmbeddableStep } from '$lib/components/RichTextEditor/ReportEmbedControls.svelte';

	type Props = {
		open: boolean;
		published: boolean;
		/** What the report embeds right now, per step (ADR-0061). */
		results: PublicStepResults[];
		steps: EmbeddableStep[];
		/** Starter-draft prompts nobody has written over yet. */
		promptsLeft: number;
		/** Without update permission the dialog only explains what is shared. */
		canPublish: boolean;
		onPublish: () => Promise<void>;
	};
	let {
		open = $bindable(),
		published,
		results,
		steps,
		promptsLeft,
		canPublish,
		onPublish
	}: Props = $props();

	let publishing = $state(false);

	const privateItems = $derived([
		'who took part and how each person voted',
		...results.flatMap((step) => {
			const blocks = sectionBlocksForTool(steps.find((s) => s.id === step.stepId)?.toolType);
			const perItem = blocks.filter((block) => block.itemName);
			const others =
				blocks.length -
				perItem.length -
				step.sectionBlocks.filter((block) => !block.itemName).length;
			return [
				...(others > 0
					? [`${others} other ${others === 1 ? 'chart' : 'charts'} from ${step.stepName}`]
					: []),
				...perItem.map((block) => `other ${block.itemName?.other} from ${step.stepName}`)
			];
		}),
		...steps
			.filter((step) => !results.some((r) => r.stepId === step.id))
			.map((step) => `everything from ${step.name}`),
		'impacts and facilitator notes'
	]);

	async function publish() {
		publishing = true;
		await onPublish();
		publishing = false;
		open = false;
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="sm:max-w-md">
		<Dialog.Header>
			<Dialog.Title class="text-2xl">
				{#if published}This report is public{:else if canPublish}Ready to publish?{:else}What
					publishing shares{/if}
			</Dialog.Title>
		</Dialog.Header>

		<section class="flex flex-col gap-3">
			<p class="flex items-center gap-2 text-base font-medium">
				<Globe class="text-primary size-5" />Anyone with the link can read
			</p>
			<ul class="flex flex-col gap-2 ps-7 text-base">
				<li>The summary and the text</li>
				{#each results as step (step.stepId)}
					{#each step.sectionBlocks as block (block.type)}
						<li>
							{embeddedSectionBlockLabel(block)}
							<span class="text-muted-foreground">from {step.stepName}</span>
						</li>
					{/each}
				{/each}
			</ul>
		</section>

		<p class="text-muted-foreground flex gap-2 text-base">
			<Lock class="mt-0.5 size-5 shrink-0" />
			<span>Stays private: {privateItems.join(', ')}.</span>
		</p>

		{#if promptsLeft > 0 && !published && canPublish}
			<p class="flex gap-2 text-base">
				<TriangleAlert class="text-destructive mt-0.5 size-5 shrink-0" />
				<span>
					{promptsLeft} starter {promptsLeft === 1 ? 'prompt is' : 'prompts are'} still in the
					report. Readers will see {promptsLeft === 1 ? 'it' : 'them'} as written.
				</span>
			</p>
		{/if}

		<Dialog.Footer>
			{#if published || !canPublish}
				<Button onclick={() => (open = false)}>Done</Button>
			{:else}
				<Button variant="ghost" onclick={() => (open = false)}>Keep editing</Button>
				<LoadingButton loading={publishing} onclick={publish}>Publish</LoadingButton>
			{/if}
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
