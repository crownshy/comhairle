<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Info } from 'lucide-svelte';

	type Props = { tourHref: string };

	let { tourHref }: Props = $props();

	// Prototype only: a real version would remember the dismissal per admin.
	let dismissed = $state(false);
</script>

{#if !dismissed}
	<div class="bg-accent flex flex-wrap items-center justify-between gap-4 rounded-2xl px-5 py-4">
		<div class="flex min-w-0 flex-[1_1_28rem] items-center gap-3">
			<Info class="text-primary size-5 shrink-0" aria-hidden="true" />
			<p class="text-base">First time here? These numbers update every few minutes.</p>
		</div>
		<div class="flex gap-2">
			<Button href={tourHref} class="rounded-full">Take a 1-minute tour</Button>
			<Button variant="ghost" class="rounded-full" onclick={() => (dismissed = true)}
				>Dismiss</Button
			>
		</div>
	</div>
{/if}
