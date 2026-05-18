<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import { ImagePlus, Trash2 } from 'lucide-svelte';
	import type { PrioritizationStore } from './store.svelte';

	let {
		store,
		proposalId,
		onClose
	}: {
		store: PrioritizationStore;
		proposalId: string;
		onClose: () => void;
	} = $props();

	let proposal = $derived(store.poll.proposals.find((p) => p.id === proposalId));
	let open = $state(true);
	let fileInput = $state<HTMLInputElement | null>(null);

	function handleClose(o: boolean) {
		if (!o) {
			open = false;
			onClose();
		}
	}

	function onPickImage(e: Event) {
		const input = e.target as HTMLInputElement;
		const file = input.files?.[0];
		if (!file) return;
		const reader = new FileReader();
		reader.onload = () => {
			store.updateProposal(proposalId, { imageDataUrl: String(reader.result) });
		};
		reader.readAsDataURL(file);
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
					<Input
						id="p-title"
						placeholder="Proposal title"
						value={proposal.title}
						oninput={(e) =>
							store.updateProposal(proposal.id, {
								title: (e.target as HTMLInputElement).value
							})}
					/>
				</div>

				<div class="flex flex-col gap-1">
					<Label>Image (optional)</Label>
					{#if proposal.imageDataUrl}
						<div class="relative w-fit">
							<img
								src={proposal.imageDataUrl}
								alt={proposal.title}
								class="max-h-48 rounded-md object-cover"
							/>
							<Button
								variant="destructive"
								size="icon"
								class="absolute top-1 right-1"
								onclick={() =>
									store.updateProposal(proposal.id, { imageDataUrl: undefined })}
								aria-label="Remove image"
							>
								<Trash2 class="size-4" />
							</Button>
						</div>
					{:else}
						<Button
							variant="outline"
							size="sm"
							onclick={() => fileInput?.click()}
							class="w-fit"
						>
							<ImagePlus class="mr-1 size-4" /> Add image
						</Button>
					{/if}
					<input
						type="file"
						accept="image/*"
						bind:this={fileInput}
						onchange={onPickImage}
						class="hidden"
					/>
				</div>

				<div class="flex flex-col gap-1">
					<Label>Body</Label>
					<RichTextEditor
						value={proposal.body || null}
						placeholder="Describe this proposal…"
						minHeight="160px"
						onChange={(json) => store.updateProposal(proposal.id, { body: json })}
					/>
				</div>
			</div>
		{/if}

		<Dialog.Footer>
			<Button onclick={() => handleClose(false)}>Done</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
