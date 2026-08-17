<script lang="ts">
	import { onMount } from 'svelte';
	import { Shuffle, Pencil, Crown, Clock } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import type { BreakoutPlanDto, EventAttendanceDto } from '@crownshy/api-client/api';
	import CreateBreakoutDialog from '$lib/components/LiveEvent/CreateBreakoutDialog.svelte';
	import type { VideoCallParticipant } from '$lib/services/videoCallService.svelte';

	type Props = {
		conversation_id: string;
		event_id: string;
		attendees: (EventAttendanceDto & { email?: string })[];
	};

	let { conversation_id, event_id, attendees }: Props = $props();

	let plan = $state<BreakoutPlanDto | null>(null);
	let loading = $state(true);
	let seeding = $state(false);
	let saving = $state(false);
	let dialogOpen = $state(false);

	/** Synthetic id for a reserved (not-yet-signed-up) invite seat. */
	const PENDING_PREFIX = 'invite:';

	async function loadPlan() {
		loading = true;
		try {
			plan = await apiClient.GetEventBreakoutPlan({
				params: { conversation_id, event_id }
			});
		} catch (e) {
			console.error(e);
			notifications.send({ message: 'Failed to load breakout plan', priority: 'ERROR' });
		} finally {
			loading = false;
		}
	}

	onMount(loadPlan);

	async function handleSeed() {
		seeding = true;
		try {
			plan = await apiClient.SeedEventBreakoutPlan(undefined, {
				params: { conversation_id, event_id }
			});
			notifications.send({ message: 'Breakout rooms auto-assigned', priority: 'INFO' });
		} catch (e) {
			console.error(e);
			notifications.send({ message: 'Failed to auto-assign rooms', priority: 'ERROR' });
		} finally {
			seeding = false;
		}
	}

	/** Plan seats → dialog participant pills (one pill per seat). */
	let dialogAssignments = $derived(
		(plan?.rooms ?? []).map((room) =>
			room.seats.map(
				(seat): VideoCallParticipant => ({
					user_id: seat.userId ?? `${PENDING_PREFIX}${seat.inviteId}`,
					username: seat.label,
					role: seat.isModerator ? 'moderator' : 'participant'
				})
			)
		)
	);

	let dialogModeratorIds = $derived(
		(plan?.rooms ?? [])
			.flatMap((room) => room.seats)
			.filter((seat) => seat.isModerator)
			.map((seat) => seat.userId ?? `${PENDING_PREFIX}${seat.inviteId}`)
	);

	let totalSeats = $derived(
		(plan?.rooms ?? []).reduce((sum, room) => sum + room.seats.length, 0)
	);

	async function handleSave(config: {
		roomAssignments: VideoCallParticipant[][];
		moderatorIds: string[];
	}) {
		saving = true;
		const mods = new Set(config.moderatorIds);
		const rooms = config.roomAssignments.map((room) => ({
			seats: room.map((p) => {
				const isPending = p.user_id.startsWith(PENDING_PREFIX);
				return {
					user_id: isPending ? null : p.user_id,
					invite_id: isPending ? p.user_id.slice(PENDING_PREFIX.length) : null,
					is_moderator: mods.has(p.user_id)
				};
			})
		}));

		try {
			plan = await apiClient.SaveEventBreakoutPlan(
				{ rooms },
				{ params: { conversation_id, event_id } }
			);
			notifications.send({ message: 'Breakout plan saved', priority: 'INFO' });
		} catch (e) {
			console.error(e);
			notifications.send({ message: 'Failed to save breakout plan', priority: 'ERROR' });
		} finally {
			saving = false;
		}
	}
</script>

<div class="flex flex-col gap-6 py-6">
	<div class="flex flex-col gap-2">
		<h2 class="text-3xl font-bold">Breakout rooms</h2>
		<p class="text-muted-foreground text-sm">
			Pre-assign people to breakout rooms before the call starts. Reserved slots are filled in
			automatically as invitees sign up, and you can still rebalance live during the call.
		</p>
	</div>

	<div class="flex flex-wrap items-center gap-3">
		<Button variant="outline" class="gap-2" onclick={handleSeed} disabled={seeding}>
			<Shuffle class="h-4 w-4" />
			{seeding ? 'Assigning…' : 'Auto-assign'}
		</Button>
		<Button
			variant="outline"
			class="gap-2"
			onclick={() => (dialogOpen = true)}
			disabled={totalSeats === 0}
		>
			<Pencil class="h-4 w-4" />
			Edit rooms
		</Button>
	</div>

	{#if loading}
		<p class="text-muted-foreground text-sm">Loading…</p>
	{:else if !plan || plan.rooms.length === 0}
		<div class="flex items-center justify-center rounded-2xl border border-dashed p-8">
			<p class="text-muted-foreground text-sm">
				No breakout plan yet. Use <span class="font-medium">Auto-assign</span> to build one from
				the current attendees and invites.
			</p>
		</div>
	{:else}
		<div class="flex flex-col gap-3">
			{#each plan.rooms as room, i}
				<div class="bg-muted flex items-start gap-3 rounded-lg border p-4">
					<div class="text-foreground w-24 shrink-0 text-base font-semibold">
						Room #{i + 1}
					</div>
					<div class="flex flex-1 flex-wrap gap-2">
						{#each room.seats as seat}
							<span
								class="bg-background inline-flex items-center gap-1.5 rounded-full px-2 py-1 text-sm shadow-sm {seat.pending
									? 'opacity-60'
									: ''}"
							>
								{#if seat.isModerator}
									<Crown class="h-3.5 w-3.5 text-amber-500" />
								{/if}
								<span class="text-foreground font-medium">{seat.label}</span>
								{#if seat.pending}
									<Clock class="text-muted-foreground h-3.5 w-3.5" />
								{/if}
							</span>
						{/each}
						{#if room.seats.length === 0}
							<span class="text-muted-foreground text-sm">Empty</span>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<CreateBreakoutDialog
	bind:open={dialogOpen}
	participants={attendees.map((a) => ({
		user_id: a.userId,
		username: a.email ?? a.userId,
		role: a.role
	}))}
	initialAssignments={dialogAssignments}
	moderatorIds={dialogModeratorIds}
	enableModerators={true}
	showDuration={false}
	confirmLabel={saving ? 'Saving…' : 'Save plan'}
	onClose={() => (dialogOpen = false)}
	onCreate={handleSave}
/>
