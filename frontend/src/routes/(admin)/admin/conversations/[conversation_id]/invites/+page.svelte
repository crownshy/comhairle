<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { page } from '$app/stores';
	import { apiClient } from '$lib/api/client';
	import BarChart from '$lib/components/BarChart.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Card from '$lib/components/ui/card';

	import EmailInviteForm from '$lib/components/ui/EmailInviteForm/EmailInviteForm.svelte';

	import { formatDistanceToNow } from 'date-fns';
	import QrCode from 'svelte-qrcode';
	import type { Invite } from '$lib/api/api.js';

	import * as Table from '$lib/components/ui/table/index.js';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import { Share2 } from 'lucide-svelte';
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

	let open_invites = $derived(invites.filter((invite) => invite.invite_type == 'open'));
	let email_invites = $derived(invites.filter((invite) => invite.invite_type.email));

	function inviteUrl(invite: Invite) {
		return `${url.origin}/conversations/${conversation.slug ?? conversation.id}/invite/${invite.id}`;
	}

	function emailInvitesSubmitted() {
		sendEmailDiaglogOpen = false;
		invalidateAll();
	}
</script>

{#snippet InviteLink(invite: Invite, label: string)}
	<div class="flex flex-row gap-x-2">
		<CopyButton copyText={inviteUrl(invite)}>Link</CopyButton>
	</div>
{/snippet}

<h1 class="mb-10 flex flex-row items-center gap-2 text-4xl"><Share2 /> Recruit</h1>

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
						{#each email_invites as invite}
							<Table.Row>
								<Table.Cell class="font-medium">{invite.invite_type.email}</Table.Cell>
								<Table.Cell>
									{@render InviteLink(invite, 'Link')}
								</Table.Cell>

								<Table.Cell>
									{formatDistanceToNow(invite.created_at, { addSuffix: true })}
								</Table.Cell>
								<Table.Cell>
									{invite.expires_at
										? formatDistanceToNow(invite.expires_at, { addSuffix: true })
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
				{#each open_invites as invite}
					<Table.Row>
						<Table.Cell>
							{@render InviteLink(invite, 'Link')}
						</Table.Cell>

						<Table.Cell>
							{formatDistanceToNow(invite.created_at, { addSuffix: true })}
						</Table.Cell>

						<Table.Cell>
							{invite.expires_at
								? formatDistanceToNow(invite.expires_at, { addSuffix: true })
								: 'Never'}
						</Table.Cell>
						<Table.Cell>
							<BarChart />
						</Table.Cell>

						<Table.Cell>
							{invite.accept_count}
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
