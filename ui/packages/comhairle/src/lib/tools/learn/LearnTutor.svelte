<script lang="ts">
	import { tick } from 'svelte';
	import { FileText, ChevronDown, AlertTriangle, RefreshCw } from 'lucide-svelte';
	import { getChatSession, type ChatMessage } from '$lib/api/chatSession.svelte';
	import type { ChatReference, ReferenceChunk } from '$lib/api/chatClient.svelte';
	import MessageWithReferences from '$lib/components/Chatbot/MessageWithReferences.svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import LearnTutorSkeleton from './LearnTutorSkeleton.svelte';

	type QA = {
		id: string;
		question: string;
		answer: string;
		reference: ChatReference | null;
		streaming: boolean;
		error: string | null;
		timestamp: Date | null;
	};

	type Props = {
		conversationId: string;
		pageTitle?: string;
		/** True while the surrounding page is loading (e.g. route transition). Disables input. */
		loading?: boolean;
	};

	let { conversationId, pageTitle = '', loading = false }: Props = $props();

	let enabled = $state(true);
	let inputVal = $state('');
	let focused = $state(false);
	let inputRef = $state<HTMLInputElement | null>(null);
	let activeChunk = $state<ReferenceChunk | null>(null);
	let initStartedFor = $state<string | null>(null);
	/** Manual overrides for collapsed state, keyed by QA id. Unset → use default. */
	let expandedOverrides = $state<Record<string, boolean>>({});

	function isExpanded(qa: QA, isNewest: boolean): boolean {
		const override = expandedOverrides[qa.id];
		if (typeof override === 'boolean') return override;
		return isNewest || qa.streaming;
	}

	function toggleQa(qa: QA, isNewest: boolean) {
		const current = isExpanded(qa, isNewest);
		expandedOverrides = { ...expandedOverrides, [qa.id]: !current };
	}

	/** Format a question's timestamp as a short relative label, e.g. "just now", "5m ago". */
	function formatTimestamp(date: Date | null): string {
		if (!date) return '';
		const diffMs = Date.now() - date.getTime();
		const sec = Math.round(diffMs / 1000);
		if (sec < 10) return 'just now';
		if (sec < 60) return `${sec}s ago`;
		const min = Math.round(sec / 60);
		if (min < 60) return `${min}m ago`;
		const hr = Math.round(min / 60);
		if (hr < 24) return `${hr}h ago`;
		const day = Math.round(hr / 24);
		if (day < 7) return `${day}d ago`;
		return date.toLocaleDateString();
	}

	const session = $derived(enabled && conversationId ? getChatSession(conversationId) : null);
	let initializing = $derived(session?.initializing ?? false);
	let chatError = $derived(session?.error ?? null);
	let isStreaming = $derived(session?.isStreaming ?? false);

	/**
	 * True from first paint until the session history has finished loading.
	 * Keyed off `initialized` (not `initializing`) so the skeleton is showing
	 * before the init effect has even fired — no flash of the empty input.
	 */
	let loadingHistory = $derived(!!session && !session.initialized && !chatError);

	/**
	 * Group session messages into Q/A pairs for inline display.
	 * A pair = a user message followed (optionally) by the next assistant message.
	 */
	let pageQAs = $derived<QA[]>(
		(() => {
			const msgs: ChatMessage[] = session?.messages ?? [];
			const out: QA[] = [];
			for (let i = 0; i < msgs.length; i++) {
				const m = msgs[i];
				if (m.role !== 'user') continue;
				const next = msgs[i + 1];
				const hasAnswer = next && next.role === 'assistant';
				out.push({
					id: m.id,
					question: m.content,
					answer: hasAnswer ? next.content : '',
					reference: hasAnswer ? next.reference : null,
					streaming: hasAnswer ? !!next.streaming : isStreaming,
					error: hasAnswer ? (next.error ?? null) : null,
					timestamp: m.timestamp ?? null
				});
				if (hasAnswer) i++;
			}
			// Newest first.
			return out.reverse();
		})()
	);

	let showPlaceholder = $derived(!focused && !inputVal);

	const quickPrompts = ['Explain this simply', 'Key takeaways?', 'Why does this matter?'];

	$effect(() => {
		if (!session) return;
		if (initStartedFor === session.conversationId) return;
		initStartedFor = session.conversationId;
		void session.init();
	});

	/** True whenever the input should be non-interactive. */
	let inputDisabled = $derived(
		loading || isStreaming || initializing || (!!chatError && pageQAs.length === 0)
	);

	async function handleAsk() {
		const question = inputVal.trim();
		if (!question || !session || inputDisabled) return;
		inputVal = '';
		focused = false;

		const contextual = pageTitle ? `[Reading "${pageTitle}"] ${question}` : question;
		await session.send(question, contextual);
	}

	async function retryLast() {
		if (!session || isStreaming) return;
		await session.retryLast();
	}

	async function retryInit() {
		if (!session) return;
		await session.retryInit();
	}

	function pickPrompt(p: string) {
		inputVal = p;
		focused = true;
		tick().then(() => inputRef?.focus());
	}

	function activate() {
		focused = true;
		tick().then(() => inputRef?.focus());
	}

	function uniqueDocsFromReference(ref: ChatReference | null): ReferenceChunk[] {
		if (!ref?.chunks?.length) return [];
		const seen = new Set<string>();
		const out: ReferenceChunk[] = [];
		for (const c of ref.chunks) {
			const key = c.document_id || c.document_name;
			if (!seen.has(key)) {
				seen.add(key);
				out.push(c);
			}
		}
		return out;
	}
</script>

{#if !loading}
	<div class="my-6">
		{#if enabled}
			<p class="text-primary mb-2 text-xs font-semibold tracking-wide uppercase">
				Learning assistant
			</p>
			<div class="text-muted-foreground mb-3 space-y-2 text-sm leading-relaxed">
				<p>Use this this space to answer questions you might have about the topic.</p>
				<p>
					It is best to ask questions that help you learn things. We will try to answer
					these questions using supporting materials that have been made avaliable for
					this conversation.
				</p>
			</div>
			<div class="bg-primary/10 rounded-lg p-4">
				<!-- Init error: shown when there's no usable session yet -->
				{#if chatError && pageQAs.length === 0 && !initializing}
					<div
						class="mb-4 flex items-start gap-3 rounded-xl border border-red-200 bg-red-50 p-4 text-sm"
					>
						<AlertTriangle class="mt-0.5 h-4 w-4 shrink-0 text-red-600" />
						<div class="min-w-0 flex-1">
							<p class="font-semibold text-red-800">Couldn't load tutor</p>
							<p class="text-red-700">{chatError}</p>
							<button
								type="button"
								class="mt-2 inline-flex items-center gap-1.5 rounded-md border border-red-300 bg-white px-2.5 py-1 text-xs font-medium text-red-700 transition-colors hover:bg-red-100"
								onclick={retryInit}
							>
								<RefreshCw class="h-3 w-3" />
								Try again
							</button>
						</div>
					</div>
				{/if}

				<!-- Loading skeleton: shown on its own until the session is ready, then the input replaces it -->
				{#if loadingHistory}
					<LearnTutorSkeleton />
				{:else if !(chatError && pageQAs.length === 0)}
					<!-- Inline prompt -->
					<div class="mb-6">
						{#if showPlaceholder}
							<button
								type="button"
								class="border-input bg-background text-muted-foreground hover:border-ring hover:text-foreground flex h-9 w-full cursor-text items-center rounded-lg border px-3 py-1 text-left text-base shadow-xs transition-colors disabled:cursor-not-allowed"
								onclick={activate}
								disabled={initializing}
							>
								Type a question here
							</button>
						{:else}
							<div
								class="border-ring ring-ring/50 bg-background flex h-9 items-center gap-2 rounded-lg border px-3 py-1 shadow-xs ring-[3px] transition-[color,box-shadow]"
							>
								<input
									bind:this={inputRef}
									bind:value={inputVal}
									onfocus={() => (focused = true)}
									onblur={() => {
										if (!inputVal) setTimeout(() => (focused = false), 50);
									}}
									onkeydown={(e) => {
										if (e.key === 'Enter') {
											e.preventDefault();
											handleAsk();
										}
									}}
									placeholder="Type your question..."
									disabled={inputDisabled}
									class="text-foreground placeholder:text-muted-foreground min-w-0 flex-1 border-none bg-transparent p-0 text-base outline-none disabled:cursor-not-allowed"
								/>
								{#if inputVal.trim()}
									<button
										type="button"
										class="text-primary shrink-0 bg-transparent p-0 text-sm font-semibold disabled:cursor-not-allowed disabled:opacity-50"
										disabled={inputDisabled}
										onclick={handleAsk}
									>
										{isStreaming ? '...' : 'Ask ↵'}
									</button>
								{/if}
							</div>
						{/if}
					</div>
				{/if}

				<!-- Inline answers (newest first) -->
				{#if pageQAs.length > 0}
					<div class="space-y-3">
						{#each pageQAs as qa, i (qa.id)}
							{@const isNewest = i === 0}
							{@const open = isExpanded(qa, isNewest)}
							{@const ts = formatTimestamp(qa.timestamp)}
							<div class="bg-card border-border/60 rounded-xl border">
								<button
									type="button"
									class="hover:bg-card/50 flex w-full items-start gap-3 rounded-xl p-4 text-left transition-colors"
									aria-expanded={open}
									onclick={() => toggleQa(qa, isNewest)}
								>
									<div class="min-w-0 flex-1">
										<div class="mb-1 flex items-center gap-2">
											<p
												class="text-primary text-[11px] font-semibold tracking-wide uppercase"
											>
												You asked
											</p>
											{#if ts}
												<span class="text-muted-foreground text-[11px]"
													>· {ts}</span
												>
											{/if}
										</div>
										<p
											class="text-foreground text-base font-semibold italic {open
												? ''
												: 'truncate'}"
										>
											"{qa.question}"
										</p>
									</div>
									<ChevronDown
										class="text-muted-foreground mt-1 h-4 w-4 shrink-0 transition-transform {open
											? 'rotate-180'
											: ''}"
									/>
								</button>

								{#if open}
									<div class="border-border/60 rounded-xl px-4 pt-3 pb-4">
										<div class="text-foreground/90 text-[15px] leading-relaxed">
											{#if qa.error && !qa.answer}
												<div
													class="flex items-start gap-2 rounded-md border border-red-200 bg-red-50 p-3 text-sm"
												>
													<AlertTriangle
														class="mt-0.5 h-4 w-4 shrink-0 text-red-600"
													/>
													<div class="min-w-0 flex-1">
														<p class="font-semibold text-red-800">
															Couldn't get an answer
														</p>
														<p class="text-red-700">{qa.error}</p>
														{#if isNewest}
															<button
																type="button"
																class="mt-2 inline-flex items-center gap-1.5 rounded-md border border-red-300 bg-white px-2.5 py-1 text-xs font-medium text-red-700 transition-colors hover:bg-red-100 disabled:opacity-50"
																onclick={retryLast}
																disabled={isStreaming}
															>
																<RefreshCw class="h-3 w-3" />
																Try again
															</button>
														{/if}
													</div>
												</div>
											{:else if qa.answer}
												<MessageWithReferences
													content={qa.answer}
													reference={qa.reference}
												/>
												{#if qa.streaming}
													<span
														class="bg-primary ml-0.5 inline-block h-4 w-1.5 animate-pulse align-middle"
													></span>
												{/if}
											{:else}
												<span class="inline-flex items-center gap-1">
													<span
														class="bg-primary/60 h-1.5 w-1.5 animate-bounce rounded-full"
													></span>
													<span
														class="bg-primary/60 h-1.5 w-1.5 animate-bounce rounded-full"
														style="animation-delay: 0.15s"
													></span>
													<span
														class="bg-primary/60 h-1.5 w-1.5 animate-bounce rounded-full"
														style="animation-delay: 0.3s"
													></span>
													<span class="text-muted-foreground ml-2 text-xs"
														>Finding an answer...</span
													>
												</span>
											{/if}
										</div>

										{#if qa.reference}
											{@const docs = uniqueDocsFromReference(qa.reference)}
											{#if docs.length > 0}
												<div class="mt-3">
													<p
														class="text-muted-foreground mb-1.5 text-[11px] font-semibold tracking-wide uppercase"
													>
														Sources
													</p>
													<div class="flex flex-wrap gap-1.5">
														{#each docs as chunk (chunk.id)}
															<button
																type="button"
																class="border-border bg-muted/40 hover:bg-primary hover:text-primary-foreground hover:border-primary text-foreground inline-flex max-w-full items-center gap-1.5 rounded-md border px-2 py-1 text-xs font-medium transition-colors"
																onclick={() =>
																	(activeChunk = chunk)}
															>
																<FileText
																	class="h-3 w-3 shrink-0"
																/>
																<span class="truncate"
																	>{chunk.document_name}</span
																>
															</button>
														{/each}
													</div>
												</div>
											{/if}
										{/if}
									</div>
								{/if}
							</div>
						{/each}
					</div>
				{/if}
			</div>
		{:else}
			<p
				class="text-muted-foreground border-border rounded-md border border-dashed p-3 text-center text-xs"
			>
				Enable the learning assistant above to ask questions about this page.
			</p>
		{/if}
	</div>
{/if}

<!-- Source chunk modal -->
<Dialog.Root open={!!activeChunk} onOpenChange={(o) => !o && (activeChunk = null)}>
	<Dialog.Content class="max-w-xl">
		{#if activeChunk}
			<Dialog.Header>
				<Dialog.Title class="flex items-start gap-2">
					<FileText class="text-primary mt-1 h-4 w-4 shrink-0" />
					<span>{activeChunk.document_name}</span>
				</Dialog.Title>
			</Dialog.Header>
			<div
				class="text-foreground/90 max-h-[55vh] overflow-y-auto text-sm leading-relaxed whitespace-pre-wrap"
			>
				{activeChunk.content
					.replace(/<[^>]*>/g, ' ')
					.replace(/\s+/g, ' ')
					.trim()}
			</div>
			<p class="text-muted-foreground border-border border-t pt-3 text-xs">
				This excerpt was retrieved from the document above and may not reflect its full
				content.
			</p>
		{/if}
	</Dialog.Content>
</Dialog.Root>
