<script lang="ts">
	import { Check } from '@lucide/svelte';

	/**
	 * Spreadsheet-style filter control. Renders as a bordered "dropdown" box with
	 * the label and a checkmark reflecting state; the whole box is the hitbox
	 * (per design feedback — not just an arrow). Less discoverable than a visible
	 * switch, which is an accepted trade-off for the spreadsheet-familiar pattern.
	 */
	interface Props {
		label: string;
		checked: boolean;
		onchange?: (next: boolean) => void;
	}

	let { label, checked = $bindable(false), onchange }: Props = $props();

	function toggle() {
		checked = !checked;
		onchange?.(checked);
	}
</script>

<button
	type="button"
	role="switch"
	aria-checked={checked}
	onclick={toggle}
	class="border-border bg-card hover:bg-muted/50 inline-flex cursor-pointer items-center justify-between gap-3 rounded-[10px] border px-3 py-2 text-xs font-medium shadow-sm transition-colors"
>
	<span class="text-foreground">{label}</span>
	<span
		class={`flex size-4 shrink-0 items-center justify-center rounded border transition-colors ${
			checked ? 'border-primary bg-primary/10 text-primary' : 'border-border text-transparent'
		}`}
	>
		<Check class="size-3" />
	</span>
</button>
