<script lang="ts">
	import ZoomIn from '@lucide/svelte/icons/zoom-in';
	import ZoomOut from '@lucide/svelte/icons/zoom-out';
	import Button from '$lib/components/ui/button/button.svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { m } from '$lib/paraglide/messages';
	import { cn } from '$lib/utils';
	import { breakoutTextZoom } from './breakoutTextZoom.svelte';

	type Props = {
		question: string;
		description?: string;
	};

	let { question, description }: Props = $props();
</script>

<div class="bg-background flex w-full shrink-0 flex-col gap-2 rounded-xl px-6 pt-3 pb-6">
	<!-- Three columns so the label stays centred over the question while the controls sit on its row -->
	<div class="grid grid-cols-[1fr_auto_1fr] items-center">
		<span class="text-primary col-start-2 text-xs leading-4 font-medium uppercase"
			>{m.live_event_question()}</span
		>
		<div class="flex justify-end">
			<Button
				variant="ghost"
				size="icon"
				class="text-muted-foreground size-11 md:size-9"
				aria-label="Smaller text"
				disabled={!breakoutTextZoom.canShrink}
				onclick={() => breakoutTextZoom.shrink()}
			>
				<ZoomOut class="size-5" />
			</Button>
			<Button
				variant="ghost"
				size="icon"
				class="text-muted-foreground size-11 md:size-9"
				aria-label="Larger text"
				disabled={!breakoutTextZoom.canGrow}
				onclick={() => breakoutTextZoom.grow()}
			>
				<ZoomIn class="size-5" />
			</Button>
		</div>
	</div>

	<!-- zoom rather than font-size, so the rich text's rem-sized headings and lists scale too -->
	<div class={cn('flex flex-col items-center gap-4', breakoutTextZoom.zoomClass)}>
		<p class="text-foreground text-center text-lg leading-7 font-semibold break-words">
			{question}
		</p>

		{#if description}
			<div class="h-0 w-full border-t"></div>
			<div class="text-foreground w-full text-base leading-6 font-normal">
				<ContentRenderer content={description} />
			</div>
		{/if}
	</div>
</div>
