<script lang="ts">
	import AArrowDown from '@lucide/svelte/icons/a-arrow-down';
	import AArrowUp from '@lucide/svelte/icons/a-arrow-up';
	import Button from '$lib/components/ui/button/button.svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { cn } from '$lib/utils';
	import { breakoutTextZoom } from './breakoutTextZoom.svelte';

	type Props = {
		question: string;
		description?: string;
	};

	let { question, description }: Props = $props();
</script>

<div class="bg-background flex w-full shrink-0 flex-col gap-2 rounded-xl px-6 pt-3 pb-6">
	<div class="flex justify-end gap-1">
		<Button
			variant="ghost"
			size="icon"
			aria-label="Smaller text"
			disabled={!breakoutTextZoom.canShrink}
			onclick={() => breakoutTextZoom.shrink()}
		>
			<AArrowDown class="h-5 w-5" />
		</Button>
		<Button
			variant="ghost"
			size="icon"
			aria-label="Larger text"
			disabled={!breakoutTextZoom.canGrow}
			onclick={() => breakoutTextZoom.grow()}
		>
			<AArrowUp class="h-5 w-5" />
		</Button>
	</div>

	<!-- zoom rather than font-size, so the rich text's rem-sized headings and lists scale too -->
	<div class={cn('flex flex-col items-center gap-4', breakoutTextZoom.zoomClass)}>
		<div class="flex flex-col items-center gap-1">
			<span class="text-primary text-xs leading-4 font-medium uppercase">Question</span>
			<p class="text-foreground text-center text-lg leading-7 font-semibold break-words">
				{question}
			</p>
		</div>

		{#if description}
			<div class="h-0 w-full border-t"></div>
			<div class="text-foreground w-full text-base leading-6 font-normal">
				<ContentRenderer content={description} />
			</div>
		{/if}
	</div>
</div>
