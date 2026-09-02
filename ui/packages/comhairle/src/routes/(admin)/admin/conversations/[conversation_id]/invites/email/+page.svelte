<script lang="ts">
	import { invalidate } from '$app/navigation';
	import EmailInviteForm from '$lib/components/ui/email-invites/EmailInviteForm.svelte';
	import EmailInvitesList from '$lib/components/ui/email-invites/EmailInvitesList.svelte';
	import { key } from '$lib/utils/invalidationKey';
	import { InviteLink } from './external';

	let { data } = $props();

	let emailInvites = $derived(
		data.invites.filter(
			(invite) =>
				typeof invite.inviteType !== 'string' &&
				'email' in invite.inviteType &&
				invite.inviteType.email
		)
	);
</script>

<EmailInviteForm
	conversationId={data.conversation.id}
	onDone={() => void invalidate(key('conversation/invites'))}
/>
<EmailInvitesList {emailInvites}>
	{#snippet inviteLink(invite)}
		<InviteLink inviteId={invite.id} conversationId={data.conversation.id} label="Link" />
	{/snippet}
</EmailInvitesList>
