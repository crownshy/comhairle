<script lang="ts">
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { buttonVariants } from '$lib/components/ui/button';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import type { MyDemographicAnswer } from '@crownshy/api-client/api';
	import { formatAnswer } from './formatAnswer';

	type Props = {
		answers: MyDemographicAnswer[];
		conversationTitles: Record<string, string>;
		onAnonymised: () => Promise<void>;
	};

	let { answers, conversationTitles, onAnonymised }: Props = $props();

	let anonymising = $state<string | null>(null);

	async function anonymise(responseId: string) {
		anonymising = responseId;
		const result = await tryCatchAsync(() =>
			apiClient.AnonymiseMyDemographicAnswer(undefined, {
				params: { response_id: responseId }
			})
		);
		anonymising = null;

		if (result.err) {
			notifications.send({ message: 'Could not anonymise this answer', priority: 'ERROR' });
			return;
		}
		notifications.send({ message: 'Answer anonymised', priority: 'INFO' });
		await onAnonymised();
	}

	const givenIn = (conversationId: string | null | undefined) =>
		conversationId ? (conversationTitles[conversationId] ?? 'A conversation') : 'Your profile';
</script>

<div class="mt-10" id="demographics">
	<h3 class="mb-2 text-2xl">Your demographic answers</h3>
	<p class="text-muted-foreground mb-6 text-base">
		Answers cannot be changed once they are submitted. You can anonymise an answer to remove it
		from your account. It will then no longer be linked to you, and the results it contributed
		to stay the same.
	</p>

	{#if answers.length === 0}
		<p class="text-muted-foreground text-base">You have not answered any questions yet.</p>
	{:else}
		<ul class="flex flex-col gap-4">
			{#each answers as { answer, version } (answer.id)}
				<li class="bg-card flex flex-col gap-3 p-4 md:flex-row md:items-center">
					<div class="flex-1">
						<p class="text-base font-semibold">{version.text}</p>
						<p class="text-base">{formatAnswer(version, answer.value)}</p>
						<p class="text-muted-foreground text-base">
							{givenIn(answer.conversation_id)} on
							{new Date(answer.created_at).toLocaleDateString()}
						</p>
					</div>
					<AlertDialog.Root>
						<AlertDialog.Trigger
							class={buttonVariants({ variant: 'outline' })}
							disabled={anonymising === answer.id}
						>
							Anonymise
						</AlertDialog.Trigger>
						<AlertDialog.Content>
							<AlertDialog.Header>
								<AlertDialog.Title>Anonymise this answer?</AlertDialog.Title>
								<AlertDialog.Description>
									It will no longer be linked to your account and will not be
									suggested the next time you are asked. This cannot be undone.
								</AlertDialog.Description>
							</AlertDialog.Header>
							<AlertDialog.Footer>
								<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
								<AlertDialog.Action onclick={() => anonymise(answer.id)}>
									Anonymise
								</AlertDialog.Action>
							</AlertDialog.Footer>
						</AlertDialog.Content>
					</AlertDialog.Root>
				</li>
			{/each}
		</ul>
	{/if}
</div>
