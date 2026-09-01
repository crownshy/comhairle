<script lang="ts">
	import { onMount } from 'svelte';
	import * as m from '$lib/paraglide/messages';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import { Sparkles, RotateCcw, Check, LoaderCircle, TriangleAlert } from 'lucide-svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { saveRound } from './summary';
	import ConsentModal from './ConsentModal.svelte';
	import ConsentToggle from './ConsentToggle.svelte';
	import type { SummaryRound } from './types';
	import type { ProgressStatus } from '@crownshy/api-client/api';

	type Props = {
		workflowStepId: string;
		workflowId: string;
		conversationId: string;
		/**
		 * All summary rounds for this participant. The parent owns generation
		 * and appends to this array; Summary is a pure renderer over it.
		 */
		rounds: SummaryRound[];
		/**
		 * True while the parent is generating a round (initial or extension).
		 * Shows the loading skeleton below the existing stack.
		 */
		pendingNextRound?: boolean;
		/**
		 * True if the last generation attempt failed. Renders a Try again
		 * button alongside the existing stack (or instead of it, on first-gen
		 * failure).
		 */
		loadError?: boolean;
		/** Step-level admin flag. When false, the consent modal/toggle never render. */
		requestUserSharePermission?: boolean;
		/** Current backend value for `permission_to_share_with_organizers`. */
		initialPermissionToShareWithOrganizers?: boolean | null;
		/** Used to recognise returning participants: 'done' means they've decided before. */
		progressStatus?: ProgressStatus;
		/** Retry the last failed generation. Required when `loadError` is true. */
		onRetryGenerate?: () => void;
		/** Fired when the participant submits the latest round. */
		onDone?: () => void;
		/** Fired when the participant clicks "I want to answer more questions". */
		onAnswerMore?: () => void;
	};

	let {
		workflowStepId,
		workflowId,
		conversationId,
		rounds,
		pendingNextRound = false,
		loadError = false,
		requestUserSharePermission = false,
		initialPermissionToShareWithOrganizers = null,
		progressStatus = 'not_started',
		onRetryGenerate,
		onDone,
		onAnswerMore
	}: Props = $props();

	let submitting = $state(false);
	// Local edited text per round (by id). Seeded lazily on first keystroke and
	// kept as the display source of truth after saving, because the parent's
	// `rounds` prop is never refreshed with the saved text (see valueFor).
	let editsById = $state<Record<string, string>>({});
	// Quiet autosave indicator per round: 'saving' → 'saved' (auto-clears after
	// 2s) or 'error' (sticks). Absent means idle. Mirrors the configure-page
	// pattern (see TranslatableField / translationSource).
	type SaveState = 'saving' | 'saved' | 'error';
	let saveStateById = $state<Record<string, SaveState>>({});
	const savedResetTimers: Record<string, ReturnType<typeof setTimeout>> = {};

	function setSaveState(id: string, state: SaveState | undefined) {
		clearTimeout(savedResetTimers[id]);
		if (state === undefined) {
			const { [id]: _, ...rest } = saveStateById;
			saveStateById = rest;
			return;
		}
		saveStateById = { ...saveStateById, [id]: state };
		if (state === 'saved') {
			savedResetTimers[id] = setTimeout(() => {
				if (saveStateById[id] === 'saved') setSaveState(id, undefined);
			}, 2_000);
		}
	}

	// Live sharing consent for this participant's thinking-space record. Hydrated
	// from the backend user_progress row; toggle/modal flips PATCH it back.
	// Backend defaults to TRUE (opt-out), so undefined falls back to true.
	let consent = $state<boolean>(initialPermissionToShareWithOrganizers ?? true);
	// "Has the participant explicitly made a sharing choice on this step?"
	// True if they've already finished this step before (returning visit) —
	// progressStatus === 'done' means they passed the modal at least once.
	// Becomes true after they pick a button in the modal this session.
	let hasDecidedConsent = $state<boolean>(progressStatus === 'done');
	let consentModalOpen = $state(false);

	const loadingMessages = [
		m.thinking_space_summary_loading_1(),
		m.thinking_space_summary_loading_2(),
		m.thinking_space_summary_loading_3()
	];
	const skeletonLines: Array<{ first: string; second: string | null }> = [
		{ first: 'w-full', second: 'w-11/12' },
		{ first: 'w-full', second: 'w-2/3' },
		{ first: 'w-10/12', second: 'w-1/2' }
	];

	let messageIndex = $state(0);
	let fading = $state(false);

	onMount(() => {
		const interval = setInterval(() => {
			fading = true;
			setTimeout(() => {
				messageIndex = (messageIndex + 1) % loadingMessages.length;
				fading = false;
			}, 300);
		}, 3500);
		return () => {
			clearInterval(interval);
			for (const timer of Object.values(savedResetTimers)) clearTimeout(timer);
		};
	});

	// Only the newest round is editable. Prior rounds are frozen
	function editLatest(id: string, value: string) {
		editsById = { ...editsById, [id]: value };
		// Clear any lingering "Saved"/"Not saved" the moment they type again.
		if (saveStateById[id]) setSaveState(id, undefined);
	}

	async function persistEdit(id: string) {
		const draft = editsById[id];
		if (draft === undefined) return;
		const trimmed = draft.trim();
		if (!trimmed) return;
		setSaveState(id, 'saving');
		const res = await tryCatchAsync(() =>
			saveRound({ workflowStepId, roundId: id, submittedText: trimmed })
		);
		if (res.err !== null) {
			console.error(res.err);
			// Signal the failure two ways: a persistent inline "Not saved" on the
			// field, plus a toast the participant can't miss (their edit is at risk).
			setSaveState(id, 'error');
			notifications.send({
				message: 'Could not save your edit. Please try again.',
				priority: 'ERROR'
			});
			return;
		}
		// Keep the saved text as the local source of truth. The parent's
		// `rounds` prop is never refreshed after a save, so dropping this
		// entry would revert the textarea to the stale pre-edit draft on blur.
		editsById = { ...editsById, [id]: trimmed };
		setSaveState(id, 'saved');
	}

	function valueFor(round: SummaryRound): string {
		return editsById[round.id] ?? round.submittedText;
	}

	async function submit() {
		const latest = rounds[rounds.length - 1];
		if (!latest) return;
		const value = valueFor(latest).trim();
		if (!value || submitting) return;
		// First-ever submit on this step: gate on the consent modal. Returning
		// participants (progressStatus === 'done') have already decided and skip
		// straight to doSubmit.
		if (requestUserSharePermission && !hasDecidedConsent) {
			consentModalOpen = true;
			return;
		}
		await doSubmit();
	}

	async function doSubmit() {
		const latest = rounds[rounds.length - 1];
		if (!latest) return;
		const value = valueFor(latest).trim();
		if (!value || submitting) return;
		submitting = true;
		if (editsById[latest.id] !== undefined) {
			const res = await tryCatchAsync(() =>
				saveRound({ workflowStepId, roundId: latest.id, submittedText: value })
			);
			if (res.err !== null) {
				console.error(res.err);
				notifications.send({
					message: 'Could not save your summary. Please try again.',
					priority: 'ERROR'
				});
				submitting = false;
				return;
			}
		}
		submitting = false;
		onDone?.();
	}

	async function patchConsent(value: boolean): Promise<boolean> {
		try {
			await apiClient.SetUserProgress(
				{ permission_to_share_with_organizers: value },
				{
					params: {
						conversation_id: conversationId,
						workflow_id: workflowId,
						workflow_step_id: workflowStepId
					},
					headers: { 'Content-Type': 'application/json' }
				}
			);
			return true;
		} catch (e) {
			console.error('thinking_space: failed to update share permission', e);
			notifications.send({
				message: 'Could not save your sharing preference. Please try again.',
				priority: 'ERROR'
			});
			return false;
		}
	}

	async function handleConsentChoice(share: boolean) {
		const ok = await patchConsent(share);
		if (!ok) return;
		consent = share;
		hasDecidedConsent = true;
		consentModalOpen = false;
		notifications.send({
			message: share
				? 'Thanks for sharing. Your responses have been sent to the organizers. You can change your mind anytime via the toggle when you come back.'
				: 'Thanks. Your thinking stays private. Only you can see it. You can change your mind anytime via the toggle when you come back.',
			priority: 'SUCCESS'
		});
		await doSubmit();
	}

	async function handleToggleChange(next: boolean) {
		const previous = consent;
		consent = next;
		const ok = await patchConsent(next);
		if (!ok) {
			consent = previous;
			return;
		}
		notifications.send({
			message: next
				? "You're now sharing your thinking with the organizers."
				: 'Your thinking is private again. Only you can see it.',
			priority: 'INFO'
		});
	}

	function frozenLabel(index: number): string {
		return `${m.round()} ${index + 1} ${m.thinking()}`;
	}

	let showFirstGenError = $derived(loadError && rounds.length === 0 && !pendingNextRound);
	let showRetryInline = $derived(loadError && rounds.length > 0 && !pendingNextRound);
</script>

<div class="mx-auto flex w-full max-w-2xl flex-1 flex-col gap-6 py-8">
	<header>
		<h2 class="text-foreground text-2xl leading-snug font-semibold sm:text-3xl">
			{m.thinking_space_review_heading()}
		</h2>
		<p class="text-muted-foreground mt-3 text-base leading-relaxed">
			{m.thinking_space_review_desc()}
		</p>
		{#if requestUserSharePermission && hasDecidedConsent}
			<div class="mt-4">
				<ConsentToggle shared={consent} onChange={handleToggleChange} />
			</div>
		{/if}
	</header>

	<!-- The summary stack is the whole screen: the statement the participant submits, plus any
	     frozen rounds behind it. The answers it was drawn from are not repeated here. -->
	<section class="space-y-6">
		{#each rounds as round, i (round.id)}
			{@const isLatest = i === rounds.length - 1}
			{#if isLatest}
				<div class="space-y-2">
					{#if saveStateById[round.id]}
						<div class="flex justify-end">
							{#if saveStateById[round.id] === 'saving'}
								<span
									class="text-muted-foreground inline-flex items-center gap-1 text-sm"
								>
									<LoaderCircle class="size-3.5 animate-spin" />
									{m.saving()}
								</span>
							{:else if saveStateById[round.id] === 'saved'}
								<span class="inline-flex items-center gap-1 text-sm text-green-600">
									<Check class="size-3.5" />
									{m.saved()}
								</span>
							{:else}
								<span
									class="text-destructive inline-flex items-center gap-1 text-sm"
								>
									<TriangleAlert class="size-3.5" />
									{m.not_saved()}
								</span>
							{/if}
						</div>
					{/if}
					<!-- Lock the previous round while the next one generates: once
					generation starts this round is about to freeze, so editing it
					would be lost. -->
					<Textarea
						value={valueFor(round)}
						oninput={(e) => editLatest(round.id, e.currentTarget.value)}
						onblur={() => persistEdit(round.id)}
						readonly={pendingNextRound}
						rows={10}
						class="bg-background rounded-xl text-base leading-relaxed {pendingNextRound
							? 'cursor-not-allowed opacity-70'
							: ''}"
						placeholder={m.thinking_space_latest_thinking()}
					/>
				</div>
			{:else}
				<div class="space-y-2">
					<p class="text-muted-foreground text-sm font-semibold tracking-wide uppercase">
						{frozenLabel(i)}
					</p>
					<p class="text-foreground/80 text-base leading-relaxed whitespace-pre-wrap">
						{valueFor(round)}
					</p>
				</div>
			{/if}
		{/each}

		{#if pendingNextRound}
			<div class="space-y-3">
				<div class="flex items-start gap-2">
					<Sparkles class="text-primary mt-0.5 size-4 shrink-0 animate-pulse" />
					<p
						class="text-muted-foreground text-base leading-relaxed transition-opacity duration-300"
						class:opacity-0={fading}
						class:opacity-100={!fading}
						aria-live="polite"
					>
						{loadingMessages[messageIndex]}
					</p>
				</div>
				<div class="border-border space-y-3 rounded-xl border px-4 py-4" aria-hidden="true">
					{#each skeletonLines as layout, i (i)}
						<div>
							<Skeleton class="h-4 {layout.first}" />
							{#if layout.second}
								<Skeleton class="mt-2 h-4 {layout.second}" />
							{/if}
						</div>
					{/each}
				</div>
			</div>
		{/if}

		{#if showFirstGenError}
			<div class="space-y-3">
				<p class="text-muted-foreground text-base">
					{m.thinking_space_summary_gen_failure()}
				</p>
				<Button
					variant="outline"
					onclick={() => onRetryGenerate?.()}
					disabled={!onRetryGenerate}
				>
					<RotateCcw class="size-4" />
					{m.try_again()}
				</Button>
			</div>
		{:else if !pendingNextRound && rounds.length > 0}
			{#if showRetryInline}
				<div
					class="border-border flex items-center justify-between gap-3 rounded-xl border px-4 py-3"
				>
					<p class="text-muted-foreground text-sm">
						{m.thinking_space_summary_gen_round_failure()}
					</p>
					<Button
						variant="outline"
						size="sm"
						onclick={() => onRetryGenerate?.()}
						disabled={!onRetryGenerate}
					>
						<RotateCcw class="size-3.5" />
						{m.try_again()}
					</Button>
				</div>
			{/if}
			<div class="flex gap-3">
				<Button
					variant="outline"
					size="lg"
					class="flex-1"
					onclick={onAnswerMore}
					disabled={!onAnswerMore}
				>
					{m.thinking_space_explore_more()}
				</Button>
				<Button
					size="lg"
					class="flex-1"
					onclick={submit}
					disabled={!valueFor(rounds[rounds.length - 1]).trim() || submitting}
				>
					<Check class="size-4" />
					{submitting ? `${m.saving()}...` : m.finish()}
				</Button>
			</div>
		{/if}
	</section>
</div>

<ConsentModal
	open={consentModalOpen}
	onOpenChange={(o) => (consentModalOpen = o)}
	onShare={() => handleConsentChoice(true)}
	onKeepPrivate={() => handleConsentChoice(false)}
/>
