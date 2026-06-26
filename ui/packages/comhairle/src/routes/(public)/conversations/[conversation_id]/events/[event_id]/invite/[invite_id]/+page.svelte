<script lang="ts">
	import { goto, invalidateAll } from '$app/navigation';
	import { resolve } from '$app/paths';
	import Button from '$lib/components/ui/button/button.svelte';
	import Spinner from '$lib/components/ui/spinner/spinner.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { onMount } from 'svelte';

	let { data } = $props();
	const { user, invite, eventId, conversationId, error: serverError } = data;

	const defaultErrorMessage = 'An error occurred registering you for this event.';

	let error = $state<string | null>(serverError ? serverError : null);

	const redirectUrl = `/conversations/${conversationId}/events/${eventId}`;

	async function acceptInviteAndRegisterForEvent() {
		try {
			try {
				await apiClient.CreateEventAttendance(
					{ role: 'participant' },
					{ params: { conversation_id: conversationId, event_id: eventId } }
				);
			} catch (e) {
				// silently ignore conflicts in case user is already registered
				if (e.response.status !== 409) throw new Error(defaultErrorMessage);
			}

			try {
				await apiClient.AcceptInvite(undefined, {
					params: { conversation_id: conversationId, invite_id: invite.id }
				});
			} catch (e) {
				// silently ignore conflicts in case invite already accepted
				if (e.response.status !== 409) throw new Error(defaultErrorMessage);
			}

			notifications.send({
				message: 'You have successfully been registered for the event',
				priority: 'INFO'
			});

			await goto(resolve(redirectUrl), { invalidateAll: true });
		} catch (e) {
			console.error(e);
			error = e.message;
		}
	}

	async function autoRegisterAndSignIn() {
		try {
			await apiClient.AutoRegisterEventAttendance(undefined, {
				params: { conversation_id: conversationId, invite_id: invite.id }
			});

			notifications.send({
				message: 'You have successfully been registered for the event',
				priority: 'INFO'
			});

			await goto(resolve(redirectUrl), { invalidateAll: true });
		} catch (e) {
			console.error(e);
			if (
				e.response.status === 409 &&
				e.response?.data?.err ===
					'An invite response has already been created for this invite by this user'
			) {
				error = 'Invite expired';
			} else {
				error = defaultErrorMessage;
			}
		}
	}

	onMount(() => {
		// TODO: after accepting the invite do we send them an email with a link to the email, just redirect or both?
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
			{#if error !== 'Invite expired'}
				<h1 class="text-4xl font-semibold">Something went wrong</h1>
			{/if}
			<p class="text-lg">{error}</p>
			{#if !user && error === 'Invite expired'}
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
