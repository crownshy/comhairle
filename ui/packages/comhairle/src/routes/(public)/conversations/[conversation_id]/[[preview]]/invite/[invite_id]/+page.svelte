<script lang="ts">
	import ConversationSummary from '$lib/components/ConversationSummary.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import PrivacyPolicyDialog from '$lib/components/PrivacyPolicyDialog.svelte';
	import * as m from '$lib/paraglide/messages';

	import { loginRedirect, signupRedirect, signupAnnonRedirect } from '$lib/urls.js';

	import { page } from '$app/state';
	import { apiClient } from '@crownshy/api-client/client';
	import { goto, invalidateAll } from '$app/navigation';
	import { env } from '$env/dynamic/public';
	import { onMount } from 'svelte';

	let loginType = $state<'automatic' | 'login'>('login');

	let privacyPolicyOpen = $state(false);

	const url = $derived(page.url);
	let { data } = $props();
	let { user, invite, conversation, error, workflows, eventId } = data;

	let pageTitle = $derived(
		conversation?.title ? `Invitation - ${conversation.title}` : 'Conversation Invite'
	);

	let firstWorkflow = $derived(workflows[0]);
	let firstWorkflowPath = $derived(
		`/conversations/${conversation.id}/workflow/${firstWorkflow.id}/next`
	);

	const inviteHeading = $derived(
		env.PUBLIC_INVITE_HEADING_TEXT || 'You have been invited to join the following conversation'
	);

	function login() {
		loginRedirect(url.toString(), 'Login to accept invite');
	}

	function create_account() {
		signupRedirect(url.toString(), 'Signup to accept invite');
	}

	function take_part_annon() {
		signupAnnonRedirect(url.toString(), 'Signup to accept invite');
	}

	function showAnnonPrivacy() {
		loginType = 'automatic';
		privacyPolicyOpen = true;
	}

	function showUserPrivacy() {
		privacyPolicyOpen = true;
	}

	async function handlePrivacyPolicyAccept() {
		try {
			if (loginType === 'automatic') {
				await apiClient.SignupAnnonUser(undefined, {});
				await acceptInvite();
				await goto(firstWorkflowPath + url.search, {
					invalidate: ['user', 'app:participation']
				});
			} else {
				await acceptInvite();
				await goto(firstWorkflowPath + url.search, { invalidate: ['app:participation'] });
			}
		} catch (e) {
			console.error(e);
		}
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
		goto('/');
	}

	onMount(() => {
		if (!user && eventId) {
			invalidateAll();
		}
	});
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle</title>
</svelte:head>

{#if invite}
	{#if eventId}
		<div class="mt-10 mb-20 flex flex-col items-center md:mb-0">
			<h1 class="mb-5 text-2xl font-bold">
				You have successfully been registered to this event
			</h1>
			<Button href={`/conversations/${conversation.id}/events/${eventId}`}
				>Click here to view the event</Button
			>
		</div>
	{/if}

	{#if conversation && !eventId}
		<div class="mt-10 mb-20 md:mb-0">
			{#if conversation.isComplete}
				<div class="flex flex-col gap-4">
					<h1 class="text-xl font-bold">{m.conversation_closed_title()}</h1>
					<p>{m.conversation_closed_description()}</p>
					<Button href="/conversations">{m.conversation_closed_link()}</Button>
				</div>
			{:else}
				<h1 class="mb-5 text-2xl font-bold">
					{inviteHeading}
				</h1>
				<ConversationSummary {conversation}>
					{#if !user && invite.loginBehaviour === 'manual' && firstWorkflow.autoLogin === false}
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
						<Button onclick={showUserPrivacy}
							>{conversation.callToAction || m.join_the_conversation()}</Button
						>
					{/if}

					{#if !user && (invite.loginBehaviour === 'auto_create_annon' || firstWorkflow.autoLogin)}
						<Button onclick={showAnnonPrivacy}
							>{conversation.callToAction || m.join_the_conversation()}</Button
						>
					{/if}
				</ConversationSummary>
			{/if}
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

<PrivacyPolicyDialog
	{conversation}
	bind:open={privacyPolicyOpen}
	onAccept={handlePrivacyPolicyAccept}
/>
