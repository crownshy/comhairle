<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { resolve } from '$app/paths';
	import { buttonVariants, LoadingButton } from '$lib/components/ui/button';
	import SelectableOptionRow from '$lib/components/SelectableOptionRow.svelte';
	import LoadingOverlay from '$lib/components/LoadingOverlay.svelte';
	import { PALETTE_TOOLS, isEventPaletteItem, type CreationKey } from '$lib/tool_meta';
	import type { DemographicSelection } from '$lib/createWorkflowStep';
	import {
		listPublishedDemographicForms,
		listPublishedDemographicQuestions,
		type PickerLoad,
		type PublishedDemographicItem
	} from '$lib/demographicCatalogue';
	import { BookOpen, ExternalLink, Check } from 'lucide-svelte';
	import DemographicItemPicker from './DemographicItemPicker.svelte';

	/**
	 * The "Add a step" dialog: a two-column picker (step-type list on the left, rich
	 * detail panel on the right). Mirrors the choose-a-template dialog's shell and
	 * reuses {@link SelectableOptionRow}. On confirm it emits the selected tool's
	 * `creationKey` (via {@link onAdd}), with the chosen question or form for demographic
	 * steps, or calls {@link onAddEvent} for an event stand-in like the video conference.
	 * The parent creates the step or event and closes the dialog.
	 */
	type Props = {
		/** Two-way open state. */
		open?: boolean;
		/** Disables the confirm button while the parent is creating the step. */
		adding?: boolean;
		conversationId: string;
		/** The hosting organisation whose question and form banks demographic steps draw from. */
		organizationId: string | null;
		/** Called with the chosen tool's `creationKey` when the user confirms a tool step. */
		onAdd: (creationKey: CreationKey, selection?: DemographicSelection) => void;
		/** Called when the user confirms an event stand-in (no backing tool, e.g. video conference). */
		onAddEvent: () => void;
	};
	let {
		open = $bindable(false),
		adding = false,
		conversationId,
		organizationId,
		onAdd,
		onAddEvent
	}: Props = $props();

	let selectedType = $state<string>(PALETTE_TOOLS[0].type);
	const selected = $derived(
		PALETTE_TOOLS.find((t) => t.type === selectedType) ?? PALETTE_TOOLS[0]
	);

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

		<div class="flex min-h-0 flex-1 items-start gap-6 overflow-hidden">
			<!-- Left: selectable step-type list -->
			<div
				class="flex w-96 shrink-0 flex-col gap-3 self-stretch overflow-y-auto pr-4 [scrollbar-gutter:stable]"
			>
				{#each PALETTE_TOOLS as tool (tool.type)}
					<SelectableOptionRow
						selected={selectedType === tool.type}
						name={tool.displayName}
						description={tool.tagline}
						disabled={missingOrganization && needsOrganization(tool)}
						onSelect={() => (selectedType = tool.type)}
					/>
				{/each}
				{#if missingOrganization}
					<p class="text-muted-foreground text-base">
						Demographic steps are unavailable because this conversation has no hosting
						organisation. Questions and forms come from the hosting organisation.
						<a
							href={resolve(
								'/(admin)/admin/conversations/[conversation_id]/configure/team',
								{ conversation_id: conversationId }
							)}
							class="text-primary underline"
						>
							Configure one
						</a>
					</p>
				{/if}
			</div>

			<!-- Right: selected step-type detail -->
			<div
				class="bg-card border-border flex min-h-0 flex-1 flex-col gap-4 self-stretch overflow-y-auto rounded-xl border p-6"
			>
				<div class="flex flex-col gap-2">
					<h3 class="text-foreground text-base font-bold">{selected.displayName}</h3>
					<p class="text-muted-foreground text-base">{selected.description}</p>

					{#if selected.infoSlug}
						<!-- Setup guide callout -->
						<a
							href={resolve('/(admin)/admin/info/tools/[tool_id]', {
								tool_id: selected.infoSlug
							})}
							target="_blank"
							rel="noopener noreferrer"
							class="bg-accent border-border hover:bg-accent/70 flex items-center gap-2 rounded-lg border px-3 py-2.5 transition-colors"
						>
							<BookOpen class="text-primary size-4 shrink-0" />
							<span class="flex flex-1 flex-col">
								<span class="text-primary text-xs font-semibold">
									Introduction & setup guide
								</span>
								<span class="text-muted-foreground text-xs">
									Learn how to configure this step, see examples, and understand
									best practices.
								</span>
							</span>
							<ExternalLink class="text-primary size-3.5 shrink-0" />
						</a>
					{/if}
				</div>

				{#if demographicNoun}
					<DemographicItemPicker
						noun={demographicNoun}
						load={pickerLoad}
						selectedId={pickedForSelected?.id ?? null}
						onSelect={(item) => (picked = { type: selected.type, item })}
					/>
				{:else}
					<!-- Example media placeholders (tokenized blocks; no photos) -->
					<div class="flex gap-2.5">
						{#each [0, 1, 2] as block (block)}
							<div
								class="bg-muted border-border relative flex h-28 flex-1 items-center justify-center overflow-hidden rounded-lg border"
							>
								<span
									class="text-primary/70 border-primary/40 -rotate-[30deg] rounded border px-2 py-0.5 text-xs font-semibold tracking-wide uppercase"
								>
									Example
								</span>
							</div>
						{/each}
					</div>
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

		<Dialog.Footer class="items-center sm:justify-between">
			{#if selected.infoSlug}
				<a
					href={resolve('/(admin)/admin/info/tools/[tool_id]', {
						tool_id: selected.infoSlug
					})}
					target="_blank"
					rel="noopener noreferrer"
					class="text-primary inline-flex items-center gap-1.5 text-sm font-medium"
				>
					<BookOpen class="size-3.5" />
					Learn more in the guidebook
				</a>
			{:else}
				<span></span>
			{/if}
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
							? onAddEvent()
							: onAdd(selected.creationKey, pickedForSelected ?? undefined)}
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
