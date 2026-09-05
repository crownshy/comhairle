<!--
	@component PROTOTYPE harness. The variant switcher and the scenario controls, pulled
	out of the display itself so no variant has to carry them in its layout.

	Deliberately ugly against the page so nobody mistakes it for part of a design being
	judged.

	Cycles on `[` and `]` rather than the arrow keys the switcher usually takes: the Deck
	variant is being judged partly on whether arrow-key advancing feels right for a
	facilitator with a clicker, so the harness must not eat them.
-->
<script lang="ts">
	import type { RoomDisplayDriver } from '$lib/room-display/driver.svelte';
	import { Button } from '$lib/components/ui/button';

	type Props = {
		variants: readonly { key: string; name: string }[];
		current: string;
		driver: RoomDisplayDriver;
		onselect: (key: string) => void;
	};

	let { variants, current, driver, onselect }: Props = $props();

	const index = $derived(
		Math.max(
			0,
			variants.findIndex((v) => v.key === current)
		)
	);
	const clock = $derived.by(() => {
		const totalMinutes = Math.floor(driver.playheadMs / 60_000);
		return `${Math.floor(totalMinutes / 60)}h ${String(totalMinutes % 60).padStart(2, '0')}m`;
	});

	function step(delta: number) {
		onselect(variants[(index + delta + variants.length) % variants.length].key);
	}

	function onkeydown(event: KeyboardEvent) {
		const target = event.target as HTMLElement | null;
		if (target?.closest('input, textarea, [contenteditable]')) return;
		if (event.key === ']') step(1);
		else if (event.key === '[') step(-1);
	}
</script>

<svelte:window {onkeydown} />

<div
	class="border-border bg-background fixed inset-x-0 bottom-4 z-50 mx-auto flex w-fit max-w-[calc(100vw-2rem)] flex-wrap items-center justify-center gap-2 rounded-full border px-3 py-2 shadow-lg"
>
	<Button variant="ghost" size="sm" onclick={() => step(-1)} aria-label="Previous variant">
		&larr;
	</Button>
	<span class="min-w-56 text-center text-base font-medium">
		{variants[index].key} · {variants[index].name}
	</span>
	<Button variant="ghost" size="sm" onclick={() => step(1)} aria-label="Next variant">
		&rarr;
	</Button>

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
	<span class="text-muted-foreground text-base">[ ]</span>
</div>
