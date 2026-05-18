<script lang="ts">
	import * as Tabs from '$lib/components/ui/tabs';
	import { PrioritisationStore } from './store.svelte';
	import PollEditor from './PollEditor.svelte';
	import LiveResults from './LiveResults.svelte';
	import ReportEditor from './ReportEditor.svelte';

	type Props = {
		conversationId?: string;
		conversation?: unknown;
		isLive?: boolean;
		workflowStep: { id: string };
	};

	let { workflowStep }: Props = $props();

	let store = $derived(new PrioritisationStore(workflowStep.id));
	let active = $state<'poll' | 'results' | 'report'>('poll');
</script>

<div class="flex flex-col gap-4">
	<Tabs.Root value={active} onValueChange={(v) => (active = v as typeof active)} class="w-full">
		<Tabs.List>
			<Tabs.Trigger value="poll">Poll</Tabs.Trigger>
			<Tabs.Trigger value="results">Results</Tabs.Trigger>
			<Tabs.Trigger value="report">Report</Tabs.Trigger>
		</Tabs.List>

		<Tabs.Content value="poll">
			<PollEditor {store} />
		</Tabs.Content>
		<Tabs.Content value="results">
			<LiveResults {store} />
		</Tabs.Content>
		<Tabs.Content value="report">
			<ReportEditor {store} />
		</Tabs.Content>
	</Tabs.Root>
</div>
