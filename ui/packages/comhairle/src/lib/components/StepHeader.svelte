<script lang="ts">
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { ChevronLeft, ChevronRight } from 'lucide-svelte';

	interface StepHeaderProps {
		currentStepNumber: number;
		totalSteps: number;
		title: string;
		description?: string;
		estimatedMinutes?: number;
		prevHref?: string;
		onNext?: () => void;
	}

	let {
		currentStepNumber,
		totalSteps,
		title,
		description,
		estimatedMinutes,
		prevHref,
		onNext
	}: StepHeaderProps = $props();
</script>

<div class="bg-background w-full md:mx-auto md:max-w-4xl md:rounded-2xl md:px-6 md:pt-4">
	<div class="flex flex-col items-center">
		<div class="flex w-full items-center justify-between md:gap-12">
			{#if prevHref}
				<a
					href={prevHref}
					class="text-muted-foreground shrink-0 p-2"
					aria-label="Previous step"
				>
					<ChevronLeft class="h-6 w-6 md:h-8 md:w-8" />
				</a>
			{:else}
				<span class="text-muted-foreground shrink-0 p-2 opacity-20" aria-hidden="true">
					<ChevronLeft class="h-6 w-6 md:h-8 md:w-8" />
				</span>
			{/if}

			<div class="flex min-w-0 flex-1 flex-col items-center">
				<p class="text-primary text-center text-sm leading-5 font-semibold">
					Step {currentStepNumber} of {totalSteps}
				</p>
				<p
					class="text-foreground mt-1 text-center text-xl leading-6 font-semibold md:text-2xl"
				>
					{title}
					{#if estimatedMinutes}
						<span
							class="text-foreground text-sm leading-5 font-medium md:text-lg md:leading-7 md:font-semibold"
						>
							({estimatedMinutes}
							{estimatedMinutes === 1 ? 'minute' : 'minutes'})
						</span>
					{/if}
				</p>
			</div>

			{#if onNext}
				<button
					onclick={onNext}
					class="text-muted-foreground shrink-0 p-2"
					aria-label="Next step"
				>
					<ChevronRight class="h-6 w-6 md:h-8 md:w-8" />
				</button>
			{:else}
				<span class="text-muted-foreground shrink-0 p-2 opacity-20" aria-hidden="true">
					<ChevronRight class="h-6 w-6 md:h-8 md:w-8" />
				</span>
			{/if}
		</div>

		{#if description}
			<div
				class="prose-sm prose-p:text-sm prose-p:text-muted-foreground prose-li:text-muted-foreground prose-li:text-sm text-muted-foreground mx-auto max-w-3xl text-center"
			>
				{#key description}
					<ContentRenderer content={description} />
				{/key}
			</div>
		{/if}
	</div>
</div>
