<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Spinner } from '$lib/components/ui/spinner';
	import { m } from '$lib/paraglide/messages';
	import { cn } from '$lib/utils';
	import { STEP_COLUMN_CLASS } from './styles';

	type Props = {
		/** Proceed lands on the thank-you page, so the button says Finish. */
		finishes?: boolean;
		loading?: boolean;
		onProceed: () => void;
	};

	let { finishes = false, loading = false, onProceed }: Props = $props();
</script>

<!-- The pager's slot on the completion screen. The single move sits in the middle (ADR-0048). -->
<div class={cn(STEP_COLUMN_CLASS, 'flex h-20 items-center')}>
	<Button
		class="h-12 w-full text-base active:scale-[0.97] motion-reduce:active:scale-100"
		disabled={loading}
		aria-busy={loading}
		onclick={onProceed}
	>
		{#if loading}
			<Spinner class="size-5" />
		{/if}
		{finishes ? m.finish() : m.step_complete_proceed()}
	</Button>
</div>
