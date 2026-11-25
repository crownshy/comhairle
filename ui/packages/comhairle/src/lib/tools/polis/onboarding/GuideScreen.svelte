<script lang="ts">
	import type { Snippet } from 'svelte';
	import ChevronRight from '@lucide/svelte/icons/chevron-right';
	import { Button } from '$lib/components/ui/button';
	import { focusOnScreenChange } from '$lib/utils/focusOnScreenChange';

	type Props = {
		title: string;
		next: string;
		onNext: () => void;
		children?: Snippet;
		/** Extra actions beside Next, e.g. "Add your own". */
		actions?: Snippet;
		/** Off for the screen that opens the step, so a fresh visit starts at the top of the page. */
		focusTitle?: boolean;
	};

	let { title, next, onNext, children, actions, focusTitle = true }: Props = $props();
</script>

<div class="flex w-full flex-col items-start gap-6">
	<h2
		tabindex="-1"
		{@attach focusTitle && focusOnScreenChange}
		class="text-card-foreground text-2xl leading-snug font-semibold outline-none sm:text-3xl"
	>
		{title}
	</h2>
	{@render children?.()}
	<div class="flex w-full flex-col items-stretch gap-3 sm:w-auto sm:flex-row sm:items-center">
		<Button size="lg" class="h-12 gap-2 px-6 text-lg" onclick={onNext}>
			{next}
			<ChevronRight aria-hidden="true" class="size-5 rtl:-scale-x-100" />
		</Button>
		{@render actions?.()}
	</div>
</div>
