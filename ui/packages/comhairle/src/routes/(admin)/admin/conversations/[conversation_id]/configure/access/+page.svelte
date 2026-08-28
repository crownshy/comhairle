<script lang="ts">
	import PageHeader from '$lib/components/PageHeader.svelte';
	import * as Form from '$lib/components/ui/form';
	import { Switch } from '$lib/components/ui/switch';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { accessSchema } from './schema';
	import FieldLabel from '../FieldLabel.svelte';
	import InfoHover from '../InfoHover.svelte';
	import CohostManager from './CohostManager.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import { camelToSnakeCase } from '$lib/utils/casingUtils';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';

	const { data } = $props();
	const { conversation, workflows, cohostOrganizations } = $derived(data);
	const workflow = $derived(workflows[0]);
	const canManageCohosts = $derived(data.user.id === conversation.ownerId);

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

	async function saveConversationToggle(field: ConversationToggle, value: boolean) {
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
		await invalidate(key('conversation'));
	}

	async function saveAutoLogin(value: boolean) {
		const res = await tryCatchAsync(() =>
			apiClient.UpdateConversationWorkflow(
				{ auto_login: value },
				{ params: { conversation_id: conversation.id, workflow_id: workflow.id } }
			)
		);
		if (res.err !== null) {
			console.error(res.err);
			$form.autoLogin = !value;
			notifications.send({ message: 'Failed to update setting', priority: 'ERROR' });
			return;
		}
		notifications.send({ message: 'Setting updated', priority: 'INFO' });
		await invalidate(key('conversation'));
	}
</script>

<PageHeader title="Access" description="Visibility, invites and participation." />

<div class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6">
	<p class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">Other configuration</p>
	<div class="flex flex-1 flex-col gap-6">
		<Form.Field form={accessForm} name="isPublic">
			<Form.Control>
				{#snippet children({ props })}
					<div class="flex items-center justify-between gap-4">
						<div class="flex flex-col gap-1">
							<FieldLabel
								label="Show conversation publicly"
								info="When this conversation is launched, anyone (even without an account) can open its documents and data. Off means only you, collaborators, and participants can."
							/>
							<Form.Description class="text-muted-foreground text-sm">
								Let anyone view this conversation's data once it's launched.
							</Form.Description>
						</div>
						<Switch
							{...props}
							bind:checked={$form.isPublic}
							onCheckedChange={(v) => saveConversationToggle('isPublic', v)}
						/>
					</div>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field form={accessForm} name="isInviteOnly">
			<Form.Control>
				{#snippet children({ props })}
					<div class="flex items-center justify-between gap-4">
						<div class="flex flex-col gap-1">
							<FieldLabel
								label="Only allow participation by invite"
								info="Only people you invite can take part. With this off, anyone with the link can participate."
							/>
							<Form.Description class="text-muted-foreground text-sm">
								Admins can invite and manage members.
							</Form.Description>
						</div>
						<Switch
							{...props}
							bind:checked={$form.isInviteOnly}
							onCheckedChange={(v) => saveConversationToggle('isInviteOnly', v)}
						/>
					</div>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field form={accessForm} name="autoLogin">
			<Form.Control>
				{#snippet children({ props })}
					<div class="flex items-center justify-between gap-4">
						<div class="flex flex-col gap-1">
							<FieldLabel
								label="Automatically log in with an anonymous account"
								info="Visitors who are not signed in get a temporary anonymous account automatically, so they can take part without registering. They can upgrade to a real account later."
							/>
							<Form.Description class="text-muted-foreground text-sm">
								Creates a temporary account for unauthenticated users.
							</Form.Description>
						</div>
						<Switch
							{...props}
							bind:checked={$form.autoLogin}
							onCheckedChange={(v) => saveAutoLogin(v)}
						/>
					</div>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field form={accessForm} name="enableSignupPrompts">
			<Form.Control>
				{#snippet children({ props })}
					<div class="flex items-center justify-between gap-4">
						<div class="flex flex-col gap-1">
							<FieldLabel
								label="Enable signup prompts"
								info="Shows prompts encouraging participants to create an account on the thank-you page after they finish."
							/>
							<Form.Description class="text-muted-foreground text-sm">
								Toggle whether to display signup prompts on thank you page.
							</Form.Description>
						</div>
						<Switch
							{...props}
							bind:checked={$form.enableSignupPrompts}
							onCheckedChange={(v) =>
								saveConversationToggle('enableSignupPrompts', v)}
						/>
					</div>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field form={accessForm} name="showThankYouPageAnnonInstructions">
			<Form.Control>
				{#snippet children({ props })}
					<div class="flex items-center justify-between gap-4">
						<div class="flex flex-col gap-1">
							<FieldLabel
								label="Show thank you page anonymous instructions"
								info="On the thank-you page, shows anonymous participants their temporary ID and how to log back in later to see the results."
							/>
							<Form.Description class="text-muted-foreground text-sm">
								Display instructions for anonymous users on the thank you page.
							</Form.Description>
						</div>
						<Switch
							{...props}
							bind:checked={$form.showThankYouPageAnnonInstructions}
							onCheckedChange={(v) =>
								saveConversationToggle('showThankYouPageAnnonInstructions', v)}
						/>
					</div>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field form={accessForm} name="showThankyouPageFeedbackButton">
			<Form.Control>
				{#snippet children({ props })}
					<div class="flex items-center justify-between gap-4">
						<div class="flex flex-col gap-1">
							<FieldLabel
								label="Show thank you page feedback button"
								info="On the thank-you page, shows participants a button to give feedback on the process."
							/>
							<Form.Description class="text-muted-foreground text-sm">
								Display the feedback button on the thank you page.
							</Form.Description>
						</div>
						<Switch
							{...props}
							bind:checked={$form.showThankyouPageFeedbackButton}
							onCheckedChange={(v) =>
								saveConversationToggle('showThankyouPageFeedbackButton', v)}
						/>
					</div>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>

		<Form.Field form={accessForm} name="allowRevisitAfterFinishing">
			<Form.Control>
				{#snippet children({ props })}
					<div class="flex items-center justify-between gap-4">
						<div class="flex flex-col gap-1">
							<FieldLabel
								label="Allow revisit after finishing"
								info="Once a participant has finished every step they reach the thank-you page. Turn this off to seal them out: no step is reachable afterwards, the revisit links disappear, and the server rejects any further contributions."
							/>
							<Form.Description class="text-muted-foreground text-sm">
								Let participants return to the steps after they have finished.
							</Form.Description>
						</div>
						<Switch
							{...props}
							bind:checked={$form.allowRevisitAfterFinishing}
							onCheckedChange={(v) =>
								saveConversationToggle('allowRevisitAfterFinishing', v)}
						/>
					</div>
				{/snippet}
			</Form.Control>
			<Form.FieldErrors />
		</Form.Field>
	</div>
</div>

<div class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6">
	<div class="lg:w-50 lg:shrink-0 lg:pt-2">
		<h3 class="text-base font-semibold">Co-hosting organizations</h3>
		<InfoHover
			info="Additional organizations that should have read access to this conversation. Search by organization name to add one, and remove it here later."
		/>
	</div>
	<div class="flex-1">
		<CohostManager
			conversationId={conversation.id}
			primaryHostOrganizationId={conversation.organizationId ?? null}
			{cohostOrganizations}
			canManage={canManageCohosts}
		/>
	</div>
</div>
