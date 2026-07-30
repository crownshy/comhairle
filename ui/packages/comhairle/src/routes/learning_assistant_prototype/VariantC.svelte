<!--
	PROTOTYPE - throwaway. Variant C: "Example prompts + trust bar". Leads with a
	grid of tappable starter questions (show-don't-tell of what it can do), a bold
	one-line purpose, and an ALWAYS-visible slim trust strip about data. Sources are
	rendered inline and richly. Reflows from 1 col (sidebar) to 2 cols (page width).

	NB: data-handling copy is placeholder pending confirmation from the team.
-->
<script lang="ts">
	import { Compass, ArrowUpRight, ShieldCheck, FileText } from 'lucide-svelte';
	import type { SampleQA } from './sample';

	let { qas, prompts, width }: { qas: SampleQA[]; prompts: string[]; width: 'sidebar' | 'page' } =
		$props();

	let inputVal = $state('');
	function pick(p: string) {
		inputVal = p;
	}
</script>

<div class="flex min-h-0 flex-1 flex-col">
	<!-- Bold purpose line -->
	<div class="mb-4 flex items-center gap-2.5">
		<Compass class="text-primary size-6 shrink-0" />
		<h2 class="text-foreground text-xl font-bold tracking-tight">
			Not sure where to start? Just ask.
		</h2>
	</div>
	<p class="text-muted-foreground mb-4 text-base leading-relaxed">
		This assistant reads the documents behind this consultation for you and answers in plain
		language - with the sources, so you can check its work.
	</p>

	<!-- Example prompt grid: primary affordance -->
	<p class="text-muted-foreground mb-2 text-xs font-semibold tracking-wide uppercase">
		Try asking
	</p>
	<div class="mb-4 grid gap-2 {width === 'page' ? 'grid-cols-2' : 'grid-cols-1'}">
		{#each prompts as p (p)}
			<button
				type="button"
				class="border-border bg-card hover:border-primary hover:bg-primary/5 group flex items-center gap-2 rounded-xl border px-3.5 py-3 text-left transition-colors"
				onclick={() => pick(p)}
			>
				<span class="text-foreground flex-1 text-base leading-snug">{p}</span>
				<ArrowUpRight
					class="text-muted-foreground group-hover:text-primary size-4 shrink-0"
				/>
			</button>
		{/each}
	</div>

	<!-- Input -->
	<div
		class="border-input bg-background focus-within:border-ring focus-within:ring-ring/50 mb-3 flex items-center gap-2 rounded-xl border px-3 py-2 shadow-xs focus-within:ring-[3px]"
	>
		<input
			bind:value={inputVal}
			placeholder="…or type your own question"
			class="text-foreground placeholder:text-muted-foreground min-w-0 flex-1 border-none bg-transparent p-1 text-base outline-none"
		/>
		<button
			type="button"
			class="bg-primary text-primary-foreground shrink-0 rounded-lg px-3 py-1.5 text-sm font-semibold disabled:opacity-40"
			disabled={!inputVal.trim()}>Ask</button
		>
	</div>

	<!-- Always-visible trust strip -->
	<div
		class="border-primary/20 bg-primary/5 mb-4 flex items-start gap-2.5 rounded-lg border px-3 py-2.5"
	>
		<ShieldCheck class="text-primary mt-0.5 size-4 shrink-0" />
		<p class="text-muted-foreground text-sm leading-snug">
			Answers use <strong class="text-foreground font-medium"
				>only this consultation's documents</strong
			>, never the open web. Your questions stay private to your session and aren't shared
			with other participants.
		</p>
	</div>

	<!-- Answered questions, inline with rich sources -->
	{#if qas.length > 0}
		<div class="min-h-0 flex-1 space-y-3 overflow-y-auto">
			{#each qas as qa (qa.id)}
				<div class="border-border/60 rounded-xl border p-4">
					<p class="text-foreground mb-2 text-base font-semibold">{qa.question}</p>
					<p class="text-foreground/90 mb-3 text-[15px] leading-relaxed">{qa.answer}</p>
					<div class="space-y-1.5">
						{#each qa.sources as s (s.name)}
							<div
								class="border-border bg-muted/30 flex items-center gap-2 rounded-lg border px-2.5 py-2"
							>
								<FileText class="text-primary size-4 shrink-0" />
								<span class="text-foreground flex-1 truncate text-sm font-medium"
									>{s.name}</span
								>
								<span class="text-muted-foreground text-xs">View excerpt</span>
							</div>
						{/each}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>
