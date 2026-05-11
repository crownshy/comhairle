<script lang="ts">
	/**
	 * Skeleton placeholder for StepHeader (title + description area).
	 * Render while route data for the next step is loading.
	 */
	import { ChevronLeft, ChevronRight } from 'lucide-svelte';
</script>

<div
	class="bg-background w-full md:mx-auto md:max-w-4xl md:rounded-2xl md:px-6 md:pt-4"
	aria-busy="true"
	aria-live="polite"
>
	<div class="flex flex-col items-center">
		<div class="flex w-full items-center justify-between md:gap-12">
			<span class="text-muted-foreground shrink-0 p-2 opacity-20" aria-hidden="true">
				<ChevronLeft class="h-6 w-6 md:h-8 md:w-8" />
			</span>

			<div class="flex min-w-0 flex-1 flex-col items-center gap-2">
				<div class="bg-muted/60 h-3 w-20 animate-pulse rounded"></div>
				<div class="bg-muted/60 h-6 w-3/5 animate-pulse rounded md:h-7"></div>
			</div>

			<span class="text-muted-foreground shrink-0 p-2 opacity-20" aria-hidden="true">
				<ChevronRight class="h-6 w-6 md:h-8 md:w-8" />
			</span>
		</div>

		<div class="mx-auto mt-4 flex w-full max-w-3xl flex-col items-center gap-2">
			<div class="bg-muted/50 h-3 w-4/5 animate-pulse rounded"></div>
			<div class="bg-muted/50 h-3 w-3/5 animate-pulse rounded"></div>
		</div>
	</div>
	<span class="sr-only">Loading step…</span>
</div>
