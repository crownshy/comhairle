<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import {
		conversationCount,
		usageForVersion,
		usageFollowingLatest,
		type DemographicForm
	} from './demographicPrototypeData';

	type Props = { form: DemographicForm; viewing: 'draft' | number };

	let { form, viewing }: Props = $props();

	const stageLabels = { draft: 'Draft', live: 'Live', closed: 'Closed' } as const;

	const list = $derived(
		viewing === 'draft' ? usageFollowingLatest(form) : usageForVersion(form, viewing)
	);
	const pinnedElsewhere = $derived(conversationCount(form) - list.length);
</script>

<div class="border-border flex flex-col gap-3 border-t pt-4">
	<div class="flex flex-col gap-1">
		<h3 class="text-base font-semibold">
			{viewing === 'draft' ? 'Publishing will reach' : `Used in v${viewing}`}
			({list.length})
		</h3>
		{#if viewing === 'draft'}
			<p class="text-muted-foreground text-sm">
				Conversations that follow the latest version pick up the next publish.
				{#if pinnedElsewhere > 0}
					{pinnedElsewhere}
					{pinnedElsewhere === 1 ? 'is' : 'are'} pinned and will not change.
				{/if}
			</p>
		{/if}
	</div>

	{#if list.length === 0}
		<p class="text-muted-foreground text-base">
			{viewing === 'draft'
				? 'No conversation follows the latest version.'
				: `No conversation uses v${viewing}.`}
		</p>
	{:else}
		<ul class="flex flex-col gap-2">
			{#each list as usage (usage.conversationId)}
				<li
					class="border-border bg-card flex items-start justify-between gap-2 rounded-lg border px-3 py-2"
				>
					<div class="flex min-w-0 flex-col">
						<span class="truncate text-base">{usage.title}</span>
						<span class="text-muted-foreground text-sm">
							{usage.pinnedVersion === null
								? 'Follows latest'
								: `Pinned to v${usage.pinnedVersion}`}
						</span>
					</div>
					<Badge variant={usage.stage === 'live' ? 'primary' : 'draft'} class="shrink-0">
						{stageLabels[usage.stage]}
					</Badge>
				</li>
			{/each}
		</ul>
	{/if}
</div>
