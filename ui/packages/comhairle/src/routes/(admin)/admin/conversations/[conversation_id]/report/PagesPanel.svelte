<script lang="ts">
	import Plus from 'lucide-svelte/icons/plus';
	import EllipsisVertical from 'lucide-svelte/icons/ellipsis-vertical';
	import Trash from 'lucide-svelte/icons/trash-2';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { cn } from '$lib/utils';
	import type { ReportPage } from './reportDocument';

	type Props = {
		pages: ReportPage[];
		/** The page scrolled to in the editor, highlighted in the list; -1 is the summary. */
		currentIndex: number;
		editable: boolean;
		/** Left out when the report has no summary to show. */
		onJumpToSummary?: () => void;
		onJump: (page: ReportPage) => void;
		onAdd: () => void;
		onRemove: (page: ReportPage) => void;
	};
	let { pages, currentIndex, editable, onJumpToSummary, onJump, onAdd, onRemove }: Props =
		$props();

	const ROW_CLASS = 'group flex items-center rounded-full transition-colors';
	const ROW_BUTTON_CLASS = 'flex min-w-0 grow items-center py-2 ps-3 text-start text-base';

	function rowClass(current: boolean) {
		return cn(
			ROW_CLASS,
			current ? 'bg-accent text-primary' : 'text-foreground hover:bg-accent/60'
		);
	}
</script>

<nav aria-label="Report pages" class="flex flex-col gap-3">
	<div class="flex items-center justify-between ps-3">
		<h2 class="text-foreground text-base font-medium">Pages</h2>
		{#if editable}
			<button
				type="button"
				class="text-muted-foreground hover:bg-accent hover:text-foreground flex size-9 items-center justify-center rounded-full transition-colors"
				aria-label="Add a page"
				title="Add a page"
				onclick={onAdd}
			>
				<Plus class="size-5" />
			</button>
		{/if}
	</div>

	<div class="flex flex-col gap-1">
		{#if onJumpToSummary}
			<button
				type="button"
				class={cn(
					rowClass(currentIndex === -1),
					ROW_BUTTON_CLASS,
					currentIndex === -1 && 'font-medium'
				)}
				aria-current={currentIndex === -1 ? 'location' : undefined}
				onclick={onJumpToSummary}
			>
				Executive summary
			</button>
		{/if}

		{#if pages.length === 0}
			<p class="text-muted-foreground ps-3 text-base italic">
				Pages you start with a page break will appear here.
			</p>
		{:else}
			<ol class="flex flex-col gap-1">
				{#each pages as page, index (index)}
					{@const current = index === currentIndex}
					<li class={rowClass(current)}>
						<button
							type="button"
							class={ROW_BUTTON_CLASS}
							aria-current={current ? 'location' : undefined}
							onclick={() => onJump(page)}
						>
							<span
								class={cn(
									'truncate',
									current && 'font-medium',
									page.empty && 'text-muted-foreground italic'
								)}
								title={page.empty
									? 'Empty pages are left out of the published report'
									: undefined}>{page.label}</span
							>
						</button>
						{#if editable}
							<DropdownMenu.Root>
								<DropdownMenu.Trigger
									class={cn(
										'text-muted-foreground hover:text-foreground me-1 flex size-8 shrink-0 items-center justify-center rounded-full opacity-0 transition-opacity group-hover:opacity-100 focus-visible:opacity-100 data-[state=open]:opacity-100',
										current && 'opacity-100'
									)}
									aria-label="Options for {page.label}"
								>
									<EllipsisVertical class="size-4" />
								</DropdownMenu.Trigger>
								<DropdownMenu.Content align="start">
									<DropdownMenu.Item onclick={() => onRemove(page)}>
										<Trash class="size-4" />Delete page
									</DropdownMenu.Item>
								</DropdownMenu.Content>
							</DropdownMenu.Root>
						{/if}
					</li>
				{/each}
			</ol>
		{/if}
	</div>
</nav>
