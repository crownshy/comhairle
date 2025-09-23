<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import * as m from '$lib/paraglide/messages.js';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '$lib/api/client';
	import { Button, buttonVariants } from './ui/button';
	import { Label } from './ui/label';
	import { Textarea } from './ui/textarea';

	type FeedbackModalProps = {
		conversation_id: string;
	};
	let { conversation_id } = $props();

	let feedback = $state('');
	let open = $state(false);

	async function submitFeedback() {
		try {
			await apiClient.CreateFeedback({ content: feedback }, { params: { conversation_id } });
			notifications.send({ message: 'Thanks for your feedback', priority: 'INFO' });
			open = false;
		} catch (e) {
			console.warn('Failed to create feedback', e);
			notifications.send({ message: 'Failed to create feedback', priority: 'ERROR' });
		}
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Trigger class={buttonVariants({ variant: 'outline-solid' })}>{m.give_feedback()}</Dialog.Trigger
	>
	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>{m.give_feedback()}</Dialog.Title>
			<Dialog.Description>
				{m.we_are_interested_in_hearing_what_is_good_and_bad_about_this_process()}
			</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<div class="flex flex-col gap-4">
				<Textarea
					id="feedback"
					placeholder={m.type_your_feedback_here()}
					bind:value={feedback}
					class="col-span-3"
				/>
			</div>
		</div>
		<Dialog.Footer>
			<Button onclick={submitFeedback} type="submit">{m.submit()}</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
