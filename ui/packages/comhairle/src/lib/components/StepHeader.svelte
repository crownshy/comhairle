<script lang="ts">
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';

	interface StepHeaderProps {
		currentStepNumber: number;
		totalSteps: number;
		title: string;
		description?: string;
		estimatedMinutes?: number;
	}

	let { currentStepNumber, totalSteps, title, description, estimatedMinutes }: StepHeaderProps =
		$props();
</script>

<div class="bg-background w-full max-w-3xl rounded-2xl p-6">
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
