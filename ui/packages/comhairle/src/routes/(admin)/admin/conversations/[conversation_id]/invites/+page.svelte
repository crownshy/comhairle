<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { page } from '$app/stores';
	import { apiClient } from '@crown-shy/api-client/client';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Card from '$lib/components/ui/card';

	import EmailInviteForm from '$lib/components/ui/EmailInviteForm/EmailInviteForm.svelte';

	import { formatDistanceToNow } from 'date-fns';
	import QrCode from 'svelte-qrcode';
	import type { InviteDto } from '@crown-shy/api-client/api';

	import * as Table from '$lib/components/ui/table/index.js';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import OpenInviteStatsBarChart from '$lib/components/OpenInviteStatsBarChart.svelte';
	import { BreadcrumbItem } from '$lib/components/ui/breadcrumb';
	import { useAdminLayoutSlots } from '../useAdminLayoutSlots.svelte.js';

	let sendEmailDiaglogOpen = $state(false);

	let url = $page.url;
	let { data } = $props();
	let invites = $derived(data.invites);

	let { conversation } = data;

	async function createInviteLink() {
		await apiClient.CreateInvite(
			{ invite_type: 'open' },
			{ params: { conversation_id: data.conversation.id } }
		);
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

	function inviteUrl(invite: InviteDto) {
		return `${url.origin}/conversations/${conversation.slug ?? conversation.id}/invite/${invite.id}`;
	}

	function emailInvitesSubmitted() {
		sendEmailDiaglogOpen = false;
		invalidateAll();
	}
	useAdminLayoutSlots({
		title: titleContentSnippet,
		breadcrumbs: breadcrumbSnippet
	});
</script>

{#snippet InviteLink(invite: InviteDto, label: string)}
	<div class="flex flex-row gap-x-2">
		<CopyButton copyText={inviteUrl(invite)}>{label}</CopyButton>
	</div>
{/snippet}

{#snippet breadcrumbSnippet()}
	<BreadcrumbItem>Recruit</BreadcrumbItem>
{/snippet}

{#snippet titleContentSnippet()}
	<h1 class="text-4xl font-bold">Recruit</h1>
{/snippet}

<Tabs.Root value="Email">
	<Tabs.List>
		<Tabs.Trigger value="Email">Email</Tabs.Trigger>
		<Tabs.Trigger value="OpenLinks">Open Links</Tabs.Trigger>
		<Tabs.Trigger value="Physical">Physical</Tabs.Trigger>
	</Tabs.List>

	<Tabs.Content value="Email">
		<EmailInviteForm conversation_id={conversation.id} onDone={emailInvitesSubmitted} />
		<Card.Root>
			<Card.Header>
				<h1 class="text-xl font-bold">Email Invite List</h1>
			</Card.Header>
			<Card.Content>
				<Table.Root>
					<Table.Header>
						<Table.Row>
							<Table.Head class="w-[100px]">Sent to</Table.Head>
							<Table.Head class="w-[100px]">Link</Table.Head>
							<Table.Head class="w-[100px]">At</Table.Head>
							<Table.Head class="w-[100px]">Expires</Table.Head>
							<Table.Head class="w-[100px]">Status</Table.Head>
						</Table.Row>
					</Table.Header>
					<Table.Body>
						{#each emailInvites as invite (invite.id)}
							<Table.Row>
								<Table.Cell class="font-medium">
									{typeof invite.inviteType !== 'string' &&
										'email' in invite.inviteType &&
										invite.inviteType.email}
								</Table.Cell>
								<Table.Cell>
									{@render InviteLink(invite, 'Link')}
								</Table.Cell>

								<Table.Cell>
									{formatDistanceToNow(invite.createdAt, { addSuffix: true })}
								</Table.Cell>
								<Table.Cell>
									{invite.expiresAt
										? formatDistanceToNow(invite.expiresAt, {
												addSuffix: true
											})
										: 'Never'}
								</Table.Cell>
								<Table.Cell>{invite.status}</Table.Cell>
							</Table.Row>
						{/each}
					</Table.Body>
				</Table.Root>
			</Card.Content>
		</Card.Root>
	</Tabs.Content>

	<Tabs.Content value="OpenLinks">
		<p>Create Invites for sharing on social media or sending as a links</p>
		<Button onclick={createInviteLink}>New Invite Link</Button>

		<Table.Root>
			<Table.Header>
				<Table.Row>
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
							<QrCode value={inviteUrl(invite)} />
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</Tabs.Content>

	<Tabs.Content value="Physical">
		<h2>Generate physical QR Codes for an inperson event</h2>
	</Tabs.Content>
</Tabs.Root>
