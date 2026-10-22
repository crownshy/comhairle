<script lang="ts">
	import type { Snippet } from 'svelte';
	import { ChevronRight } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';

	type Props = {
		title: string;
		next: string;
		onNext: () => void;
		children?: Snippet;
		/** Extra actions beside Next, e.g. "Add your own". */
		actions?: Snippet;
	};

	let { title, next, onNext, children, actions }: Props = $props();
</script>

<div class="flex w-full flex-col items-start gap-6">
	<h2 class="text-card-foreground text-2xl leading-snug font-semibold sm:text-3xl">{title}</h2>
	{@render children?.()}
	<div class="flex w-full flex-col items-stretch gap-3 sm:w-auto sm:flex-row sm:items-center">
		<Button size="lg" class="h-12 gap-2 px-6 text-lg" onclick={onNext}>
			{next}
			<ChevronRight class="size-5 rtl:-scale-x-100" />
		</Button>
		{@render actions?.()}
	</div>
</div>
