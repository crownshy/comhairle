<script lang="ts">
	/**
	 * Skeleton placeholder for the LearnTutor "You asked" area.
	 * Render while the chat session history (getSession) is loading.
	 */
</script>

<div class="space-y-3" aria-busy="true" aria-live="polite">
	<div class="border-border/60 rounded-xl border p-4">
		<div class="bg-muted/60 mb-2 h-2.5 w-20 animate-pulse rounded"></div>
		<div class="bg-muted/60 mb-3 h-4 w-3/4 animate-pulse rounded"></div>
		<div class="space-y-2">
			<div class="bg-muted/40 h-3 w-full animate-pulse rounded"></div>
			<div class="bg-muted/40 h-3 w-5/6 animate-pulse rounded"></div>
			<div class="bg-muted/40 h-3 w-2/3 animate-pulse rounded"></div>
		</div>
	</div>
	<p class="text-muted-foreground text-center text-xs">Loading tutor session…</p>
	<span class="sr-only">Loading tutor session</span>
</div>
