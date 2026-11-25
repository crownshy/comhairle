<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import ActionHint from './ActionHint.svelte';

	type Props = { tourHref: string };

	let { tourHref }: Props = $props();

	// TODO: remember the dismissal per admin (it resets on reload).
	let dismissed = $state(false);
</script>

{#if !dismissed}
	<ActionHint tone="info">
		First time here? These numbers update every few minutes.
		{#snippet actions()}
			<Button href={tourHref} class="rounded-full">Take a 1-minute tour</Button>
			<Button variant="ghost" class="rounded-full" onclick={() => (dismissed = true)}
				>Dismiss</Button
			>
		{/snippet}
	</ActionHint>
{/if}
