<script lang="ts">
	/**
	 * Skeleton placeholder for the LearnTutor input + "You asked" area.
	 * Render while the chat session history (getSession) is loading. It mirrors
	 * the real layout (input box then answer card) so nothing shifts on reveal.
	 */
</script>

<div aria-busy="true" aria-live="polite">
	<!-- Input field placeholder: reserves the same box the real input takes -->
	<div
		class="border-input bg-background mb-6 flex h-9 w-full items-center rounded-lg border px-3 py-1"
	>
		<div class="bg-muted/60 h-4 w-40 animate-pulse rounded"></div>
	</div>
	<div class="bg-card border-border/60 rounded-xl border p-4">
		<div class="bg-muted/60 mb-2 h-2.5 w-20 animate-pulse rounded"></div>
		<div class="bg-muted/60 mb-3 h-4 w-3/4 animate-pulse rounded"></div>
		<div class="space-y-2">
			<div class="bg-muted/40 h-3 w-full animate-pulse rounded"></div>
			<div class="bg-muted/40 h-3 w-5/6 animate-pulse rounded"></div>
			<div class="bg-muted/40 h-3 w-2/3 animate-pulse rounded"></div>
		</div>
	</div>
	<span class="sr-only">Loading learning assistant</span>
</div>
