<script lang="ts">
	import { TimeField } from 'bits-ui';
	import {
		getLocalTimeZone,
		toCalendarDate,
		type DateValue,
		Time
	} from '@internationalized/date';
	import CalendarIcon from '@lucide/svelte/icons/calendar';
	import X from '@lucide/svelte/icons/x';
	import * as Popover from '$lib/components/ui/popover';
	import { Calendar } from '$lib/components/ui/calendar';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import {
		dateTimeBoundsError,
		dateTimePartsToIso,
		formatDateTime,
		isoToDateTimeParts,
		type DateTimeBounds
	} from '$lib/utils/date-time';

	type Props = {
		/** UTC ISO timestamp, or null when unset. */
		value: string | null;
		onValueChange: (value: string | null) => void;
		/** Accessible name, e.g. "Launch date". */
		label: string;
		/** Exclusive bounds as UTC ISO timestamps. */
		bounds?: DateTimeBounds;
		timeZone?: string;
		placeholder?: string;
		disabled?: boolean;
		id?: string;
		class?: string;
	};

	let {
		value,
		onValueChange,
		label,
		bounds = {},
		timeZone = getLocalTimeZone(),
		placeholder = 'Not set',
		disabled = false,
		id,
		class: className
	}: Props = $props();

	let open = $state(false);
	let draftDate = $state<DateValue | undefined>();
	let draftTime = $state<Time | undefined>();

	const draftIso = $derived(
		draftDate && draftTime
			? dateTimePartsToIso({ date: toCalendarDate(draftDate), time: draftTime }, timeZone)
			: null
	);
	const boundsError = $derived(draftIso ? dateTimeBoundsError(draftIso, bounds, timeZone) : null);
	const earliestDate = $derived(
		bounds.after ? isoToDateTimeParts(bounds.after, timeZone).date : undefined
	);
	const latestDate = $derived(
		bounds.before ? isoToDateTimeParts(bounds.before, timeZone).date : undefined
	);

	function handleOpenChange(isOpen: boolean) {
		if (!isOpen) return;
		const parts = value ? isoToDateTimeParts(value, timeZone) : undefined;
		draftDate = parts?.date;
		draftTime = parts?.time;
	}

	function confirm() {
		if (!draftIso || boundsError) return;
		onValueChange(draftIso);
		open = false;
	}

	function clear() {
		onValueChange(null);
		open = false;
	}
</script>

<div class={cn('flex items-center gap-2', className)}>
	<Popover.Root bind:open onOpenChange={handleOpenChange}>
		<Popover.Trigger
			{id}
			{disabled}
			aria-label={value ? `${label}: ${formatDateTime(value, timeZone)}` : `Set ${label}`}
			class={cn(
				buttonVariants({ variant: 'outline' }),
				'w-full max-w-xs justify-start text-base font-normal',
				!value && 'text-muted-foreground'
			)}
		>
			<CalendarIcon class="size-4 opacity-60" />
			{value ? formatDateTime(value, timeZone) : placeholder}
		</Popover.Trigger>
		<Popover.Content
			role="dialog"
			aria-label={label}
			class="flex w-auto flex-col gap-0 p-0"
			align="start"
		>
			<Calendar
				type="single"
				bind:value={draftDate}
				minValue={earliestDate}
				maxValue={latestDate}
				calendarLabel={label}
			/>
			<div class="border-border flex flex-col gap-1 border-t px-3 py-3">
				<TimeField.Root
					bind:value={draftTime}
					hourCycle={24}
					granularity="minute"
					locale="en-GB"
				>
					<div class="flex items-center justify-between gap-3">
						<TimeField.Label class="text-sm font-semibold">Time</TimeField.Label>
						<TimeField.Input
							class={cn(
								'border-input bg-background dark:bg-input/30 flex h-9 items-center rounded-lg border px-2 text-base shadow-xs',
								'focus-within:border-ring focus-within:ring-ring/50 focus-within:ring-[3px]'
							)}
						>
							{#snippet children({ segments })}
								{#each segments as { part, value: segmentValue }, index (part + index)}
									<TimeField.Segment
										{part}
										class={cn(
											'rounded-sm px-1 tabular-nums',
											'focus:bg-accent focus:text-accent-foreground focus:outline-none',
											'data-placeholder:text-muted-foreground',
											part === 'literal' && 'text-muted-foreground px-1.5'
										)}
									>
										{segmentValue}
									</TimeField.Segment>
								{/each}
							{/snippet}
						</TimeField.Input>
					</div>
				</TimeField.Root>
				<p class="text-muted-foreground text-sm">24-hour clock, {timeZone}</p>
				{#if boundsError}
					<p class="text-destructive text-sm" role="alert">{boundsError}</p>
				{/if}
			</div>
			<div class="border-border flex items-center gap-2 border-t px-3 py-3">
				{#if value}
					<Button variant="ghost" size="sm" class="text-destructive" onclick={clear}>
						Remove
					</Button>
				{/if}
				<Button variant="outline" size="sm" class="ml-auto" onclick={() => (open = false)}>
					Cancel
				</Button>
				<Button size="sm" disabled={!draftIso || !!boundsError} onclick={confirm}
					>Set</Button
				>
			</div>
		</Popover.Content>
	</Popover.Root>
	{#if value && !disabled}
		<Button
			variant="ghost"
			size="icon"
			aria-label={`Remove ${label}`}
			title={`Remove ${label}`}
			onclick={clear}
		>
			<X class="size-4" />
		</Button>
	{/if}
</div>
