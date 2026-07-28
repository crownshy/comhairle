<script lang="ts">
	import { slide } from 'svelte/transition';
	import * as Table from '$lib/components/ui/table';
	import { ChevronDown } from 'lucide-svelte';
	import type { RankedProposal } from '@crownshy/api-client/api';
	import Crown from '$lib/components/icons/Crown.svelte';
	import { Progress } from '$lib/components/ui/progress';

	type Props = {
		proposals: RankedProposal[];
	};
	let { proposals }: Props = $props();

	const ROW_ANIMATION_DURATION_MS = 1000;
	let rowLimit = $state(4);
	let visibleRows = $derived(proposals.slice(0, rowLimit));
	let maxRating = $derived(proposals.length > 0 ? proposals[0].alignmentRating : 100);

	function getRatingPercentage(rating: number) {
		return (rating / maxRating) * 100;
	}
</script>

<Table.Root class="w-full table-fixed">
	<Table.Header>
		<Table.Row class="text-xs">
			<Table.Head class="text-muted-foreground w-[10%] px-5"
				><div class="flex justify-center">Ranking</div></Table.Head
			>
			<Table.Head class="text-muted-foreground w-2/5 px-5">Proposal</Table.Head>
			<Table.Head class="text-muted-foreground w-2/5 px-5">Alignment Score</Table.Head>
		</Table.Row>
	</Table.Header>
	<Table.Body>
		{#each visibleRows as proposal, index (proposal.id)}
			<Table.Row
				class="hover:[&,&>svelte-css-wrapper]:[&>th,td]:bg-accent hover:cursor-pointer"
			>
				<Table.Cell class="text-muted-foreground border-l-6 px-5 py-9">
					<div
						transition:slide={{ duration: ROW_ANIMATION_DURATION_MS }}
						class="flex justify-center"
					>
						{#if index === 0}
							<Crown />
						{:else}
							{index + 1}
						{/if}
					</div>
				</Table.Cell>
				<Table.Cell class="text-foreground h-auto px-5 text-wrap!">
					<div transition:slide={{ duration: ROW_ANIMATION_DURATION_MS }}>
						{proposal.title}
					</div>
				</Table.Cell>
				<Table.Cell class="text-primary h-auto px-5 text-wrap!">
					<div
						transition:slide={{ duration: ROW_ANIMATION_DURATION_MS }}
						class="flex items-center gap-4"
					>
						<Progress value={getRatingPercentage(proposal.alignmentRating)} />
						<span>{proposal.alignmentRating}</span>
					</div>
				</Table.Cell>
			</Table.Row>
		{/each}
	</Table.Body>
</Table.Root>

{#if rowLimit < proposals.length}
	<button
		class="bg-border flex w-full items-center justify-center gap-2 py-3 text-center"
		type="button"
		onclick={() => (rowLimit = proposals.length)}
		>See all {proposals.length} proposals <ChevronDown /></button
	>
{/if}
