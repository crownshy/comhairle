<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import type { UserOrganizationAccess } from '@crownshy/api-client/api';
	import * as Select from '$lib/components/ui/select';
	import { Label } from '$lib/components/ui/label';
	import { notifications } from '$lib/notifications.svelte';
	import { apiErrorMessage } from '$lib/utils/apiErrorMessage';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { key } from '$lib/utils/invalidationKey';
	import {
		currentHostingOrganizationName,
		hostingOrganizationOptions
	} from './hostingOrganizations';

	type Props = {
		conversationId: string;
		organizationId: string | null;
		organizations: UserOrganizationAccess[];
	};
	let { conversationId, organizationId, organizations }: Props = $props();

	const NO_ORGANIZATION = '';

	let selectedId = $derived(organizationId ?? NO_ORGANIZATION);
	let saving = $state(false);
	let error = $state('');

	const options = $derived(hostingOrganizationOptions(organizations));
	const selectedName = $derived(
		currentHostingOrganizationName(organizations, selectedId || null) ??
			'No hosting organisation'
	);

	async function changeOrganization(nextId: string) {
		if (saving || nextId === (organizationId ?? NO_ORGANIZATION)) return;
		saving = true;
		error = '';
		const result = await tryCatchAsync(() =>
			apiClient.SetConversationHostingOrganization(
				{ organization_id: nextId === NO_ORGANIZATION ? null : nextId },
				{ params: { conversation_id: conversationId } }
			)
		);
		saving = false;
		if (result.err !== null) {
			selectedId = organizationId ?? NO_ORGANIZATION;
			error = apiErrorMessage(result.err, 'Could not change the hosting organisation.');
			return;
		}
		notifications.send({ message: 'Hosting organisation updated', priority: 'INFO' });
		await Promise.all([
			invalidate(key('admin/conversation')),
			invalidate(key('admin/conversation/meta')),
			invalidate('conversation:workflow')
		]);
	}
</script>

<section class="border-border flex flex-col gap-3 border-t py-6">
	<h3 class="text-base font-semibold">Hosting organisation</h3>
	<p class="text-muted-foreground text-base">
		Demographic questions and forms are chosen from the hosting organisation. You can only host
		a conversation under an organisation you are an admin of.
	</p>
	<div class="flex flex-col gap-2">
		<Label for="hosting-organization" class="text-base">Organisation</Label>
		<Select.Root
			type="single"
			bind:value={selectedId}
			onValueChange={changeOrganization}
			disabled={saving}
		>
			<Select.Trigger id="hosting-organization" class="w-72">
				{selectedName}
			</Select.Trigger>
			<Select.Content>
				<Select.Item value={NO_ORGANIZATION} label="No hosting organisation">
					No hosting organisation
				</Select.Item>
				{#each options as option (option.id)}
					<Select.Item value={option.id} label={option.name}>{option.name}</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
	</div>
	{#if options.length === 0}
		<p class="text-muted-foreground text-base">
			You are not an admin of any organisation, so you cannot set one here.
		</p>
	{/if}
	{#if error}
		<p class="text-destructive text-base" role="alert">{error}</p>
	{/if}
</section>
