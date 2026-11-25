<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { m } from '$lib/paraglide/messages';
	import { notifications } from '$lib/notifications.svelte';
	import { goto, invalidate } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import PrivacyPolicyDialog from '$lib/components/PrivacyPolicyDialog.svelte';
	import { key } from '$lib/utils/invalidationKey';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { HttpStatus } from '$lib/utils/constants';
	import { type ApiError } from '@crownshy/api-client/api';
	import { pendingWork } from '$lib/stores/pendingWork.svelte';
	import { resolve } from '$app/paths';
	import { ChevronLeft, MessageSquareText } from '@lucide/svelte';
	import ConversationStatusBadge from '$lib/components/ConversationStatusBadge.svelte';
	import Conversation from '$lib/interfaces/Conversation';
	import { Form, Input, Submit, type OnSubmit } from '$lib/components/EasyForm';
	import { LoadingButton } from '$lib/components/ui/button';
	import { type ApiResponse } from '$lib/types';

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

	let codeResent = $state(false);

	let guestStep = $state<'closed' | 'choice' | 'code'>('closed');

	function handleNotification(status: number, message: string) {
		notifications.addFlash({
			message:
				status === HttpStatus.TooManyRequests
					? 'Too many attempts, please wait a moment and try again'
					: message,
			priority: 'ERROR'
		});
	}

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

	async function doNewGuestJoin() {
		if (isSubmitting) return;
		isSubmitting = true;

		const result = await tryCatchAsync(() => apiClient.SignupGuestUser(undefined, {}));

		isSubmitting = false;
		if (result.err !== null) {
			notifications.send({
				message: 'Failed to join as a guest, try again later',
				priority: 'ERROR'
			});
			return;
		}

		await invalidate(key('user'));
		guestStep = 'closed';
		handleJoin();
	}

	const submitGuestCode: OnSubmit = async (formData) => {
		if (isSubmitting) return;
		isSubmitting = true;

		const guestCode = formData.get('guest-code') as string | null;
		if (!guestCode) {
			return;
		}

		const result = await tryCatchAsync(() =>
			apiClient.LoginGuestUser({ guest_code: guestCode })
		);

		isSubmitting = false;
		if (result.err !== null) {
			notifications.addFlash({
				message: 'Invalid guest ID',
				priority: 'ERROR'
			});
			return;
		}

		await invalidate(key('user'));
		guestStep = 'closed';
		joinOrReturn();
	};

	const submitEmail: OnSubmit = async (formData) => {
		if (isSubmitting) return;
		isSubmitting = true;

		const email = formData.get('email') as string | null;
		if (!email) {
			return;
		}

		const createOtp = await tryCatchAsync<ApiResponse<'CreateOtp'>, ApiError>(() =>
			apiClient.CreateOtp({ email })
		);

		if (createOtp.ok !== null) {
			isSubmitting = false;
			emailValue = email;
			emailStep = 'code';
			return;
		}

		if (createOtp.err.status === HttpStatus.NotFound) {
			const signup = await tryCatchAsync<ApiResponse<'SignupOtp'>, ApiError>(() =>
				apiClient.SignupOtp({ email })
			);
			isSubmitting = false;
			if (signup.err !== null) {
				handleNotification(signup.err.status, 'Failed to sign up with this email');
				return;
			}
			await invalidate(key('user'));
			emailStep = 'closed';
			handleJoin();
			return;
		}

		handleNotification(createOtp.err.status, 'Failed to send one-time-passcode');
		isSubmitting = false;
	};

	const resendCode: OnSubmit = async (formData) => {
		if (isSubmitting) return;
		isSubmitting = true;
		codeResent = false;

		const email = formData.get('email') as string | null;
		if (!email) {
			return;
		}

		const result = await tryCatchAsync<ApiResponse<'CreateOtp'>, ApiError>(() =>
			apiClient.CreateOtp({ email })
		);

		isSubmitting = false;
		if (result.err !== null) {
			handleNotification(result.err.status, 'Failed to send one-time-passcode');
			return;
		}
		codeResent = true;
	};

	const submitCode: OnSubmit = async (formData) => {
		if (isSubmitting) return;
		isSubmitting = true;

		const otp = formData.get('otp') as string | null;
		if (!emailValue || !otp) {
			return;
		}

		const result = await tryCatchAsync<ApiResponse<'LoginOtpUser'>, ApiError>(() =>
			apiClient.LoginOtpUser({ email: emailValue, code: otp })
		);

		isSubmitting = false;
		if (result.err !== null) {
			handleNotification(result.err.status, 'Invalid or expired code');
			return;
		}

		await invalidate(key('user'));
		emailStep = 'closed';
		joinOrReturn();
	};

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
									<LoadingButton
										class="mt-5 w-full md:w-fit"
										onclick={handleJoin}
										loading={isSubmitting}
									>
										{conversation.callToAction || m.join_the_conversation()}
									</LoadingButton>
								{/if}
							{:else if joinsAnonymously}
								<LoadingButton
									class="mt-5 w-full md:w-fit"
									onclick={handleJoin}
									loading={isSubmitting}
								>
									{conversation.callToAction || m.join_the_conversation()}
								</LoadingButton>
							{:else if emailStep === 'email'}
								<Form
									class="mt-5 flex w-full flex-col gap-2 md:w-fit"
									onsubmit={submitEmail}
								>
									<Input
										required
										name="email"
										type="email"
										placeholder={m.enter_your_email()}
										disabled={isSubmitting}
									/>
									<Submit text={m.continue_button()} {isSubmitting} />
								</Form>
							{:else if emailStep === 'code'}
								<Form id="resend-code-form" onsubmit={resendCode}>
									<Form
										id="submit-code-form"
										class="mt-5 flex w-full flex-col gap-2 md:w-fit"
										onsubmit={submitCode}
									>
										<h2 class="text-lg font-semibold">
											{m.check_your_email()}
										</h2>
										<p class="text-muted-foreground max-w-prose text-base">
											{m.otp_sent_to_email({ email: emailValue })}
										</p>
										<Input
											required
											name="otp"
											placeholder={m.otp_placeholder()}
											disabled={isSubmitting}
										/>
										{#if codeResent}
											<p class="text-base">{m.otp_resent()}</p>
										{/if}
										<Submit
											text={m.verify_code()}
											{isSubmitting}
											form="submit-code-form"
										/>
										<p class="text-muted-foreground text-base">
											{m.resend_otp()}
											<Submit
												variant="link"
												form="resend-code-form"
												disabled={isSubmitting}
												text={m.send_again()}
											/>
										</p>
										<Button
											variant="link"
											onclick={() => {
												codeResent = false;
												emailStep = 'email';
											}}
											disabled={isSubmitting}
										>
											{m.use_a_different_email()}
										</Button>
									</Form>
								</Form>
							{:else if guestStep === 'choice'}
								<div class="mt-5 flex w-full flex-col gap-2 md:w-fit">
									<LoadingButton loading={isSubmitting} onclick={doNewGuestJoin}>
										{m.start_as_new_guest()}
									</LoadingButton>
									<Button
										variant="outline"
										onclick={() => void (guestStep = 'code')}
										disabled={isSubmitting}
									>
										{m.have_a_guest_code()}
									</Button>
								</div>
							{:else if guestStep === 'code'}
								<Form
									class="mt-5 flex w-full flex-col gap-2 md:w-fit"
									onsubmit={submitGuestCode}
								>
									<Input
										required
										name="guest-code"
										placeholder={m.anonymous_id()}
										disabled={isSubmitting}
									/>
									<Submit text={m.continue_with_guest_code()} {isSubmitting} />
								</Form>
							{:else}
								<Button
									class="mt-5 w-full md:w-fit"
									onclick={() => void (emailStep = 'email')}
									disabled={isSubmitting}
								>
									{m.continue_with_email()}
								</Button>
								<Button
									class="mt-5 w-full md:w-fit"
									variant="outline"
									onclick={() => void (guestStep = 'choice')}
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
