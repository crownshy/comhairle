<script lang="ts">
	import { ChevronDown, Check } from '@lucide/svelte';
	import * as Popover from '$lib/components/ui/popover';

	interface Props {
		/** Column header label, e.g. "Author". */
		label: string;
		/** Popover option label, phrased as the positive action, e.g. "Include all statements". */
		optionLabel: string;
		/** Controlled — checked = the option is INCLUDED (shown). */
		checked?: boolean;
		align?: 'left' | 'center' | 'right';
		onchange?: (next: boolean) => void;
	}

	let { label, optionLabel, checked = false, align = 'left', onchange }: Props = $props();

	const justify = {
		left: 'justify-start',
		center: 'justify-center',
		right: 'justify-end'
	} as const;

	function toggle() {
		onchange?.(!checked);
	}
</script>

<Popover.Root>
	<Popover.Trigger
		class={`text-foreground hover:text-foreground/70 flex w-full cursor-pointer items-center gap-1 text-sm font-semibold whitespace-nowrap uppercase transition-colors ${justify[align]}`}
	>
		{label}
		<ChevronDown class="size-3 shrink-0" />
	</Popover.Trigger>
	<Popover.Content align="start" class="w-72 overflow-hidden rounded-[10px] p-0 shadow-md">
		<button
			type="button"
			onclick={toggle}
			class="hover:bg-muted/50 border-border flex w-full cursor-pointer items-center justify-between gap-3 border-b px-4 py-3 text-left transition-colors"
		>
			<span class="text-foreground text-base font-medium whitespace-nowrap"
				>{optionLabel}</span
			>
			{#if checked}<Check class="text-primary size-5 shrink-0" />{/if}
		</button>
	</Popover.Content>
</Popover.Root>
