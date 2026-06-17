<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { page } from '$app/state';
	import Button from '$lib/components/ui/button/button.svelte';

	import EmailInviteForm from '$lib/components/ui/email-invites/EmailInviteForm.svelte';
	import InviteLabelDialog from '$lib/components/InviteLabelDialog.svelte';
	import SubTabStrip from '$lib/components/SubTabStrip.svelte';

	import { formatDistanceToNow } from 'date-fns';
	import QrCode from 'svelte-qrcode';
	import type { InviteDto } from '@crownshy/api-client/api';

	import * as Table from '$lib/components/ui/table/index.js';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import OpenInviteStatsBarChart from '$lib/components/OpenInviteStatsBarChart.svelte';
	import EmailInvitesList from '$lib/components/ui/email-invites/EmailInvitesList.svelte';
	import { inviteUrl } from '$lib/utils/invites.js';

	let labelDialogOpen = $state(false);
	let selectedInvite = $state<InviteDto | null>(null);

	let url = $derived(page.url);
	let { data } = $props();
	let invites = $derived(data.invites);

	let { conversation } = data;

	let activeTab = $derived(page.url.searchParams.get('subtab') ?? 'email');

	function createInviteLink() {
		selectedInvite = null;
		labelDialogOpen = true;
	}

	function editInviteLabel(invite: InviteDto) {
		selectedInvite = invite;
		labelDialogOpen = true;
	}

	async function handleLabelSaved() {
		await invalidateAll();
	}

	let openInvites = $derived(invites.filter((invite) => invite.inviteType == 'open'));
	let emailInvites = $derived(
		invites.filter(
			(invite) =>
				typeof invite.inviteType !== 'string' &&
				'email' in invite.inviteType &&
				invite.inviteType.email
		)
	);

	function emailInvitesSubmitted() {
		invalidateAll();
	}
</script>

<svelte:head>
	<title>Manage Invites - Comhairle Admin</title>
</svelte:head>

{#snippet InviteLink(invite: InviteDto, label: string)}
	<div class="flex flex-row gap-x-2">
		<CopyButton copyText={inviteUrl(url, invite, conversation)}>{label}</CopyButton>
	</div>
{/snippet}

<div class="-mx-4 -mt-8 mb-8 sm:-mx-8 sm:-mt-10 lg:-mx-16">
	<SubTabStrip
		items={[
			{ label: 'Email', value: 'email' },
			{ label: 'Open Links', value: 'open-links' },
			{ label: 'Physical', value: 'physical' }
		]}
		defaultValue="email"
	/>
</div>

<h1 class="mb-4 text-3xl font-bold">Recruit</h1>

{#if activeTab === 'email'}
	<EmailInviteForm conversationId={conversation.id} onDone={emailInvitesSubmitted} />
	<EmailInvitesList {emailInvites} inviteLink={InviteLink} />
{:else if activeTab === 'open-links'}
	<p>Create Invites for sharing on social media or sending as a links</p>
	<Button onclick={createInviteLink}>New Invite Link</Button>

	<Table.Root>
		<Table.Header>
			<Table.Row>
				<Table.Head class="w-[150px]">Label</Table.Head>
				<Table.Head class="w-[100px]">Link</Table.Head>
				<Table.Head class="w-[100px]">Created At</Table.Head>
				<Table.Head class="w-[100px]">Expires</Table.Head>
				<Table.Head class="w-[100px]">Stats</Table.Head>
				<Table.Head class="w-[100px]">Accepted</Table.Head>
				<Table.Head class="w-[100px]">QRCode</Table.Head>
			</Table.Row>
		</Table.Header>
		<Table.Body>
			{#each openInvites as invite (invite.id)}
				<Table.Row>
					<Table.Cell>
						<Button
							variant="ghost"
							size="sm"
							onclick={() => editInviteLabel(invite)}
							class="h-auto p-1 font-normal"
						>
							{invite.label || '(click to add label)'}
						</Button>
					</Table.Cell>

					<Table.Cell>
						{@render InviteLink(invite, 'Link')}
					</Table.Cell>

					<Table.Cell>
						{formatDistanceToNow(invite.createdAt, { addSuffix: true })}
					</Table.Cell>

					<Table.Cell>
						{invite.expiresAt
							? formatDistanceToNow(invite.expiresAt, { addSuffix: true })
							: 'Never'}
					</Table.Cell>
					<Table.Cell>
						<OpenInviteStatsBarChart
							conversation_id={conversation.id}
							invite_id={invite.id}
						/>
					</Table.Cell>

					<Table.Cell>
						{invite.acceptCount}
					</Table.Cell>

					<Table.Cell>
						<QrCode value={inviteUrl(url, invite, conversation)} />
					</Table.Cell>
				</Table.Row>
			{/each}
		</Table.Body>
	</Table.Root>
{:else if activeTab === 'physical'}
	<h2>Generate physical QR Codes for an inperson event</h2>
{/if}

<InviteLabelDialog
	bind:open={labelDialogOpen}
	invite={selectedInvite}
	conversationId={conversation.id}
	onSave={handleLabelSaved}
/>
