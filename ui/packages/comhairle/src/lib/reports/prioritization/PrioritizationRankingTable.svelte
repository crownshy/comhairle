<script lang="ts">
	import { slide } from 'svelte/transition';
	import { ChevronDown } from 'lucide-svelte';
	import * as Table from '$lib/components/ui/table';
	import Crown from '$lib/components/icons/Crown.svelte';
	import DivergingProgress from '../DivergingProgress.svelte';

	export type RankingRow = {
		key: string;
		title: string;
		/** `null` when too few reviewers answered to show a score. */
		alignment: number | null;
	};

	type Props = {
		rows: RankingRow[];
		/** The shared scale every row's bar is drawn against. */
		domain: [number, number];
		scoreHeading: string;
	};

	let { rows, domain, scoreHeading }: Props = $props();

	const ROWS_SHOWN_FIRST = 4;
	const ROW_ANIMATION_DURATION_MS = 1000;
	let rowLimit = $state(ROWS_SHOWN_FIRST);
	let visibleRows = $derived(rows.slice(0, rowLimit));
</script>

<Table.Root class="w-full table-fixed">
	<Table.Header>
		<Table.Row class="text-xs">
			<Table.Head class="text-muted-foreground w-[10%] px-5"
				><div class="flex justify-center">Ranking</div></Table.Head
			>
			<Table.Head class="text-muted-foreground w-2/5 px-5">Proposal</Table.Head>
			<Table.Head class="text-muted-foreground w-2/5 px-5">{scoreHeading}</Table.Head>
		</Table.Row>
	</Table.Header>
	<Table.Body>
		{#each visibleRows as row, index (row.key)}
			<Table.Row class="hover:[&,&>svelte-css-wrapper]:[&>th,td]:bg-accent">
				<Table.Cell class="text-muted-foreground border-l-6 px-5 py-9">
					<div
						transition:slide={{ duration: ROW_ANIMATION_DURATION_MS }}
						class="flex justify-center"
					>
						{#if row.alignment === null}
							<span aria-label="Not ranked">-</span>
						{:else if index === 0}
							<Crown />
						{:else}
							{index + 1}
						{/if}
					</div>
				</Table.Cell>
				<Table.Cell class="text-foreground h-auto px-5 text-base text-wrap!">
					<div transition:slide={{ duration: ROW_ANIMATION_DURATION_MS }}>
						{row.title}
					</div>
				</Table.Cell>
				<Table.Cell class="text-primary h-auto px-5 text-wrap!">
					<div
						transition:slide={{ duration: ROW_ANIMATION_DURATION_MS }}
						class="flex items-center gap-4"
					>
						{#if row.alignment === null}
							<span class="text-muted-foreground text-base"
								>Too few reviews to show</span
							>
						{:else}
							<DivergingProgress
								value={row.alignment}
								min={domain[0]}
								max={domain[1]}
							/>
						{/if}
					</div>
				</Table.Cell>
			</Table.Row>
		{/each}
	</Table.Body>
</Table.Root>

{#if rowLimit < rows.length}
	<button
		class="bg-border flex w-full items-center justify-center gap-2 py-3 text-center text-base"
		type="button"
		onclick={() => (rowLimit = rows.length)}
		>See all {rows.length} proposals <ChevronDown /></button
	>
{/if}
