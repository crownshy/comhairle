<!--
	PROTOTYPE - throwaway. Variant B: "About panel + chat thread". Info lives in a
	compact 3-section "About" accordion pinned to the top; the exchange reads as a
	real top-down conversation thread; the input is pinned to the BOTTOM (messenger
	style). Structurally the inverse of the current newest-first card list.

	NB: data-handling copy is placeholder pending confirmation from the team.
-->
<script lang="ts">
	import { Info, BookOpen, MessageCircleQuestion, Lock, FileText } from 'lucide-svelte';
	import type { SampleQA } from './sample';

	let { qas }: { qas: SampleQA[] } = $props();

	// Thread reads oldest-first, like a chat.
	let thread = $derived([...qas].reverse());

	let inputVal = $state('');
	let openSection = $state<string | null>('does');

	const sections = [
		{
			key: 'does',
			icon: BookOpen,
			title: 'What it does',
			body: 'Answers your questions using the documents attached to this consultation, so you can get up to speed without reading everything.'
		},
		{
			key: 'ask',
			icon: MessageCircleQuestion,
			title: 'What you can ask',
			body: 'Anything about the proposal: what is being decided, why, the trade-offs, costs, timelines, or how it affects you. Every answer shows its sources.'
		},
		{
			key: 'data',
			icon: Lock,
			title: 'Your data & privacy',
			body: 'Answers only ever come from this consultation’s documents, never the open web. Your questions stay in your own session and are not shared with other participants.'
		}
	];

	function toggle(key: string) {
		openSection = openSection === key ? null : key;
	}
</script>

<div class="flex min-h-0 flex-1 flex-col">
	<!-- About accordion (pinned top) -->
	<div class="border-border bg-card mb-3 shrink-0 overflow-hidden rounded-xl border">
		<div class="border-border/60 flex items-center gap-2 border-b px-4 py-2.5">
			<Info class="text-primary size-4" />
			<span class="text-foreground text-base font-semibold">About this assistant</span>
		</div>
		{#each sections as s (s.key)}
			{@const open = openSection === s.key}
			<button
				type="button"
				class="hover:bg-muted/40 flex w-full items-center gap-2.5 px-4 py-2.5 text-left transition-colors {open
					? 'bg-muted/30'
					: ''}"
				aria-expanded={open}
				onclick={() => toggle(s.key)}
			>
				<s.icon class="text-primary size-4 shrink-0" />
				<span class="text-foreground flex-1 text-base font-medium">{s.title}</span>
				<span class="text-muted-foreground text-lg leading-none">{open ? '-' : '+'}</span>
			</button>
			{#if open}
				<p
					class="text-muted-foreground border-border/60 border-b px-4 pt-0 pb-3 pl-[42px] text-base leading-relaxed"
				>
					{s.body}
				</p>
			{/if}
		{/each}
	</div>

	<!-- Conversation thread (scrolls, grows) -->
	<div class="min-h-0 flex-1 space-y-4 overflow-y-auto py-1">
		{#if thread.length === 0}
			<p class="text-muted-foreground py-8 text-center text-base">
				Ask your first question below to get started.
			</p>
		{/if}
		{#each thread as qa (qa.id)}
			<!-- user bubble, right-aligned -->
			<div class="flex justify-end">
				<div
					class="bg-primary text-primary-foreground max-w-[85%] rounded-2xl rounded-br-sm px-3.5 py-2 text-base"
				>
					{qa.question}
				</div>
			</div>
			<!-- assistant bubble, left-aligned -->
			<div class="flex justify-start">
				<div
					class="bg-muted text-foreground max-w-[90%] rounded-2xl rounded-bl-sm px-3.5 py-2.5"
				>
					<p class="text-[15px] leading-relaxed">{qa.answer}</p>
					<div
						class="border-border/60 mt-2.5 flex flex-wrap items-center gap-1.5 border-t pt-2.5"
					>
						<span
							class="text-muted-foreground mr-0.5 text-[11px] font-semibold tracking-wide uppercase"
							>Sources</span
						>
						{#each qa.sources as s (s.name)}
							<span
								class="border-border bg-background text-foreground inline-flex max-w-full items-center gap-1.5 rounded-md border px-2 py-0.5 text-xs font-medium"
							>
								<FileText class="size-3 shrink-0" />
								<span class="truncate">{s.name}</span>
							</span>
						{/each}
					</div>
				</div>
			</div>
		{/each}
	</div>

	<!-- Input pinned bottom -->
	<div
		class="border-border bg-background mt-3 flex shrink-0 items-center gap-2 rounded-full border px-4 py-2 shadow-xs"
	>
		<input
			bind:value={inputVal}
			placeholder="Message the assistant…"
			class="text-foreground placeholder:text-muted-foreground min-w-0 flex-1 border-none bg-transparent p-0.5 text-base outline-none"
		/>
		<button
			type="button"
			class="bg-primary text-primary-foreground shrink-0 rounded-full px-4 py-1.5 text-sm font-semibold disabled:opacity-40"
			disabled={!inputVal.trim()}>Send</button
		>
	</div>
</div>
