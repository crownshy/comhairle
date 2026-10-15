<script lang="ts">
	import { buttonVariants } from '$lib/components/ui/button';
	import ChevronLeft from 'svelte-radix/ChevronLeft.svelte';
	import ChevronRight from 'svelte-radix/ChevronRight.svelte';
	import { cn } from '$lib/utils.js';

	import { m } from '$lib/paraglide/messages';
	import { getPage, setPage, calcPageCount } from './utils';

	type Props = { url: URL; pageSize: number; count: number };

	const { url, pageSize, count }: Props = $props();

	const pageNumber = $derived(getPage(url));
	const pageCount = $derived(calcPageCount({ pageSize, total: count }));
	const linkButtonClass = buttonVariants({ variant: 'outline', size: 'sm' });
</script>

<!-- The page links only change ?page= on the current URL, so there is no route to resolve. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->
<nav class="flex flex-col items-center" aria-label="pagination">
	<ul class="flex flex-row items-center gap-2">
		<li>
			{#if pageNumber === 1}
				<span class={cn(linkButtonClass, 'text-muted-foreground cursor-not-allowed')}>
					<ChevronLeft class="h-4 w-4" />
					{m.previous()}
				</span>
			{:else}
				<a
					href={setPage(url, pageNumber - 1).toString()}
					class={linkButtonClass}
					aria-label={`Go to page ${pageNumber - 1}`}
				>
					<ChevronLeft class="h-4 w-4" />
					{m.previous()}
				</a>
			{/if}
		</li>
		<li><span class="text-base">{m.page_x_of_y({ x: pageNumber, y: pageCount })}</span></li>
		<li>
			{#if pageCount > pageNumber}
				<a
					href={setPage(url, pageNumber + 1).toString()}
					class={linkButtonClass}
					aria-label={`Go to page ${pageNumber + 1}`}
				>
					{m.next()}
					<ChevronRight class="h-4 w-4" />
				</a>
			{:else}
				<span class={cn(linkButtonClass, 'text-muted-foreground cursor-not-allowed')}>
					{m.next()}
					<ChevronRight class="h-4 w-4" />
				</span>
			{/if}
		</li>
	</ul>
</nav>
