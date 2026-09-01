<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Spinner } from '$lib/components/ui/spinner';
	import * as m from '$lib/paraglide/messages';

	let {
		loading = false,
		onProceed
	}: {
		loading?: boolean;
		onProceed: () => void;
	} = $props();
</script>

<!-- The pager's slot on the completion screen. One thing at a time (ADR-0018): nothing to
	page through here, so the arrows and the brief chip give way to the single action. -->
<div class="bg-background border-t md:border-t-0">
	<div class="mx-auto flex h-20 w-full max-w-5xl items-center px-4 md:px-6">
		<Button class="h-12 w-full text-base" disabled={loading} onclick={onProceed}>
			{#if loading}
				<Spinner class="size-5" />
			{/if}
			{m.step_complete_proceed()}
		</Button>
	</div>
</div>
