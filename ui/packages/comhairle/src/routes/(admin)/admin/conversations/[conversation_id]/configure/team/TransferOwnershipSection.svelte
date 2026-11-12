<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { notifications } from '$lib/notifications.svelte';
	import { apiErrorMessage } from '$lib/utils/apiErrorMessage';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { key } from '$lib/utils/invalidationKey';

	type Props = {
		conversationId: string;
	};
	let { conversationId }: Props = $props();

	let email = $state('');
	let confirming = $state(false);
	let transferring = $state(false);
	let error = $state('');

	const trimmedEmail = $derived(email.trim());

	async function transferOwnership() {
		transferring = true;
		error = '';
		const result = await tryCatchAsync(() =>
			apiClient.TransferConversationOwnership(
				{ new_owner_email: trimmedEmail },
				{ params: { conversation_id: conversationId } }
			)
		);
		transferring = false;
		confirming = false;
		if (result.err !== null) {
			error = apiErrorMessage(result.err, 'Could not transfer ownership.');
			return;
		}
		notifications.send({
			message: `Ownership transferred to ${trimmedEmail}`,
			priority: 'INFO'
		});
		email = '';
		await Promise.all([
			invalidate(key('admin/conversation')),
			invalidate(key('admin/conversation/meta')),
			invalidate(key('admin/conversation/permissions'))
		]);
	}
</script>

<section class="border-border flex flex-col gap-3 border-t py-6">
	<h3 class="text-base font-semibold">Transfer ownership</h3>
	<p class="text-muted-foreground text-base">
		Give this conversation to another admin user. You keep admin access, and the new owner can
		remove it. Organisations cannot own conversations.
	</p>
	<form
		class="flex flex-col gap-2"
		onsubmit={(event) => {
			event.preventDefault();
			confirming = true;
		}}
	>
		<Label for="new-owner-email" class="text-base">New owner email</Label>
		<div class="flex flex-wrap items-center gap-3">
			<Input
				id="new-owner-email"
				type="email"
				class="w-72"
				placeholder="name@example.com"
				bind:value={email}
			/>
			<Button type="submit" variant="outline" disabled={trimmedEmail === '' || transferring}>
				Transfer ownership
			</Button>
		</div>
	</form>
	{#if error}
		<p class="text-destructive text-base" role="alert">{error}</p>
	{/if}
</section>

<AlertDialog.Root bind:open={confirming}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Transfer ownership to {trimmedEmail}?</AlertDialog.Title>
			<AlertDialog.Description>
				They become the owner and only they can transfer it again. You keep admin access to
				this conversation.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel disabled={transferring}>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action disabled={transferring} onclick={transferOwnership}>
				Transfer ownership
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
