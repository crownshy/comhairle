<script lang="ts">
	import { buttonVariants } from '$lib/components/ui/button';
	import ChevronLeft from 'svelte-radix/ChevronLeft.svelte';
	import ChevronRight from 'svelte-radix/ChevronRight.svelte';
	import { cn } from '$lib/utils.js';

	import * as m from '$lib/paraglide/messages';
	import { getPage, setPage, calcPageCount } from './utils';

	const { url, pageSize, count }: { url: URL; pageSize: number; count: number } = $props();

	const pageNumber = $derived(getPage(url));
	const pageCount = $derived(calcPageCount({ pageSize, total: count }));
	const linkBtnCls = buttonVariants({ variant: 'outline-solid', size: 'sm' });
</script>

<nav class="flex flex-col items-center" aria-label="pagination">
	<ul class="flex flex-row items-center gap-2">
		<li>
			{#if pageNumber === 1}
				<span class={cn(linkBtnCls, 'text-muted-foreground cursor-not-allowed')}>
					<ChevronLeft class="h-4 w-4" />
					{m.previous()}
				</span>
			{:else}
				<a
					href={setPage(url, pageNumber - 1).toString()}
					class={linkBtnCls}
					aria-label={`Go to page ${pageNumber - 1}`}
				>
					<ChevronLeft class="h-4 w-4" />
					{m.previous()}
				</a>
			{/if}
		</li>
		<li><span class="text-xs">{m.page_x_of_y({ x: pageNumber, y: pageCount })}</span></li>
		<li>
			{#if pageCount > pageNumber}
				<a
					href={setPage(url, pageNumber + 1).toString()}
					class={linkBtnCls}
					aria-label={`Go to page ${pageNumber + 1}`}
				>
					{m.next()}
					<ChevronRight class="h-4 w-4" />
				</a>
			{:else}
				<span class={cn(linkBtnCls, 'text-muted-foreground cursor-not-allowed')}>
					{m.next()}
					<ChevronRight class="h-4 w-4" />
				</span>
			{/if}
		</li>
	</ul>
</nav>
