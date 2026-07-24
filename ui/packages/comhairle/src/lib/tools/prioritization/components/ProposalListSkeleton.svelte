<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Skeleton } from '$lib/components/ui/skeleton';

	type Props = {
		/** How many placeholder proposal cards to render. */
		count?: number;
	};

	let { count = 3 }: Props = $props();
</script>

<!-- Loading placeholder for the proposals list. Mirrors the real card (title, a
	 couple of section lines, Edit/Delete actions) so the list doesn't shift when
	 the proposals arrive. -->
<ul class="space-y-3" aria-hidden="true">
	{#each Array(count) as _, i (i)}
		<Card.Root>
			<Card.Header class="flex flex-row items-start justify-between gap-4">
				<div class="min-w-0 flex-1 space-y-2.5">
					<Skeleton class="bg-primary/15 h-6 w-56 max-w-full rounded-md" />
					<Skeleton class="bg-primary/10 h-4 w-full max-w-md rounded-md" />
					<Skeleton class="bg-primary/10 h-4 w-2/3 max-w-sm rounded-md" />
				</div>
				<div class="flex shrink-0 gap-2">
					<Skeleton class="bg-primary/10 h-9 w-20 rounded-md" />
					<Skeleton class="bg-primary/10 h-9 w-20 rounded-md" />
				</div>
			</Card.Header>
		</Card.Root>
	{/each}
</ul>
