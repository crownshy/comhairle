<script lang="ts">
	import PageHeader from '$lib/components/PageHeader.svelte';
	import * as Form from '$lib/components/ui/form';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import Combobox from '$lib/components/ui/combobox/combobox.svelte';
	import { Button } from '$lib/components/ui/button';
	import { LoaderCircle, Trash } from '@lucide/svelte';
	import { Switch } from '$lib/components/ui/switch';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { accessSchema } from './schema';
	import FieldLabel from '../FieldLabel.svelte';
	import InfoHover from '../InfoHover.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import type { LocalizedOrganizationDto } from '@crownshy/api-client/api';
	import { camelToSnakeCase } from '$lib/utils/casingUtils';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';
	import { onMount } from 'svelte';

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

	let adding = $state(false);
	let loadingOrganizations = $state(false);
	let removingOrganizationId = $state<string | null>(null);
	let selectedOrganization = $state<{ value: string; label: string } | undefined>(undefined);
	let organizations = $state<LocalizedOrganizationDto[]>([]);

	onMount(async () => {
		if (!canManageCohosts) return;

		loadingOrganizations = true;
		const response = await tryCatchAsync(() =>
			apiClient.ListOrganizations({
				queries: { limit: 500 }
			})
		);
		loadingOrganizations = false;

		if (response.err !== null) {
			console.error(response.err);
			notifications.send({
				message: 'Failed to load organizations',
				priority: 'ERROR'
			});
			return;
		}

		organizations = response.ok.records;
	});

	const cohostOrganizationIds = $derived(
		new Set(cohostOrganizations.map((organization) => organization.id))
	);
	const availableOptions = $derived(
		organizations
			.filter((organization) => organization.id !== conversation.organizationId)
			.filter((organization) => !cohostOrganizationIds.has(organization.id))
			.map((organization) => ({ value: organization.id, label: organization.name }))
			.sort((left, right) => left.label.localeCompare(right.label))
	);

	async function addCohost() {
		if (!selectedOrganization) return;

		adding = true;
		const response = await tryCatchAsync(() =>
			apiClient.AddConversationCoHostOrganization(
				{ organization_id: selectedOrganization.value },
				{ params: { conversation_id: conversation.id } }
			)
		);
		adding = false;

		if (response.err !== null) {
			console.error(response.err);
			notifications.send({
				message: 'Failed to add co-host organization',
				priority: 'ERROR'
			});
			return;
		}

		selectedOrganization = undefined;
		notifications.send({
			message: 'Co-host organization added',
			priority: 'INFO'
		});
		await invalidate('conversation:meta');
	}

	async function removeCohost(organizationId: string) {
		removingOrganizationId = organizationId;
		const response = await tryCatchAsync(() =>
			apiClient.RemoveConversationCoHostOrganization(undefined, {
				params: { conversation_id: conversation.id, cohost_id: organizationId }
			})
		);
		removingOrganizationId = null;

		if (response.err !== null) {
			console.error(response.err);
			notifications.send({
				message: 'Failed to remove co-host organization',
				priority: 'ERROR'
			});
			return;
		}

		notifications.send({
			message: 'Co-host organization removed',
			priority: 'INFO'
		});
		await invalidate('conversation:meta');
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
		<section class="flex flex-col gap-4">
			<div class="flex flex-col gap-1">
				<p class="text-muted-foreground text-sm">
					Grant read access to additional organizations on this conversation.
				</p>
			</div>

			{#if canManageCohosts}
				<div class="flex flex-col gap-3 rounded-lg border p-4 md:flex-row md:items-center">
					<div class="min-w-0 flex-1">
						<Combobox
							items={availableOptions}
							selectedItem={selectedOrganization}
							placeholder="Search organizations by name"
							emptyMessage={loadingOrganizations
								? 'Loading organizations...'
								: 'No organizations available'}
							onSelect={(item) => {
								selectedOrganization = item;
							}}
						/>
					</div>
					<Button
						type="button"
						disabled={!selectedOrganization || adding || loadingOrganizations}
						onclick={addCohost}
					>
						{#if adding}
							<LoaderCircle class="mr-2 size-4 animate-spin" />
						{/if}
						Add co-host
					</Button>
				</div>
			{/if}

			<Card.Root>
				<Card.Content class="pt-6">
					{#if cohostOrganizations.length === 0}
						<p class="text-muted-foreground text-sm">
							No co-hosting organizations added yet.
						</p>
					{:else}
						<Table.Root>
							<Table.Header>
								<Table.Row>
									<Table.Head>Organization</Table.Head>
									<Table.Head>Role</Table.Head>
									{#if canManageCohosts}
										<Table.Head class="w-24">Actions</Table.Head>
									{/if}
								</Table.Row>
							</Table.Header>
							<Table.Body>
								{#each cohostOrganizations as organization (organization.id)}
									<Table.Row>
										<Table.Cell>{organization.name}</Table.Cell>
										<Table.Cell>{organization.roleName}</Table.Cell>
										{#if canManageCohosts}
											<Table.Cell>
												<Button
													type="button"
													variant="ghost"
													size="icon"
													disabled={removingOrganizationId ===
														organization.id}
													onclick={() => removeCohost(organization.id)}
												>
													{#if removingOrganizationId === organization.id}
														<LoaderCircle class="size-4 animate-spin" />
													{:else}
														<Trash class="size-4" />
													{/if}
													<span class="sr-only">Remove co-host</span>
												</Button>
											</Table.Cell>
										{/if}
									</Table.Row>
								{/each}
							</Table.Body>
						</Table.Root>
					{/if}
				</Card.Content>
			</Card.Root>
		</section>
	</div>
</div>
