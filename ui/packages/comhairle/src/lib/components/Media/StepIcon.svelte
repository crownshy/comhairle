<script lang="ts" module>
	export const StepProgress = {
		Waiting: -1,
		Loading: 0,
		Complete: 1
	} as const;
</script>

<script lang="ts">
	import Check from '@lucide/svelte/icons/check';
	import Minus from '@lucide/svelte/icons/minus';
	import X from '@lucide/svelte/icons/x';
	import type { IconProps } from '@lucide/svelte';
	import { Spinner } from '../ui/spinner';
	import { type Component } from 'svelte';

	type Props = {
		state: number;
		error?: boolean;
	};

	let { state = StepProgress.Waiting, error }: Props = $props();
</script>

{#snippet StateIcon(I: Component<IconProps>, classes: string)}
	<div class="flex size-5 items-center justify-center rounded-full {classes}">
		<I class="size-3" strokeWidth={4.5} />
	</div>
{/snippet}

{#if error}
	{@render StateIcon(X, 'bg-red-500/80 text-white')}
{:else if state >= StepProgress.Complete}
	{@render StateIcon(Check, 'bg-green-500/80 text-white')}
{:else if state === StepProgress.Loading}
	<Spinner class="text-primary size-5" strokeWidth={4} />
{:else if state <= StepProgress.Waiting}
	{@render StateIcon(Minus, 'bg-slate-500/80 text-white')}
{/if}
