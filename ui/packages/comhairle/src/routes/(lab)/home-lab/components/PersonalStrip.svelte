<script lang="ts">
	import { ArrowRight, MessageSquareText } from 'lucide-svelte';
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
	};

	let { inProgress, inProgressHref, stepIndex, stepCount, reportReady, reportHref }: Props =
		$props();
</script>

<!-- The conversation's image next to its title, so people recognise it at a glance. -->
{#snippet thumbnail(conversation: LabConversation)}
	<div
		class="bg-primary/10 relative h-16 w-24 shrink-0 overflow-hidden rounded-lg sm:h-20 sm:w-28"
	>
		{#if conversation.imageUrl}
			<img
				src={conversation.imageUrl as string}
				alt=""
				class="absolute inset-0 size-full object-cover"
			/>
		{:else}
			<MessageSquareText class="text-primary/40 absolute inset-0 m-auto size-6" />
		{/if}
	</div>
{/snippet}

<div class="grid gap-4 md:grid-cols-2">
	{#if inProgress && inProgressHref}
		<section class="bg-card border-border flex flex-col gap-4 rounded-2xl border p-5">
			<h2 class="text-xl font-bold">Pick up where you left off</h2>
			<div class="flex items-center gap-4">
				{@render thumbnail(inProgress)}
				<div class="flex min-w-0 grow flex-col gap-2">
					<span class="text-lg leading-snug font-semibold">{inProgress.title}</span>
					<Progress value={(stepIndex / stepCount) * 100} class="h-2" />
					<span class="text-muted-foreground text-base"
						>Step {stepIndex + 1} of {stepCount}</span
					>
				</div>
			</div>
			<Button
				href={inProgressHref}
				class="mt-auto h-12 self-start rounded-full px-6 text-base font-bold"
			>
				Continue <ArrowRight class="size-5" />
			</Button>
		</section>
	{/if}
	{#if reportReady && reportHref}
		<section class="bg-card border-border flex flex-col gap-4 rounded-2xl border p-5">
			<h2 class="flex items-center gap-2 text-xl font-bold">
				Results ready
				<span class="bg-foreground text-background rounded-full px-2 py-0.5 text-sm"
					>New</span
				>
			</h2>
			<div class="flex items-center gap-4">
				{@render thumbnail(reportReady)}
				<span class="min-w-0 grow text-lg leading-snug font-semibold"
					>{reportReady.title}</span
				>
			</div>
			<Button
				href={reportHref}
				variant="outline"
				class="mt-auto h-12 self-start rounded-full px-6 text-base font-bold"
			>
				See results <ArrowRight class="size-5" />
			</Button>
		</section>
	{/if}
</div>
