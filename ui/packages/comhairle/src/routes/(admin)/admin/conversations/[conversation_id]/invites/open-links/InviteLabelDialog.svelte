<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import type { InviteDto } from '@crownshy/api-client/api';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import type { Snippet } from 'svelte';
	import { handleSubmit } from '$lib/utils/form';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';

	type Props = {
		conversationId: string;
		children: Snippet;
	} & (
		| {
				type: 'new';
				inviteId?: undefined;
		  }
		| {
				type: 'update';
				inviteId: InviteDto['id'];
		  }
	);

	let { conversationId, children, ...props }: Props = $props();

	let open = $state<boolean>(false);

	async function handleSave(label?: string | undefined) {
		let request;

		switch (props.type) {
			case 'new': {
				request = tryCatchAsync(() =>
					apiClient.CreateInvite(
						{ invite_type: 'open', label },
						{ params: { conversation_id: conversationId } }
					)
				);
				break;
			}
			case 'update': {
				request = tryCatchAsync(() =>
					apiClient.UpdateInvite(
						{ label },
						{ params: { conversation_id: conversationId, invite_id: props.inviteId } }
					)
				);
				break;
			}
		}

		const result = await request;

		if (result.err !== null) {
			notifications.addFlash({
				message: `Could not ${props.type === 'new' ? 'create' : 'update'} invite, please try again`,
				priority: 'ERROR'
			});
			return;
		}

		open = false;
		await invalidate(key('conversation/invites'));
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Trigger>
		{@render children()}
	</Dialog.Trigger>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title
				>{props.type === 'new' ? 'Create New Invite' : 'Update Invite Label'}</Dialog.Title
			>
			<Dialog.Description>
				{props.type === 'new'
					? 'Add a label to help you identify this invite link.'
					: 'Update the label for this invite link.'}
			</Dialog.Description>
		</Dialog.Header>

		<form
			method="DIALOG"
			onsubmit={handleSubmit((formData) => {
				const label = formData.get('label')?.toString() ?? undefined;
				handleSave(label);
			})}
		>
			<div class="space-y-4 py-4">
				<div class="space-y-2">
					<Label for="label">Label (optional)</Label>
					<Input
						id="label"
						name="label"
						placeholder="e.g., Social Media Campaign, Newsletter, etc."
					/>
				</div>
			</div>

			<Dialog.Footer>
				<Button type="button" variant="outline" onclick={() => (open = false)}
					>Cancel</Button
				>
				<Button type="submit">{props.type === 'new' ? 'Create' : 'Save'}</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
