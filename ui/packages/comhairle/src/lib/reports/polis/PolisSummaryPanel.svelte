<!--
	@component Trigger + result panel for the sensemaking-generated Polis summary
	(consensus / disagreement / per-group descriptions). Kicks off POST
	/tools/polis/summary, polls the returned job via GET /jobs/{job_id}, and on
	completion invalidates `polis:summary` so the parent loader re-fetches the
	persisted result through `summary`. `summary` is the last-generated row (or
	null if none exists yet); this component never fabricates a result, it only
	renders what the server already generated.
-->
<script lang="ts">
	import type { WikiPollConversationSummary } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidate } from '$app/navigation';
	import * as Card from '$lib/components/ui/card';
	import { LoadingButton } from '$lib/components/ui/button';
	import { Sparkles } from '@lucide/svelte';

	type Props = {
		workflowStepId: string;
		summary: WikiPollConversationSummary | null;
	};

	let { workflowStepId, summary }: Props = $props();

	type SummaryResult = {
		consensus_description?: string;
		disagrement_description?: string;
		group_descriptions?: Record<
			string,
			{ name?: string; description?: string; unique_statement?: string }
		>;
	};

	const result = $derived((summary?.result ?? null) as SummaryResult | null);
	const groupEntries = $derived(Object.entries(result?.group_descriptions ?? {}));

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
		if (!jobId || jobStatus === 'completed') return;
		const timeout = setTimeout(poll, 5000);
		return () => clearTimeout(timeout);
	});
</script>

<div class="flex flex-col gap-4">
	<div class="flex flex-wrap items-center justify-between gap-3">
		<h2 class="text-foreground text-xl font-semibold">AI summary</h2>
		<LoadingButton size="sm" loading={isWorking} onclick={requestSummary}>
			<Sparkles class="size-4" />
			{summary ? 'Regenerate summary' : 'Generate summary'}
		</LoadingButton>
	</div>

	{#if isWorking}
		<p class="text-muted-foreground text-base">
			Generating summary{jobStatus ? ` (${jobStatus})` : ''}…
		</p>
	{/if}

	{#if result}
		<Card.Root class="gap-4 rounded-md p-4 shadow-none">
			{#if result.consensus_description}
				<div>
					<p class="text-muted-foreground text-sm">AREA OF CONSENSUS</p>
					<p class="text-foreground text-base leading-7">
						{result.consensus_description}
					</p>
				</div>
			{/if}
			{#if result.disagrement_description}
				<div>
					<p class="text-muted-foreground text-sm">AREA OF DISAGREEMENT</p>
					<p class="text-foreground text-base leading-7">
						{result.disagrement_description}
					</p>
				</div>
			{/if}
			{#if groupEntries.length > 0}
				<div class="flex flex-col gap-3">
					<p class="text-muted-foreground text-sm">OPINION GROUPS</p>
					{#each groupEntries as [groupId, description] (groupId)}
						<div class="bg-accent border-border rounded-[10px] border p-3">
							<p class="text-foreground font-medium">
								{description.name ?? `Group ${groupId}`}
							</p>
							{#if description.description}
								<p class="text-primary text-base leading-6">
									{description.description}
								</p>
							{/if}
						</div>
					{/each}
				</div>
			{/if}
		</Card.Root>
	{:else if !isWorking}
		<p class="text-muted-foreground text-base">
			No AI summary yet. Generate one once there are enough votes and statements.
		</p>
	{/if}
</div>
