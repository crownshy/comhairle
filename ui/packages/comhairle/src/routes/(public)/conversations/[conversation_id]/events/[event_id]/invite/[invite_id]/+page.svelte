<script lang="ts">
	import { goto, invalidate } from '$app/navigation';
	import { apiErrorStatus, tryCatchAsync } from '$lib/utils/errorHandling';
	import { key } from '$lib/utils/invalidationKey';
	import { resolve } from '$app/paths';
	import Button from '$lib/components/ui/button/button.svelte';
	import Spinner from '$lib/components/ui/spinner/spinner.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { onMount } from 'svelte';

	let { data } = $props();
	const { user, invite, eventId, conversationId, error: serverError } = data;

	const defaultErrorMessage = 'An error occurred registering you for this event.';
	const INVITE_EXPIRED = 'Invite expired';
	const INVITE_ALREADY_ANSWERED =
		'An invite response has already been created for this invite by this user';

	let error = $state<string | null>(serverError ? serverError : null);

	const redirectUrl = `/conversations/${conversationId}/events/${eventId}`;

	function apiErrorText(failure: unknown) {
		return (failure as { response?: { data?: { err?: string } } }).response?.data?.err;
	}

	function isInviteAlreadyAnswered(failure: unknown) {
		return apiErrorStatus(failure) === 409 && apiErrorText(failure) === INVITE_ALREADY_ANSWERED;
	}

	function registrationFailureMessage(failure: unknown) {
		if (apiErrorStatus(failure) === 410 || isInviteAlreadyAnswered(failure)) {
			return INVITE_EXPIRED;
		}
		return apiErrorStatus(failure) === 409
			? (apiErrorText(failure) ?? defaultErrorMessage)
			: defaultErrorMessage;
	}

	async function finishRegistration() {
		notifications.send({
			message: 'You have successfully been registered for the event',
			priority: 'INFO'
		});

		await goto(resolve(redirectUrl), { invalidate: [key('public/event')] });
	}

	async function acceptInviteAndRegisterForEvent() {
		const accepted = await tryCatchAsync(() =>
			apiClient.AcceptInvite(undefined, {
				params: { conversation_id: conversationId, invite_id: invite.id }
			})
		);

		// Someone reopening an invite they already accepted is registered, so carry on.
		if (accepted.err !== null && !isInviteAlreadyAnswered(accepted.err)) {
			error = registrationFailureMessage(accepted.err);
			return;
		}

		await finishRegistration();
	}

	async function autoRegisterAndSignIn() {
		const registered = await tryCatchAsync(() =>
			apiClient.AutoRegisterEventAttendance(undefined, {
				params: { conversation_id: conversationId, invite_id: invite.id }
			})
		);

		if (registered.err !== null) {
			error = registrationFailureMessage(registered.err);
			return;
		}

		await invalidate(key('user'));
		await finishRegistration();
	}

	onMount(() => {
		if (user) {
			acceptInviteAndRegisterForEvent();
		} else {
			autoRegisterAndSignIn();
		}
	});
</script>

<svelte:head>
	<title>Event Invite - Comhairle</title>
</svelte:head>

<div class="flex flex-col items-center justify-center gap-4 py-20 text-center">
	{#if !error}
		<div class="flex h-full w-full items-center justify-center">
			<Spinner class="h-20 w-20" />
		</div>
	{:else}
		<div class="flex flex-col items-center gap-4 whitespace-pre-wrap">
			{#if error !== INVITE_EXPIRED}
				<h1 class="text-4xl font-semibold">Something went wrong</h1>
			{/if}
			<p class="text-lg">{error}</p>
			{#if !user && error === INVITE_EXPIRED}
				<Button
					href={`/auth/login-otp/send?backTo=${encodeURIComponent(`/conversations/${conversationId}/events/${eventId}`)}`}
					>Send a new link?</Button
				>
			{:else}
				<Button href="/">Return to home page</Button>
			{/if}
		</div>
	{/if}
</div>
