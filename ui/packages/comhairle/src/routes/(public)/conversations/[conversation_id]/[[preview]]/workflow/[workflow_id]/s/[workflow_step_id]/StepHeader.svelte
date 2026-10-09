<script lang="ts">
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { m } from '$lib/paraglide/messages';
	import { cn } from '$lib/utils';
	import type { ComhairleDocument } from '@crownshy/api-client/api';
	import { STEP_COLUMN_CLASS } from './styles';

	interface StepHeaderProps {
		currentStepNumber: number;
		totalSteps: number;
		title: string;
		description?: string;
		estimatedMinutes?: number;
		availableDocuments?: ComhairleDocument[];
		conversationId?: string;
	}

	let {
		currentStepNumber,
		totalSteps,
		title,
		description,
		estimatedMinutes,
		availableDocuments = [],
		conversationId
	}: StepHeaderProps = $props();
</script>

<!-- "Step N of M" stays visible because the progress bar is aria-hidden. -->
<div class={cn(STEP_COLUMN_CLASS, 'flex flex-col items-center pt-3 pb-2')}>
	<p class="text-primary text-center text-sm leading-5 font-semibold">
		{m.step_x_of_y({ current: currentStepNumber, total: totalSteps })}
	</p>
	<p class="text-foreground mt-1 text-center text-xl leading-6 font-semibold md:text-2xl">
		{title}
		{#if estimatedMinutes}
			<span class="text-foreground text-base font-medium md:text-lg md:font-semibold">
				{estimatedMinutes === 1
					? m.step_one_minute()
					: m.step_minutes({ count: estimatedMinutes })}
			</span>
		{/if}
	</p>

	{#if description}
		<div class="prose-p:text-base prose-li:text-base mx-auto max-w-3xl text-center text-base">
			<ContentRenderer content={description} {availableDocuments} {conversationId} />
		</div>
	{/if}
</div>
