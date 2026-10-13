<!--
	@component Trigger for the sensemaking-generated Polis summary (consensus /
	disagreement / per-group descriptions). Kicks off POST /tools/polis/summary,
	polls the returned job via GET /jobs/{job_id}, and on completion invalidates
	`polis:summary` so the parent loader re-fetches the persisted result. Button-only:
	the generated text itself is rendered by the parent (AreaOfConsensus cards,
	OpinionGroups), not here.
-->
<script lang="ts">
	import type { WikiPollConversationSummary } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidate } from '$app/navigation';
	import { LoadingButton } from '$lib/components/ui/button';
	import { Sparkles } from '@lucide/svelte';
	import { formatDistanceToNow } from 'date-fns';

	type Props = {
		workflowStepId: string;
		summary: WikiPollConversationSummary | null;
	};

	let { workflowStepId, summary }: Props = $props();

	let requesting = $state(false);
	let jobId = $state<string | null>(null);
	let jobStatus = $state<string | null>(null);
	let jobError = $state<string | null>(null);

	const isWorking = $derived(requesting || (jobId !== null && jobStatus !== 'completed'));

	async function requestSummary() {
		requesting = true;
		jobError = null;

		const res = await tryCatchAsync(() =>
			apiClient.PolisRequestSummary({ workflow_step_id: workflowStepId })
		);

		requesting = false;

		if (res.err !== null) {
			console.error('PolisRequestSummary failed', res.err);
			notifications.send({ priority: 'ERROR', message: 'Failed to start Polis summary job' });
			return;
		}

		jobId = res.ok.job_id;
		jobStatus = 'pending';
	}

	async function poll() {
		if (!jobId) return;

		const res = await tryCatchAsync(() => apiClient.GetJob({ params: { job_id: jobId! } }));

		if (res.err !== null) {
			console.error('GetJob failed', res.err);
			return;
		}

		jobStatus = res.ok.status ?? null;

		if (jobStatus === 'completed') {
			jobId = null;
			await invalidate('polis:summary');
			return;
		}

		if (jobStatus === 'error') {
			jobError = res.ok.error ?? 'Summary generation failed';
			notifications.send({ priority: 'ERROR', message: jobError });
			jobId = null;
		}
	}

	$effect(() => {
		if (!jobId || jobStatus !== 'pending') return;
		const timeout = setTimeout(poll, 5000);
		return () => clearTimeout(timeout);
	});
</script>

<div class="flex flex-col items-stretch gap-1">
	<LoadingButton class="w-full" size="sm" loading={isWorking} onclick={requestSummary}>
		<Sparkles class="size-4" />
		{summary ? 'Regenerate summary' : 'Generate summary'}
	</LoadingButton>
	{#if isWorking}
		<p class="text-muted-foreground text-center text-xs">
			Generating summary{jobStatus ? ` (${jobStatus})` : ''}…
		</p>
	{:else if summary}
		<p class="text-muted-foreground text-center text-xs">
			Last generated {formatDistanceToNow(summary.created_at, { addSuffix: true })}
		</p>
	{/if}
</div>
