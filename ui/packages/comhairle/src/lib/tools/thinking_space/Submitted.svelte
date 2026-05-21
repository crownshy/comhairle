<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Check } from 'lucide-svelte';

	type Props = {
		claimCount: number;
		onDone?: () => void;
	};

	let { claimCount, onDone }: Props = $props();
</script>

<div class="mx-auto flex max-w-md flex-col items-center px-6 py-16 text-center">
	<div
		class="bg-primary/10 text-primary mb-6 flex size-16 items-center justify-center rounded-full"
	>
		<Check class="size-8" />
	</div>
	<h2 class="text-foreground text-3xl font-semibold tracking-tight">Thank you</h2>
	<p class="text-foreground mt-2 text-base">
		{claimCount} view{claimCount === 1 ? '' : 's'} submitted.
	</p>
	<p class="text-muted-foreground mx-auto mt-3 max-w-sm text-sm leading-relaxed">
		You can come back later to see what others think and revise your views.
	</p>

	<div class="mt-8 flex flex-wrap justify-center gap-3">
		{#if onDone}
			<Button onclick={onDone}>Continue</Button>
		{/if}
	</div>
</div>
