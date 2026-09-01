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
	import StepChrome from '$lib/components/participant/StepChrome.svelte';
	import type { StepItem } from '$lib/components/participant/stepItems';
	import { stepPreviews } from '$lib/components/participant/stepPreview';
	import StepZeroScreen from './StepZeroScreen.svelte';
	import BeforeYouStart from './BeforeYouStart.svelte';
	import { beforeYouStartPages } from '$lib/components/participant/beforeYouStart';

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

	let steps = $derived(stepPreviews(data.workflowSteps));
	let pages = $derived(beforeYouStartPages(conversation, steps));

	/**
	 * The landing page is Step zero: the progress bar carries its own segment ahead of the
	 * workflow's, so a participant sees the shape of the whole journey before joining and the
	 * bar does not appear out of nowhere on the first Step (ADR-0021).
	 *
	 * The intro segment is excluded from "Step N of M", so adding it here cannot change the
	 * number of steps a participant is quoted.
	 */
	let stepItems = $derived<StepItem[]>([
		{
			id: 'landing',
			name: m.landing_before_you_start(),
			status: 'current',
			isIntro: true
		},
		...steps.map((step) => ({ id: step.id, name: step.name, status: 'upcoming' as const }))
	]);

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
	<!-- The cover owns the first viewport: chrome, cover, call to action, nothing below the
	     fold until you scroll. `min-h` rather than a fixed height because the chrome grows on
	     a narrow screen and the cover must be allowed to push past the fold rather than clip. -->
	<div class="flex min-h-[100dvh] flex-col pb-28">
		<StepChrome
			steps={stepItems}
			currentIndex={0}
			label={m.landing_before_you_start()}
			fill={0}
			showSupport={false}
			{preview}
		/>

		<StepZeroScreen
			{conversation}
			{steps}
			onReadMore={pages.length ? scrollToDetail : undefined}
		/>
	</div>

	<BeforeYouStart
		{pages}
		{steps}
		conversationId={conversation.id}
		availableDocuments={data.availableDocuments}
	/>

	<!-- Fixed rather than sticky: the call to action has to survive the whole scroll through
	     the detail, not just the cover. Both blocks above reserve its height. -->
	<div class="bg-background fixed inset-x-0 bottom-0 z-30 border-t">
		<div class="mx-auto flex w-full max-w-5xl flex-col gap-2 px-5 pt-3 pb-5 md:px-6">
			{#if user && participation}
				<Button
					class="h-12 w-full text-base md:mx-auto md:w-80"
					variant="primaryDark"
					href={returnPath}
				>
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
		</div>
	</div>

	<PrivacyPolicyDialog
		{conversation}
		availableDocuments={data.availableDocuments}
		bind:open={privacyPolicyOpen}
		onAccept={handlePrivacyPolicyAccept}
	/>
{/if}
