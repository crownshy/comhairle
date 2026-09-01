<!--
	Stand-in for the voting screen while the first statement is fetched. It mirrors that
	screen band for band, vh clamps included, so the placeholders sit where the statement,
	the thumbs and "add your own" are about to appear instead of collapsing to a strip of
	grey lines floating in an empty card.
-->
<div class="flex w-full flex-1 flex-col" aria-busy="true" aria-live="polite">
	<div
		class="mx-auto flex w-full max-w-[808px] flex-1 animate-pulse flex-col items-start gap-[clamp(0.75rem,2.2vh,1.5rem)] px-6 py-[clamp(1rem,2.5vh,3rem)] md:px-16"
	>
		<!-- Statement -->
		<div class="flex w-full flex-1 items-center">
			<div class="flex w-full flex-col gap-3">
				<div
					class="bg-foreground/10 h-[clamp(1.25rem,3.2vh,2rem)] w-full rounded sm:h-[clamp(1.5rem,4vh,2.25rem)]"
				></div>
				<div
					class="bg-foreground/10 h-[clamp(1.25rem,3.2vh,2rem)] w-4/5 rounded sm:h-[clamp(1.5rem,4vh,2.25rem)]"
				></div>
				<div
					class="bg-foreground/10 h-[clamp(1.25rem,3.2vh,2rem)] w-3/5 rounded sm:h-[clamp(1.5rem,4vh,2.25rem)]"
				></div>
			</div>
		</div>

		<!-- Agree / disagree -->
		<div class="flex w-full max-w-[560px] items-center justify-between">
			<div
				class="bg-primary/20 aspect-square w-[44%] max-w-[clamp(5rem,17vh,148px)] shrink-0 rounded-full"
			></div>
			<div
				class="bg-primary/20 aspect-square w-[44%] max-w-[clamp(5rem,17vh,148px)] shrink-0 rounded-full"
			></div>
		</div>
	</div>

	<!-- Add your own opinion -->
	<div
		class="bg-background flex w-full shrink-0 animate-pulse flex-col items-center gap-2 rounded-b-2xl border-t px-6 py-[clamp(0.75rem,2vh,1.25rem)]"
	>
		<div class="bg-foreground/10 h-5 w-56 rounded [@media(max-height:700px)]:hidden"></div>
		<div class="bg-primary/20 h-6 w-44 rounded"></div>
	</div>

	<span class="sr-only">Loading opinions…</span>
</div>
