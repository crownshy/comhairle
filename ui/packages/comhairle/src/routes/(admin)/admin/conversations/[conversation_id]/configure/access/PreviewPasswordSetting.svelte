<script lang="ts">
	import { page } from '$app/state';
	import { invalidate } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import type { PreviewPasswordStatus } from '@crownshy/api-client/api';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { key } from '$lib/utils/invalidationKey';
	import { conversation_url } from '$lib/urls';
	import FieldLabel from '../FieldLabel.svelte';

	const MIN_LENGTH = 8;

	type Props = {
		conversationId: string;
		conversationSlug: string;
		isLive: boolean;
		status: PreviewPasswordStatus | null;
		canEdit: boolean;
	};

	let { conversationId, conversationSlug, isLive, status, canEdit }: Props = $props();

	let password = $state('');
	let isSaving = $state(false);
	let isEditing = $state(false);

	const previewLink = $derived(page.url.origin + conversation_url(conversationSlug, true));
	const tooShort = $derived(password.length < MIN_LENGTH);
	const showForm = $derived(!status?.enabled || isEditing);

	async function save() {
		if (tooShort || isSaving) return;
		isSaving = true;
		const result = await tryCatchAsync(() =>
			apiClient.SetConversationPreviewPassword(
				{ password },
				{ params: { conversation_id: conversationId } }
			)
		);
		isSaving = false;
		if (result.err !== null) {
			notifications.send({
				message: 'Failed to save the preview password',
				priority: 'ERROR'
			});
			return;
		}
		password = '';
		isEditing = false;
		notifications.send({ message: 'Preview password saved', priority: 'INFO' });
		await invalidate(key('admin/conversation/preview-password'));
	}

	async function turnOff() {
		if (isSaving) return;
		isSaving = true;
		const result = await tryCatchAsync(() =>
			apiClient.ClearConversationPreviewPassword(undefined, {
				params: { conversation_id: conversationId }
			})
		);
		isSaving = false;
		if (result.err !== null) {
			notifications.send({
				message: 'Failed to turn off the preview password',
				priority: 'ERROR'
			});
			return;
		}
		notifications.send({ message: 'Preview password turned off', priority: 'INFO' });
		await invalidate(key('admin/conversation/preview-password'));
	}
</script>

<div class="flex flex-col gap-4">
	<FieldLabel
		type="plain"
		label="Preview password"
		info="Lets people without a Comhairle account open the preview link before launch. Everyone uses the same password. Changing it or turning it off locks out everyone who used the old one. Access also runs out after 7 days."
	/>

	{#if isLive}
		<p class="text-muted-foreground text-base">
			This conversation is live, so anyone with the link can already take part.
		</p>
	{:else}
		{#if status?.enabled && status.updatedAt}
			<p class="text-base">
				Password set on {new Date(status.updatedAt).toLocaleDateString()}.
			</p>
		{:else}
			<p class="text-muted-foreground text-base">
				No password set. Only your team can open the preview.
			</p>
		{/if}

		{#if canEdit}
			{#if showForm}
				<form
					class="flex flex-col gap-2 md:flex-row md:items-start"
					onsubmit={(e) => {
						e.preventDefault();
						save();
					}}
				>
					<div class="flex flex-1 flex-col gap-1">
						<Input
							type="text"
							autocomplete="off"
							placeholder="New preview password"
							bind:value={password}
						/>
						<p class="text-muted-foreground text-sm">
							At least {MIN_LENGTH} characters.
						</p>
					</div>
					<Button type="submit" disabled={tooShort || isSaving}>
						{status?.enabled ? 'Change password' : 'Set password'}
					</Button>
					{#if isEditing}
						<Button variant="outline" onclick={() => (isEditing = false)}>Cancel</Button
						>
					{/if}
				</form>
			{:else}
				<div class="flex flex-wrap gap-2">
					<Button variant="outline" onclick={() => (isEditing = true)}
						>Change password</Button
					>
					<Button variant="outline" onclick={turnOff} disabled={isSaving}>Turn off</Button
					>
				</div>
			{/if}
		{/if}

		{#if status?.enabled}
			<div class="flex flex-col gap-1">
				<p class="text-sm font-semibold">Link to share with the password</p>
				<div class="bg-muted rounded-md px-3 py-2 text-base">
					<CopyButton copyText={previewLink}>
						<span class="truncate">{previewLink}</span>
					</CopyButton>
				</div>
			</div>
		{/if}
	{/if}
</div>
