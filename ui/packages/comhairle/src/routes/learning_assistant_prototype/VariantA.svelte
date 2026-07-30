<!--
	PROTOTYPE - throwaway. Variant A: "Guided intro" - single column, progressive
	disclosure. Purpose header → capability list → input as hero → collapsible
	"how your data is used" note → Q&A history. Calm, low-commitment onboarding.

	NB: data-handling copy is placeholder pending confirmation from the team.
-->
<script lang="ts">
	import { Sparkles, BookOpen, Quote, ShieldCheck, ChevronDown, FileText } from 'lucide-svelte';
	import type { SampleQA } from './sample';

	let { qas }: { qas: SampleQA[] } = $props();

	let inputVal = $state('');
	let dataOpen = $state(false);

	const capabilities = [
		{
			icon: BookOpen,
			text: 'Ask anything about this consultation and the documents behind it'
		},
		{ icon: Quote, text: 'Get plain-language answers with the exact sources they came from' },
		{ icon: Sparkles, text: 'Great for catching up before you share your view' }
	];
</script>

<div class="flex min-h-0 flex-1 flex-col">
	<!-- Purpose header -->
	<div class="mb-4 flex items-start gap-3">
		<div
			class="bg-primary/10 text-primary flex size-10 shrink-0 items-center justify-center rounded-xl"
		>
			<Sparkles class="size-5" />
		</div>
		<div>
			<h2 class="text-foreground text-lg font-semibold">Learning assistant</h2>
			<p class="text-muted-foreground text-base leading-snug">
				A guide to help you understand this consultation before you take part.
			</p>
		</div>
	</div>

	<!-- Capability list -->
	<ul class="border-border bg-card mb-4 space-y-2.5 rounded-xl border p-4">
		{#each capabilities as cap (cap.text)}
			<li class="flex items-start gap-3">
				<cap.icon class="text-primary mt-0.5 size-4 shrink-0" />
				<span class="text-foreground text-base leading-snug">{cap.text}</span>
			</li>
		{/each}
	</ul>

	<!-- Input as hero -->
	<div
		class="border-ring bg-background focus-within:ring-ring/50 mb-3 flex items-center gap-2 rounded-xl border px-3 py-2 shadow-xs focus-within:ring-[3px]"
	>
		<input
			bind:value={inputVal}
			placeholder="Ask a question, e.g. “what is being decided?”"
			class="text-foreground placeholder:text-muted-foreground min-w-0 flex-1 border-none bg-transparent p-1 text-base outline-none"
		/>
		<button
			type="button"
			class="bg-primary text-primary-foreground shrink-0 rounded-lg px-3 py-1.5 text-sm font-semibold disabled:opacity-40"
			disabled={!inputVal.trim()}>Ask</button
		>
	</div>

	<!-- Collapsible data note -->
	<div class="border-border mb-4 rounded-xl border">
		<button
			type="button"
			class="flex w-full items-center gap-2.5 px-4 py-3 text-left"
			aria-expanded={dataOpen}
			onclick={() => (dataOpen = !dataOpen)}
		>
			<ShieldCheck class="text-primary size-4 shrink-0" />
			<span class="text-foreground flex-1 text-base font-medium"
				>What happens to your questions</span
			>
			<ChevronDown
				class="text-muted-foreground size-4 transition-transform {dataOpen
					? 'rotate-180'
					: ''}"
			/>
		</button>
		{#if dataOpen}
			<div
				class="text-muted-foreground border-border/60 space-y-2 border-t px-4 py-3 text-base leading-relaxed"
			>
				<p>
					Answers come <strong class="text-foreground font-medium">only</strong> from the documents
					attached to this consultation. The assistant doesn't search the web or invent facts.
				</p>
				<p>
					Your questions are saved to your own session so you can return to them. They are
					used to answer you, not published to other participants.
				</p>
				<p>
					It can still get things wrong. Always check the linked sources before relying on
					an answer.
				</p>
			</div>
		{/if}
	</div>

	<!-- Q&A history -->
	{#if qas.length > 0}
		<p class="text-muted-foreground mb-2 text-xs font-semibold tracking-wide uppercase">
			Your questions
		</p>
		<div class="min-h-0 flex-1 space-y-3 overflow-y-auto">
			{#each qas as qa (qa.id)}
				<div class="border-border/60 bg-card rounded-xl border p-4">
					<div class="mb-1 flex items-center gap-2">
						<p class="text-primary text-[11px] font-semibold tracking-wide uppercase">
							You asked
						</p>
						<span class="text-muted-foreground text-[11px]">· {qa.timeLabel}</span>
					</div>
					<p class="text-foreground mb-2 text-base font-semibold italic">
						"{qa.question}"
					</p>
					<p class="text-foreground/90 text-[15px] leading-relaxed">{qa.answer}</p>
					<div class="mt-3 flex flex-wrap gap-1.5">
						{#each qa.sources as s (s.name)}
							<span
								class="border-border bg-muted/40 text-foreground inline-flex max-w-full items-center gap-1.5 rounded-md border px-2 py-1 text-xs font-medium"
							>
								<FileText class="size-3 shrink-0" />
								<span class="truncate">{s.name}</span>
							</span>
						{/each}
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>
