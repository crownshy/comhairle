<script lang="ts">
	/**
	 * The participant-facing Learning assistant.
	 *
	 * One prominent ask bar, and below it exactly one answer at full size. Everything
	 * asked before collapses into a short list that can be promoted back into focus. The
	 * standing explanation that used to sit above the input is now an intro shown once per
	 * conversation, with the rest behind "Learn more" (see ADR-0028).
	 */
	import { untrack } from 'svelte';
	import {
		Search,
		ArrowUp,
		AlertTriangle,
		RefreshCw,
		FileText,
		ChevronRight,
		Lock,
		HelpCircle,
		X
	} from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';
	import type { ReferenceChunk } from '$lib/api/chatClient.svelte';
	import PdfDocumentDialog from '$lib/components/PdfViewer/PdfDocumentDialog.svelte';
	import AssistantAnswerSkeleton from './AssistantAnswerSkeleton.svelte';
	import AssistantAnswer from './AssistantAnswer.svelte';
	import AssistantLearnMore from './AssistantLearnMore.svelte';
	import {
		createAssistantState,
		formatTimestamp,
		hasSeenIntro,
		markIntroSeen,
		uniqueDocs
	} from './assistantState.svelte';

	type Props = {
		conversationId: string;
		pageTitle?: string;
		/** True while the surrounding page is loading (e.g. route transition). Disables input. */
		loading?: boolean;
		/**
		 * Presentation context. 'inline' (default) is the learn-page embed. 'sidebar' drops
		 * the heading (the drawer tab already titles it) and lets the panel scroll itself.
		 */
		variant?: 'inline' | 'sidebar';
	};

	let { conversationId, pageTitle = '', loading = false, variant = 'inline' }: Props = $props();

	const assistant = createAssistantState({
		getConversationId: () => conversationId,
		getPageTitle: () => pageTitle,
		getLoading: () => loading
	});

	const introPoints = [
		m.learning_assistant_intro_point_1(),
		m.learning_assistant_intro_point_2(),
		m.learning_assistant_intro_point_3()
	];

	let inputVal = $state('');
	let introOpen = $state(untrack(() => !hasSeenIntro(conversationId)));
	let learnMoreOpen = $state(false);
	let pickedId = $state<string | null>(null);

	// The newest answer is what is in focus; an explicit pick from the history wins until
	// the next question is asked.
	let focused = $derived(
		assistant.newestFirst.find((qa) => qa.id === pickedId) ?? assistant.newestFirst[0] ?? null
	);
	let earlier = $derived(assistant.newestFirst.filter((qa) => qa.id !== focused?.id));
	let focusedDocs = $derived(uniqueDocs(focused?.reference ?? null));

	function dismissIntro() {
		introOpen = false;
		markIntroSeen(conversationId);
	}

	async function submit(question: string) {
		if (!question.trim()) return;
		inputVal = '';
		pickedId = null;
		dismissIntro();
		await assistant.ask(question);
	}

	function openSource(chunk: ReferenceChunk) {
		assistant.openSource(chunk);
	}
</script>

{#if !loading}
	<section
		class="flex flex-col gap-6 {variant === 'sidebar'
			? 'min-h-0 flex-1 overflow-y-auto'
			: 'my-6'}"
	>
		{#if introOpen}
			<div class="bg-accent border-border relative rounded-2xl border p-5">
				<button
					type="button"
					class="text-muted-foreground hover:text-foreground absolute top-3 right-3 rounded-lg p-1 transition-colors"
					aria-label={m.learning_assistant_intro_dismiss()}
					onclick={dismissIntro}
				>
					<X class="h-5 w-5" />
				</button>
				<h2 class="text-foreground pr-8 text-xl font-bold">
					{m.learning_assistant_intro_title()}
				</h2>
				<ul class="mt-3 space-y-2">
					{#each introPoints as point (point)}
						<li class="text-foreground flex gap-2.5 text-base leading-snug">
							<span class="bg-primary mt-2 h-1.5 w-1.5 shrink-0 rounded-full"></span>
							{point}
						</li>
					{/each}
				</ul>
				<p class="text-muted-foreground mt-3 flex items-start gap-2 text-base leading-snug">
					<Lock class="mt-0.5 h-4 w-4 shrink-0" />
					{m.learning_assistant_privacy_notice()}
				</p>
				<div class="mt-4 flex flex-wrap items-center gap-3">
					<button
						type="button"
						class="bg-primary text-primary-foreground hover:bg-primary/90 rounded-lg px-4 py-2 text-base font-semibold transition-colors"
						onclick={dismissIntro}
					>
						{m.learning_assistant_intro_dismiss()}
					</button>
					<button
						type="button"
						class="text-primary text-base font-semibold underline underline-offset-4"
						aria-expanded={learnMoreOpen}
						onclick={() => (learnMoreOpen = !learnMoreOpen)}
					>
						{m.learning_assistant_learn_more()}
					</button>
				</div>
				{#if learnMoreOpen}
					<div class="border-border mt-4 border-t pt-4">
						<AssistantLearnMore />
					</div>
				{/if}
			</div>
		{/if}

		<div>
			<!-- In the drawer the tab already names the assistant, so only the way back into
			     the explanation is worth the row. -->
			<div
				class="mb-2 flex flex-col items-start gap-2 sm:flex-row sm:items-center sm:justify-between sm:gap-3"
			>
				{#if variant === 'inline'}
					<h2 class="text-foreground text-lg font-bold">{m.learning_assistant()}</h2>
				{:else}
					<p class="text-muted-foreground min-w-0 flex-1 text-base">
						{m.learning_assistant_tagline()}
					</p>
				{/if}
				{#if !introOpen}
					<button
						type="button"
						class="text-muted-foreground hover:text-foreground inline-flex shrink-0 items-center gap-1.5 text-base font-medium transition-colors"
						onclick={() => {
							introOpen = true;
							learnMoreOpen = true;
						}}
					>
						<HelpCircle class="h-4 w-4" />
						{m.learning_assistant_what_is_this()}
					</button>
				{/if}
			</div>

			<div
				class="border-input focus-within:border-ring focus-within:ring-ring/40 bg-background flex items-center gap-3 rounded-2xl border-2 px-4 py-3 transition-[box-shadow,border-color] focus-within:ring-[3px]"
			>
				<Search class="text-muted-foreground h-5 w-5 shrink-0" />
				<input
					bind:value={inputVal}
					onkeydown={(e) => {
						if (e.key === 'Enter') {
							e.preventDefault();
							submit(inputVal);
						}
					}}
					placeholder={m.learning_assistant_input_placeholder()}
					disabled={assistant.inputDisabled}
					class="text-foreground placeholder:text-muted-foreground min-w-0 flex-1 border-none bg-transparent text-lg outline-none disabled:cursor-not-allowed"
				/>
				<button
					type="button"
					class="bg-primary text-primary-foreground grid h-10 w-10 shrink-0 place-items-center rounded-xl transition-opacity disabled:opacity-40"
					disabled={assistant.inputDisabled || !inputVal.trim()}
					aria-label={m.ask()}
					onclick={() => submit(inputVal)}
				>
					<ArrowUp class="h-5 w-5" />
				</button>
			</div>
		</div>

		{#if assistant.fatalError}
			<div
				class="border-destructive/30 bg-destructive/5 flex items-start gap-3 rounded-2xl border p-4"
			>
				<AlertTriangle class="text-destructive mt-0.5 h-5 w-5 shrink-0" />
				<div class="min-w-0 flex-1 text-base">
					<p class="text-destructive font-semibold">{m.learning_assistant_load_fail()}</p>
					<p class="text-foreground/80">{assistant.fatalError}</p>
					<button
						type="button"
						class="border-destructive/40 text-destructive hover:bg-destructive/10 mt-3 inline-flex items-center gap-2 rounded-lg border px-3 py-1.5 text-base font-medium transition-colors"
						onclick={() => assistant.retryInit()}
					>
						<RefreshCw class="h-4 w-4" />
						{m.try_again()}
					</button>
				</div>
			</div>
		{:else if assistant.loadingHistory}
			<!-- The ask bar above is already real, so only the answer card is missing. -->
			<AssistantAnswerSkeleton />
		{:else if focused}
			{@const ts = formatTimestamp(focused.timestamp)}
			<article class="bg-card border-border rounded-2xl border p-5 shadow-sm">
				<p class="text-muted-foreground text-sm font-semibold">
					{m.you_asked()}{ts ? ` · ${ts}` : ''}
				</p>
				<h3 class="text-foreground mt-1 mb-4 text-xl leading-snug font-bold">
					{focused.question}
				</h3>
				<AssistantAnswer
					qa={focused}
					onOpenSource={openSource}
					onRetry={() => assistant.retryLast()}
					canRetry={focused.id === assistant.newestFirst[0]?.id}
				/>

				{#if focusedDocs.length > 0}
					<div class="border-border mt-5 border-t pt-4">
						<p class="text-muted-foreground mb-2 text-sm font-semibold">
							{m.learning_assistant_sources()}
						</p>
						<div class="flex flex-col gap-1">
							{#each focusedDocs as chunk (chunk.id)}
								<button
									type="button"
									class="hover:bg-muted text-foreground flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-left text-base transition-colors"
									onclick={() => openSource(chunk)}
								>
									<FileText class="text-primary h-5 w-5 shrink-0" />
									<span class="min-w-0 flex-1 truncate font-medium"
										>{chunk.document_name}</span
									>
									<ChevronRight class="text-muted-foreground h-5 w-5 shrink-0" />
								</button>
							{/each}
						</div>
					</div>
				{/if}
			</article>
		{/if}

		{#if earlier.length > 0}
			<div>
				<p class="text-muted-foreground mb-2 text-sm font-semibold">
					{m.learning_assistant_earlier()}
				</p>
				<div class="border-border divide-border divide-y overflow-hidden rounded-xl border">
					{#each earlier as qa (qa.id)}
						<button
							type="button"
							class="hover:bg-muted flex w-full items-center gap-3 px-4 py-3 text-left transition-colors"
							onclick={() => (pickedId = qa.id)}
						>
							<span class="text-foreground min-w-0 flex-1 truncate text-base">
								{qa.question}
							</span>
							<ChevronRight class="text-muted-foreground h-5 w-5 shrink-0" />
						</button>
					{/each}
				</div>
			</div>
		{/if}
	</section>
{/if}

<!-- Every source (uploaded files and the synced learn-content PDF) opens in the shared
     document viewer, with the retrieved passage highlighted when position data is available. -->
<PdfDocumentDialog
	bind:open={assistant.viewerOpen}
	kind={assistant.activeSource?.kind ?? 'pdf'}
	src={assistant.activeSource?.src ?? null}
	name={assistant.activeSource?.name ?? 'Document'}
	downloadHref={assistant.activeSource?.downloadHref ?? null}
	highlights={assistant.activeSource?.highlights ?? []}
	page={assistant.activeSource?.page ?? null}
/>
