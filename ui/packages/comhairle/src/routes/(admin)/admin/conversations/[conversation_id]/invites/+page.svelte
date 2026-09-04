<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { page } from '$app/state';
	import Button from '$lib/components/ui/button/button.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';

	import EmailInviteForm from '$lib/components/ui/email-invites/EmailInviteForm.svelte';
	import InviteLabelDialog from '$lib/components/InviteLabelDialog.svelte';

	import { formatDistanceToNow } from 'date-fns';
	import QrCode from 'svelte-qrcode';
	import type { InviteDto } from '@crownshy/api-client/api';

	import * as Table from '$lib/components/ui/table/index.js';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import OpenInviteStatsBarChart from '$lib/components/OpenInviteStatsBarChart.svelte';
	import EmailInvitesList from '$lib/components/ui/email-invites/EmailInvitesList.svelte';
	import { inviteUrl, embedInviteUrl } from '$lib/utils/invites.js';
	import { key } from '$lib/utils/invalidationKey.js';

	let labelDialogOpen = $state(false);
	let selectedInvite = $state<InviteDto | null>(null);

	let url = $derived(page.url);
	let { data } = $props();
	let invites = $derived(data.invites);

	let { conversation } = data;

	// The sub-tab strip (Row 3) is server-rendered by the conversation layout from INVITE_SUBTABS;
	// this page just reads `?subtab=` to pick which section to show.
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
		await invalidate(key('conversation/invites'));
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
		invalidate(key('conversation/invites'));
	}
</script>

<svelte:head>
	<title>Manage Invites - Comhairle Admin</title>
</svelte:head>

{#snippet InviteLink(invite: InviteDto, label: string)}
	<div class="flex flex-row gap-x-2">
		<CopyButton copyText={inviteUrl(url, invite.id, conversation.id)}>{label}</CopyButton>
	</div>
{/snippet}

<PageHeader title="Recruit" />

{#if activeTab === 'email'}
	<EmailInviteForm conversationId={conversation.id} onDone={emailInvitesSubmitted} />
	<EmailInvitesList {emailInvites} inviteLink={InviteLink} />
{:else if activeTab === 'open-links'}
	<div class="space-y-4">
		<div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
			<p class="text-muted-foreground max-w-prose">
				Create invite links to share on social media or send directly to people.
			</p>
			<Button onclick={createInviteLink}>New Invite Link</Button>
		</div>

		{#if openInvites.length === 0}
			<div class="text-muted-foreground rounded-lg border border-dashed p-8 text-center">
				No invite links yet. Create one to start recruiting participants.
			</div>
		{:else}
			<div class="overflow-x-auto">
				<Table.Root>
					<Table.Header>
						<Table.Row>
							<Table.Head class="min-w-[160px]">Label</Table.Head>
							<Table.Head class="w-[72px]">Link</Table.Head>
							<Table.Head class="w-[130px]">Created</Table.Head>
							<Table.Head class="w-[130px]">Expires</Table.Head>
							<Table.Head class="min-w-[260px]">Stats</Table.Head>
							<Table.Head class="w-[90px] text-center">Accepted</Table.Head>
							<Table.Head class="w-[150px] text-center">QR code</Table.Head>
							<Table.Head class="w-[150px] text-center">Embed QR code</Table.Head>
						</Table.Row>
					</Table.Header>
					<Table.Body>
						{#each openInvites as invite (invite.id)}
							<Table.Row>
								<Table.Cell class="font-medium">
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

								<Table.Cell class="text-muted-foreground">
									{formatDistanceToNow(invite.createdAt, { addSuffix: true })}
								</Table.Cell>

								<Table.Cell class="text-muted-foreground">
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

								<Table.Cell class="text-center font-medium tabular-nums">
									{invite.acceptCount}
								</Table.Cell>

								<Table.Cell>
									<div class="flex justify-center">
										<QrCode
											value={inviteUrl(url, invite.id, conversation.id)}
											size="512"
											padding={null}
											errorCorrection="M"
											className="h-28 w-28"
										/>
									</div>
								</Table.Cell>
								<Table.Cell>
									<div class="flex justify-center">
										<QrCode
											value={embedInviteUrl(url, invite.id, conversation.id)}
											size="512"
											padding={null}
											errorCorrection="M"
											className="h-28 w-28"
										/>
									</div>
								</Table.Cell>
							</Table.Row>
						{/each}
					</Table.Body>
				</Table.Root>
			</div>
		{/if}
	</div>
{:else if activeTab === 'physical'}
	<h2>Generate physical QR Codes for an inperson event</h2>
{/if}

<InviteLabelDialog
	bind:open={labelDialogOpen}
	invite={selectedInvite}
	conversationId={conversation.id}
	onSave={handleLabelSaved}
/>
