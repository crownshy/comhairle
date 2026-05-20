<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import { LoaderCircle } from 'lucide-svelte';
	import { getAdapter, getStepContext } from '../context';
	import type { Proposal, TextContentWithTranslations } from '../types';

	type Props = {
		open: boolean;
		proposal?: Proposal | null;
		onOpenChange: (open: boolean) => void;
		onCreated?: (created: Proposal) => void;
	};

	let { open, proposal = null, onOpenChange, onCreated }: Props = $props();

	const adapter = getAdapter();
	const ctx = getStepContext();
	const isEditing = $derived(!!proposal);

	/** CREATE-mode local state — primary locale only. Once created, the caller refreshes the list and the new proposal can be reopened in EDIT mode to author non-primary languages via TranslatableField. */
	let draftTitle = $state('');
	let draftBody = $state('');
	let creating = $state(false);
	let errorMessage = $state<string | null>(null);

	$effect(() => {
		if (open && !proposal) {
			draftTitle = '';
			draftBody = '';
			errorMessage = null;
		}
	});

	async function createNew() {
		if (!draftTitle.trim() || !draftBody.trim()) {
			errorMessage = 'Title and body are required.';
			return;
		}
		creating = true;
		errorMessage = null;
		try {
			const created = await adapter.createProposal({
				title: draftTitle.trim(),
				body: draftBody.trim()
			});
			onCreated?.(created);
			onOpenChange(false);
		} catch (e) {
			errorMessage = e instanceof Error ? e.message : 'Failed to create proposal.';
		} finally {
			creating = false;
		}
	}

	/** EDIT-mode helpers. TranslatableField wants a `Translation`-shaped object plus a `value` and `onValueChange` for the primary-locale inline editor. We mirror the latest text back into local state so the inputs feel responsive while the field handles persistence. */
	let titleValue = $state('');
	let bodyValue = $state('');

	function primaryContent(env: TextContentWithTranslations | undefined): string {
		if (!env) return '';
		const primary = env.textTranslations.find(
			(t) => t.locale === env.textContent.primaryLocale
		);
		return primary?.content ?? '';
	}

	$effect(() => {
		if (open && proposal) {
			titleValue = primaryContent(proposal.titleTranslations);
			bodyValue = primaryContent(proposal.bodyTranslations);
			errorMessage = null;
		}
	});
</script>

<Dialog.Root {open} onOpenChange={(o) => onOpenChange(o)}>
	<Dialog.Content class="max-h-[90vh] min-w-[70vw] overflow-y-auto">
		<Dialog.Header>
			<Dialog.Title>
				{isEditing ? 'Edit proposal' : 'New proposal'}
			</Dialog.Title>
			<Dialog.Description>
				{isEditing
					? 'Edit the title and body. Use the language badges to translate into other supported languages.'
					: `Write the proposal in ${ctx.formatLocale(ctx.primaryLocale)}. You can add translations after creating it.`}
			</Dialog.Description>
		</Dialog.Header>

		{#if isEditing && proposal}
			<div class="space-y-5 py-2">
				<div class="space-y-2">
					<Label>Title</Label>
					<TranslatableField
						value={titleValue}
						onValueChange={(v) => (titleValue = v)}
						primaryLocale={ctx.primaryLocale}
						supportedLanguages={ctx.supportedLocales}
						editorType="plain"
						placeholder="Proposal title"
						dialogTitle="Translate title"
						translation={proposal.titleTranslations}
					/>
				</div>

				<div class="space-y-2">
					<Label>Body</Label>
					<TranslatableField
						value={bodyValue}
						onValueChange={(v) => (bodyValue = v)}
						primaryLocale={ctx.primaryLocale}
						supportedLanguages={ctx.supportedLocales}
						editorType="rich"
						placeholder="Describe the proposal"
						minHeight="160px"
						dialogTitle="Translate body"
						translation={proposal.bodyTranslations}
					/>
				</div>
			</div>
		{:else}
			<div class="space-y-5 py-2">
				<div class="space-y-2">
					<Label for="create-title">Title</Label>
					<Input id="create-title" bind:value={draftTitle} placeholder="Proposal title" />
				</div>
				<div class="space-y-2">
					<Label>Body</Label>
					<RichTextEditor
						value={draftBody}
						placeholder="Describe the proposal"
						minHeight="160px"
						onChange={(v) => (draftBody = v)}
					/>
				</div>
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
