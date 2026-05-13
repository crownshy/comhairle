<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Sprout, Clock, Lock, MessageCircleQuestion } from 'lucide-svelte';

	type Props = {
		topic: string;
		description?: string;
		questionCount: number;
		followUpCount: number;
		onStart: () => void;
	};

	let { topic, description, questionCount, followUpCount, onStart }: Props = $props();

	let totalSteps = $derived(questionCount * (1 + followUpCount));
	let estimateMinutes = $derived(Math.max(3, totalSteps));
</script>

<div class="mx-auto flex max-w-xl flex-col items-center px-6 py-12 text-center">
	<div
		class="bg-primary/10 text-primary mb-6 flex size-14 items-center justify-center rounded-full"
	>
		<Sprout class="size-7" />
	</div>

	<h1 class="text-foreground mb-3 text-3xl font-semibold tracking-tight">
		{topic || 'Welcome'}
	</h1>

	<ul class="mb-10 w-full max-w-sm space-y-3 text-left">
		<li class="flex items-start gap-3">
			<MessageCircleQuestion class="text-primary mt-0.5 size-4 shrink-0" />
			<span class="text-foreground text-sm">
				{questionCount} question{questionCount === 1 ? '' : 's'}{followUpCount > 0
					? `, with ${followUpCount} follow-up${followUpCount === 1 ? '' : 's'} each`
					: ''}
			</span>
		</li>
		<li class="flex items-start gap-3">
			<Lock class="text-primary mt-0.5 size-4 shrink-0" />
			<span class="text-foreground text-sm">
				Your views stay private until you approve them at the end.
			</span>
		</li>
		<li class="flex items-start gap-3">
			<Clock class="text-primary mt-0.5 size-4 shrink-0" />
			<span class="text-foreground text-sm">
				About {estimateMinutes} minute{estimateMinutes === 1 ? '' : 's'}.
			</span>
		</li>
	</ul>

	<Button size="lg" onclick={onStart}>Begin</Button>
</div>
