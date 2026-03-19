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

<!-- Desktop -->
<div class="bg-background hidden w-full max-w-3xl rounded-2xl p-6 md:block">
	<div class="mx-auto flex max-w-lg flex-col items-center gap-2">
		<p class="text-primary text-center text-2xl leading-7 font-semibold">
			Step {currentStepNumber} of {totalSteps}
		</p>
		<h2 class="text-foreground text-center text-4xl leading-[48px] font-semibold">
			{title}
		</h2>
		{#if estimatedMinutes}
			<p class="text-foreground text-center text-2xl leading-7 font-semibold">
				({estimatedMinutes}
				{estimatedMinutes === 1 ? 'minute' : 'minutes'})
			</p>
		{/if}
		{#if description}
			<div
				class="prose-sm prose-p:text-base prose-li:text-base text-muted-foreground mx-auto max-w-3xl text-center"
			>
				{#key description}
					<ContentRenderer content={description} />
				{/key}
			</div>
		{/if}
	</div>
</div>

<!-- Mobile -->
<div class="bg-background w-full md:hidden">
	<div class="flex flex-col items-center gap-2">
		<div class="flex w-full items-center justify-between">
			{#if prevHref}
				<a
					href={prevHref}
					class="text-muted-foreground shrink-0 p-2"
					aria-label="Previous step"
				>
					<ChevronLeft class="h-6 w-6" />
				</a>
			{:else}
				<span class="text-muted-foreground shrink-0 p-2 opacity-20" aria-hidden="true">
					<ChevronLeft class="h-6 w-6" />
				</span>
			{/if}

			<div class="flex min-w-0 flex-1 flex-col items-center">
				<p class="text-primary text-center text-base leading-6 font-medium">
					Step {currentStepNumber} of {totalSteps}
				</p>
				<p class="text-foreground mt-1 text-center text-xl leading-6 font-semibold">
					{title}
					{#if estimatedMinutes}
						<span class="text-foreground text-sm leading-5 font-medium">
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
					<ChevronRight class="h-6 w-6" />
				</button>
			{:else}
				<span class="text-muted-foreground shrink-0 p-2 opacity-20" aria-hidden="true">
					<ChevronRight class="h-6 w-6" />
				</span>
			{/if}
		</div>

		{#if description}
			<div
				class="prose-sm prose-p:text-sm prose-li:text-sm text-muted-foreground text-center"
			>
				{#key description}
					<ContentRenderer content={description} />
				{/key}
			</div>
		{/if}
	</div>
</div>
