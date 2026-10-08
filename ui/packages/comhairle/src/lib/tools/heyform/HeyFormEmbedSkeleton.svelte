<!--
	Stands in for the participant-facing form while its cross-origin iframe boots. Painted in the
	same tokens the form is handed (see embedTheme), so the handover does not flash a white sheet
	into a dark page.
-->
<div
	class="bg-card mx-auto mt-1 min-h-110 w-full max-w-2xl overflow-hidden rounded-xl"
	aria-busy="true"
	aria-live="polite"
>
	<div class="flex h-full flex-col justify-start gap-4 px-8 pt-10 sm:px-12 sm:pt-14">
		<!-- Question number / label -->
		<div class="bg-foreground/10 h-3 w-16 animate-pulse rounded"></div>

		<!-- Question -->
		<div class="bg-foreground/10 h-7 w-3/4 animate-pulse rounded"></div>
		<div class="bg-foreground/5 h-4 w-1/2 animate-pulse rounded"></div>

		<!-- Answer choices -->
		<div class="mt-4 flex flex-col gap-3">
			<div class="bg-foreground/5 h-11 w-full animate-pulse rounded-md"></div>
			<div class="bg-foreground/5 h-11 w-full animate-pulse rounded-md"></div>
			<div class="bg-foreground/5 h-11 w-2/3 animate-pulse rounded-md"></div>
		</div>

		<!-- Submit -->
		<div class="bg-primary/20 mt-4 h-10 w-28 animate-pulse rounded-md"></div>
	</div>

	<span class="sr-only">Loading survey…</span>
</div>
