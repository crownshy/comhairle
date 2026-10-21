<script lang="ts">
	import PageHeader from '$lib/components/PageHeader.svelte';
	import * as Form from '$lib/components/ui/form';
	import { Switch } from '$lib/components/ui/switch';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { accessSchema } from './schema';
	import FieldLabel from '../FieldLabel.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import { camelToSnakeCase } from '$lib/utils/casingUtils';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';
	import type { WorkflowStepWithTranslationsDto } from '@crownshy/api-client/api';
	import { permissions } from '$lib/permissions.svelte';

	const { data } = $props();
	const { conversation } = $derived(data);
	const canAdmin = $derived(
		permissions.can('conversation', 'conversation_admin', conversation.id)
	);

	let workflowId = $state<WorkflowStepWithTranslationsDto['id'] | null>(null);

	let accessForm = superForm(
		{
			isPublic: data.conversation.isPublic,
			isInviteOnly: data.conversation.isInviteOnly,
			autoLogin: data.workflows[0].autoLogin,
			enableSignupPrompts: data.conversation.enableSignupPrompts,
			showThankYouPageAnnonInstructions: data.conversation.showThankYouPageAnnonInstructions,
			showThankyouPageFeedbackButton: data.conversation.showThankyouPageFeedbackButton,
			allowRevisitAfterFinishing: data.conversation.allowRevisitAfterFinishing
		},
		{
			validators: zodClient(accessSchema),
			taintedMessage: false,
			validationMethod: 'oninput'
		}
	);

	const { form } = $derived(accessForm);

	type ConversationToggle =
		| 'isPublic'
		| 'isInviteOnly'
		| 'enableSignupPrompts'
		| 'showThankYouPageAnnonInstructions'
		| 'showThankyouPageFeedbackButton'
		| 'allowRevisitAfterFinishing';

	type AccessToggle = ConversationToggle | 'autoLogin';

	async function saveConversationToggle(field: ConversationToggle, value: boolean) {
		if (!canAdmin) return;
		const res = await tryCatchAsync(() =>
			apiClient.UpdateConversation(
				{ [camelToSnakeCase(field)]: value },
				{ params: { conversation_id: conversation.id } }
			)
		);
		if (res.err !== null) {
			console.error(res.err);
			$form[field] = !value;
			notifications.send({ message: 'Failed to update setting', priority: 'ERROR' });
			return;
		}
		notifications.send({ message: 'Setting updated', priority: 'INFO' });
		await invalidate(key('admin/conversation'));
	}

	async function saveAutoLogin(value: boolean) {
		if (!canAdmin) return;
		if (workflowId === null) {
			const workflows = await tryCatchAsync(() =>
				apiClient.ListConversationWorkflows({
					params: { conversation_id: conversation.id }
				})
			);

			if (workflows.err !== null) {
				notifications.addFlash({
					message: 'Could not get the workflow, please try again',
					priority: 'ERROR'
				});
				return;
			}

			workflowId = workflows.ok[0].id;
		}

		const updateConversationWorkflow = await tryCatchAsync(() =>
			apiClient.UpdateConversationWorkflow(
				{ auto_login: value },
				{ params: { conversation_id: conversation.id, workflow_id: workflowId! } }
			)
		);
		if (updateConversationWorkflow.err !== null) {
			console.error(updateConversationWorkflow.err);
			$form.autoLogin = !value;
			notifications.send({ message: 'Failed to update setting', priority: 'ERROR' });
			return;
		}
		notifications.send({ message: 'Setting updated', priority: 'INFO' });
		await invalidate(key('admin/conversation/meta'));
	}
</script>

<PageHeader level="h2" title="Access" description="Visibility, invites and participation." />

{#snippet toggleRow(
	name: AccessToggle,
	label: string,
	info: string,
	description: string,
	onChange: (value: boolean) => void
)}
	<Form.Field form={accessForm} {name} class="py-3">
		<Form.Control>
			{#snippet children({ props })}
				<div class="flex items-center justify-between gap-6">
					<div class="flex flex-col gap-1">
						<FieldLabel {label} {info} class="lg:w-auto" />
						<Form.Description class="text-muted-foreground text-sm">
							{description}
						</Form.Description>
					</div>
					<Switch
						{...props}
						bind:checked={$form[name]}
						disabled={!canAdmin}
						onCheckedChange={onChange}
					/>
				</div>
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
{/snippet}

<div class="divide-border flex max-w-3xl flex-col divide-y">
	<section class="flex flex-col py-6">
		<h3 class="text-base font-semibold">Visibility</h3>
		<div class="flex flex-col">
			{@render toggleRow(
				'isPublic',
				'Show conversation publicly',
				'When this conversation is launched, anyone (even without an account) can open its documents and data. Off means only you, collaborators, and participants can.',
				"Let anyone view this conversation's data once it's launched.",
				(v) => saveConversationToggle('isPublic', v)
			)}
		</div>
	</section>

	<section class="flex flex-col py-6">
		<h3 class="text-base font-semibold">Who can take part</h3>
		<div class="flex flex-col">
			{@render toggleRow(
				'isInviteOnly',
				'Only allow participation by invite',
				'Only people you invite can take part. With this off, anyone with the link can participate.',
				'Admins can invite and manage members.',
				(v) => saveConversationToggle('isInviteOnly', v)
			)}
			{@render toggleRow(
				'autoLogin',
				'Automatically log in with an anonymous account',
				'Visitors who are not signed in get a temporary anonymous account automatically, so they can take part without registering. They can upgrade to a real account later.',
				'Creates a temporary account for unauthenticated users.',
				(v) => saveAutoLogin(v)
			)}
		</div>
	</section>

	<section class="flex flex-col py-6">
		<h3 class="text-base font-semibold">After finishing</h3>
		<div class="flex flex-col">
			{@render toggleRow(
				'allowRevisitAfterFinishing',
				'Allow revisit after finishing',
				'Once a participant has finished every step they reach the thank-you page. Turn this off to seal them out: no step is reachable afterwards, the revisit links disappear, and the server rejects any further contributions.',
				'Let participants return to the steps after they have finished.',
				(v) => saveConversationToggle('allowRevisitAfterFinishing', v)
			)}
			{@render toggleRow(
				'enableSignupPrompts',
				'Enable signup prompts',
				'Shows prompts encouraging participants to create an account on the thank-you page after they finish.',
				'Toggle whether to display signup prompts on thank you page.',
				(v) => saveConversationToggle('enableSignupPrompts', v)
			)}
			{@render toggleRow(
				'showThankYouPageAnnonInstructions',
				'Show thank you page anonymous instructions',
				'On the thank-you page, shows anonymous participants their temporary ID and how to log back in later to see the results.',
				'Display instructions for anonymous users on the thank you page.',
				(v) => saveConversationToggle('showThankYouPageAnnonInstructions', v)
			)}
			{@render toggleRow(
				'showThankyouPageFeedbackButton',
				'Show thank you page feedback button',
				'On the thank-you page, shows participants a button to give feedback on the process.',
				'Display the feedback button on the thank you page.',
				(v) => saveConversationToggle('showThankyouPageFeedbackButton', v)
			)}
		</div>
	</section>
</div>
