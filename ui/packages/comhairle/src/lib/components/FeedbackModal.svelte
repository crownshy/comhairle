<script lang="ts">
	import { apiClient } from '@crownshy/api-client/client';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Textarea } from '$lib/components/ui/textarea';
	import { notifications } from '$lib/notifications.svelte';
	import { m } from '$lib/paraglide/messages';
	import { cn } from '$lib/utils';
	import { tryCatchAsync } from '$lib/utils/errorHandling';

	type Props = {
		conversationId: string;
		/** Extra classes for the trigger button. */
		class?: string;
	};
	let { conversationId, class: className }: Props = $props();

	let feedback = $state('');
	let open = $state(false);
	let submitting = $state(false);
	// Success toasts are switched off app-wide, so a thank-you replaces the form instead.
	let sent = $state(false);

	async function submitFeedback() {
		submitting = true;
		const submission = await tryCatchAsync(() =>
			apiClient.CreateFeedback(
				{ content: feedback },
				{ params: { conversation_id: conversationId } }
			)
		);
		submitting = false;

		if (submission.err !== null) {
			notifications.send({ message: m.feedback_send_failed(), priority: 'ERROR' });
			return;
		}
		sent = true;
	}

	function resetWhenClosed(isOpen: boolean) {
		if (isOpen) return;
		feedback = '';
		sent = false;
	}
</script>

<Dialog.Root bind:open onOpenChangeComplete={resetWhenClosed}>
	<Dialog.Trigger class={cn(buttonVariants({ variant: 'outline', size: 'xl' }), className)}
		>{m.share_feedback()}</Dialog.Trigger
	>
	<Dialog.Content class="gap-6 p-8 sm:max-w-2xl">
		<Dialog.Header class="gap-3">
			<Dialog.Title class="text-2xl">{m.share_feedback()}</Dialog.Title>
			<Dialog.Description class="text-lg">
				{m.we_are_interested_in_hearing_what_is_good_and_bad_about_this_process()}
			</Dialog.Description>
		</Dialog.Header>
		{#if sent}
			<p class="text-lg" role="status">{m.feedback_thanks()}</p>
			<Button size="xl" class="w-full" onclick={() => (open = false)}>{m.done()}</Button>
		{:else}
			<Textarea
				id="feedback"
				aria-label={m.share_feedback()}
				placeholder={m.type_your_feedback_here()}
				bind:value={feedback}
				class="min-h-40 text-lg md:text-lg"
			/>
			<Button
				size="xl"
				class="w-full"
				onclick={submitFeedback}
				disabled={submitting || feedback.trim() === ''}
			>
				{m.submit()}
			</Button>
		{/if}
	</Dialog.Content>
</Dialog.Root>
