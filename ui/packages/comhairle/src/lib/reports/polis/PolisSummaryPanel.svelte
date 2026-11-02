<!--
	@component Trigger for the sensemaking-generated Polis summary (consensus /
	disagreement / per-group descriptions). Kicks off POST /tools/polis/summary,
	polls the returned job via `useJob`, and on completion invalidates the Polis
	summary key so the parent loader re-fetches the persisted result. Button-only:
	the generated text itself is rendered by the parent (AreaOfConsensus cards,
	OpinionGroups), not here.
-->
<script lang="ts">
	import type { WikiPollConversationSummary } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { notifications } from '$lib/notifications.svelte';
	import { useJob } from '$lib/hooks/use-job.svelte';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';
	import { LoadingButton } from '$lib/components/ui/button';
	import { Sparkles } from '@lucide/svelte';
	import { formatDistanceToNow } from 'date-fns';

	type Props = {
		workflowStepId: string;
		summary: WikiPollConversationSummary | null;
	};

	let { workflowStepId, summary }: Props = $props();

	let requesting = $state(false);
	const job = useJob({
		onComplete: () => invalidate(key('admin/polis/summary')),
		onError: (message) => notifications.send({ priority: 'ERROR', message })
	});

	const isWorking = $derived(requesting || job.isWorking);

	async function requestSummary() {
		requesting = true;

		const res = await tryCatchAsync(() =>
			apiClient.PolisRequestSummary({ workflow_step_id: workflowStepId })
		);

		requesting = false;

		if (res.err !== null) {
			console.error('PolisRequestSummary failed', res.err);
			notifications.send({ priority: 'ERROR', message: 'Failed to start Polis summary job' });
			return;
		}

		job.start(res.ok.job_id);
	}
</script>

<div class="flex flex-col items-stretch gap-1">
	<LoadingButton class="w-full" size="sm" loading={isWorking} onclick={requestSummary}>
		<Sparkles class="size-4" />
		{summary ? 'Regenerate summary' : 'Generate summary'}
	</LoadingButton>
	{#if isWorking}
		<p class="text-muted-foreground text-center text-xs">
			Generating summary{job.status ? ` (${job.status})` : ''}…
		</p>
	{:else if summary}
		<p class="text-muted-foreground text-center text-xs">
			Last generated {formatDistanceToNow(summary.created_at, { addSuffix: true })}
		</p>
	{/if}
</div>
