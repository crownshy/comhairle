<script lang="ts">
	import { Button, LoadingButton } from '$lib/components/ui/button';
	import { fade } from 'svelte/transition';
	import { ChevronLeft, MessageSquare } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';

	type Props = {
		value: string;
		submitting: boolean;
		submitted: boolean;
		error: boolean;
		/** Whether to open on the rules rather than the composer. First visit only. */
		startOnGuidance: boolean;
		onClose: () => void;
		/** Editing clears a failed submit, so the error banner does not outlive the attempt. */
		onEdit: () => void;
		/** Called once the participant has read the rules, so they are not shown again. */
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

	// True only while the rules are the screen you arrived on. Reopening them later from the
	// composer makes back mean "return to what I was writing", not "leave".
	let guidanceIsEntry = $state(startOnGuidance);

	let composer = $state<HTMLTextAreaElement | null>(null);

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
     covering the pager stops a stray forward tap from completing the step mid-sentence. -->
<div
	class="bg-background fixed inset-0 z-50 flex flex-col"
	role="dialog"
	aria-modal="true"
	aria-label={m.polis_add_your_own_opinion()}
	in:fade={{ duration: 200 }}
>
	<div class="mx-auto flex h-[72px] w-full max-w-5xl shrink-0 items-center px-4 md:h-20 md:px-6">
		<button
			type="button"
			class="text-foreground inline-flex items-center gap-1"
			onclick={goBack}
		>
			<ChevronLeft class="size-6 shrink-0" />
			<span class="text-base font-medium">{m.pager_back()}</span>
		</button>
	</div>

	<div class="min-h-0 flex-1 overflow-y-auto px-6 pb-10">
		<div class="mx-auto flex w-full max-w-[520px] flex-col items-center gap-6">
			<MessageSquare class="text-primary size-10 shrink-0" aria-hidden="true" />

			{#if showingGuidance}
				<div class="flex w-full flex-col items-center gap-6" in:fade={{ duration: 200 }}>
					<h2 class="text-primary text-center text-3xl font-bold">
						{m.polis_guidance_title()}
					</h2>
					<p class="text-muted-foreground text-center text-base">
						{m.polis_guidance_intro()}
					</p>
					<ul
						class="text-foreground bg-muted/50 flex w-full list-outside list-disc flex-col gap-3 rounded-2xl py-6 pr-6 pl-10 text-base"
					>
						<li>{m.polis_tip_agreeable()}</li>
						<li>{m.polis_tip_one_idea()}</li>
						<li>{m.polis_tip_no_jargon()}</li>
						<li>{m.polis_tip_many_statements()}</li>
						<li>{m.polis_tip_come_back()}</li>
					</ul>
					<Button
						variant="default"
						size="lg"
						class="h-12 w-full text-base"
						onclick={dismissGuidance}
					>
						{m.polis_guidance_continue()}
					</Button>
				</div>
			{:else}
				<div class="flex w-full flex-col items-center gap-6" in:fade={{ duration: 200 }}>
					<h2 class="text-primary text-center text-3xl font-bold">
						{m.polis_add_opinion()}
					</h2>

					{#if submitted}
						<div
							class="bg-primary/10 text-primary w-full rounded-lg p-4 text-center font-medium"
						>
							{m.polis_opinion_submitted()}
						</div>
					{:else if error}
						<div
							class="bg-destructive/10 text-destructive w-full rounded-lg p-4 text-center font-medium"
						>
							{m.something_went_wrong()}
						</div>
					{/if}

					<textarea
						bind:this={composer}
						bind:value
						oninput={onEdit}
						placeholder={m.polis_opinion_placeholder()}
						class="bg-card text-foreground placeholder:text-muted-foreground border-input focus:ring-primary/30 min-h-[220px] w-full resize-none rounded-2xl border p-5 text-base outline-none focus:ring-2"
					></textarea>

					<div class="flex w-full flex-col items-center gap-3">
						<LoadingButton
							variant="default"
							size="lg"
							loading={submitting}
							disabled={!value.trim()}
							onclick={onSubmit}
							class="h-12 w-full text-base"
						>
							{m.submit()}
						</LoadingButton>
						<LoadingButton
							variant="link"
							size="lg"
							loading={submitting}
							disabled={!value.trim()}
							onclick={onSubmitAndAddAnother}
							class="text-base font-medium"
						>
							{m.polis_submit_and_add_another()}
						</LoadingButton>
					</div>

					<button
						type="button"
						class="text-muted-foreground hover:text-foreground text-sm font-medium underline decoration-dotted underline-offset-4 transition-colors"
						onclick={() => (showingGuidance = true)}
					>
						{m.polis_guidance_reopen()}
					</button>
				</div>
			{/if}
		</div>
	</div>
</div>
