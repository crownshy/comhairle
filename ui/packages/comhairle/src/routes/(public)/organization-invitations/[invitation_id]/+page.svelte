<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { apiClient } from '@crownshy/api-client/client';
	import type { ApiError, InvitationDetails } from '@crownshy/api-client/api';
	import { Button } from '$lib/components/ui/button';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { snakeToStartCase } from '$lib/utils/casingUtils';

	let { data } = $props();

	let accepting = $state(false);
	let error = $state('');

	const signInUrl = $derived(
		`/auth/login?backTo=${encodeURIComponent(`/organization-invitations/${data.invitationId}`)}`
	);

	async function accept(invitation: InvitationDetails) {
		accepting = true;
		error = '';
		const result = await tryCatchAsync<InvitationDetails, ApiError>(() =>
			apiClient.AcceptOrganizationConversationInvitation(undefined, {
				params: { invitation_id: invitation.id }
			})
		);
		accepting = false;
		if (result.err !== null) {
			error = 'Could not accept this invitation. It may have expired or already been used.';
			return;
		}
		notifications.send({
			message: `${invitation.organization_name} has been added to ${invitation.conversation_title}`,
			priority: 'INFO'
		});
		await goto(resolve('/'));
	}
</script>

<svelte:head>
	<title>Conversation invitation - Comhairle</title>
</svelte:head>

<div class="mx-auto flex max-w-xl flex-col gap-4 py-20">
	{#if !data.user}
		<h1 class="text-2xl font-semibold">Sign in to view this invitation</h1>
		<p class="text-base">
			Sign in with an account that administers the organization to accept this invitation.
		</p>
		<Button href={signInUrl} class="self-start">Sign in</Button>
	{:else if data.notFound || !data.invitation}
		<h1 class="text-2xl font-semibold">Invitation not found</h1>
		<p class="text-base">This invitation has expired, was already used, or does not exist.</p>
	{:else}
		{@const invitation = data.invitation}
		<h1 class="text-2xl font-semibold">
			Add {invitation.organization_name} to {invitation.conversation_title}
		</h1>
		<p class="text-base">
			Accepting gives {invitation.organization_name} these roles on the conversation:
		</p>
		<ul class="list-disc pl-6 text-base">
			{#each invitation.roles as role (role)}
				<li>{snakeToStartCase(role)}</li>
			{/each}
		</ul>
		{#if !invitation.can_accept}
			<p class="text-muted-foreground text-base" role="status">
				Only an administrator of {invitation.organization_name} can accept this invitation, or
				the person who sent it can no longer grant these roles.
			</p>
		{/if}
		{#if error}
			<p class="text-destructive text-base" role="alert">{error}</p>
		{/if}
		<Button
			class="self-start"
			disabled={!invitation.can_accept || accepting}
			onclick={() => accept(invitation)}
		>
			Accept invitation
		</Button>
	{/if}
</div>
