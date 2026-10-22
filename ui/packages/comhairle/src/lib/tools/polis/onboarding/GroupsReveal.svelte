<script lang="ts">
	import { m } from '$lib/paraglide/messages';
	import type { ReportComment } from '../reportTypes';
	import type { EarlySnapshot, GroupPortrait } from './onboarding';

	type Props = {
		portrait: GroupPortrait | undefined;
		snapshot: EarlySnapshot;
	};

	let { portrait, snapshot }: Props = $props();

	const largest = $derived(Math.max(1, ...(portrait?.groups.map((g) => g.size) ?? [])));
</script>

{#snippet leaning(heading: string, comments: ReportComment[])}
	{#if comments.length > 0}
		<div class="flex flex-col gap-2">
			<p class="text-muted-foreground text-base">{heading}</p>
			{#each comments as comment (comment.tid)}
				<p class="bg-card rounded-xl border p-3 text-base leading-snug">"{comment.text}"</p>
			{/each}
		</div>
	{/if}
{/snippet}

{#if portrait}
	<div class="flex w-full flex-col gap-6">
		<p class="text-card-foreground text-lg">{m.polis_onboarding_groups_body()}</p>
		<ul class="flex w-full flex-col gap-3">
			{#each portrait.groups as group (group.groupId)}
				<li class="flex items-center gap-3">
					<div class="bg-muted h-8 min-w-0 flex-1 overflow-hidden rounded-full">
						<div
							class={group.isYours ? 'bg-primary h-full' : 'bg-foreground/25 h-full'}
							style="width: {(group.size / largest) * 100}%"
						></div>
					</div>
					<span class="w-32 shrink-0 text-base">
						{m.polis_groups_people({ count: group.size })}
						{#if group.isYours}
							<span class="text-primary font-semibold">· {m.polis_groups_you()}</span>
						{/if}
					</span>
				</li>
			{/each}
		</ul>
		{@render leaning(m.polis_groups_agreed(), portrait.agreed)}
		{@render leaning(m.polis_groups_disagreed(), portrait.disagreed)}
		<p class="text-muted-foreground text-base">{m.polis_onboarding_groups_changes()}</p>
	</div>
{:else}
	<div class="flex w-full flex-col gap-4">
		<p class="text-card-foreground text-lg">{m.polis_onboarding_no_groups_body()}</p>
		{#if snapshot.votes > 0}
			<p class="text-muted-foreground text-base">
				{m.polis_onboarding_so_far({ people: snapshot.people, votes: snapshot.votes })}
			</p>
		{/if}
		{@render leaning(m.polis_onboarding_most_agreed(), snapshot.mostAgreed)}
	</div>
{/if}
