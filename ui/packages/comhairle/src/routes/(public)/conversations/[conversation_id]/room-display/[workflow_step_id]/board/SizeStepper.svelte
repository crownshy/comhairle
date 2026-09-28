<!-- @component Minus and plus buttons around a size multiplier. -->
<script lang="ts">
	import { Minus, Plus } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';

	type Props = {
		value: number;
		min: number;
		max: number;
		step: number;
		/** What is being sized, for the buttons' accessible names. */
		label: string;
		onchange: (value: number) => void;
	};

	let { value, min, max, step, label, onchange }: Props = $props();

	// Rounded to two decimals so float noise like 1.1500000000000001 never reaches the board.
	function nudge(direction: 1 | -1) {
		const next = Math.min(max, Math.max(min, value + direction * step));
		onchange(Number(next.toFixed(2)));
	}
</script>

<div class="flex items-center gap-1" role="group" aria-label="{label} size">
	<Button
		variant="outline"
		size="icon"
		class="size-8"
		disabled={value <= min}
		aria-label="Make {label} smaller"
		onclick={() => nudge(-1)}
	>
		<Minus />
	</Button>
	<span class="text-foreground w-14 text-center text-base tabular-nums">
		{value.toFixed(2)}&times;
	</span>
	<Button
		variant="outline"
		size="icon"
		class="size-8"
		disabled={value >= max}
		aria-label="Make {label} bigger"
		onclick={() => nudge(1)}
	>
		<Plus />
	</Button>
</div>
