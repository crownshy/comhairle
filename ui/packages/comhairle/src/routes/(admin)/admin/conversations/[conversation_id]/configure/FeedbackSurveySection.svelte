<script lang="ts">
	/* The preview link is built by $lib/urls, not from a typed route id, so resolve() has
	   nothing to resolve. */
	/* eslint-disable svelte/no-navigation-without-resolve */
	/**
	 * The Feedback tab: the survey asked on the thank-you page.
	 *
	 * One HeyForm form per conversation, owned by the conversation rather than by a step. Until
	 * it exists the tab explains what it is and offers to create it. Once it does, the tab is the
	 * builder (the same one a survey step uses) over the responses so far, with removal at the
	 * bottom where it will not be pressed by accident.
	 */
	import { invalidate } from '$app/navigation';
	import { ExternalLink, LoaderCircle } from 'lucide-svelte';
	import type { ConversationWithTranslations, FeedbackSurveyDto } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { Button } from '$lib/components/ui/button';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import HeyFormManage from '$lib/tools/heyform/HeyFormManage.svelte';
	import SurveyInsights from '$lib/reports/survey/SurveyInsights.svelte';
	import type { SurveyQuestion } from '$lib/reports/survey/insights-loader';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { thank_you_page } from '$lib/urls';

	let {
		conversation,
		workflowId,
		survey,
		insights
	}: {
		conversation: ConversationWithTranslations;
		workflowId: string | undefined;
		survey: FeedbackSurveyDto | null;
		insights: SurveyQuestion[] | null;
	} = $props();

	// Only a HeyForm config can be built and embedded. Anything else is treated as no survey.
	let heyform = $derived(survey?.toolConfig.type === 'heyform' ? survey.toolConfig : null);

	let creating = $state(false);
	let removing = $state(false);
	let removeOpen = $state(false);

	let previewUrl = $derived(
		workflowId ? thank_you_page(conversation.id, workflowId, true) : null
	);

	async function create() {
		creating = true;
		const res = await tryCatchAsync(() =>
			apiClient.CreateFeedbackSurvey(undefined, {
				params: { conversation_id: conversation.id }
			})
		);
		creating = false;
		if (res.err !== null) {
			notifications.send({
				message: 'Could not create the feedback survey',
				priority: 'ERROR'
			});
			return;
		}
		await invalidate('conversation:feedback-survey');
	}

	async function remove() {
		removing = true;
		const res = await tryCatchAsync(() =>
			apiClient.DeleteFeedbackSurvey(undefined, {
				params: { conversation_id: conversation.id }
			})
		);
		removing = false;
		if (res.err !== null) {
			notifications.send({
				message: 'Could not remove the feedback survey',
				priority: 'ERROR'
			});
			return;
		}
		removeOpen = false;
		notifications.send({ message: 'Feedback survey removed', priority: 'INFO' });
		await invalidate('conversation:feedback-survey');
	}
</script>

{#if !heyform}
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<p class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">Feedback survey</p>
		<div class="flex flex-1 flex-col gap-4">
			<p class="text-muted-foreground text-sm">
				Once a participant has finished every step, the thank-you page can ask them how it
				went. It is not a step: it does not count towards their progress, they can ignore
				it, and their answers are kept apart from what they contributed to the conversation.
			</p>
			<p class="text-muted-foreground text-sm">
				Creating it gives you an empty survey to write the questions in. Multiple choice and
				long text questions both report well.
			</p>
			<div>
				<Button onclick={create} disabled={creating}>
					{#if creating}
						<LoaderCircle class="mr-2 size-4 animate-spin" />
					{/if}
					Create feedback survey
				</Button>
			</div>
		</div>
	</div>
{:else}
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<div class="flex flex-col gap-1 lg:w-50 lg:shrink-0 lg:pt-2">
			<p class="text-sm font-semibold">Questions</p>
			<p class="text-muted-foreground text-sm">
				Changes save as you make them. Participants see the questions as they are the moment
				they open the survey.
			</p>
			{#if previewUrl}
				<a
					href={previewUrl}
					target="_blank"
					rel="noopener"
					class="text-primary mt-2 inline-flex items-center gap-1 text-sm underline underline-offset-4"
				>
					Preview the thank-you page
					<ExternalLink class="size-3.5" aria-hidden="true" />
				</a>
			{/if}
		</div>
		<!-- The builder fills whatever box it is given, so it gets a tall one. -->
		<div class="border-border h-[75vh] min-h-120 flex-1 overflow-hidden rounded-lg border">
			<HeyFormManage
				conversation_id={conversation.id}
				survey_url={heyform.server_url}
				survey_id={heyform.survey_id}
				admin_user={heyform.admin_user}
				admin_password={heyform.admin_password}
				workspace_id={heyform.workspace_id}
				project_id={heyform.project_id}
			/>
		</div>
	</div>

	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<p class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">Responses</p>
		<div class="flex-1">
			<SurveyInsights data={insights ?? []} />
		</div>
	</div>

	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<p class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">Remove</p>
		<div class="flex flex-1 flex-col gap-3">
			<p class="text-muted-foreground text-sm">
				Takes the survey off the thank-you page and deletes the questions and every
				response.
			</p>
			<div>
				<Button variant="destructive" onclick={() => (removeOpen = true)}>
					Remove feedback survey
				</Button>
			</div>
		</div>
	</div>

	<AlertDialog.Root bind:open={removeOpen}>
		<AlertDialog.Content>
			<AlertDialog.Header>
				<AlertDialog.Title>Remove the feedback survey?</AlertDialog.Title>
				<AlertDialog.Description>
					The questions and every response so far are deleted and cannot be recovered.
				</AlertDialog.Description>
			</AlertDialog.Header>
			<AlertDialog.Footer class="flex-col-reverse sm:flex-row">
				<AlertDialog.Cancel class="w-full sm:w-auto" disabled={removing}
					>Cancel</AlertDialog.Cancel
				>
				<AlertDialog.Action
					class="bg-destructive hover:bg-destructive/90 w-full text-white sm:w-auto"
					disabled={removing}
					onclick={(e) => {
						e.preventDefault();
						remove();
					}}
				>
					{#if removing}
						<LoaderCircle class="mr-2 size-4 animate-spin" />
					{/if}
					Remove
				</AlertDialog.Action>
			</AlertDialog.Footer>
		</AlertDialog.Content>
	</AlertDialog.Root>
{/if}
