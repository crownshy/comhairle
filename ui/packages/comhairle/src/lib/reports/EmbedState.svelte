<script lang="ts">
	import Loader from 'lucide-svelte/icons/loader-circle';

	/** Loading and unavailable shells shared by every tool's live report embed. */
	let { state }: { state: 'loading' | 'unavailable' } = $props();
</script>

{#if state === 'loading'}
	<div
		class="border-border text-muted-foreground flex items-center justify-center gap-2 rounded-xl border border-dashed p-8 text-base"
	>
		<Loader class="size-4 animate-spin" />
		Loading component…
	</div>
{:else}
	<div
		class="border-border bg-card text-muted-foreground rounded-xl border border-dashed p-8 text-center text-base"
	>
		This component's data is no longer available.
	</div>
{/if}
