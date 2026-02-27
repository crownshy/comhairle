<script lang="ts">
	import type { PageProps } from '../$types.js';
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as m from '$lib/paraglide/messages';
	import { notifications } from '$lib/notifications.svelte.js';
	import { goto, invalidateAll } from '$app/navigation';
	import { apiClient } from '@crown-shy/api-client/client';
	import { page } from '$app/state';
	import ConversationSummary from '$lib/components/ConversationSummary.svelte';
	import { loginRedirect, signupRedirect } from '$lib/urls.js';

	let { data }: PageProps = $props();
	let { conversation, workflows, participation } = data;
	let user = $derived(data.user);

	let firstWorkflow = $derived(workflows[0]);

	let url = $derived(page.url);

	let firstWorkflowPath = $derived(`${url}/workflow/${firstWorkflow.id}/next`);

	async function redirectToLogin() {
		loginRedirect(url.toString(), 'Login to join the conversation');
	}

	// Register a new annon user, sign them up for
	// the workflow and redirect to it
	async function registerAnnonUserSignupAndRedirect() {
		await apiClient.SignupAnnonUser(undefined, {});

		await apiClient.RegisterUserForWorkflow(undefined, {
			params: { conversation_id: data.conversation.id, workflow_id: firstWorkflow.id }
		});

		await invalidateAll();
		goto(firstWorkflowPath);
	}

	async function redirectToSignIn() {
		signupRedirect(url.toString(), 'Signup to join the conversation');
	}

	async function registerUser() {
		try {
			await apiClient.RegisterUserForWorkflow(undefined, {
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
		}
	}
</script>

<div class="pt-5 md:pt-20">
	{#if conversation}
		<div class="hidden md:block">
			<Breadcrumbs {conversation} />
		</div>
		<ConversationSummary {conversation}>
			{#if user}
				{#if participation}
					<Button class="mt-5 w-full md:w-fit" href={firstWorkflowPath}
						>{m.jump_back_in()}</Button
					>
				{:else}
					<Button class="mt-5 w-full md:w-fit" onclick={registerUser}
						>{m.join_the_conversation()}</Button
					>
				{/if}
			{:else if firstWorkflow.autoLogin}
				<Button class="mt-5 w-full md:w-fit" onclick={registerAnnonUserSignupAndRedirect}
					>{m.join_the_conversation()}</Button
				>
			{:else}
				<Button class="mt-5 w-full md:w-fit" onclick={redirectToLogin}
					>{m.login_to_take_part()}</Button
				>
				<Button class="mt-5 w-full md:w-fit" onclick={redirectToSignIn}
					>{m.signup_to_take_part()}</Button
				>
			{/if}
		</ConversationSummary>
	{:else}
		<h1>Conversation not found</h1>
	{/if}
</div>
