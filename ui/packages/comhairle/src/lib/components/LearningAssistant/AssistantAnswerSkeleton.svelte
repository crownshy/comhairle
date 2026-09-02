<script lang="ts">
	/**
	 * Placeholder for the focused answer card. Mirrors the frame AssistantAnswer sits in
	 * (rounded-2xl card, label, question, answer body) so the reveal swaps content and
	 * leaves the layout alone.
	 */
	import { Skeleton } from '$lib/components/ui/skeleton';
</script>

<div class="bg-card border-border rounded-2xl border p-5 shadow-sm" aria-busy="true">
	<Skeleton class="h-3.5 w-24" />
	<Skeleton class="mt-2 mb-4 h-6 w-3/4" />
	<div class="space-y-2.5">
		<Skeleton class="h-4 w-full" />
		<Skeleton class="h-4 w-5/6" />
		<Skeleton class="h-4 w-2/3" />
	</div>
</div>
