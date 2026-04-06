<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import type { InviteDto } from '@crownshy/api-client/api';

	let {
		open = $bindable(false),
		invite,
		conversationId,
		onSave
	}: {
		open: boolean;
		invite: InviteDto | null;
		conversationId: string;
		onSave: () => void;
	} = $props();

	let label = $state(invite?.label || '');
	let isNew = $derived(!invite);

	$effect(() => {
		if (invite) {
			label = invite.label || '';
		}
	});

	async function handleSave() {
		try {
			if (isNew) {
				// Create new invite with label
				await apiClient.CreateInvite(
					{ invite_type: 'open', label: label || undefined },
					{ params: { conversation_id: conversationId } }
				);
			} else if (invite) {
				// Update existing invite
				await apiClient.UpdateInvite(
					{ label: label || undefined },
					{ params: { conversation_id: conversationId, invite_id: invite.id } }
				);
			}
			open = false;
			onSave();
		} catch (error) {
			console.error('Failed to save invite:', error);
		}
	}

	function handleOpenChange(newOpen: boolean) {
		open = newOpen;
		if (!newOpen) {
			label = '';
		}
	}
</script>

<Dialog.Root {open} onOpenChange={handleOpenChange}>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>{isNew ? 'Create New Invite' : 'Update Invite Label'}</Dialog.Title>
			<Dialog.Description>
				{isNew
					? 'Add a label to help you identify this invite link.'
					: 'Update the label for this invite link.'}
			</Dialog.Description>
		</Dialog.Header>

		<div class="space-y-4 py-4">
			<div class="space-y-2">
				<Label for="label">Label (optional)</Label>
				<Input
					id="label"
					bind:value={label}
					placeholder="e.g., Social Media Campaign, Newsletter, etc."
				/>
			</div>
		</div>

		<Dialog.Footer>
			<Button variant="outline" onclick={() => (open = false)}>Cancel</Button>
			<Button onclick={handleSave}>{isNew ? 'Create' : 'Save'}</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
