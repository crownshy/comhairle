<script>
	import EmailInvitesList from '$lib/components/ui/email-invites/EmailInvitesList.svelte';
	import EmailInviteForm from '$lib/components/ui/email-invites/EmailInviteForm.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { InviteLink } from './external';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';

	const { data, params } = $props();
	const { conversation_id, event_id } = $derived(params);
	const { streamedEmailInvites } = $derived(data);
</script>

<div class="border-border flex flex-col gap-4 border-t py-6 lg:gap-6">
	<Label class="text-sm font-semibold lg:shrink-0 lg:pt-2">Email invites</Label>

	<EmailInviteForm
		conversationId={conversation_id}
		eventId={event_id}
		onDone={() => void invalidate(key('conversation/event'))}
	/>

	{#await streamedEmailInvites}
		<Skeleton class="h-37 w-full" />
	{:then emailInvites}
		{#if emailInvites.err !== null}
			{notifications.addFlash({
				message: `Could not load email invites: ${emailInvites.err}`,
				priority: 'ERROR'
			})}
			<span>No invites to show</span>
		{:else}
			<EmailInvitesList emailInvites={emailInvites.ok}>
				{#snippet inviteLink(invite)}
					<InviteLink
						label="Link"
						inviteId={invite.id}
						conversationId={conversation_id}
					/>
				{/snippet}
			</EmailInvitesList>
		{/if}
	{/await}
</div>
