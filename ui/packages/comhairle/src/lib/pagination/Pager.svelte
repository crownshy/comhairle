<script lang="ts">
	import ChevronLeft from '@lucide/svelte/icons/chevron-left';
	import ChevronRight from '@lucide/svelte/icons/chevron-right';
	import { Button } from '$lib/components/ui/button';
	import * as Pagination from '$lib/components/ui/pagination';
	import { m } from '$lib/paraglide/messages';
	import { getPage, setPage, calcPageCount, pageItems } from './utils';

	type Props = { url: URL; pageSize: number; count: number };

	const { url, pageSize, count }: Props = $props();

	const pageNumber = $derived(getPage(url));
	const pageCount = $derived(calcPageCount({ pageSize, total: count }));
	const items = $derived(pageItems({ current: pageNumber, pageCount }));
	const hrefFor = (page: number) => setPage(url, page).toString();
</script>

<!-- bits-ui's Pagination renders buttons, so the shadcn layout is used with plain links instead
	 to keep middle-click and no-JS navigation working. -->
<nav class="flex justify-center" aria-label="pagination">
	<Pagination.Content class="gap-2">
		<Pagination.Item>
			<Button
				href={hrefFor(pageNumber - 1)}
				disabled={pageNumber === 1}
				variant="ghost"
				class="text-base"
				aria-label={m.previous()}
			>
				<ChevronLeft />
				<span class="hidden sm:inline">{m.previous()}</span>
			</Button>
		</Pagination.Item>

		{#each items as item (item.type === 'page' ? item.page : item.key)}
			<Pagination.Item class="hidden sm:block">
				{#if item.type === 'page'}
					<Button
						href={hrefFor(item.page)}
						variant={item.page === pageNumber ? 'outline' : 'ghost'}
						size="icon"
						class="text-base"
						aria-current={item.page === pageNumber ? 'page' : undefined}
						aria-label={m.page_x_of_y({ x: item.page, y: pageCount })}
					>
						{item.page}
					</Button>
				{:else}
					<Pagination.Ellipsis class="text-muted-foreground" />
				{/if}
			</Pagination.Item>
		{/each}

		<Pagination.Item class="text-base sm:hidden">
			{m.page_x_of_y({ x: pageNumber, y: pageCount })}
		</Pagination.Item>

		<Pagination.Item>
			<Button
				href={hrefFor(pageNumber + 1)}
				disabled={pageNumber >= pageCount}
				variant="ghost"
				class="text-base"
				aria-label={m.next()}
			>
				<span class="hidden sm:inline">{m.next()}</span>
				<ChevronRight />
			</Button>
		</Pagination.Item>
	</Pagination.Content>
</nav>
