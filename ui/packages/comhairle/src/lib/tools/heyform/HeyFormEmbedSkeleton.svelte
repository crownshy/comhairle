<!--
	Stands in for the participant-facing form while its cross-origin iframe boots.
-->
<div
	class="mx-auto mt-1 min-h-110 w-full max-w-2xl overflow-hidden rounded-xl bg-white"
	aria-busy="true"
	aria-live="polite"
>
	<div class="flex h-full flex-col justify-start gap-4 px-8 pt-10 sm:px-12 sm:pt-14">
		<!-- Question number / label -->
		<div class="h-3 w-16 animate-pulse rounded bg-neutral-200"></div>

		<!-- Question -->
		<div class="h-7 w-3/4 animate-pulse rounded bg-neutral-200"></div>
		<div class="h-4 w-1/2 animate-pulse rounded bg-neutral-200/70"></div>

		<!-- Answer choices -->
		<div class="mt-4 flex flex-col gap-3">
			<div class="h-11 w-full animate-pulse rounded-md bg-neutral-200/70"></div>
			<div class="h-11 w-full animate-pulse rounded-md bg-neutral-200/70"></div>
			<div class="h-11 w-2/3 animate-pulse rounded-md bg-neutral-200/70"></div>
		</div>

		<!-- Submit -->
		<div class="mt-4 h-10 w-28 animate-pulse rounded-md bg-neutral-200"></div>
	</div>

	<span class="sr-only">Loading survey…</span>
</div>
