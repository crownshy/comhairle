<script lang="ts">
	import { TimeField } from 'bits-ui';
	import { Time } from '@internationalized/date';
	import { cn } from '$lib/utils.js';

	type Props = {
		/** Value in HH:mm 24-hour format (e.g. "14:30"). */
		value?: string;
		onValueChange?: (value: string) => void;
		name?: string;
		id?: string;
		/** 12 (AM/PM) or 24. Defaults to 12. */
		hourCycle?: 12 | 24;
		disabled?: boolean;
		class?: string;
		'aria-invalid'?: boolean;
	};

	let {
		value = $bindable(''),
		onValueChange,
		name,
		id,
		hourCycle = 12,
		disabled = false,
		class: className,
		'aria-invalid': ariaInvalid
	}: Props = $props();

	function stringToTime(s: string | undefined): Time | undefined {
		if (!s) return undefined;
		const [h, m] = s.split(':').map(Number);
		if (Number.isNaN(h) || Number.isNaN(m)) return undefined;
		return new Time(h, m);
	}

	function timeToString(t: Time | undefined): string {
		if (!t) return '';
		return `${String(t.hour).padStart(2, '0')}:${String(t.minute).padStart(2, '0')}`;
	}

	/**
	 * Pure-derived: bits-ui receives a fresh Time built from the parent's string. The
	 * `nextString !== value` guard in onValueChange breaks the controlled-echo cycle.
	 */
	let internalValue = $derived(stringToTime(value));
</script>

<TimeField.Root
	value={internalValue}
	onValueChange={(t) => {
		const nextString = timeToString(t as Time | undefined);
		if (nextString !== value) {
			value = nextString;
			onValueChange?.(nextString);
		}
	}}
	{hourCycle}
	{disabled}
	granularity="minute"
>
	<TimeField.Input
		{id}
		{name}
		aria-invalid={ariaInvalid}
		class={cn(
			'border-input bg-background ring-offset-background placeholder:text-muted-foreground flex h-9 w-full min-w-0 items-center rounded-lg border px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm',
			'focus-within:border-ring focus-within:ring-ring/50 focus-within:ring-[3px]',
			'aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive',
			className
		)}
	>
		{#snippet children({ segments })}
			{#each segments as { part, value: segValue }, i (i)}
				<TimeField.Segment
					{part}
					class={cn(
						'rounded-sm px-1 tabular-nums',
						'focus:bg-accent focus:text-accent-foreground focus:outline-none',
						'data-placeholder:text-muted-foreground',
						part === 'literal' && 'text-muted-foreground px-0'
					)}
				>
					{segValue}
				</TimeField.Segment>
			{/each}
		{/snippet}
	</TimeField.Input>
</TimeField.Root>
