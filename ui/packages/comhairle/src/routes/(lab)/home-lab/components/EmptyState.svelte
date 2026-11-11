<script lang="ts">
	import { BellRing, MessagesSquare } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';

	let { orgName }: { orgName: string } = $props();
	let subscribed = $state(false);
</script>

<section
	class="bg-card border-border flex flex-col items-center gap-4 rounded-3xl border px-6 py-10 text-center"
>
	<span class="bg-primary/10 text-primary flex size-16 items-center justify-center rounded-full">
		<MessagesSquare class="size-8" />
	</span>
	<h2 class="text-2xl font-bold">No open conversations right now</h2>
	<p class="text-muted-foreground max-w-md text-base">
		{orgName} opens new conversations a few times a year. We can let you know when the next one starts.
	</p>
	{#if subscribed}
		<p class="text-primary text-base font-semibold">Thanks, we’ll email you.</p>
	{:else}
		<Button
			class="h-12 rounded-full px-6 text-base font-bold"
			onclick={() => (subscribed = true)}
		>
			<BellRing class="size-5" /> Tell me when one opens
		</Button>
	{/if}
</section>
