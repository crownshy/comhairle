<script lang="ts">
	import * as Pagination from '$lib/components/ui/pagination';

	type Props = {
		count: number;
		perPage: number;
		page: number;
		onPageChange: (page: number) => void;
	};

	let { count, perPage, page, onPageChange }: Props = $props();

	const firstShown = $derived((page - 1) * perPage + 1);
	const lastShown = $derived(Math.min(page * perPage, count));
</script>

{#if count > perPage}
	<div class="flex flex-col items-center gap-2 sm:flex-row sm:justify-between">
		<p class="text-muted-foreground text-base tabular-nums">
			Showing {firstShown} to {lastShown} of {count}
		</p>
		<Pagination.Root {count} {perPage} {page} {onPageChange} class="mx-0 w-auto">
			{#snippet children({ pages, currentPage })}
				<Pagination.Content>
					<Pagination.Item>
						<Pagination.PrevButton class="text-base" />
					</Pagination.Item>
					{#each pages as pageItem (pageItem.key)}
						{#if pageItem.type === 'ellipsis'}
							<Pagination.Item class="hidden sm:block">
								<Pagination.Ellipsis />
							</Pagination.Item>
						{:else}
							<Pagination.Item class="hidden sm:block">
								<Pagination.Link
									page={pageItem}
									isActive={currentPage === pageItem.value}
									class="text-base"
								/>
							</Pagination.Item>
						{/if}
					{/each}
					<Pagination.Item>
						<Pagination.NextButton class="text-base" />
					</Pagination.Item>
				</Pagination.Content>
			{/snippet}
		</Pagination.Root>
	</div>
{/if}
