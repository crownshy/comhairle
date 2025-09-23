<script lang="ts">
	import ConversationSummary from '$lib/components/ConversationSummary.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { loginRedirect, signupRedirect, signupAnnonRedirect } from '$lib/urls.js';
	import { page } from '$app/stores';
	import { apiClient } from '$lib/api/client.js';

	let url = $page.url;
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
	}

	async function rejectInvite() {
		await apiClient.RejectInvite(undefined, {
			params: { conversation_id: conversation!.id, invite_id: invite!.id }
		});
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
					<p>To join this conversation please either</p>
					<Button onclick={login}>Login</Button>
					<Button onclick={create_account}>Create an account</Button>
					<Button onclick={take_part_annon}>Take part anonymously</Button>
				{/if}

				{#if user}
					<Button onclick={acceptInvite}>Accept</Button>
					<Button onclick={rejectInvite}>Reject</Button>
				{/if}

				{#if !user && invite.login_behaviour === 'auto_create_annon'}
					<Button>Join</Button>
				{/if}
			</ConversationSummary>
		</div>
	{/if}
{/if}

{#if error}
	<div class="flex h-full flex-col justify-center align-middle">
		<div class="text-center">
			{#if error == 'Invite does not match logged in user'}
				<h1>You are not the intended user for this invite</h1>
				<p>If you think you should be, check if you are logged in with the correct account</p>
				<Button class="mt-10" href="/conversations">Find Other Conversations</Button>
			{/if}
			{#if error == 'This invite has expired'}
				<h1>This invite has expired</h1>
			{/if}
		</div>
	</div>
{/if}
