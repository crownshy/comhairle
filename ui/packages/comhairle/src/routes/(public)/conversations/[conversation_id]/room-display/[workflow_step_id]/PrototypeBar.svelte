<!--
	@component Transport controls for the demo's scripted run, pulled out of the
	display itself so no variant has to carry them in its layout. Dev builds only.

	Deliberately ugly against the page so nobody mistakes it for part of the display.
	Does not bind the arrow keys: the Deck is driven by them, and a clicker sends them.
-->
<script lang="ts">
	import type { RoomDisplayDriver } from '$lib/room-display/driver.svelte';
	import { Button } from '$lib/components/ui/button';

	type Props = {
		driver: RoomDisplayDriver;
	};

	let { driver }: Props = $props();

	const clock = $derived.by(() => {
		const totalMinutes = Math.floor(driver.playheadMs / 60_000);
		return `${Math.floor(totalMinutes / 60)}h ${String(totalMinutes % 60).padStart(2, '0')}m`;
	});
</script>

<div
	class="border-border bg-background fixed inset-x-0 bottom-4 z-50 mx-auto flex w-fit max-w-[calc(100vw-2rem)] flex-wrap items-center justify-center gap-2 rounded-full border px-3 py-2 shadow-lg"
>
	<span class="text-muted-foreground text-base font-medium">Demo</span>
	<span class="bg-border mx-1 h-6 w-px"></span>
	<span class="text-muted-foreground text-base tabular-nums">{clock}</span>
	<span class="text-muted-foreground text-base">{driver.stage}</span>
	<Button variant="ghost" size="sm" onclick={() => driver.toggle()}>
		{driver.playing ? 'Pause' : 'Play'}
	</Button>
	<Button variant="ghost" size="sm" onclick={() => driver.seek(0)}>Restart</Button>
	<Button variant="ghost" size="sm" onclick={() => driver.seekToStage('shaped')}>Shaped</Button>
	<Button
		variant="ghost"
		size="sm"
		onclick={() => (driver.mode === 'driven' ? driver.releaseControl() : driver.takeControl())}
	>
		{driver.mode === 'driven' ? 'Driven' : 'Ambient'}
	</Button>
</div>
