<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { resolve } from '$app/paths';
	import { buttonVariants, LoadingButton } from '$lib/components/ui/button';
	import SelectableOptionRow from '$lib/components/SelectableOptionRow.svelte';
	import LoadingOverlay from '$lib/components/LoadingOverlay.svelte';
	import { PALETTE_TOOLS, isEventPaletteItem, type CreationKey } from '$lib/tool_meta';
	import type { DemographicSelection } from '$lib/demographicCatalogue';
	import {
		listPublishedDemographicForms,
		listPublishedDemographicQuestions,
		type PickerLoad,
		type PublishedDemographicItem
	} from '$lib/demographicCatalogue';
	import ExternalLink from '@lucide/svelte/icons/external-link';
	import Check from '@lucide/svelte/icons/check';
	import { goto, invalidate } from '$app/navigation';
	import { createWorkflowStep } from './createWorkflowstep';
	import { newStepHighlight } from '$lib/stores/newStepHighlight.svelte';
	import { key } from '$lib/utils/invalidationKey';
	import type { ConversationDto, WorkflowDto } from '@crownshy/api-client/api';
	import { notifications } from '$lib/notifications.svelte';
	import DemographicItemPicker from './DemographicItemPicker.svelte';
	import NoOrganizationStepRow from './NoOrganizationStepRow.svelte';

	/**
	 * The "Add a step" dialog: a two-column picker (step-type list on the left, rich
	 * detail panel on the right). Mirrors the choose-a-template dialog's shell and
	 * reuses {@link SelectableOptionRow}. On confirm it creates the step for the selected tool
	 * (with the chosen question or form for demographic steps), or hands off to the
	 * create-event flow for an event stand-in like the video conference.
	 */
	type Props = {
		workflowId: WorkflowDto['id'];
		conversationId: ConversationDto['id'];
		highestStepOrder: number;
		open?: boolean;
		/** The hosting organisation whose question and form banks demographic steps draw from. */
		organizationId: string | null;
	};

	let {
		workflowId,
		conversationId,
		organizationId,
		highestStepOrder,
		open = $bindable(false)
	}: Props = $props();

	let adding = $state<boolean>(false);

	let selectedType = $state<string>(PALETTE_TOOLS[0].type);
	const selected = $derived(
		PALETTE_TOOLS.find((t) => t.type === selectedType) ?? PALETTE_TOOLS[0]
	);

	async function addStep(creationKey: CreationKey, selection?: DemographicSelection) {
		if (adding) return;
		adding = true;

		const created = await createWorkflowStep({
			creationKey,
			conversationId,
			workflowId,
			highestStepOrder,
			selection
		});

		adding = false;

		if (created.err !== null) {
			console.error(created.err);
			notifications.send({ priority: 'ERROR', message: 'Failed to create step' });
		}

		if (!created.ok) return;
		await invalidate(key('admin/conversation/workflow'));
		await invalidate(key('admin/conversation/design/workflow'));
		notifications.send({ priority: 'INFO', message: 'Step added' });
		newStepHighlight.flag(created.ok.id);
		open = false;

		// Return to the design board (works from either entry point: the board's own Add step
		// button or the workflow strip's while inside a step editor). The design page reacts to
		// `newStepHighlight` by scrolling the new card into view and briefly highlighting it, so
		// the operator sees exactly which step was just created instead of landing in its editor.
		await goto(
			resolve('/(admin)/admin/conversations/[conversation_id]/design', {
				conversation_id: conversationId
			})
		);
	}

	// The "Online video conference" palette entry has no backing workflow tool, so
	// adding it creates a conversation Event instead. Hand off to the create-event
	// flow, where the organiser sets the required date, time, and details.
	function addEvent() {
		open = false;
		goto(
			resolve('/(admin)/admin/conversations/[conversation_id]/events/new', {
				conversation_id: conversationId
			})
		);
	}

	let questionLoad = $state<PickerLoad>({ status: 'loading' });
	let formLoad = $state<PickerLoad>({ status: 'loading' });
	let picked = $state<{ type: string; item: PublishedDemographicItem } | null>(null);

	const selectedCreationKey = $derived(
		isEventPaletteItem(selected) ? null : selected.creationKey
	);
	const demographicNoun = $derived(
		selectedCreationKey === 'Demographic Question'
			? 'question'
			: selectedCreationKey === 'Demographic Form'
				? 'form'
				: null
	);
	const pickerLoad = $derived(
		selectedCreationKey === 'Demographic Form' ? formLoad : questionLoad
	);
	function needsOrganization(tool: (typeof PALETTE_TOOLS)[number]): boolean {
		return (
			!isEventPaletteItem(tool) &&
			(tool.creationKey === 'Demographic Question' || tool.creationKey === 'Demographic Form')
		);
	}
	const missingOrganization = $derived(organizationId === null);
	const pickedForSelected = $derived(
		picked && picked.type === selected.type ? picked.item : null
	);

	async function loadPickers() {
		const [questions, forms] = await Promise.all([
			listPublishedDemographicQuestions(organizationId),
			listPublishedDemographicForms(organizationId)
		]);
		questionLoad =
			questions.err === null ? { status: 'ready', items: questions.ok } : { status: 'error' };
		formLoad = forms.err === null ? { status: 'ready', items: forms.ok } : { status: 'error' };
	}

	// Reload on every open so questions and forms published since the last open appear.
	$effect(() => {
		if (open) void loadPickers();
	});
</script>

<Dialog.Root bind:open>
	<Dialog.Content
		class="flex max-h-[90vh] w-full max-w-5xl flex-col gap-6 overflow-hidden sm:max-w-5xl"
	>
		<Dialog.Header>
			<Dialog.Title class="text-center text-2xl font-semibold">Add a step</Dialog.Title>
			<Dialog.Description class="text-center">
				Choose a step type below. You can configure details later.
			</Dialog.Description>
		</Dialog.Header>

		<!-- Phones: one scrolling column, list as a 2-up grid above the detail.
			Tablet and up: side by side, each column scrolling on its own. -->
		<div
			class="-mx-1 flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto px-1 md:mx-0 md:flex-row md:items-start md:gap-6 md:overflow-hidden md:px-0 md:py-1"
		>
			<!-- p-1 leaves room for the focus outline, which the scroll edge would otherwise
				clip; md:-my-1 cancels it so the first option lines up with the detail panel. -->
			<div
				class="grid shrink-0 grid-cols-2 gap-2 p-1 md:-my-1 md:flex md:w-60 md:flex-col md:gap-3 md:self-stretch md:overflow-y-auto md:pr-3 md:[scrollbar-gutter:stable] lg:w-72"
			>
				{#each PALETTE_TOOLS as tool (tool.type)}
					{#if missingOrganization && needsOrganization(tool)}
						<NoOrganizationStepRow name={tool.displayName} {conversationId} />
					{:else}
						<SelectableOptionRow
							selected={selectedType === tool.type}
							name={tool.displayName}
							onSelect={() => (selectedType = tool.type)}
							class="max-md:h-full max-md:p-3"
						/>
					{/if}
				{/each}
			</div>

			<div
				class="bg-card border-border flex shrink-0 flex-col gap-4 rounded-xl border p-4 md:min-h-0 md:flex-1 md:shrink md:self-stretch md:overflow-y-auto md:p-6"
			>
				{#if selected.preview}
					<div class="bg-muted shrink-0 overflow-hidden rounded-lg p-4">
						<img
							src={selected.preview.src}
							alt={selected.preview.alt}
							class="mx-auto max-h-64 w-auto rounded-md object-contain"
						/>
					</div>
				{/if}

				<div class="flex flex-col gap-2">
					<div class="flex items-baseline justify-between gap-4">
						<h3 class="text-foreground text-lg font-bold">{selected.displayName}</h3>
						{#if selected.infoSlug}
							<a
								href={resolve('/(admin)/admin/info/tools/[tool_id]', {
									tool_id: selected.infoSlug
								})}
								target="_blank"
								rel="noopener noreferrer"
								class="text-primary inline-flex shrink-0 items-center gap-1.5 text-base font-medium hover:underline"
							>
								<ExternalLink class="size-4" />
								Learn more
								<span class="sr-only"
									>about {selected.displayName} (opens in a new tab)</span
								>
							</a>
						{/if}
					</div>
					<p class="text-muted-foreground text-base">{selected.description}</p>
				</div>

				{#if demographicNoun}
					<DemographicItemPicker
						noun={demographicNoun}
						load={pickerLoad}
						selectedId={pickedForSelected?.id ?? null}
						onSelect={(item) => (picked = { type: selected.type, item })}
					/>
				{/if}

				{#if selected.bestFor.length}
					<section class="flex flex-col gap-2">
						<h4 class="text-muted-foreground text-sm">BEST FOR</h4>
						{#each selected.bestFor as item (item)}
							<div class="flex items-start gap-2">
								<span
									class="bg-accent mt-0.5 flex size-4 shrink-0 items-center justify-center rounded-full"
								>
									<Check class="text-primary size-2.5" />
								</span>
								<span class="text-muted-foreground text-base">{item}</span>
							</div>
						{/each}
					</section>
				{/if}

				{#if selected.features.length}
					<section class="flex flex-col gap-2">
						<h4 class="text-muted-foreground text-sm">FEATURES</h4>
						{#each selected.features as item (item)}
							<div class="flex items-start gap-2">
								<span
									class="bg-accent mt-0.5 flex size-4 shrink-0 items-center justify-center rounded-full"
								>
									<Check class="text-primary size-2.5" />
								</span>
								<span class="text-muted-foreground text-base">{item}</span>
							</div>
						{/each}
					</section>
				{/if}

				{#if selected.whatYoudGet.length}
					<section class="flex flex-col gap-2">
						<h4 class="text-muted-foreground text-sm">WHAT YOU'D GET</h4>
						{#each selected.whatYoudGet as item (item)}
							<div class="flex items-start gap-2">
								<span
									class="bg-accent mt-0.5 flex size-4 shrink-0 items-center justify-center rounded-full"
								>
									<Check class="text-primary size-2.5" />
								</span>
								<span class="text-muted-foreground text-base">{item}</span>
							</div>
						{/each}
					</section>
				{/if}
			</div>
		</div>

		<Dialog.Footer class="items-center sm:justify-end">
			<div class="flex items-center gap-2">
				<Dialog.Close class={buttonVariants({ variant: 'outline', size: 'sm' })}>
					Cancel
				</Dialog.Close>
				<LoadingButton
					variant="default"
					size="sm"
					loading={adding}
					disabled={demographicNoun !== null && pickedForSelected === null}
					onclick={() =>
						isEventPaletteItem(selected)
							? addEvent()
							: addStep(selected.creationKey, pickedForSelected ?? undefined)}
				>
					+ Add this step
				</LoadingButton>
			</div>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<!-- The dialog stays open while the step is created, so a button spinner alone is easy to
	miss (Polis/HeyForm can take a few seconds). Mirror the create-conversation overlay. -->
<LoadingOverlay open={adding} message="Adding step…" />
