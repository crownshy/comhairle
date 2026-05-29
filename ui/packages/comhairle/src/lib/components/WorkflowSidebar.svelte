<script lang="ts">
	import { cn } from '$lib/utils';
	import { Check, Lock, ShieldCheck } from 'lucide-svelte';
	import type { StepItem } from '$lib/components/StepSelector.svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import type { ComhairleDocument } from '@crownshy/api-client/api';

	interface WorkflowSidebarProps {
		steps: StepItem[];
		currentStepNumber: number;
		totalSteps: number;
		currentTitle: string;
		currentDescription?: string;
		dataUsageNote?: string;
		availableDocuments?: ComhairleDocument[];
		conversationId?: string;
	}

	let {
		steps,
		currentStepNumber,
		totalSteps,
		currentTitle,
		currentDescription,
		dataUsageNote,
		availableDocuments = [],
		conversationId
	}: WorkflowSidebarProps = $props();

	const fallbackDataUsage =
		'Placeholder: a short note here will explain how the answers and inputs from this step will be used. Facilitators will be able to author this per step.';

	function lineColor(prevStatus: StepItem['status']): string {
		return prevStatus === 'upcoming' ? 'bg-primary/20' : 'bg-primary';
	}

	let scrollContainer = $state<HTMLElement | null>(null);
	let currentStepId = $derived(steps.find((s) => s.status === 'current')?.id);

	$effect(() => {
		currentStepId;
		if (!scrollContainer) return;
		const el = scrollContainer.querySelector<HTMLElement>('[data-current="true"]');
		if (!el) return;
		const prefersReducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
		const targetTop = el.offsetTop - scrollContainer.clientHeight / 2 + el.offsetHeight / 2;
		scrollContainer.scrollTo({
			top: Math.max(targetTop, 0),
			behavior: prefersReducedMotion ? 'auto' : 'smooth'
		});
	});
</script>

<aside class="hidden h-full w-80 shrink-0 flex-col lg:flex" aria-label="Workflow steps">
	<!-- Current step header (pinned) -->
	<div class="border-border shrink-0 border-b px-6 pt-8 pb-6">
		<p class="text-primary text-xs font-semibold tracking-wider uppercase">
			Step {currentStepNumber} of {totalSteps}
		</p>
		<h1 class="text-foreground mt-2 text-2xl leading-tight font-bold">
			{currentTitle}
		</h1>
		{#if currentDescription}
			<div
				class="prose-sm prose-p:text-sm prose-p:text-muted-foreground prose-li:text-sm prose-li:text-muted-foreground text-muted-foreground mt-3"
			>
				{#key currentDescription}
					<ContentRenderer
						content={currentDescription}
						{availableDocuments}
						{conversationId}
					/>
				{/key}
			</div>
		{/if}

		<!-- Data usage transparency note -->
		<div class="bg-primary/5 border-primary/20 mt-5 rounded-md border-l-2 p-3">
			<div class="flex items-start gap-2.5">
				<ShieldCheck class="text-primary mt-0.5 h-4 w-4 shrink-0" aria-hidden="true" />
				<div class="min-w-0">
					<p class="text-foreground text-xs font-semibold tracking-wide uppercase">
						How we'll use this data
					</p>
					<p class="text-muted-foreground mt-1 text-xs leading-relaxed">
						{dataUsageNote ?? fallbackDataUsage}
					</p>
				</div>
			</div>
		</div>
	</div>

	<!-- Timeline (scrollable) -->
	<div class="shrink-0 px-6 pt-6">
		<p class="text-muted-foreground text-xs font-medium tracking-wider uppercase">Progress</p>
	</div>

	<nav bind:this={scrollContainer} class="min-h-0 flex-1 overflow-y-auto px-6 pt-4 pb-8">
		<ol class="relative">
			{#each steps as step, index (step.id)}
				{@const isLast = index === steps.length - 1}
				{@const isCurrent = step.status === 'current'}
				{@const clickable = step.status === 'completed' && step.href}

				<li class="relative pb-6" data-current={isCurrent ? 'true' : undefined}>
					{#if !isLast}
						<span
							aria-hidden="true"
							class={cn(
								'absolute top-10 left-[27px] -ml-px h-full w-0.5',
								lineColor(step.status)
							)}
						></span>
					{/if}

					{#if clickable}
						<a
							href={step.href}
							class="group hover:bg-background/60 relative flex items-start gap-4 rounded-lg p-2 transition-colors"
							aria-label="Step {index + 1}: {step.name} (completed, click to return)"
						>
							{@render dot(step, index)}
							{@render label(step, index)}
						</a>
					{:else}
						<div
							class={cn(
								'relative flex items-start gap-4 rounded-lg p-2',
								isCurrent && 'bg-background/80 ring-primary/20 ring-1'
							)}
							aria-current={isCurrent ? 'step' : undefined}
						>
							{@render dot(step, index)}
							{@render label(step, index)}
						</div>
					{/if}
				</li>
			{/each}
		</ol>
	</nav>
</aside>

{#snippet dot(step: StepItem, index: number)}
	{@const isLocked = step.status === 'upcoming'}
	{@const isCurrent = step.status === 'current'}
	{@const isDone = step.status === 'completed' || step.status === 'completed-locked'}
	{@const isCompletedLocked = step.status === 'completed-locked'}

	<div
		class={cn(
			'relative z-10 flex h-10 w-10 shrink-0 items-center justify-center rounded-full border-2',
			isCurrent && 'border-primary bg-primary',
			isDone && !isCompletedLocked && 'border-primary bg-primary',
			isCompletedLocked && 'border-ring/40 bg-popover',
			isLocked && 'border-muted-foreground/30 bg-background'
		)}
	>
		{#if isDone && !isCompletedLocked}
			<Check class="text-primary-foreground h-5 w-5" />
		{:else if isCompletedLocked}
			<Check class="text-muted-foreground h-5 w-5" />
		{:else if isCurrent}
			<span class="bg-background h-3 w-3 rounded-full"></span>
			<span
				aria-hidden="true"
				class="bg-primary/30 absolute -inset-1 -z-10 animate-ping rounded-full motion-reduce:hidden"
			></span>
		{:else if isLocked}
			<span class="text-muted-foreground/60 text-sm font-medium">{index + 1}</span>
		{/if}
	</div>
{/snippet}

{#snippet label(step: StepItem, index: number)}
	{@const isLocked = step.status === 'upcoming'}
	{@const isCurrent = step.status === 'current'}
	{@const isCompletedLocked = step.status === 'completed-locked'}

	<div class="flex min-w-0 flex-1 flex-col pt-1">
		<span
			class={cn(
				'text-xs font-medium tracking-wider uppercase',
				isCurrent && 'text-primary',
				isLocked && 'text-muted-foreground/60',
				!isCurrent && !isLocked && 'text-muted-foreground'
			)}
		>
			Step {index + 1}
			{#if isLocked}
				<Lock class="ml-1 inline h-3 w-3" aria-hidden="true" />
			{/if}
		</span>
		<span
			class={cn(
				'truncate text-sm font-semibold group-hover:underline',
				isCurrent && 'text-foreground',
				isLocked && 'text-muted-foreground/60',
				isCompletedLocked && 'text-muted-foreground',
				!isCurrent && !isLocked && !isCompletedLocked && 'text-foreground'
			)}
			title={step.name}
		>
			{step.name}
		</span>
	</div>
{/snippet}
