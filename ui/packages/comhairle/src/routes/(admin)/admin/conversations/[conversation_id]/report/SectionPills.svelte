<script lang="ts">
	import X from 'lucide-svelte/icons/x';
	import { REPORT_PILL_CLASS } from '$lib/reports/styles';
	import { cn } from '$lib/utils';
	import type { ReportSection } from './reportDocument';

	type Props = {
		sections: ReportSection[];
		editable: boolean;
		onJump: (section: ReportSection) => void;
		onRemove: (section: ReportSection) => void;
	};
	let { sections, editable, onJump, onRemove }: Props = $props();
</script>

{#if sections.length > 0}
	<nav
		aria-label="Report sections"
		class="bg-background/95 sticky top-0 z-10 -mx-5 px-5 py-3 backdrop-blur"
	>
		<ul class="flex flex-wrap gap-2">
			{#each sections as section (section.index)}
				<li
					class={cn(
						REPORT_PILL_CLASS,
						'bg-accent text-accent-foreground flex items-center gap-1 py-0',
						editable && 'pe-1'
					)}
				>
					<button type="button" class="py-1.5" onclick={() => onJump(section)}>
						{section.label}
					</button>
					{#if editable}
						<button
							type="button"
							class="hover:bg-background/60 rounded-full p-1"
							aria-label="Remove section {section.label}"
							title="Remove this section (Ctrl+Z to undo)"
							onclick={() => onRemove(section)}
						>
							<X class="size-3.5" />
						</button>
					{/if}
				</li>
			{/each}
		</ul>
	</nav>
{/if}
