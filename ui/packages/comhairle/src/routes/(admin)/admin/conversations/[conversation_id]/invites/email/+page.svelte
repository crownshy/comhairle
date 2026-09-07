<script lang="ts">
	import { invalidate } from '$app/navigation';
	import EmailInviteForm from '$lib/components/ui/email-invites/EmailInviteForm.svelte';
	import EmailInvitesList from '$lib/components/ui/email-invites/EmailInvitesList.svelte';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { key } from '$lib/utils/invalidationKey';
	import { InviteLink } from './external';

	let { data } = $props();
</script>

<EmailInviteForm
	conversationId={data.conversation.id}
	onDone={() => void invalidate(key('conversation/invites'))}
/>
{#await data.invites}
	<Skeleton />
{:then invites}
	{#if invites.err !== null}
		<span class="text-muted-foreground font-bold">Could not load invites please try again</span>
	{:else}
		<EmailInvitesList emailInvites={invites.ok.emailInvites}>
			{#snippet inviteLink(invite)}
				<InviteLink
					inviteId={invite.id}
					conversationId={data.conversation.id}
					label="Link"
				/>
			{/snippet}
		</EmailInvitesList>
	{/if}
{/await}
