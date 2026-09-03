<script lang="ts">
	import type { PageProps } from './$types.js';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Spinner } from '$lib/components/ui/spinner';
	import * as m from '$lib/paraglide/messages';
	import { notifications } from '$lib/notifications.svelte.js';
	import { goto } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import { page } from '$app/state';
	import { loginRedirect, signupRedirect } from '$lib/urls.js';
	import PrivacyPolicyDialog from '$lib/components/PrivacyPolicyDialog.svelte';
	import LandingShell from '$lib/components/participant/LandingShell.svelte';
	import { stepPreviews } from '$lib/components/participant/stepPreview';
	import { beforeYouStartPages } from '$lib/components/participant/beforeYouStart';
	import { routeProgress } from '$lib/stores/routeProgress.svelte';

	let { data }: PageProps = $props();
	let { conversation, workflows, participation, preview } = data;
	let user = $derived(data.user);
	let pageTitle = $derived(conversation?.title ?? 'Conversation');

	let privacyPolicyOpen = $state(false);
	let isSubmitting = $state(false);
	// The return link is a plain navigation, so nothing in the button changes until
	// the next page has loaded. Flip it on click so the tap registers.
	let isReturning = $state(false);
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

	async function doJoin() {
		// The join is two API calls and a page load before anything moves. Hold the
		// global progress bar up for the whole of it, not just the navigation.
		routeProgress.start();
		try {
			if (!user && firstWorkflow.autoLogin) {
				await registerGuestUserSignupAndRedirect();
			} else {
				await registerUser();
			}
		} finally {
			routeProgress.stop();
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

	// Register a new guest user, sign them up for
	// the workflow and redirect to it
	async function registerGuestUserSignupAndRedirect() {
		await apiClient.SignupGuestUser(undefined, {});

		await apiClient.RegisterUserForConversationWorkflow(undefined, {
			params: { conversation_id: data.conversation.id, workflow_id: firstWorkflow.id }
		});

		await goto(firstWorkflowPath, { invalidateAll: true });
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

			await goto(firstWorkflowPath);
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

	let steps = $derived(stepPreviews(data.workflowSteps));
	let pages = $derived(beforeYouStartPages(conversation, steps));

	function scrollToDetail() {
		document
			.getElementById('conversation-detail')
			?.scrollIntoView({ behavior: 'smooth', block: 'start' });
	}
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle</title>
</svelte:head>

{#if !conversation}
	<div class="mx-auto w-full max-w-5xl px-5 py-20 md:px-6">
		<h1 class="text-2xl font-semibold">Conversation not found</h1>
	</div>
{:else if conversation.isComplete}
	<div class="mx-auto flex w-full max-w-5xl flex-col gap-4 px-5 py-20 md:px-6">
		<h1 class="text-2xl font-semibold">{m.conversation_closed_title()}</h1>
		<p class="text-base">{m.conversation_closed_description()}</p>
		<Button class="w-fit" href="/conversations">{m.conversation_closed_link()}</Button>
	</div>
{:else}
	<LandingShell
		{conversation}
		{steps}
		{pages}
		availableDocuments={data.availableDocuments}
		{preview}
		onReadMore={pages.length ? scrollToDetail : undefined}
	>
		{#snippet callToAction()}
			{#if user && participation}
				<Button
					class="h-12 w-full text-base md:mx-auto md:w-80"
					variant="primaryDark"
					href={returnPath}
					onclick={() => (isReturning = true)}
				>
					{#if isReturning}
						<Spinner class="mr-2 size-4" />
					{/if}
					{conversation.callToAction || m.jump_back_in()}
				</Button>
			{:else if user || firstWorkflow.autoLogin}
				<Button
					class="h-12 w-full text-base md:mx-auto md:w-80"
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
					class="h-12 w-full text-base md:mx-auto md:w-80"
					onclick={redirectToSignIn}
					disabled={isSubmitting}
				>
					{#if isSubmitting}
						<Spinner class="mr-2 size-4" />
					{/if}
					{m.signup_to_take_part()}
				</Button>
				<Button
					variant="ghost"
					class="h-10 w-full text-base md:mx-auto md:w-80"
					onclick={redirectToLogin}
					disabled={isSubmitting}
				>
					{m.login_to_take_part()}
				</Button>
			{/if}
		{/snippet}
	</LandingShell>

	<PrivacyPolicyDialog
		{conversation}
		availableDocuments={data.availableDocuments}
		bind:open={privacyPolicyOpen}
		onAccept={handlePrivacyPolicyAccept}
	/>
{/if}
