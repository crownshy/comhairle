<script lang="ts">
	import * as Tabs from '$lib/components/ui/tabs';
	import { PrioritizationStore } from './store.svelte';
	import PollEditor from './PollEditor.svelte';
	import LiveResults from './LiveResults.svelte';
	import ReportEditor from './ReportEditor.svelte';

	type ConversationLike = {
		primaryLocale?: string | null;
		supportedLanguages?: string[] | null;
	};

	type Props = {
		conversationId?: string;
		conversation?: ConversationLike;
		isLive?: boolean;
		workflowStep: { id: string };
	};

	let { conversation, workflowStep }: Props = $props();

	let primaryLocale = $derived(conversation?.primaryLocale ?? 'en');
	let supportedLanguages = $derived(conversation?.supportedLanguages ?? ['en']);

	// Admin context: load proposals with full translation data so the editor
	// can drive `TranslatableField` against each proposal's TextContentId.
	let store = $derived(new PrioritizationStore(workflowStep.id, { isAdmin: true }));
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
			<PollEditor {store} {primaryLocale} {supportedLanguages} />
		</Tabs.Content>
		<Tabs.Content value="results">
			<LiveResults {store} />
		</Tabs.Content>
		<Tabs.Content value="report">
			<ReportEditor {store} />
		</Tabs.Content>
	</Tabs.Root>
</div>
