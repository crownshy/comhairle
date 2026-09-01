<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import type { EventAttendanceEtx } from '@crownshy/api-client/api';
	import type { PendingInvite } from './types';

	type Attendee = EventAttendanceEtx;

	type Props = {
		attendees: Attendee[];
		pendingInvites: PendingInvite[];
		/** Set a registered attendee's role by attendance id. */
		onSetRole: (attendanceId: string, role: string) => void;
	};

	let { attendees, pendingInvites, onSetRole }: Props = $props();

	/** Roles an admin can assign, in display order. 'participant' is the plain,
	 *  non-privileged role a registered attendee gets by default. */
	const ROLES = [
		{ value: 'participant', label: 'Attendee' },
		{ value: 'facilitator', label: 'Facilitator' },
		{ value: 'moderator', label: 'Moderator' }
	];

	function displayName(a: Attendee): string {
		return a.email && a.email.trim() ? a.email : a.userId.slice(0, 8);
	}
</script>

<div class="flex w-full flex-col gap-6">
	<section class="flex flex-col gap-3">
		<h3 class="text-sm font-semibold">Registered attendees</h3>
		{#if attendees.length === 0}
			<p class="text-muted-foreground text-sm">No one has registered for this event yet.</p>
		{:else}
			<ul class="flex flex-col divide-y">
				{#each attendees as attendee (attendee.id)}
					<li
						class="flex flex-col gap-2 py-3 sm:flex-row sm:items-center sm:justify-between"
					>
						<span class="text-sm">{displayName(attendee)}</span>
						<div class="flex gap-1">
							{#each ROLES as role (role.value)}
								<Button
									size="sm"
									variant={attendee.role === role.value ? 'default' : 'outline'}
									aria-pressed={attendee.role === role.value}
									disabled={attendee.role === role.value}
									onclick={() => onSetRole(attendee.id, role.value)}
								>
									{role.label}
								</Button>
							{/each}
						</div>
					</li>
				{/each}
			</ul>
		{/if}
	</section>

	{#if pendingInvites.length > 0}
		<section class="flex flex-col gap-3">
			<h3 class="text-sm font-semibold">Invited (not registered yet)</h3>
			<p class="text-muted-foreground text-xs">
				These people have been invited but haven't registered, so a role can't be assigned
				until they join.
			</p>
			<ul class="flex flex-col divide-y opacity-60">
				{#each pendingInvites as invite (invite.id)}
					<li class="flex items-center justify-between py-3">
						<span class="text-sm">{invite.email}</span>
						<span class="text-muted-foreground text-xs capitalize">{invite.status}</span
						>
					</li>
				{/each}
			</ul>
		</section>
	{/if}
</div>
