<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { Label } from '$lib/components/ui/label';
	import { apiClient } from '@crownshy/api-client/client';
	import { key } from '$lib/utils/invalidationKey';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Skeleton } from '$lib/components/ui/skeleton';

	const { data, params } = $props();
	const { streamedAttendees, streamedPendingInvites } = $derived(data);

	async function handleSetAttendeeRole(attendanceId: string, role: string) {
		const result = await tryCatchAsync(() =>
			apiClient.UpdateEventAttendance(
				{ role },
				{
					params: {
						conversation_id: params.conversation_id,
						event_id: params.event_id,
						attendance_id: attendanceId
					}
				}
			)
		);

		if (result.err !== null) {
			console.error(result.err);
			notifications.send({
				priority: 'ERROR',
				message: 'Failed to update role'
			});
		}

		notifications.send({
			priority: 'INFO',
			message: 'Role updated'
		});

		await invalidate(key('event/facilitators'));
	}

	/** Roles an admin can assign, in display order. 'participant' is the plain,
	 *  non-privileged role a registered attendee gets by default. */
	const ROLES = [
		{ value: 'participant', label: 'Attendee' },
		{ value: 'facilitator', label: 'Facilitator' },
		{ value: 'moderator', label: 'Moderator' }
	];
</script>

<div class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6">
	<div class="contents">
		<Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">Facilitators</Label>
		<div class="flex w-full flex-col gap-6">
			<section class="flex flex-col gap-3">
				<h3 class="text-sm font-semibold">Registered attendees</h3>
				{#await streamedAttendees}
					<Skeleton class="h-11 w-full" />
				{:then attendees}
					{#if attendees.err !== null}
						<p class="text-muted-foreground text-sm">
							Error retrieving attendees please try again
						</p>
					{:else if attendees.ok.length === 0}
						<p class="text-muted-foreground text-sm">
							No one has registered for this event yet.
						</p>
					{:else}
						<ul class="flex flex-col divide-y">
							{#each attendees.ok as attendee (attendee.id)}
								<li
									class="flex flex-col gap-2 py-3 sm:flex-row sm:items-center sm:justify-between"
								>
									<span class="text-sm"
										>{attendee.email?.trim() ?? attendee.userId.slice(8)}</span
									>
									<div class="flex gap-1">
										{#each ROLES as role (role.value)}
											<Button
												size="sm"
												variant={attendee.role === role.value
													? 'default'
													: 'outline'}
												aria-pressed={attendee.role === role.value}
												disabled={attendee.role === role.value}
												onclick={() =>
													handleSetAttendeeRole(attendee.id, role.value)}
											>
												{role.label}
											</Button>
										{/each}
									</div>
								</li>
							{/each}
						</ul>
					{/if}
				{/await}
			</section>

			<section class="flex flex-col gap-3">
				<h3 class="text-sm font-semibold">Invited (not registered yet)</h3>
				<p class="text-muted-foreground text-xs">
					These people have been invited but haven't registered, so a role can't be
					assigned until they join.
				</p>
				{#await streamedPendingInvites}
					<Skeleton class="h-10 w-full" />
				{:then pendingInvites}
					{#if pendingInvites.err !== null}
						<p>Error loading pending invites please try again</p>
					{:else if pendingInvites.ok.length === 0}
						<p class="text-muted-foreground pt-3 text-xs">No pending invites</p>
					{:else}
						<ul class="flex flex-col divide-y opacity-60">
							{#each pendingInvites.ok as invite (invite.id)}
								<li class="flex items-center justify-between py-3">
									<span class="text-sm">{invite.email}</span>
									<span class="text-muted-foreground text-xs capitalize"
										>{invite.status}</span
									>
								</li>
							{/each}
						</ul>
					{/if}
				{/await}
			</section>
		</div>
	</div>
</div>
