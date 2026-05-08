<script lang="ts">
	import { TimeRangeField } from 'bits-ui';
	import { Time } from '@internationalized/date';
	import { cn } from '$lib/utils.js';

	type Props = {
		/** Start time in HH:mm 24-hour format. */
		startValue?: string;
		/** End time in HH:mm 24-hour format. */
		endValue?: string;
		onStartValueChange?: (value: string) => void;
		onEndValueChange?: (value: string) => void;
		startName?: string;
		endName?: string;
		startId?: string;
		endId?: string;
		hourCycle?: 12 | 24;
		disabled?: boolean;
		class?: string;
		'aria-invalid'?: boolean;
	};

	let {
		startValue = $bindable(''),
		endValue = $bindable(''),
		onStartValueChange,
		onEndValueChange,
		startName,
		endName,
		startId,
		endId,
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
	 * Pure-derived: bits-ui receives fresh Time objects built from the parent's strings.
	 * The `next* !== *Value` guards in onValueChange break the controlled-echo cycle.
	 */
	let internalValue = $derived({
		start: stringToTime(startValue),
		end: stringToTime(endValue)
	});

	const types = ['start', 'end'] as const;
</script>

<TimeRangeField.Root
	value={internalValue}
	onValueChange={(v) => {
		const next = v ?? { start: undefined, end: undefined };
		const nextStart = timeToString(next.start as Time | undefined);
		const nextEnd = timeToString(next.end as Time | undefined);
		if (nextStart !== startValue) {
			startValue = nextStart;
			onStartValueChange?.(nextStart);
		}
		if (nextEnd !== endValue) {
			endValue = nextEnd;
			onEndValueChange?.(nextEnd);
		}
	}}
	{hourCycle}
	{disabled}
	granularity="minute"
	aria-invalid={ariaInvalid}
	class={cn(
		'border-input bg-background flex h-9 w-full max-w-xs items-center rounded-lg border px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm',
		'focus-within:border-ring focus-within:ring-ring/50 focus-within:ring-[3px]',
		'aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive',
		'data-invalid:ring-destructive/20 dark:data-invalid:ring-destructive/40 data-invalid:border-destructive',
		className
	)}
>
	{#each types as type (type)}
		<TimeRangeField.Input
			{type}
			id={type === 'start' ? startId : endId}
			name={type === 'start' ? startName : endName}
			class="contents"
		>
			{#snippet children({ segments })}
				{#each segments as { part, value: segValue }, i (part + i)}
					<TimeRangeField.Segment
						{part}
						class={cn(
							'rounded-sm px-1 tabular-nums',
							'focus:bg-accent focus:text-accent-foreground focus:outline-none',
							'data-placeholder:text-muted-foreground',
							part === 'literal' && 'text-muted-foreground px-0'
						)}
					>
						{segValue}
					</TimeRangeField.Segment>
				{/each}
			{/snippet}
		</TimeRangeField.Input>
		{#if type === 'start'}
			<span aria-hidden="true" class="text-muted-foreground px-2">to</span>
		{/if}
	{/each}
</TimeRangeField.Root>
