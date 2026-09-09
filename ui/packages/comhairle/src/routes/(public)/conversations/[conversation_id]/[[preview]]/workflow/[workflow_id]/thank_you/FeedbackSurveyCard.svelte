<script lang="ts">
	/**
	 * The invitation to say how the flow went, and the survey behind it.
	 *
	 * The card has two states, asked and done. Its button opens the survey over the whole
	 * screen (FeedbackSurveyOverlay); closing that without finishing leaves the card as it was,
	 * so the offer stands. When the form reports it is finished, the card thanks them and tells
	 * the API, which is where "done" lives: the page loads it with the survey, so the answer is
	 * the same on any device and after clearing the browser.
	 */
	import { MessageSquareHeart, Check } from 'lucide-svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { Button } from '$lib/components/ui/button';
	import FeedbackSurveyOverlay from './FeedbackSurveyOverlay.svelte';
	import { haptic } from '$lib/utils/haptics';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import * as m from '$lib/paraglide/messages';

	let {
		conversationId,
		userId,
		surveyId,
		serverUrl,
		surveyUrl,
		completed,
		appearDelayMs = 0
	}: {
		conversationId: string;
		userId: string;
		surveyId: string;
		serverUrl: string;
		surveyUrl: string;
		/** Whether this user already finished it, as the API has it. */
		completed: boolean;
		/** How long to wait before sliding in, so the ask lands after the stats settle. */
		appearDelayMs?: number;
	} = $props();

	// Writable, so finishing flips the card without waiting for the page to reload its data.
	let done = $derived(completed);
	let open = $state(false);

	async function finished() {
		open = false;
		done = true;
		haptic('success');
		// The answers are already on HeyForm by now. A failed marker only means the card may
		// ask again next visit, so it is not surfaced as an error.
		await tryCatchAsync(() =>
			apiClient.CompleteFeedbackSurvey(undefined, {
				params: { conversation_id: conversationId }
			})
		);
	}
</script>

<section
	class="bg-card text-card-foreground border-border animate-in fade-in slide-in-from-bottom-2 fill-mode-both w-full rounded-2xl border p-5 text-left duration-500 motion-reduce:animate-none md:p-6"
	style="animation-delay: {appearDelayMs}ms"
	aria-labelledby="feedback-survey-title"
>
	{#if done}
		<div class="flex items-start gap-4">
			<span
				class="bg-step-complete text-step-complete-foreground flex size-10 shrink-0 items-center justify-center rounded-full"
				aria-hidden="true"
			>
				<Check class="size-5" />
			</span>
			<div class="flex flex-col gap-1">
				<h2 id="feedback-survey-title" class="text-lg font-semibold">
					{m.thank_you_feedback_done_title()}
				</h2>
				<p class="text-muted-foreground text-base">{m.thank_you_feedback_done_body()}</p>
			</div>
		</div>
	{:else}
		<div class="flex items-start gap-4">
			<span
				class="bg-accent text-accent-foreground flex size-10 shrink-0 items-center justify-center rounded-full"
				aria-hidden="true"
			>
				<MessageSquareHeart class="size-5" />
			</span>
			<div class="flex min-w-0 flex-1 flex-col gap-1">
				<p id="feedback-survey-title" class="text-base">{m.thank_you_feedback_body()}</p>
			</div>
		</div>

		<div class="mt-5 flex justify-center sm:justify-end">
			<Button size="lg" class="w-full sm:w-auto" onclick={() => (open = true)}>
				{m.thank_you_feedback_cta()}
			</Button>
		</div>
	{/if}
</section>

{#if open}
	<FeedbackSurveyOverlay
		{userId}
		{surveyId}
		{serverUrl}
		{surveyUrl}
		onDone={finished}
		onClose={() => (open = false)}
	/>
{/if}
