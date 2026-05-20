<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import type { PrioritizationStore } from './store.svelte';

	let {
		store,
		proposalId,
		primaryLocale,
		supportedLanguages,
		onClose
	}: {
		store: PrioritizationStore;
		proposalId: string;
		primaryLocale: string;
		supportedLanguages: string[];
		onClose: () => void;
	} = $props();

	let proposal = $derived(store.poll.proposals.find((p) => p.id === proposalId));
	let open = $state(true);

	function handleClose(o: boolean) {
		if (!o) {
			open = false;
			onClose();
		}
	}
</script>

<Dialog.Root bind:open onOpenChange={handleClose}>
	<Dialog.Content class="max-h-[90vh] max-w-2xl min-w-[80vw] overflow-y-auto">
		<Dialog.Header>
			<Dialog.Title>
				Edit proposal {proposal?.order}
			</Dialog.Title>
			<Dialog.Description>
				All proposals share the same questions, defined in the Questions tab.
			</Dialog.Description>
		</Dialog.Header>

		{#if proposal}
			<div class="flex flex-col gap-4">
				<div class="flex flex-col gap-1">
					<Label for="p-title">Title</Label>
					<TranslatableField
						value={proposal.title}
						onValueChange={(v) => store.updateProposal(proposal.id, { title: v })}
						translation={proposal.titleTranslation}
						{primaryLocale}
						{supportedLanguages}
						placeholder="Proposal title"
						dialogTitle="Proposal title translations"
					/>
				</div>

				<div class="flex flex-col gap-1">
					<Label>Body</Label>
					<TranslatableField
						value={proposal.body || null}
						onValueChange={(v) => store.updateProposal(proposal.id, { body: v })}
						translation={proposal.bodyTranslation}
						{primaryLocale}
						{supportedLanguages}
						editorType="rich"
						placeholder="Describe this proposal…"
						minHeight="160px"
						dialogTitle="Proposal body translations"
					/>
				</div>
			</div>
		{/if}

		<Dialog.Footer>
			<Button onclick={() => handleClose(false)}>Done</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
