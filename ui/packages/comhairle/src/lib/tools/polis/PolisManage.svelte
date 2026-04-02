<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { camelToSentenceCase, camelToSnakeCase } from '$lib/utils/casingUtils';
	import { apiClient } from '@crownshy/api-client/client';
	import { useDebounce } from 'runed';

	let {
		toolConfig,
		conversationId,
		workflowId,
		workflowStepId
	}: {
		toolConfig: any; // TODO:
		conversationId: string;
		workflowId: string;
		workflowStepId: string;
	} = $props();

	const {
		poll_id: polisId,
		server_url: polisUrl,
		admin_password: adminPassword,
		admin_user: adminUser,
		required_votes: requiredVotes
	} = $derived(toolConfig);

	let base_url = $derived(polisUrl.startsWith('https://') ? polisUrl : `https://${polisUrl}`);
	let url = $derived(`${base_url}/m/${polisId}`);
	let iframe = $state();
	let firstLoad = $state(true);

	function tryLogin() {
		if (firstLoad) {
			iframe.contentWindow.postMessage(
				{ user: adminUser, password: adminPassword, type: 'POLIS_LOGIN' },
				base_url
			);
			firstLoad = false;
		}
	}

	const debouncedUpdateToolConfig = useDebounce(async (e: Event, field: string) => {
		const value = +(e.target as HTMLInputElement).value;

		try {
			await apiClient.UpdateConversationWorkflowStep(
				{
					preview_tool_config: { ...toolConfig, [camelToSnakeCase(field)]: value }
				},
				{
					params: {
						conversation_id: conversationId,
						workflow_id: workflowId,
						workflow_step_id: workflowStepId
					}
				}
			);
			notifications.send({
				priority: 'INFO',
				message: `Updated ${camelToSentenceCase(field)}`
			});
			await invalidateAll();
		} catch (e) {
			console.error(e);
			notifications.send({ priority: 'ERROR', message: 'Failed to update tool config' });
		}
	}, 500);

	function handleUpdateToolConfig(e: Event, field: string) {
		debouncedUpdateToolConfig(e, field);
	}
</script>

<h2 class="my-5 text-2xl font-bold">Polis Setup</h2>

<div class="mb-8">
	<div class="flex flex-col">
		<Label for="requiredVotes" class="text-lg font-semibold">Required votes</Label>
		<span class="mb-4 text-sm"
			>Number of votes required before a user is able to progress to the next step</span
		>
		<Input
			id="requiredVotes"
			name="requiredVotes"
			type="number"
			class="w-1/4"
			defaultvalue={requiredVotes}
			oninput={(e) => handleUpdateToolConfig(e, 'requiredVotes')}
		/>
	</div>
</div>

<div class="grid grid-cols-[1fr_30vw]">
	<iframe
		bind:this={iframe}
		onload={tryLogin}
		src={url}
		title="Polis poll"
		style="width:100%;height:100%"
	></iframe>
	<div>
		<h2 class="text-xl">Guidance</h2>
		<p class="my-5">
			Polis seed statements are the initial set of statements provided to participants in a
			Polis conversation to help spark discussion and guide the direction of the debate. They
			serve as a starting framework, giving people something concrete to agree or disagree
			with before participants begin contributing their own ideas. Well-crafted seed
			statements cover a broad range of perspectives on the issue at hand, ensuring that
			participants from different backgrounds and viewpoints feel invited to engage. They
			don’t aim to be exhaustive but instead create an entry point that stimulates
			conversation and reveals patterns of agreement and disagreement across diverse groups.
		</p>
		<p>
			When writing good Polis seed statements, clarity and neutrality are key. Avoid jargon,
			complex phrasing, or emotionally charged language that might alienate participants. The
			statements should be concise, specific, and testable—something participants can
			reasonably say “agree” or “disagree” to, rather than vague sentiments. It’s also helpful
			to include a variety of framings: some that reflect common arguments, some that test
			assumptions, and some that raise trade-offs or edge cases. Finally, balance matters; if
			all the seed statements lean heavily toward one viewpoint, the conversation will feel
			biased from the start. A thoughtful mix ensures participants see themselves reflected
			early on, which increases engagement and makes it easier for the group’s collective
			intelligence to emerge.
		</p>
	</div>
</div>
