<script lang="ts">
	import { tick } from 'svelte';
	import { Sparkles, X, FileText } from 'lucide-svelte';
	import {
		ChatClient,
		type ChatReference,
		type ReferenceChunk
	} from '$lib/api/chatClient.svelte';
	import MessageWithReferences from '$lib/components/Chatbot/MessageWithReferences.svelte';
	import * as Dialog from '$lib/components/ui/dialog';

	type QA = {
		id: string;
		question: string;
		answer: string;
		reference: ChatReference | null;
		streaming: boolean;
	};

	type Props = {
		conversationId: string;
		pageKey: number;
		pageTitle?: string;
	};

	let { conversationId, pageKey, pageTitle = '' }: Props = $props();

	let enabled = $state(true);
	let qasByPage = $state<Record<number, QA[]>>({});
	let inputVal = $state('');
	let focused = $state(false);
	let inputRef = $state<HTMLInputElement | null>(null);
	let activeChunk = $state<ReferenceChunk | null>(null);
	let chatError = $state<string | null>(null);
	let initializing = $state(false);

	let client = $state<ChatClient | null>(null);
	let initialized = false;
	let liveQaId = $state<string | null>(null);

	let pageQAs = $derived(qasByPage[pageKey] ?? []);
	let showPlaceholder = $derived(!focused && !inputVal);
	let isStreaming = $derived(client?.isStreaming ?? false);

	const quickPrompts = ['Explain this simply', 'Key takeaways?', 'Why does this matter?'];

	$effect(() => {
		if (!enabled || initialized) return;
		initialized = true;
		void initClient();
	});

	async function initClient() {
		try {
			initializing = true;
			const c = new ChatClient(conversationId);
			const session = await c.getSession();
			if (!session) {
				chatError = c.error || 'Failed to load tutor session';
				return;
			}
			client = c;
		} catch (e) {
			chatError = e instanceof Error ? e.message : 'Failed to initialize tutor';
		} finally {
			initializing = false;
		}
	}

	// Mirror streaming answer into the live QA
	$effect(() => {
		if (!client) return;
		const id = liveQaId;
		const answer = client.currentAnswer;
		const reference = client.currentReference;
		if (!id) return;
		const list = qasByPage[pageKey];
		if (!list) return;
		const idx = list.findIndex((q) => q.id === id);
		if (idx === -1) return;
		const next = [...list];
		next[idx] = { ...next[idx], answer, reference };
		qasByPage = { ...qasByPage, [pageKey]: next };
	});

	async function handleAsk() {
		const question = inputVal.trim();
		if (!question || !client || isStreaming || initializing) return;
		inputVal = '';
		focused = false;

		const id = `qa-${Date.now()}`;
		const newQa: QA = { id, question, answer: '', reference: null, streaming: true };
		const list = qasByPage[pageKey] ?? [];
		qasByPage = { ...qasByPage, [pageKey]: [...list, newQa] };
		liveQaId = id;

		const contextual = pageTitle ? `[Reading "${pageTitle}"] ${question}` : question;

		await client.send(contextual);

		// finalize
		const finalList = qasByPage[pageKey];
		if (finalList) {
			const idx = finalList.findIndex((q) => q.id === id);
			if (idx !== -1) {
				const next = [...finalList];
				next[idx] = { ...next[idx], streaming: false };
				qasByPage = { ...qasByPage, [pageKey]: next };
			}
		}
		liveQaId = null;

		if (client.error) chatError = client.error;
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

<div class="border-border my-6 pt-6">
	{#if enabled}
		{#if chatError}
			<div class="mb-4 rounded-md border border-red-200 bg-red-50 p-3 text-sm text-red-700">
				{chatError}
			</div>
		{/if}

		<!-- Inline answers -->
		{#if pageQAs.length > 0}
			<div class="space-y-5">
				{#each pageQAs as qa (qa.id)}
					<div class="border-border/60 rounded-xl border p-4">
						<p
							class="text-primary mb-1 text-[11px] font-semibold tracking-wide uppercase"
						>
							You asked
						</p>
						<p class="text-foreground mb-3 text-base font-semibold italic">
							"{qa.question}"
						</p>

						<div class="text-foreground/90 text-[15px] leading-relaxed">
							{#if qa.answer}
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
												onclick={() => (activeChunk = chunk)}
											>
												<FileText class="h-3 w-3 shrink-0" />
												<span class="truncate">{chunk.document_name}</span>
											</button>
										{/each}
									</div>
								</div>
							{/if}
						{/if}
					</div>
				{/each}
			</div>
		{/if}

		<!-- Inline prompt -->
		<div class="mt-4">
			<div
				class="border-b transition-colors {focused
					? 'border-primary'
					: 'border-border'} pb-1.5"
			>
				{#if showPlaceholder}
					<button
						type="button"
						class="text-muted-foreground flex w-full items-center bg-transparent p-0 text-left text-base"
						onclick={activate}
						disabled={initializing || !!chatError}
					>
						{initializing
							? 'Loading tutor session...'
							: 'Anything unclear? Type a question here'}
						<span
							class="bg-primary caret-blink ml-0.5 inline-block h-5 w-0.5 align-middle"
						></span>
					</button>
				{:else}
					<div class="flex items-center gap-2">
						<input
							bind:this={inputRef}
							bind:value={inputVal}
							onfocus={() => (focused = true)}
							onblur={() => {
								if (!inputVal) setTimeout(() => (focused = false), 150);
							}}
							onkeydown={(e) => {
								if (e.key === 'Enter') {
									e.preventDefault();
									handleAsk();
								}
							}}
							placeholder="Type your question..."
							disabled={isStreaming}
							class="text-foreground placeholder:text-muted-foreground min-w-0 flex-1 border-none bg-transparent p-0 text-base outline-none"
						/>
						{#if inputVal.trim()}
							<button
								type="button"
								class="text-primary shrink-0 bg-transparent p-0 text-sm font-semibold disabled:opacity-50"
								disabled={isStreaming}
								onclick={handleAsk}
							>
								{isStreaming ? '...' : 'Ask ↵'}
							</button>
						{/if}
					</div>
				{/if}
			</div>

			<div class="mt-3 flex flex-col gap-1.5">
				<p class="text-muted-foreground text-xs">
					Just for you — questions and answers aren't submitted or shared. Try:
				</p>
				<div class="flex flex-wrap gap-1.5">
					{#each quickPrompts as p (p)}
						<button
							type="button"
							class="bg-accent/30 text-accent-foreground hover:bg-primary hover:text-primary-foreground rounded-full px-3 py-1 text-xs font-medium transition-colors"
							onclick={() => pickPrompt(p)}
						>
							{p}
						</button>
					{/each}
				</div>
			</div>
		</div>
	{:else}
		<p
			class="text-muted-foreground border-border rounded-md border border-dashed p-3 text-center text-xs"
		>
			Enable the tutor bot above to ask questions about this page.
		</p>
	{/if}
</div>

<!-- Source chunk modal -->
<Dialog.Root open={!!activeChunk} onOpenChange={(o) => !o && (activeChunk = null)}>
	<Dialog.Content class="max-w-xl">
		{#if activeChunk}
			<Dialog.Header>
				<Dialog.Title class="flex items-start gap-2">
					<FileText class="text-primary mt-1 h-4 w-4 shrink-0" />
					<span>{activeChunk.document_name}</span>
				</Dialog.Title>
				{#if activeChunk.similarity != null}
					<Dialog.Description>
						Relevance: {(activeChunk.similarity * 100).toFixed(1)}%
					</Dialog.Description>
				{/if}
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

<style>
	.caret-blink {
		animation: caret-blink 1s step-end infinite;
	}
	@keyframes caret-blink {
		0%,
		100% {
			opacity: 1;
		}
		50% {
			opacity: 0;
		}
	}
</style>
