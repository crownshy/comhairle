<script lang="ts">
	import { onMount } from 'svelte';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { m } from '$lib/paraglide/messages';
	import { apiClient } from '@crownshy/api-client/client';
	import { type PublishedReason } from '@crownshy/api-client/api';
	import { tryCatchAsync } from '$lib/utils/errorHandling';

	type Props = { workflowStepId: string; isPreview: boolean };

	let { workflowStepId, isPreview }: Props = $props();

	let reasons = $state<PublishedReason[] | null>(null);
	let loadFailed = $state(false);

	onMount(async () => {
		const result = await tryCatchAsync(() =>
			apiClient.PolisGetPublishedModerationPolicy({
				queries: { workflow_step_id: workflowStepId, preview: isPreview }
			})
		);
		if (result.err !== null) {
			loadFailed = true;
			return;
		}
		reasons = result.ok;
	});
</script>

<div class="flex flex-col gap-4">
	<p class="text-card-foreground text-base">{m.polis_moderation_policy_intro()}</p>

	{#if loadFailed}
		<p class="text-destructive text-base">{m.something_went_wrong()}</p>
	{:else if reasons === null}
		<div class="flex flex-col gap-3">
			<Skeleton class="h-12 w-full" />
			<Skeleton class="h-12 w-full" />
			<Skeleton class="h-12 w-full" />
		</div>
	{:else}
		<ul class="flex flex-col gap-3">
			{#each reasons as reason (reason.label)}
				<li class="text-card-foreground text-base">
					<p class="font-semibold">{reason.label}</p>
					{#if reason.description}
						<p class="text-muted-foreground">{reason.description}</p>
					{/if}
				</li>
			{/each}
		</ul>
	{/if}

	<p class="text-card-foreground text-base">{m.polis_moderation_policy_outcome()}</p>
</div>
