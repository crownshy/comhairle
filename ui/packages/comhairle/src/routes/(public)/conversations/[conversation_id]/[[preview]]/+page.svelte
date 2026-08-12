<script lang="ts">
	import type { PageProps } from '../$types.js';
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Spinner } from '$lib/components/ui/spinner';
	import * as m from '$lib/paraglide/messages';
	import { notifications } from '$lib/notifications.svelte.js';
	import { goto, invalidateAll } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import { page } from '$app/state';
	import ConversationSummary from '$lib/components/ConversationSummary.svelte';
	import { loginRedirect, signupRedirect } from '$lib/urls.js';
	import PrivacyPolicyDialog from '$lib/components/PrivacyPolicyDialog.svelte';

	let { data }: PageProps = $props();
	let { conversation, workflows, participation, preview } = data;
	let user = $derived(data.user);
	let pageTitle = $derived(conversation?.title ?? 'Conversation');

	let privacyPolicyOpen = $state(false);
	let isSubmitting = $state(false);
	let privacyAccepted = $state(false);

	function handleJoin() {
		if (isSubmitting) return;
		isSubmitting = true;
		if (conversation.shortPrivacyPolicy) {
			privacyAccepted = false;
			privacyPolicyOpen = true;
		} else {
			doJoin();
		}
	}

	function doJoin() {
		if (!user && firstWorkflow.autoLogin) {
			registerAnnonUserSignupAndRedirect();
		} else {
			registerUser();
		}
	}

	function handlePrivacyPolicyAccept() {
		privacyAccepted = true;
		doJoin();
	}

	$effect(() => {
		if (!privacyPolicyOpen && !privacyAccepted && isSubmitting) {
			isSubmitting = false;
		}
	});

	let firstWorkflow = $derived(workflows[0]);

	let url = $derived(page.url);

	let firstWorkflowPath = $derived(
		`/conversations/${conversation.slug}${preview ? '/preview' : ''}/workflow/${firstWorkflow.id}/next`
	);
	const returnPath = $derived(
		`/conversations/${conversation.slug}${preview ? '/preview' : ''}/workflow/${firstWorkflow.id}/return`
	);

	async function redirectToLogin() {
		if (isSubmitting) return;
		isSubmitting = true;
		loginRedirect(url.pathname, 'Login to join the conversation');
	}

	// Register a new annon user, sign them up for
	// the workflow and redirect to it
	async function registerAnnonUserSignupAndRedirect() {
		await apiClient.SignupAnnonUser(undefined, {});

		await apiClient.RegisterUserForConversationWorkflow(undefined, {
			params: { conversation_id: data.conversation.id, workflow_id: firstWorkflow.id }
		});

		goto(firstWorkflowPath, { invalidateAll: true });
	}

	async function redirectToSignIn() {
		if (isSubmitting) return;
		isSubmitting = true;
		signupRedirect(url.pathname, 'Signup to join the conversation');
	}

	async function registerUser() {
		try {
			await apiClient.RegisterUserForConversationWorkflow(undefined, {
				params: { conversation_id: data.conversation.id, workflow_id: firstWorkflow.id }
			});

			notifications.addFlash({
				message: `You are part of the "${conversation.title}" conversation!`
			});

			goto(firstWorkflowPath);
		} catch (e) {
			let message;

			if (e instanceof Error) message = e.message;
			else message = String(e);

			console.warn(`Failed to register user for workflow ${message}`);

			notifications.send({
				message: 'Failed to sign you up for the conversation, try again later',
				priority: 'ERROR'
			});
			isSubmitting = false;
		}
	}
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle</title>
</svelte:head>

<div class="pt-5 pb-10 md:pt-20">
	{#if conversation}
		<div class="hidden md:block">
			<Breadcrumbs {conversation} />
		</div>
		{#if conversation.isComplete}
			<div class="flex flex-col gap-4">
				<h1 class="text-xl font-bold">{m.conversation_closed_title()}</h1>
				<p>{m.conversation_closed_description()}</p>
				<Button href="/conversations">{m.conversation_closed_link()}</Button>
			</div>
		{:else}
			<ConversationSummary {conversation}>
				{#if user}
					{#if participation}
						<Button class="mt-5 w-full md:w-fit" variant="primaryDark" href={returnPath}
							>{m.jump_back_in()}</Button
						>
					{:else}
						<Button
							class="mt-5 w-full md:w-fit"
							onclick={handleJoin}
							disabled={isSubmitting}
						>
							{#if isSubmitting}
								<Spinner class="mr-2 size-4" />
							{/if}
							{conversation.callToAction || m.join_the_conversation()}
						</Button>
					{/if}
				{:else if firstWorkflow.autoLogin}
					<Button
						class="mt-5 w-full md:w-fit"
						onclick={handleJoin}
						disabled={isSubmitting}
					>
						{#if isSubmitting}
							<Spinner class="mr-2 size-4" />
						{/if}
						{conversation.callToAction || m.join_the_conversation()}
					</Button>
				{:else}
					<Button
						class="mt-5 w-full md:w-fit"
						onclick={redirectToLogin}
						disabled={isSubmitting}
					>
						{#if isSubmitting}
							<Spinner class="mr-2 size-4" />
						{/if}
						{m.login_to_take_part()}
					</Button>
					<Button
						class="mt-5 w-full md:w-fit"
						onclick={redirectToSignIn}
						disabled={isSubmitting}
					>
						{#if isSubmitting}
							<Spinner class="mr-2 size-4" />
						{/if}
						{m.signup_to_take_part()}
					</Button>
				{/if}
			</ConversationSummary>

			<PrivacyPolicyDialog
				{conversation}
				availableDocuments={data.availableDocuments}
				bind:open={privacyPolicyOpen}
				onAccept={handlePrivacyPolicyAccept}
			/>
		{/if}
	{:else}
		<h1>Conversation not found</h1>
	{/if}
</div>
