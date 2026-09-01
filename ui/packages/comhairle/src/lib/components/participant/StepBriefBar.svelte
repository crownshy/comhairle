<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import SlideDots from './SlideDots.svelte';

	let {
		index,
		count,
		label,
		onForward
	}: {
		index: number;
		count: number;
		/** Next while slides remain, then Start on the cover or Close in the hint. */
		label: string;
		onForward: () => void;
	} = $props();
</script>

<!-- What the step brief gets instead of the pager (ADR-0023). A deck has one way through it,
	so forward is the full width of the bar and the dots say how far in the reader is. -->
<div class="bg-background border-t md:border-t-0">
	<div class="mx-auto flex w-full max-w-5xl flex-col items-center gap-4 px-4 pt-4 pb-5 md:px-6">
		<SlideDots {index} {count} />
		<Button class="h-12 w-full max-w-md text-base" onclick={onForward}>{label}</Button>
	</div>
</div>
