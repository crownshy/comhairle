<script lang="ts">
	import * as Form from '$lib/components/ui/form';
	import { Switch } from '$lib/components/ui/switch';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { accessSchema } from './accessSchema';
	import FieldLabel from '../FieldLabel.svelte';
	import PreviewPasswordSetting from './PreviewPasswordSetting.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import { camelToSnakeCase } from '$lib/utils/casingUtils';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';
	import type {
		ConversationWithTranslations,
		PreviewPasswordStatus,
		WorkflowDto
	} from '@crownshy/api-client/api';
	import { permissions } from '$lib/permissions.svelte';
	import { onDestroy } from 'svelte';

	type Props = {
		conversation: ConversationWithTranslations;
		workflow: WorkflowDto;
		previewPasswordStatus: PreviewPasswordStatus | null;
	};

	let { conversation, workflow, previewPasswordStatus }: Props = $props();

	const canAdmin = $derived(
		permissions.can('conversation', 'conversation_admin', conversation.id)
	);

	const accessForm = superForm(
		{
			isPublic: conversation.isPublic,
			isInviteOnly: conversation.isInviteOnly,
			autoLogin: workflow.autoLogin,
			enableSignupPrompts: conversation.enableSignupPrompts,
			showThankYouPageAnnonInstructions: conversation.showThankYouPageAnnonInstructions,
			showThankyouPageFeedbackButton: conversation.showThankyouPageFeedbackButton,
			allowRevisitAfterFinishing: conversation.allowRevisitAfterFinishing
		},
		{
			validators: zodClient(accessSchema),
			taintedMessage: false,
			validationMethod: 'oninput'
		}
	);

	const { form } = accessForm;

	type ConversationToggle =
		| 'isPublic'
		| 'isInviteOnly'
		| 'enableSignupPrompts'
		| 'showThankYouPageAnnonInstructions'
		| 'showThankyouPageFeedbackButton'
		| 'allowRevisitAfterFinishing';

	type AccessToggle = ConversationToggle | 'autoLogin';

	// A toggle saves without reloading; the conversation reloads once when the page is left.
	let hasSavedConversation = false;
	let hasSavedWorkflow = false;
	onDestroy(() => {
		if (hasSavedConversation) invalidate(key('admin/conversation'));
		if (hasSavedWorkflow) invalidate(key('admin/conversation/meta'));
	});

	async function saveConversationToggle(field: ConversationToggle, value: boolean) {
		if (!canAdmin) return;
		const result = await tryCatchAsync(() =>
			apiClient.UpdateConversation(
				{ [camelToSnakeCase(field)]: value },
				{ params: { conversation_id: conversation.id } }
			)
		);
		if (result.err !== null) {
			$form[field] = !value;
			notifications.send({ message: 'Failed to update setting', priority: 'ERROR' });
			return;
		}
		hasSavedConversation = true;
		notifications.send({ message: 'Setting updated', priority: 'INFO' });
	}

	async function saveAutoLogin(value: boolean) {
		if (!canAdmin) return;
		const result = await tryCatchAsync(() =>
			apiClient.UpdateConversationWorkflow(
				{ auto_login: value },
				{ params: { conversation_id: conversation.id, workflow_id: workflow.id } }
			)
		);
		if (result.err !== null) {
			$form.autoLogin = !value;
			notifications.send({ message: 'Failed to update setting', priority: 'ERROR' });
			return;
		}
		hasSavedWorkflow = true;
		notifications.send({ message: 'Setting updated', priority: 'INFO' });
	}
</script>

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

<section class="border-border flex flex-col gap-1 border-t pt-6" aria-labelledby="access-heading">
	<h3 id="access-heading" class="text-lg font-semibold">Access</h3>
	<p class="text-muted-foreground text-base">Visibility, invites and participation.</p>

	<div class="divide-border flex max-w-3xl flex-col divide-y">
		<section class="flex flex-col py-6">
			<h4 class="text-base font-semibold">Visibility</h4>
			<div class="flex flex-col">
				{@render toggleRow(
					'isPublic',
					'Show conversation publicly',
					'When this conversation is launched, anyone (even without an account) can open its documents and data. Off means only you, collaborators, and participants can.',
					"Let anyone view this conversation's data once it's launched.",
					(value) => saveConversationToggle('isPublic', value)
				)}
			</div>
		</section>

		<section class="flex flex-col py-6">
			<h4 class="text-base font-semibold">Who can take part</h4>
			<div class="flex flex-col">
				{@render toggleRow(
					'isInviteOnly',
					'Only allow participation by invite',
					'Only people you invite can take part. With this off, anyone with the link can participate.',
					'Admins can invite and manage members.',
					(value) => saveConversationToggle('isInviteOnly', value)
				)}
				{@render toggleRow(
					'autoLogin',
					'Automatically log in with an anonymous account',
					'Visitors who are not signed in get a temporary anonymous account automatically, so they can take part without registering. They can upgrade to a real account later.',
					'Creates a temporary account for unauthenticated users.',
					(value) => saveAutoLogin(value)
				)}
			</div>
		</section>

		<section class="flex flex-col py-6">
			<h4 class="text-base font-semibold">After finishing</h4>
			<div class="flex flex-col">
				{@render toggleRow(
					'allowRevisitAfterFinishing',
					'Allow revisit after finishing',
					'Once a participant has finished every step they reach the thank-you page. Turn this off to seal them out: no step is reachable afterwards, the revisit links disappear, and the server rejects any further contributions.',
					'Let participants return to the steps after they have finished.',
					(value) => saveConversationToggle('allowRevisitAfterFinishing', value)
				)}
				{@render toggleRow(
					'enableSignupPrompts',
					'Enable signup prompts',
					'Shows prompts encouraging participants to create an account on the thank-you page after they finish.',
					'Toggle whether to display signup prompts on thank you page.',
					(value) => saveConversationToggle('enableSignupPrompts', value)
				)}
				{@render toggleRow(
					'showThankYouPageAnnonInstructions',
					'Show thank you page anonymous instructions',
					'On the thank-you page, shows anonymous participants their temporary ID and how to log back in later to see the results.',
					'Display instructions for anonymous users on the thank you page.',
					(value) => saveConversationToggle('showThankYouPageAnnonInstructions', value)
				)}
				{@render toggleRow(
					'showThankyouPageFeedbackButton',
					'Show thank you page feedback button',
					'On the thank-you page, shows participants a button to give feedback on the process.',
					'Display the feedback button on the thank you page.',
					(value) => saveConversationToggle('showThankyouPageFeedbackButton', value)
				)}
			</div>
		</section>

		<section class="flex flex-col gap-4 py-6">
			<h4 class="text-base font-semibold">Preview link</h4>
			<PreviewPasswordSetting
				conversationId={conversation.id}
				conversationSlug={conversation.slug ?? conversation.id}
				isLive={conversation.isLive}
				status={previewPasswordStatus}
				canEdit={canAdmin}
			/>
		</section>
	</div>
</section>
