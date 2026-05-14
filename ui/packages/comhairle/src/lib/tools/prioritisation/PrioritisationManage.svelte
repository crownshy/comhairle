<script lang="ts">
	import * as Tabs from '$lib/components/ui/tabs';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { ExternalLink } from 'lucide-svelte';
	import { PrioritisationStore } from './store.svelte';
	import PollEditor from './PollEditor.svelte';
	import ParticipantPreview from './ParticipantPreview.svelte';
	import PollManagement from './PollManagement.svelte';
	import LiveResults from './LiveResults.svelte';
	import ReportEditor from './ReportEditor.svelte';
	import QRJoin from './QRJoin.svelte';

	type Props = {
		conversationId?: string;
		conversation?: unknown;
		workflowStep: { id: string; name?: string };
		isLive?: boolean;
	};

	let { workflowStep }: Props = $props();

	const store = new PrioritisationStore(workflowStep.id);
	let activeTab = $state<'edit' | 'preview' | 'manage' | 'results' | 'report' | 'qr'>('edit');

	function setTab(v: string) {
		activeTab = v as typeof activeTab;
	}

	let stateBadgeVariant = $derived.by(() => {
		switch (store.poll.state) {
			case 'published':
				return 'default' as const;
			case 'paused':
				return 'secondary' as const;
			case 'ended':
				return 'outline' as const;
			default:
				return 'secondary' as const;
		}
	});
</script>

<div class="flex w-full flex-col gap-4">
	<div class="flex items-center justify-between gap-4">
		<div class="flex items-center gap-3">
			<h2 class="text-xl font-semibold">Prioritisation Poll</h2>
			<Badge variant={stateBadgeVariant}>{store.poll.state}</Badge>
			{#if store.poll.joinCode}
				<span class="text-muted-foreground text-sm">
					Join code: <span class="font-mono font-semibold">{store.poll.joinCode}</span>
				</span>
			{/if}
		</div>
		<div class="flex items-center gap-2">
			{#if store.poll.state === 'draft'}
				<Button
					onclick={() => {
						const r = store.publish();
						if (!r.ok) alert(r.reason);
						else activeTab = 'manage';
					}}
					disabled={store.validatePublish().length > 0}
				>
					Publish
				</Button>
			{:else if store.poll.state === 'published' || store.poll.state === 'paused'}
				<Button variant="outline" onclick={() => store.unpublish()}>Unpublish</Button>
			{/if}
		</div>
	</div>

	{#if store.validatePublish().length > 0 && store.poll.state === 'draft'}
		<div class="bg-muted text-muted-foreground rounded-md p-3 text-sm">
			Cannot publish yet: {store.validatePublish().join('; ')}
		</div>
	{/if}

	<Tabs.Root value={activeTab} onValueChange={setTab}>
		<Tabs.List>
			<Tabs.Trigger value="edit">Edit</Tabs.Trigger>
			<Tabs.Trigger value="preview">Preview</Tabs.Trigger>
			<Tabs.Trigger value="manage">Manage</Tabs.Trigger>
			<Tabs.Trigger value="results">Results</Tabs.Trigger>
			<Tabs.Trigger value="qr">QR / Join</Tabs.Trigger>
			<Tabs.Trigger value="report">Report</Tabs.Trigger>
		</Tabs.List>

		<Tabs.Content value="edit">
			<PollEditor {store} />
		</Tabs.Content>

		<Tabs.Content value="preview">
			<div class="mb-3 flex items-center justify-between">
				<p class="text-muted-foreground text-sm">
					Read-only preview of what participants will see.
				</p>
			</div>
			<ParticipantPreview {store} />
		</Tabs.Content>

		<Tabs.Content value="manage">
			<PollManagement {store} />
		</Tabs.Content>

		<Tabs.Content value="results">
			<LiveResults {store} />
		</Tabs.Content>

		<Tabs.Content value="qr">
			<QRJoin {store} />
		</Tabs.Content>

		<Tabs.Content value="report">
			<ReportEditor {store} />
		</Tabs.Content>
	</Tabs.Root>
</div>
