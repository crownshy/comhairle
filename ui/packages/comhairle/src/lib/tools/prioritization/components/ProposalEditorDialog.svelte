<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import { getLanguageName } from '$lib/config/languages';
	import { LoaderCircle, Plus, Trash2 } from 'lucide-svelte';
	import type { PrioritizationStore } from '../store.svelte';
	import type { Proposal, TextContentWithTranslations } from '../types';

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

	const isEditing = $derived(!!proposal);

	/** In edit mode the list refreshes after section add/delete, producing a new
	 * proposal object. Track the latest version from the store so the editor
	 * always renders the current set of sections. */
	const liveProposal = $derived(
		proposal ? (store.proposals.find((p) => p.id === proposal.id) ?? proposal) : null
	);

	/** CREATE-mode local state — primary locale only. Once created, the list refreshes and the new proposal can be reopened in EDIT mode to author non-primary languages via TranslatableField. Draft sections carry a stable client id so the rich-text editors keep their identity when a section is removed. */
	let draftTitle = $state('');
	let draftSeq = 0;
	let draftSections = $state<{ id: number; body: string }[]>([{ id: 0, body: '' }]);
	let creating = $state(false);
	let errorMessage = $state<string | null>(null);

	$effect(() => {
		if (open && !proposal) {
			draftTitle = '';
			draftSeq = 1;
			draftSections = [{ id: 0, body: '' }];
			errorMessage = null;
		}
	});

	function addDraftSection() {
		draftSections = [...draftSections, { id: draftSeq++, body: '' }];
	}

	function removeDraftSection(id: number) {
		draftSections = draftSections.filter((s) => s.id !== id);
	}

	async function createNew() {
		const sections = draftSections.map((s) => s.body.trim()).filter((s) => s.length > 0);
		if (!draftTitle.trim() || sections.length === 0) {
			errorMessage = 'A title and at least one section are required.';
			return;
		}
		creating = true;
		errorMessage = null;
		try {
			await store.create({ title: draftTitle.trim(), sections });
			onOpenChange(false);
		} catch (e) {
			errorMessage = e instanceof Error ? e.message : 'Failed to create proposal.';
		} finally {
			creating = false;
		}
	}

	/** EDIT-mode helpers. TranslatableField wants a `Translation`-shaped object plus a `value` and `onValueChange` for the primary-locale inline editor. We mirror the latest text back into local state so the inputs feel responsive while the field handles persistence. */
	let titleValue = $state('');
	let sectionValues = $state<Record<string, string>>({});
	let addingSection = $state(false);

	function primaryContent(env: TextContentWithTranslations | undefined): string {
		if (!env) return '';
		const primary = env.textTranslations.find(
			(t) => t.locale === env.textContent.primaryLocale
		);
		return primary?.content ?? '';
	}

	$effect(() => {
		if (open && liveProposal) {
			titleValue = primaryContent(liveProposal.titleTranslations);
			const next: Record<string, string> = {};
			for (const section of liveProposal.sections) {
				next[section.id] = primaryContent(section.bodyTranslations);
			}
			sectionValues = next;
			errorMessage = null;
		}
	});

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
</script>

<Dialog.Root {open} onOpenChange={(o) => onOpenChange(o)}>
	<Dialog.Content class="max-h-[90vh] min-w-[70vw] overflow-y-auto">
		<Dialog.Header>
			<Dialog.Title>
				{isEditing ? 'Edit proposal' : 'New proposal'}
			</Dialog.Title>
			<Dialog.Description>
				{isEditing
					? 'Edit the title and sections. Use the language badges to translate into other supported languages.'
					: `Write the proposal in ${getLanguageName(primaryLocale)}. You can add translations after creating it.`}
			</Dialog.Description>
		</Dialog.Header>

		{#if isEditing && liveProposal}
			<div class="space-y-5 py-2">
				<div class="space-y-2">
					<Label>Title</Label>
					<TranslatableField
						value={titleValue}
						onValueChange={(v) => (titleValue = v)}
						{primaryLocale}
						supportedLanguages={supportedLocales}
						editorType="plain"
						placeholder="Proposal title"
						dialogTitle="Translate title"
						translation={liveProposal.titleTranslations}
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
						<TranslatableField
							value={sectionValues[section.id] ?? ''}
							onValueChange={(v) => (sectionValues[section.id] = v)}
							{primaryLocale}
							supportedLanguages={supportedLocales}
							editorType="rich"
							placeholder="Describe this section"
							minHeight="160px"
							dialogTitle="Translate section"
							translation={section.bodyTranslations}
						/>
					</div>
				{/each}

				<Button variant="outline" size="sm" onclick={addSection} disabled={addingSection}>
					{#if addingSection}<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />{:else}<Plus
							class="mr-2 h-4 w-4"
						/>{/if}
					Add section
				</Button>
			</div>
		{:else}
			<div class="space-y-5 py-2">
				<div class="space-y-2">
					<Label for="create-title">Title</Label>
					<Input id="create-title" bind:value={draftTitle} placeholder="Proposal title" />
				</div>
				{#each draftSections as section, i (section.id)}
					<div class="space-y-2">
						<div class="flex items-center justify-between">
							<Label>Section {i + 1}</Label>
							<Button
								variant="ghost"
								size="sm"
								class="text-destructive hover:text-destructive"
								disabled={draftSections.length <= 1}
								onclick={() => removeDraftSection(section.id)}
							>
								<Trash2 class="mr-2 h-3.5 w-3.5" /> Remove
							</Button>
						</div>
						<RichTextEditor
							value={section.body}
							placeholder="Describe this section"
							minHeight="160px"
							onChange={(v) => (section.body = v)}
						/>
					</div>
				{/each}
				<Button variant="outline" size="sm" onclick={addDraftSection}>
					<Plus class="mr-2 h-4 w-4" /> Add section
				</Button>
			</div>
		{/if}

		{#if errorMessage}
			<p class="text-destructive text-sm">{errorMessage}</p>
		{/if}

		<Dialog.Footer>
			{#if isEditing}
				<Button onclick={() => onOpenChange(false)}>Done</Button>
			{:else}
				<Button variant="outline" onclick={() => onOpenChange(false)} disabled={creating}>
					Cancel
				</Button>
				<Button onclick={createNew} disabled={creating}>
					{#if creating}<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />{/if}
					Create proposal
				</Button>
			{/if}
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
