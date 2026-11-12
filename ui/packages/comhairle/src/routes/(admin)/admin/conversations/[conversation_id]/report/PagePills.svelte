<script lang="ts">
	import X from 'lucide-svelte/icons/x';
	import { REPORT_PILL_CLASS } from '$lib/reports/styles';
	import { cn } from '$lib/utils';
	import type { ReportPage } from './reportDocument';

	type Props = {
		pages: ReportPage[];
		editable: boolean;
		onJump: (page: ReportPage) => void;
		onRemove: (page: ReportPage) => void;
		class?: string;
	};
	let { pages, editable, onJump, onRemove, class: className }: Props = $props();
</script>

{#if pages.length > 0}
	<nav aria-label="Report pages" class={className}>
		<ul class="flex flex-wrap gap-2">
			{#each pages as page, index (index)}
				<li
					class={cn(
						REPORT_PILL_CLASS,
						'bg-accent text-accent-foreground flex items-center gap-1 py-0',
						editable && 'pe-1'
					)}
				>
					<button
						type="button"
						class={cn('min-w-0 truncate py-1.5', page.empty && 'italic opacity-70')}
						onclick={() => onJump(page)}
					>
						{page.label}
					</button>
					{#if editable}
						<button
							type="button"
							class="hover:bg-background/60 rounded-full p-1"
							aria-label="Remove page {page.label}"
							title="Remove this page (Ctrl+Z to undo)"
							onclick={() => onRemove(page)}
						>
							<X class="size-3.5" />
						</button>
					{/if}
				</li>
			{/each}
		</ul>
	</nav>
{/if}
