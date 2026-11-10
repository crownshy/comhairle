<script lang="ts">
	import { REPORT_EYEBROW_CLASS } from '$lib/reports/styles';
	import { cn } from '$lib/utils';
	import type { ReportOutlineSection } from './reportDocument';

	type Props = {
		sections: ReportOutlineSection[];
		/** Scrolls the editor to the body's top-level node at this index. */
		onJump: (nodeIndex: number) => void;
	};
	let { sections, onJump }: Props = $props();

	const OUTLINE_LINK_CLASS =
		'hover:bg-accent hover:text-foreground w-full rounded-md px-2 py-1 text-start transition-colors';
</script>

{#if sections.length > 0}
	<nav aria-label="Report outline" class="flex flex-col gap-2">
		<p class={REPORT_EYEBROW_CLASS}>Outline</p>
		<ol class="flex flex-col gap-1">
			{#each sections as section (section.index)}
				<li>
					<button
						type="button"
						class={cn(OUTLINE_LINK_CLASS, 'text-foreground text-base font-medium')}
						onclick={() => onJump(section.firstNodeIndex)}
					>
						{section.label}
					</button>
					{#if section.cards.length > 1}
						<ol class="ms-3 flex flex-col border-s ps-1">
							{#each section.cards.slice(1) as card (card.firstNodeIndex)}
								<li>
									<button
										type="button"
										class={cn(
											OUTLINE_LINK_CLASS,
											'text-muted-foreground text-sm'
										)}
										onclick={() => onJump(card.firstNodeIndex)}
									>
										{card.title}
									</button>
								</li>
							{/each}
						</ol>
					{/if}
				</li>
			{/each}
		</ol>
	</nav>
{/if}
