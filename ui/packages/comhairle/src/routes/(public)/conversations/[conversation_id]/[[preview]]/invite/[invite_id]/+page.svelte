<script lang="ts">
	import ConversationSummary from '$lib/components/ConversationSummary.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import {
		loginRedirect,
		signupRedirect,
		signupAnnonRedirect,
		conversation_url
	} from '$lib/urls.js';

	import { page } from '$app/state';
	import { apiClient } from '@crown-shy/api-client/client';
	import { goto } from '$app/navigation';

	const url = $derived(page.url);
	let { data } = $props();
	let { user, invite, conversation, error } = data;

	function login() {
		loginRedirect(url.toString(), 'Login to accept invite');
	}

	function create_account() {
		signupRedirect(url.toString(), 'Signup to accept invite');
	}

	function take_part_annon() {
		signupAnnonRedirect(url.toString(), 'Signup to accept invite');
	}

	async function acceptInvite() {
		await apiClient.AcceptInvite(undefined, {
			params: { conversation_id: conversation!.id, invite_id: invite!.id }
		});
		goto(conversation_url(conversation.id));
	}

	async function rejectInvite() {
		await apiClient.RejectInvite(undefined, {
			params: { conversation_id: conversation!.id, invite_id: invite!.id }
		});
		goto('/');
	}
</script>

{#if invite}
	{#if conversation}
		<div class="mt-10">
			<h1 class="mb-5 text-2xl font-bold">
				You have been invited to join the following conversation
			</h1>
			<ConversationSummary {conversation}>
				{#if !user && invite.login_behaviour === 'manual'}
					<p class="mb-5">To join this conversation please either</p>
					{#if !user && typeof invite.inviteType !== 'string' && 'email' in invite.inviteType && invite.inviteType.email}
						<div class="mb-5 flex flex-row gap-2">
							<Button onclick={login}>Login</Button>
							<Button onclick={create_account}>Create an account</Button>
						</div>
						<p>
							using the email account <span class="font-bold"
								>{invite.inviteType.email}</span
							>
						</p>
					{:else}
						<div class="flex flex-col gap-2">
							<Button onclick={login}>Login</Button>
							<Button onclick={create_account}>Create an account</Button>
							<Button onclick={take_part_annon}>Take part anonymously</Button>
						</div>
					{/if}
				{/if}

				{#if user}
					<Button onclick={acceptInvite}>Accept Invite</Button>
					<Button onclick={rejectInvite}>Reject Invite</Button>
				{/if}

				{#if !user && invite.loginBehaviour === 'auto_create_annon'}
					<Button>Join</Button>
				{/if}
			</ConversationSummary>
		</div>
	{/if}
{/if}

{#if error}
	<div class="flex h-full flex-col items-center align-middle">
		<div class="my-auto text-center">
			{#if error == 'Invite does not match logged in user'}
				<h1>You are not the intended user for this invite</h1>
				<p>
					If you think you should be, check if you are logged in with the correct account
				</p>
				<Button class="mt-10" href="/conversations">Find Other Conversations</Button>
			{/if}
			{#if error == 'This invite has expired'}
				<h1>This invite has expired</h1>
			{/if}
		</div>
	</div>
{/if}
