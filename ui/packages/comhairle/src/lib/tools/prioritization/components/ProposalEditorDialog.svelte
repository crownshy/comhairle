<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Label } from '$lib/components/ui/label';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import { createTextContentSource } from '$lib/components/Translation/translationSource.svelte';
	import {
		isAutosaveSource,
		type TranslationSource
	} from '$lib/components/Translation/translationUtils';
	import ProposalSectionField from './ProposalSectionField.svelte';
	import { isTiptapJson, extractTextFromTiptap } from '$lib/utils/tiptapUtils';
	import { LoaderCircle, Plus, Trash2 } from 'lucide-svelte';
	import type { PrioritizationStore } from '../store.svelte';
	import type { Proposal } from '../types';
	import type { Locale } from '$lib/paraglide/runtime';

	type Props = {
		open: boolean;
		proposal?: Proposal | null;
		store: PrioritizationStore;
		primaryLocale: Locale;
		supportedLocales: Locale[];
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

	// "New proposal" creates an empty draft up front so it can use the same edit UI. `draftId` is set
	// only for a draft we created, which is deleted if it's cancelled or left empty.
	let draftId = $state<string | null>(null);
	let preparing = $state(false);
	let closing = $state(false);

	// Section add/delete replaces the proposal object in the store, so always read the latest copy.
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

	// Every source (title here, sections via ProposalSectionField) registers so closing can flush
	// pending saves. The list lives in a store, not route data, so `refresh` reloads the store.
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

	// Saves don't reload the list behind the dialog, so closing syncs it once.
	async function flushAll() {
		await Promise.allSettled(
			Array.from(sources.values(), (s) => (isAutosaveSource(s) ? s.sync() : s.flush()))
		);
	}

	/** Flushes pending edits, then closes. Deletes a draft we created if it was discarded or left empty. */
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
