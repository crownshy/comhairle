<script lang="ts">
	import { Button, LoadingButton } from '$lib/components/ui/button';
	import { Portal } from 'bits-ui';
	import { fade } from 'svelte/transition';
	import { ChevronLeft, Check } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';

	type Props = {
		value: string;
		submitting: boolean;
		submitted: boolean;
		error: boolean;
		/** Whether to open on the tips rather than the composer. First visit only. */
		startOnGuidance: boolean;
		onClose: () => void;
		/** Editing clears a failed submit, so the error banner does not outlive the attempt. */
		onEdit: () => void;
		/** Called once the participant has read the tips, so they are not shown again. */
		onGuidanceRead: () => void;
		onSubmit: () => void;
		onSubmitAndAddAnother: () => void;
	};

	let {
		value = $bindable(),
		submitting,
		submitted,
		error,
		startOnGuidance,
		onClose,
		onEdit,
		onGuidanceRead,
		onSubmit,
		onSubmitAndAddAnother
	}: Props = $props();

	let showingGuidance = $state(startOnGuidance);

	// True only while the tips are the screen you arrived on. Reopening them later from the
	// composer makes back mean "return to what I was writing", not "leave".
	let guidanceIsEntry = $state(startOnGuidance);

	let composer = $state<HTMLTextAreaElement | null>(null);

	// Polis statements are voted on one at a time, so they have to stay short enough to read
	// at a glance. Enforced here as well as by maxlength, since a bound value can arrive long.
	const MAX_LENGTH = 200;

	const remaining = $derived(MAX_LENGTH - value.length);
	const canSubmit = $derived(value.trim().length > 0 && value.length <= MAX_LENGTH);

	function goBack() {
		if (showingGuidance && !guidanceIsEntry) {
			showingGuidance = false;
			return;
		}
		onClose();
	}

	function dismissGuidance() {
		showingGuidance = false;
		guidanceIsEntry = false;
		onGuidanceRead();
		// The composer is the point of the screen, so land the caret in it rather than
		// making someone tap twice.
		requestAnimationFrame(() => composer?.focus());
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key !== 'Escape') return;
		event.preventDefault();
		goBack();
	}

	// Nothing behind the overlay should scroll while it is up.
	$effect(() => {
		const previous = document.body.style.overflow;
		document.body.style.overflow = 'hidden';
		return () => {
			document.body.style.overflow = previous;
		};
	});
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- A takeover rather than a panel inside the step: writing an opinion is its own task, and
     covering the pager stops a stray forward tap from completing the step mid-sentence.

     Portalled to the body so it sits above the step shell whatever that does with stacking:
     inside the shell's scroll it painted under the chrome and the pager, which hid its top
     row and cut its content off at both ends.

     Three bands: the back row, the scroll, and the action bar. Only the middle scrolls, so
     the primary action is always on screen without scrolling past the tips or the text. -->
<Portal>
	<div
		class="bg-background fixed inset-0 z-50 flex flex-col"
		role="dialog"
		aria-modal="true"
		aria-label={m.polis_add_your_own_opinion()}
		in:fade={{ duration: 200 }}
	>
		<div
			class="mx-auto flex h-[72px] w-full max-w-5xl shrink-0 items-center justify-between px-4 md:h-20 md:px-6"
		>
			<button
				type="button"
				class="text-foreground inline-flex items-center gap-1"
				onclick={goBack}
			>
				<ChevronLeft class="size-6 shrink-0" />
				<span class="text-base font-medium">{m.pager_back()}</span>
			</button>
			{#if !showingGuidance}
				<button
					type="button"
					class="text-primary text-base font-medium underline decoration-dotted underline-offset-4"
					onclick={() => (showingGuidance = true)}
				>
					{m.polis_guidance_reopen()}
				</button>
			{/if}
		</div>

		{#if showingGuidance}
			<div class="min-h-0 flex-1 overflow-y-auto px-6 pb-6" in:fade={{ duration: 200 }}>
				<div class="mx-auto flex w-full max-w-[520px] flex-col gap-6">
					<h2 class="text-primary text-3xl font-bold">
						{m.polis_guidance_title()}
					</h2>
					<p class="text-muted-foreground text-lg">
						{m.polis_guidance_intro()}
					</p>
					<ul
						class="text-foreground bg-muted/50 flex w-full list-outside list-disc flex-col gap-4 rounded-2xl py-6 pr-6 pl-10 text-lg"
					>
						<li>{m.polis_tip_agreeable()}</li>
						<li>{m.polis_tip_one_idea()}</li>
						<li>{m.polis_tip_no_jargon()}</li>
						<li>{m.polis_tip_many_statements()}</li>
						<li>{m.polis_tip_come_back()}</li>
					</ul>
				</div>
			</div>

			<div
				class="bg-background shrink-0 border-t px-6 pt-4 pb-[max(1.5rem,env(safe-area-inset-bottom))]"
			>
				<div class="mx-auto w-full max-w-[520px]">
					<Button
						variant="default"
						size="lg"
						class="h-12 w-full text-base"
						onclick={dismissGuidance}
					>
						{m.polis_guidance_continue()}
					</Button>
				</div>
			</div>
		{:else}
			<div class="min-h-0 flex-1 overflow-y-auto px-6 pb-6" in:fade={{ duration: 200 }}>
				<div class="mx-auto flex min-h-full w-full max-w-[520px] flex-col gap-5">
					<h2 class="text-primary text-3xl font-bold">
						{m.polis_add_opinion()}
					</h2>

					<!-- The box takes whatever height is left, so a long opinion has room and a
					     short screen still shows the whole thing without the box scrolling. -->
					<!-- Read-only while a submit is in flight and through the confirmation beat, so
					     the words stay on screen as sent rather than editable and about to vanish. -->
					<textarea
						bind:this={composer}
						bind:value
						oninput={onEdit}
						placeholder={m.polis_opinion_placeholder()}
						maxlength={MAX_LENGTH}
						readonly={submitting}
						class="bg-card text-foreground placeholder:text-muted-foreground border-input focus:ring-primary/30 min-h-[160px] w-full flex-1 resize-none rounded-2xl border p-5 text-lg outline-none focus:ring-2"
					></textarea>
					<p
						class="self-end text-base {remaining <= 0
							? 'text-destructive'
							: 'text-muted-foreground'}"
						aria-live="polite"
					>
						{remaining === 1
							? m.polis_characters_left_one({ count: remaining })
							: m.polis_characters_left({ count: remaining })}
					</p>
				</div>
			</div>

			<div
				class="bg-background shrink-0 border-t px-6 pt-4 pb-[max(1.5rem,env(safe-area-inset-bottom))]"
			>
				<!-- Every state of the bar is the same two rows at the same heights, so nothing
				     above it moves when a submit lands or fails. The confirmation sits where the
				     tap did: the button becomes the receipt, in the completion green rather than
				     the primary it was a moment ago, and the second row says what happens next.
				     A failure takes the second row too, with the button left in place to retry. -->
				<div class="mx-auto flex w-full max-w-[520px] flex-col items-center gap-2">
					{#if submitted}
						<!-- The same Button, so the pill does not change shape under the tap. Inert
						     rather than disabled: disabled would fade it, and this is the one moment
						     it should be at full strength. -->
						<Button
							variant="default"
							size="lg"
							tabindex={-1}
							aria-hidden="true"
							class="bg-step-complete text-step-complete-foreground hover:bg-step-complete pointer-events-none h-12 w-full text-base font-semibold"
						>
							<Check class="size-5" strokeWidth={3} />
							{m.polis_opinion_added()}
						</Button>
					{:else}
						<LoadingButton
							variant="default"
							size="lg"
							loading={submitting}
							disabled={!canSubmit}
							onclick={onSubmit}
							class="h-12 w-full text-base"
						>
							{m.polis_submit_opinion()}
						</LoadingButton>
					{/if}
					<div class="flex h-10 w-full items-center justify-center" aria-live="polite">
						{#if submitted}
							<p class="text-muted-foreground text-center text-base" role="status">
								{m.polis_opinion_submitted()}
							</p>
						{:else if error}
							<p
								class="text-destructive text-center text-base font-medium"
								role="alert"
							>
								{m.something_went_wrong()}
							</p>
						{:else}
							<LoadingButton
								variant="link"
								size="lg"
								loading={submitting}
								disabled={!canSubmit}
								onclick={onSubmitAndAddAnother}
								class="text-base font-medium"
							>
								{m.polis_submit_and_add_another()}
							</LoadingButton>
						{/if}
					</div>
				</div>
			</div>
		{/if}
	</div>
</Portal>
