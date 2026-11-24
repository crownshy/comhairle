<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { Input } from '$lib/components/ui/input';
	import { Spinner } from '$lib/components/ui/spinner';
	import { m } from '$lib/paraglide/messages';
	import { notifications } from '$lib/notifications.svelte.js';
	import { goto, invalidate } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import PrivacyPolicyDialog from '$lib/components/PrivacyPolicyDialog.svelte';
	import { key } from '$lib/utils/invalidationKey.js';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { HttpStatus } from '$lib/utils/constants';
	import type { ApiError } from '@crownshy/api-client/api';
	import { pendingWork } from '$lib/stores/pendingWork.svelte';
	import { resolve } from '$app/paths';
	import { ChevronLeft, MessageSquareText } from '@lucide/svelte';
	import ConversationStatusBadge from '$lib/components/ConversationStatusBadge.svelte';
	import Conversation from '$lib/interfaces/Conversation';

	let { data } = $props();
	let { conversation, workflows, participation, preview, user } = $derived(data);
	let pageTitle = $derived(conversation?.title ?? 'Conversation');
	let statusLabel = $derived(
		conversation.isComplete ? m.conversation_status_ended() : m.conversation_status_open()
	);

	let privacyPolicyOpen = $state(false);
	let isSubmitting = $state(false);

	let emailStep = $state<'closed' | 'email' | 'code'>('closed');
	let emailValue = $state('');
	let codeValue = $state('');
	let emailError = $state<string | null>(null);
	let codeResent = $state(false);

	let guestStep = $state<'closed' | 'choice' | 'code'>('closed');
	let guestCodeValue = $state('');
	let guestError = $state<string | null>(null);

	function handleJoin() {
		if (isSubmitting) return;
		isSubmitting = true;
		if (conversation.shortPrivacyPolicy) {
			privacyPolicyOpen = true;
		} else {
			doJoin();
		}
	}

	// Joining is API calls and then a page load. Both are awaited so the progress bar stays
	// up until the step page has loaded.
	async function doJoin() {
		pendingWork.start();
		const joined = await tryCatchAsync<void, ApiError>(() =>
			!user && joinsAnonymously ? registerGuestUserSignupAndRedirect() : registerUser()
		);
		pendingWork.stop();

		if (joined.err !== null) {
			notifications.send({
				message: 'Failed to sign you up for the conversation, try again later',
				priority: 'ERROR'
			});
			isSubmitting = false;
		}
	}

	// Someone signing back in may already be part of this conversation, so send them back
	// into it rather than registering them a second time.
	function joinOrReturn() {
		if (participation) {
			// eslint-disable-next-line svelte/no-navigation-without-resolve -- built from the conversation slug
			goto(returnPath);
		} else {
			handleJoin();
		}
	}

	function handlePrivacyPolicyDismiss() {
		isSubmitting = false;
	}

	let firstWorkflow = $derived(workflows[0]);
	// Someone sent a preview link has no account and no reason to make one, so a preview
	// always offers the one-click anonymous join, whatever the conversation's own setting.
	let joinsAnonymously = $derived(firstWorkflow.autoLogin || preview);

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

		// eslint-disable-next-line svelte/no-navigation-without-resolve -- built from the conversation slug
		await goto(firstWorkflowPath, { invalidate: [key('user'), key('public/conversation')] });
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
		joinOrReturn();
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
					emailError =
						signup.err.status === HttpStatus.TooManyRequests
							? 'Too many attempts, please wait a moment and try again'
							: 'Failed to sign up with this email';
					isSubmitting = false;
					return;
				}
				await invalidate(key('user'));
				isSubmitting = false;
				emailStep = 'closed';
				handleJoin();
				return;
			}

			emailError =
				createOtp.err.status === HttpStatus.TooManyRequests
					? 'Too many attempts, please wait a moment and try again'
					: 'Failed to send one-time-passcode';
			isSubmitting = false;
			return;
		}

		isSubmitting = false;
		emailStep = 'code';
	}

	async function resendCode() {
		if (isSubmitting) return;
		isSubmitting = true;
		emailError = null;
		codeResent = false;

		const result = await tryCatchAsync<void, ApiError>(() =>
			apiClient.CreateOtp({ email: emailValue })
		);

		isSubmitting = false;
		if (result.err !== null) {
			emailError =
				result.err.status === HttpStatus.TooManyRequests
					? 'Too many attempts, please wait a moment and try again'
					: 'Failed to send one-time-passcode';
			return;
		}
		codeResent = true;
	}

	function changeEmail() {
		codeValue = '';
		emailError = null;
		codeResent = false;
		emailStep = 'email';
	}

	async function submitCode() {
		if (!codeValue || isSubmitting) return;
		isSubmitting = true;
		emailError = null;

		const result = await tryCatchAsync<unknown, ApiError>(() =>
			apiClient.LoginOtpUser({ email: emailValue, code: codeValue })
		);

		if (result.err !== null) {
			emailError =
				result.err.status === HttpStatus.TooManyRequests
					? 'Too many attempts, please wait a moment and try again'
					: 'Invalid or expired code';
			isSubmitting = false;
			return;
		}

		await invalidate(key('user'));
		isSubmitting = false;
		emailStep = 'closed';
		joinOrReturn();
	}

	async function registerUser() {
		await apiClient.RegisterUserForConversationWorkflow(undefined, {
			params: { conversation_id: data.conversation.id, workflow_id: firstWorkflow.id }
		});

		notifications.addFlash({
			message: `You are part of the "${conversation.title}" conversation!`
		});

		// eslint-disable-next-line svelte/no-navigation-without-resolve -- built from the conversation slug
		await goto(firstWorkflowPath);
	}
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle</title>
</svelte:head>

<div class="pb-10">
	{#if conversation}
		{#if conversation.isComplete}
			<div class="mx-auto flex max-w-3xl flex-col gap-4 px-4 pt-10 md:pt-20">
				<h1 class="text-xl font-bold">{m.conversation_closed_title()}</h1>
				<p>{m.conversation_closed_description()}</p>
				<Button href="/conversations">{m.conversation_closed_link()}</Button>
			</div>
		{:else}
			<!-- Top band on white: back link, status, title and a short description, then a white bar
	with the join controls that sits across the join with the full-width image below.
	Organiser, time needed and closing date belong in the bar too, once the API has them. -->
			<div class="bg-background">
				<div
					class="mx-auto flex w-full max-w-5xl flex-col gap-5 px-4 pt-6 md:px-8 md:pt-10"
				>
					<a
						href={resolve('/')}
						class="text-muted-foreground hover:text-foreground -ml-1 flex w-fit items-center gap-1 text-base font-medium"
					>
						<ChevronLeft class="size-5" />
						{m.conversations_all_heading()}
					</a>
					<div class="flex flex-col items-start gap-4 md:px-6">
						<h1
							class="text-3xl leading-tight font-bold tracking-tight text-balance sm:text-4xl lg:text-5xl"
						>
							{conversation.title}
						</h1>
						{#if conversation.shortDescription}
							<p class="line-clamp-3 max-w-3xl text-lg">
								{conversation.shortDescription}
							</p>
						{/if}
					</div>

					<!-- Pulled down over the image. -->
					<div
						class="bg-card ring-border/60 relative z-10 -mb-24 flex flex-col gap-4 rounded-3xl p-5 shadow-lg ring-1 sm:-mb-20 md:flex-row md:items-center md:justify-between md:gap-6 md:p-6"
					>
						<div
							class="flex items-baseline justify-between gap-4 md:flex-col md:justify-start md:gap-1 md:pl-2"
						>
							<span class="text-muted-foreground text-base">
								{m.conversation_status_heading()}
							</span>
							<ConversationStatusBadge
								status={Conversation.getStatus(conversation)}
								label={statusLabel}
								class="text-lg font-semibold"
							/>
						</div>
						<!-- The join controls bring their own top margin from the old layout; cancel it here. -->
						<div class="flex flex-col gap-2 *:mt-0 md:items-end">
							{#if user}
								{#if participation}
									<Button
										class="mt-5 w-full md:w-fit"
										variant="primaryDark"
										href={returnPath}
									>
										{conversation.callToAction || m.jump_back_in()}
									</Button>
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
							{:else if joinsAnonymously}
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
									<h2 class="text-lg font-semibold">{m.check_your_email()}</h2>
									<p class="text-muted-foreground max-w-prose text-base">
										{m.otp_sent_to_email({ email: emailValue })}
									</p>
									<Input
										placeholder={m.otp_placeholder()}
										bind:value={codeValue}
										autocomplete="one-time-code"
										required
										disabled={isSubmitting}
									/>
									{#if emailError}
										<p class="text-destructive text-sm">{emailError}</p>
									{:else if codeResent}
										<p class="text-base">{m.otp_resent()}</p>
									{/if}
									<Button type="submit" disabled={isSubmitting}>
										{#if isSubmitting}
											<Spinner class="mr-2 size-4" />
										{/if}
										{m.verify_code()}
									</Button>
									<p class="text-muted-foreground text-base">
										{m.resend_otp()}
										<button
											type="button"
											class="text-primary underline"
											onclick={resendCode}
											disabled={isSubmitting}
										>
											{m.send_again()}
										</button>
									</p>
									<button
										type="button"
										class="text-primary self-start text-base underline"
										onclick={changeEmail}
										disabled={isSubmitting}
									>
										{m.use_a_different_email()}
									</button>
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
						</div>
					</div>
				</div>
			</div>

			<div class="bg-primary/10 relative h-72 w-full overflow-hidden sm:h-96 lg:h-128">
				{#if conversation.imageUrl}
					<img
						src={conversation.imageUrl}
						alt="Heading image for conversation {conversation.title}"
						class="absolute inset-0 size-full object-cover"
					/>
				{:else}
					<MessageSquareText class="text-primary/30 absolute inset-0 m-auto size-20" />
				{/if}
			</div>

			{#if conversation.description}
				<div class="mx-auto w-full max-w-3xl px-4 pt-10 md:px-0 md:pt-14">
					<p class="text-foreground text-lg whitespace-pre-line">
						{conversation.description}
					</p>
				</div>
			{/if}

			<PrivacyPolicyDialog
				{conversation}
				availableDocuments={data.availableDocuments}
				bind:open={privacyPolicyOpen}
				onAccept={doJoin}
				onDismiss={handlePrivacyPolicyDismiss}
			/>
		{/if}
	{:else}
		<h1 class="mx-auto max-w-3xl px-4 pt-10">Conversation not found</h1>
	{/if}
</div>
