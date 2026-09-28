<script lang="ts">
	import { Check, TriangleAlert } from 'lucide-svelte';
	import { Spinner } from '$lib/components/ui/spinner';
	import { cn } from '$lib/utils';
	import type { SaveStatus } from './autosave.svelte';

	type Props = {
		status: SaveStatus;
	};

	let { status }: Props = $props();

	const colorsByStatus: Record<SaveStatus, string> = {
		idle: '',
		saving: 'bg-primary text-primary-foreground',
		saved: 'bg-muted text-muted-foreground',
		error: 'bg-destructive/10 text-destructive'
	};
</script>

{#if status !== 'idle'}
	<span
		class={cn(
			'ml-auto flex items-center gap-1.5 rounded-full px-2.5 py-1 text-sm',
			colorsByStatus[status]
		)}
		aria-live="polite"
	>
		{#if status === 'saving'}
			<Spinner class="size-3.5" /> Saving…
		{:else if status === 'saved'}
			<Check class="size-3.5" /> Saved
		{:else}
			<TriangleAlert class="size-3.5" /> Not saved
		{/if}
	</span>
{/if}
