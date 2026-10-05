<script lang="ts">
	import Breadcrumbs from '$lib/components/Breadcrumbs.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Input } from '$lib/components/ui/input';
	import { Spinner } from '$lib/components/ui/spinner';
	import * as m from '$lib/paraglide/messages';
	import { notifications } from '$lib/notifications.svelte.js';
	import { goto, invalidate } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import ConversationSummary from '$lib/components/ConversationSummary.svelte';
	import PrivacyPolicyDialog from '$lib/components/PrivacyPolicyDialog.svelte';
	import { key } from '$lib/utils/invalidationKey.js';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { HttpStatus } from '$lib/utils/constants';
	import type { ApiError } from '@crownshy/api-client/api';

	let { data } = $props();
	let { conversation, workflows, participation, preview } = data;
	let user = $derived(data.user);
	let pageTitle = $derived(conversation?.title ?? 'Conversation');

	let privacyPolicyOpen = $state(false);
	let isSubmitting = $state(false);
	let privacyAccepted = $state(false);

	let emailStep = $state<'closed' | 'email' | 'code'>('closed');
	let emailValue = $state('');
	let codeValue = $state('');
	let emailError = $state<string | null>(null);

	let guestStep = $state<'closed' | 'choice' | 'code'>('closed');
	let guestCodeValue = $state('');
	let guestError = $state<string | null>(null);

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
			registerGuestUserSignupAndRedirect();
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

	let firstWorkflowPath = $derived(
		`/conversations/${conversation.slug}${preview ? '/preview' : ''}/workflow/${firstWorkflow.id}/next`
	);
	const returnPath = $derived(
		`/conversations/${conversation.slug}${preview ? '/preview' : ''}/workflow/${firstWorkflow.id}/return`
	);

	// Register a new guest user, sign them up for
	// the workflow and redirect to it
	async function registerGuestUserSignupAndRedirect() {
		await apiClient.SignupGuestUser(undefined, {});

		await apiClient.RegisterUserForConversationWorkflow(undefined, {
			params: { conversation_id: data.conversation.id, workflow_id: firstWorkflow.id }
		});

		goto(firstWorkflowPath, { invalidate: [key('user'), key('public/conversation')] });
	}

	function openGuestChoice() {
		guestStep = 'choice';
		guestError = null;
	}

	function openGuestCodeStep() {
		guestStep = 'code';
		guestError = null;
	}

	function handleNewGuestJoin() {
		if (isSubmitting) return;
		isSubmitting = true;
		doNewGuestJoin();
	}

	async function doNewGuestJoin() {
		const result = await tryCatchAsync<unknown, ApiError>(() =>
			apiClient.SignupGuestUser(undefined, {})
		);
		if (result.err !== null) {
			notifications.send({
				message: 'Failed to join as a guest, try again later',
				priority: 'ERROR'
			});
			isSubmitting = false;
			return;
		}
		await invalidate(key('user'));
		isSubmitting = false;
		guestStep = 'closed';
		handleJoin();
	}

	async function submitGuestCode() {
		if (!guestCodeValue || isSubmitting) return;
		isSubmitting = true;
		guestError = null;

		const result = await tryCatchAsync<unknown, ApiError>(() =>
			apiClient.LoginGuestUser({ guest_code: guestCodeValue })
		);

		if (result.err !== null) {
			guestError = 'Invalid guest ID';
			isSubmitting = false;
			return;
		}

		await invalidate(key('user'));
		isSubmitting = false;
		guestStep = 'closed';
		handleJoin();
	}

	function openEmailStep() {
		emailStep = 'email';
		emailError = null;
	}

	async function submitEmail() {
		if (!emailValue || isSubmitting) return;
		isSubmitting = true;
		emailError = null;

		const createOtp = await tryCatchAsync<void, ApiError>(() =>
			apiClient.CreateOtp({ email: emailValue })
		);

		if (createOtp.err !== null) {
			if (createOtp.err.status === HttpStatus.NotFound) {
				const signup = await tryCatchAsync<unknown, ApiError>(() =>
					apiClient.SignupOtp({ email: emailValue })
				);
				if (signup.err !== null) {
					emailError = 'Failed to sign up with this email';
					isSubmitting = false;
					return;
				}
				await invalidate(key('user'));
				isSubmitting = false;
				emailStep = 'closed';
				handleJoin();
				return;
			}

			emailError = 'Failed to send one-time-passcode';
			isSubmitting = false;
			return;
		}

		isSubmitting = false;
		emailStep = 'code';
	}

	async function submitCode() {
		if (!codeValue || isSubmitting) return;
		isSubmitting = true;
		emailError = null;

		const result = await tryCatchAsync<unknown, ApiError>(() =>
			apiClient.LoginOtpUser({ email: emailValue, code: codeValue })
		);

		if (result.err !== null) {
			emailError = 'Invalid or expired code';
			isSubmitting = false;
			return;
		}

		await invalidate(key('user'));
		isSubmitting = false;
		emailStep = 'closed';
		handleJoin();
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
							>{conversation.callToAction || m.jump_back_in()}</Button
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
				{:else if emailStep === 'email'}
					<form
						class="mt-5 flex w-full flex-col gap-2 md:w-fit"
						onsubmit={(e) => {
							e.preventDefault();
							submitEmail();
						}}
					>
						<Input
							type="email"
							placeholder={m.enter_your_email()}
							bind:value={emailValue}
							required
							disabled={isSubmitting}
						/>
						{#if emailError}
							<p class="text-destructive text-sm">{emailError}</p>
						{/if}
						<Button type="submit" disabled={isSubmitting}>
							{#if isSubmitting}
								<Spinner class="mr-2 size-4" />
							{/if}
							{m.continue_button()}
						</Button>
					</form>
				{:else if emailStep === 'code'}
					<form
						class="mt-5 flex w-full flex-col gap-2 md:w-fit"
						onsubmit={(e) => {
							e.preventDefault();
							submitCode();
						}}
					>
						<span class="inline-block">{emailValue}</span>
						<Input
							placeholder={m.otp_placeholder()}
							bind:value={codeValue}
							required
							disabled={isSubmitting}
						/>
						{#if emailError}
							<p class="text-destructive text-sm">{emailError}</p>
						{/if}
						<Button type="submit" disabled={isSubmitting}>
							{#if isSubmitting}
								<Spinner class="mr-2 size-4" />
							{/if}
							{m.verify_code()}
						</Button>
					</form>
				{:else if guestStep === 'choice'}
					<div class="mt-5 flex w-full flex-col gap-2 md:w-fit">
						<Button onclick={handleNewGuestJoin} disabled={isSubmitting}>
							{#if isSubmitting}
								<Spinner class="mr-2 size-4" />
							{/if}
							{m.start_as_new_guest()}
						</Button>
						<Button
							variant="outline"
							onclick={openGuestCodeStep}
							disabled={isSubmitting}
						>
							{m.have_a_guest_code()}
						</Button>
					</div>
				{:else if guestStep === 'code'}
					<form
						class="mt-5 flex w-full flex-col gap-2 md:w-fit"
						onsubmit={(e) => {
							e.preventDefault();
							submitGuestCode();
						}}
					>
						<Input
							placeholder={m.anonymous_id()}
							bind:value={guestCodeValue}
							required
							disabled={isSubmitting}
						/>
						{#if guestError}
							<p class="text-destructive text-sm">{guestError}</p>
						{/if}
						<Button type="submit" disabled={isSubmitting}>
							{#if isSubmitting}
								<Spinner class="mr-2 size-4" />
							{/if}
							{m.continue_with_guest_code()}
						</Button>
					</form>
				{:else}
					<Button
						class="mt-5 w-full md:w-fit"
						onclick={openEmailStep}
						disabled={isSubmitting}
					>
						{m.continue_with_email()}
					</Button>
					<Button
						class="mt-5 w-full md:w-fit"
						variant="outline"
						onclick={openGuestChoice}
						disabled={isSubmitting}
					>
						{m.join_as_guest()}
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
