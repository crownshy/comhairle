<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Label } from '$lib/components/ui/label';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import { createTextContentSource } from '$lib/components/Translation/translationSource.svelte';
	import {
		hasUnsavedChanges,
		type TranslationSource
	} from '$lib/components/Translation/translationUtils';
	import { guardUnsavedChanges } from '$lib/utils/unsavedChangesGuard.svelte';
	import ProposalSectionField from './ProposalSectionField.svelte';
	import { isTiptapJson, extractTextFromTiptap } from '$lib/utils/tiptapUtils';
	import { LoaderCircle, Plus, Trash2 } from 'lucide-svelte';
	import type { PrioritizationStore } from '../store.svelte';
	import type { Proposal } from '../types';

	type Props = {
		open: boolean;
		proposal?: Proposal | null;
		store: PrioritizationStore;
		primaryLocale: string;
		supportedLocales: string[];
		onOpenChange: (open: boolean) => void;
	};

	let {
		open,
		proposal = null,
		store,
		primaryLocale,
		supportedLocales,
		onOpenChange
	}: Props = $props();

	let errorMessage = $state<string | null>(null);

	/** Persist-first create: opening "New proposal" immediately creates an empty draft server-side so
	 * the same edit UI (with translation badges) can back it. An untouched draft is cleaned up on close;
	 * an explicit Cancel discards it. `draftId` is only set when we created the draft ourselves. */
	let draftId = $state<string | null>(null);
	let preparing = $state(false);
	let closing = $state(false);

	/** In edit mode the list refreshes after section add/delete, producing a new proposal object; in
	 * create mode we track the draft we just made. Either way, read the latest version from the store
	 * so the editor always renders the current set of sections. */
	const liveProposal = $derived.by((): Proposal | null => {
		if (proposal) return store.proposals.find((p) => p.id === proposal.id) ?? proposal;
		if (draftId) return store.proposals.find((p) => p.id === draftId) ?? null;
		return null;
	});

	const isEditing = $derived(!!proposal);

	// Kick off (or reset) the draft as the dialog opens/closes in create mode.
	$effect(() => {
		if (open && !proposal && draftId === null && !preparing) {
			void startDraft();
		}
		if (!open) {
			draftId = null;
			errorMessage = null;
		}
	});

	async function startDraft() {
		preparing = true;
		errorMessage = null;
		try {
			const created = await store.create({ title: '', sections: [''] });
			// If the dialog was closed while we were creating, don't leave an orphan draft.
			if (!open) {
				await store.remove(created.id).catch(() => {});
				return;
			}
			draftId = created.id;
		} catch (e) {
			errorMessage = e instanceof Error ? e.message : 'Failed to start a new proposal.';
		} finally {
			preparing = false;
		}
	}

	// Title owns its own source (ADR-0005); sections own theirs via ProposalSectionField. Every source
	// is registered here so the dialog can flush pending debounced saves before it closes. `refresh`
	// points at the store's silent reload: the prioritization list is self-managed, so `invalidateAll`
	// alone would leave saves reconciling against stale data (see store.reload / translationSource).
	const sources = new Map<string, TranslationSource>();

	function registerSource(id: string, source: TranslationSource) {
		sources.set(id, source);
	}
	function unregisterSource(id: string) {
		sources.delete(id);
	}

	const titleSource = createTextContentSource({
		getTranslation: () => liveProposal?.titleTranslations,
		getPrimaryLocale: () => primaryLocale,
		getSupportedLanguages: () => supportedLocales,
		refresh: () => store.reload()
	});
	registerSource('__title__', titleSource);

	// Warn on refresh / tab-close / in-app navigation while any title or section save is still pending,
	// so a mid-debounce edit isn't silently lost. The dialog is a persistent instance, so this registers
	// once; the getter reads the live source set at event time.
	guardUnsavedChanges(() => [...sources.values()].some(hasUnsavedChanges));

	let addingSection = $state(false);

	async function addSection() {
		if (!liveProposal) return;
		addingSection = true;
		errorMessage = null;
		try {
			await store.addSection(liveProposal.id, '');
		} catch (e) {
			errorMessage = e instanceof Error ? e.message : 'Failed to add section.';
		} finally {
			addingSection = false;
		}
	}

	async function removeSection(sectionId: string) {
		if (!liveProposal) return;
		errorMessage = null;
		try {
			await store.removeSection(liveProposal.id, sectionId);
		} catch (e) {
			errorMessage = e instanceof Error ? e.message : 'Failed to delete section.';
		}
	}

	function isBlank(content: string | undefined): boolean {
		if (!content) return true;
		const text = isTiptapJson(content) ? extractTextFromTiptap(content) : content;
		return text.trim().length === 0;
	}

	function isProposalEmpty(p: Proposal): boolean {
		return isBlank(p.title) && p.sections.every((s) => isBlank(s.body));
	}

	async function flushAll() {
		await Promise.allSettled([...sources.values()].map((s) => s.flush()));
	}

	/** Commit pending edits, then close. A self-created draft is deleted when discarded outright or
	 * left completely empty; a real (edited) proposal is always kept. */
	async function finalizeAndClose(opts: { discard?: boolean } = {}) {
		if (closing) return;
		closing = true;
		try {
			await flushAll();
			if (draftId && !proposal) {
				const current = store.proposals.find((p) => p.id === draftId);
				const shouldDelete = opts.discard || (current ? isProposalEmpty(current) : true);
				if (shouldDelete) {
					await store.remove(draftId).catch(() => {});
				}
			}
			draftId = null;
			onOpenChange(false);
		} finally {
			closing = false;
		}
	}
</script>

<Dialog.Root
	{open}
	onOpenChange={(o) => {
		if (!o) void finalizeAndClose();
		else onOpenChange(true);
	}}
>
	<Dialog.Content class="h-[90vh] min-w-[80vw] overflow-y-auto">
		<Dialog.Header>
			<Dialog.Title>
				{isEditing ? 'Edit proposal' : 'New proposal'}
			</Dialog.Title>
			<Dialog.Description>
				Edit the title and sections. Use the language badges to translate into other
				supported languages.
			</Dialog.Description>
		</Dialog.Header>

		{#if preparing && !liveProposal}
			<div class="text-muted-foreground flex items-center gap-2 py-10">
				<LoaderCircle class="h-4 w-4 animate-spin" />
				Preparing the proposal...
			</div>
		{:else if liveProposal}
			<div class="space-y-5 py-2">
				<div class="space-y-2">
					<Label>Title</Label>
					<TranslatableField
						source={titleSource}
						{primaryLocale}
						supportedLanguages={supportedLocales}
						editorType="plain"
						placeholder="Proposal title"
						dialogTitle="Translate title"
					/>
				</div>

				{#each liveProposal.sections as section, i (section.id)}
					<div class="space-y-2">
						<div class="flex items-center justify-between">
							<Label>Section {i + 1}</Label>
							<Button
								variant="ghost"
								size="sm"
								class="text-destructive hover:text-destructive"
								disabled={liveProposal.sections.length <= 1}
								onclick={() => removeSection(section.id)}
							>
								<Trash2 class="mr-2 h-3.5 w-3.5" /> Remove
							</Button>
						</div>
						<ProposalSectionField
							{section}
							{primaryLocale}
							{supportedLocales}
							refresh={() => store.reload()}
							{registerSource}
							{unregisterSource}
						/>
					</div>
				{/each}

				<Button variant="outline" size="sm" onclick={addSection} disabled={addingSection}>
					{#if addingSection}<LoaderCircle
							class="mr-2 h-4 w-4 animate-spin"
						/>{:else}<Plus class="mr-2 h-4 w-4" />{/if}
					Add section
				</Button>
			</div>
		{/if}

		{#if errorMessage}
			<p class="text-destructive text-sm">{errorMessage}</p>
		{/if}

		<Dialog.Footer>
			{#if isEditing}
				<Button onclick={() => finalizeAndClose()} disabled={closing}>
					{#if closing}<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />{/if}
					Done
				</Button>
			{:else}
				<Button
					variant="outline"
					onclick={() => finalizeAndClose({ discard: true })}
					disabled={closing || preparing}
				>
					Cancel
				</Button>
				<Button onclick={() => finalizeAndClose()} disabled={closing || preparing}>
					{#if closing}<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />{/if}
					Done
				</Button>
			{/if}
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
