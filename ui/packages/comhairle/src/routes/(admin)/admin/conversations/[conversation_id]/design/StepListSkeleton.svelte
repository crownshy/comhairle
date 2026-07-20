<script lang="ts">
	import { Skeleton } from '$lib/components/ui/skeleton';

	type Props = {
		/** How many placeholder step cards to render. */
		count?: number;
	};

	let { count = 3 }: Props = $props();
</script>

<!-- Loading placeholder for the process-steps list. Mirrors the real step card
	 (grip, numbered badge, name + type lines, actions) so the list doesn't shift
	 when the steps arrive. -->
<div class="flex flex-col gap-2.5" aria-hidden="true">
	{#each Array(count) as _, i (i)}
		<!-- Brand-tinted shimmer so it reads against the card instead of gray-on-gray. -->
		<div class="border-border bg-card flex items-center gap-4 rounded-xl border p-4">
			<Skeleton class="bg-primary/10 size-5 shrink-0 rounded" />
			<Skeleton class="bg-primary/20 size-6 shrink-0 rounded-lg" />
			<div class="flex min-w-0 flex-1 flex-col gap-2">
				<Skeleton class="bg-primary/15 h-5 w-48 max-w-full rounded-md" />
				<Skeleton class="bg-primary/10 h-4 w-24 rounded-md" />
			</div>
			<Skeleton class="bg-primary/10 size-9 shrink-0 rounded-md" />
		</div>
	{/each}
</div>
