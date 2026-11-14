<script lang="ts" module>
	export const StepProgress = {
		Error: -2,
		Waiting: -1,
		Loading: 0,
		Complete: 1
	} as const;
</script>

<script lang="ts">
	import { Check, type Icon, Minus, X } from 'lucide-svelte';
	import { Spinner } from '../ui/spinner';
	import { type ComponentType } from 'svelte';

	type Props = {
		state: number;
	};

	let { state = StepProgress.Waiting }: Props = $props();
</script>

{#snippet StateIcon(I: ComponentType<Icon>, classes: string)}
	<div class="flex size-5 items-center justify-center rounded-full {classes}">
		<I class="size-3" strokeWidth={4.5} />
	</div>
{/snippet}

{#if state >= StepProgress.Complete}
	{@render StateIcon(Check, 'bg-green-500/80 text-white')}
{:else if state === StepProgress.Loading}
	<Spinner class="text-primary size-5" strokeWidth={4} />
{:else if state === StepProgress.Waiting}
	{@render StateIcon(Minus, 'bg-slate-500/80 text-white')}
{:else if state <= StepProgress.Error}
	{@render StateIcon(X, 'bg-red-500/80 text-white')}
{/if}
