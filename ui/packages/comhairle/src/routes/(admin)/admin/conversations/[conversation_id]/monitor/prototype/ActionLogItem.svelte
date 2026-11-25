<script lang="ts">
	import { cn } from '$lib/utils';
	import { ACTION_KINDS, formatActionDay } from './actionKinds';
	import type { ActivityLogEntry } from './sampleMonitorData';

	type Props = { entry: ActivityLogEntry };

	let { entry }: Props = $props();

	let kind = $derived(ACTION_KINDS[entry.kind]);
</script>

<!-- One logged action, as listed in the signups chart's key panel. -->
<div
	class={cn(
		'bg-card flex items-start gap-3 rounded-xl border p-3',
		entry.scheduled && 'border-dashed'
	)}
>
	<span
		class={cn('inline-flex size-9 shrink-0 items-center justify-center rounded-lg', kind.tile)}
	>
		<kind.icon class="size-4" aria-hidden="true" />
	</span>
	<div class="flex min-w-0 flex-1 flex-col">
		<span class="text-base font-semibold">{entry.title}</span>
		{#if entry.detail}
			<span class="text-muted-foreground text-sm">{entry.detail}</span>
		{/if}
	</div>
	<span class="text-muted-foreground shrink-0 text-right text-sm">
		{formatActionDay(entry.date)}
		{#if entry.scheduled}<br /><span class="font-semibold">Planned</span>{/if}
	</span>
</div>
