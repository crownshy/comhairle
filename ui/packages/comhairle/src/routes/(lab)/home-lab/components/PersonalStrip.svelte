<script lang="ts">
	import { ArrowRight, FileText } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Progress } from '$lib/components/ui/progress';
	import type { LabConversation } from '../mockData';

	type Props = {
		inProgress?: LabConversation;
		inProgressHref?: string;
		stepIndex: number;
		stepCount: number;
		reportReady?: LabConversation;
		reportHref?: string;
		yourView: string;
	};

	let {
		inProgress,
		inProgressHref,
		stepIndex,
		stepCount,
		reportReady,
		reportHref,
		yourView
	}: Props = $props();
</script>

<section class="grid gap-4 md:grid-cols-2">
	{#if inProgress && inProgressHref}
		<div class="bg-card border-border flex flex-col gap-3 rounded-2xl border p-5">
			<h2 class="text-xl font-bold">Pick up where you left off</h2>
			<span class="text-lg leading-snug font-semibold">{inProgress.title}</span>
			<div class="flex flex-col gap-1.5">
				<Progress value={(stepIndex / stepCount) * 100} class="h-2" />
				<span class="text-muted-foreground text-base"
					>Step {stepIndex + 1} of {stepCount}</span
				>
			</div>
			<Button href={inProgressHref} class="mt-auto h-12 rounded-full text-base font-bold">
				Continue <ArrowRight class="size-5" />
			</Button>
		</div>
	{/if}
	{#if reportReady && reportHref}
		<div class="bg-card border-border flex flex-col gap-3 rounded-2xl border p-5">
			<h2 class="flex items-center gap-2 text-xl font-bold">
				Results are in
				<span class="bg-foreground text-background rounded-full px-2 py-0.5 text-sm"
					>New</span
				>
			</h2>
			<span class="text-lg leading-snug font-semibold">{reportReady.title}</span>
			<span class="text-muted-foreground text-base">{yourView}</span>
			<Button
				href={reportHref}
				variant="outline"
				class="mt-auto h-12 rounded-full text-base font-bold"
			>
				<FileText class="size-5" /> Read your report
			</Button>
		</div>
	{/if}
</section>
