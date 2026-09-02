<script lang="ts">
	import { Button } from '$lib/components/ui/button';

	let {
		label,
		onForward
	}: {
		/** Next while pages remain, then Start on the cover or Close in the hint. */
		label: string;
		onForward: () => void;
	} = $props();
</script>

<!-- What the step brief gets instead of the pager (ADR-0023). A brief has one way through it,
	so forward is the full width of the bar and there is nothing else in it. -->
<div class="bg-background">
	<div class="mx-auto flex w-full max-w-5xl flex-col items-center gap-4 px-4 pt-4 pb-5 md:px-6">
		<Button class="h-12 w-full max-w-md text-base" onclick={onForward}>{label}</Button>
	</div>
</div>
