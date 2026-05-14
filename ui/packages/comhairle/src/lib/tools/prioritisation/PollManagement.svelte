<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Badge } from '$lib/components/ui/badge';
	import type { PrioritisationStore } from './store.svelte';

	let { store }: { store: PrioritisationStore } = $props();

	let now = $state(Date.now());
	$effect(() => {
		const t = setInterval(() => (now = Date.now()), 1000);
		return () => clearInterval(t);
	});

	let timeLeft = $derived(store.timeLeftSeconds(now));

	function fmtTime(s: number | null): string {
		if (s === null) return '∞';
		const m = Math.floor(s / 60)
			.toString()
			.padStart(2, '0');
		const sec = Math.floor(s % 60)
			.toString()
			.padStart(2, '0');
		return `${m}:${sec}`;
	}

	let confirmEnd = $state(false);
</script>

<div class="flex flex-col gap-4">
	<Card.Root>
		<Card.Header>
			<Card.Title>Poll overview</Card.Title>
		</Card.Header>
		<Card.Content class="grid gap-3 sm:grid-cols-2">
			<div>
				<div class="text-muted-foreground text-xs">Poll title</div>
				<div class="font-medium">{store.poll.title || 'Untitled'}</div>
			</div>
			<div>
				<div class="text-muted-foreground text-xs">Status</div>
				<Badge>{store.poll.state}</Badge>
			</div>
			<div>
				<div class="text-muted-foreground text-xs">Time left</div>
				<div class="font-mono">{fmtTime(timeLeft)}</div>
			</div>
			<div>
				<div class="text-muted-foreground text-xs">Submissions</div>
				<div class="font-medium">{store.submissions.length} completed</div>
			</div>
		</Card.Content>
		<Card.Footer class="flex flex-wrap gap-2">
			{#if store.poll.state === 'published'}
				<Button variant="outline" onclick={() => store.pause()}>Pause the poll</Button>
			{:else if store.poll.state === 'paused'}
				<Button onclick={() => store.resume()}>Resume the poll</Button>
			{/if}
			<Button
				variant="destructive"
				onclick={() => (confirmEnd = true)}
				disabled={store.poll.state === 'ended' || store.poll.state === 'draft'}
			>
				End the poll
			</Button>
		</Card.Footer>
	</Card.Root>

	<Card.Root>
		<Card.Header>
			<Card.Title>Invite participants</Card.Title>
			<Card.Description>
				Show the QR code (see the QR / Join tab) or share the join code.
			</Card.Description>
		</Card.Header>
		<Card.Content>
			<div class="flex items-center gap-3">
				<span class="text-muted-foreground text-sm">Join code:</span>
				<span class="bg-muted rounded-md px-3 py-1 font-mono text-lg font-semibold"
					>{store.poll.joinCode}</span
				>
			</div>
		</Card.Content>
	</Card.Root>
</div>

<AlertDialog.Root bind:open={confirmEnd}>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>End this poll?</AlertDialog.Title>
			<AlertDialog.Description>
				Participants will no longer be able to submit answers. This cannot be undone.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action onclick={() => store.end()}>End poll</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
