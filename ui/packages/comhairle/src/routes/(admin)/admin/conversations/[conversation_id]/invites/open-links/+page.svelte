<script lang="ts">
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
	import * as Table from '$lib/components/ui/table';
	import { embedInviteUrl, inviteUrl } from '$lib/utils/invites';
	import QrCode from 'svelte-qrcode';
	import InviteLink from '../InviteLink.svelte';
	import InviteLabelDialog from './InviteLabelDialog.svelte';
	import InviteStatsBarChart from './InviteStatsBarChart.svelte';
	import { formatDistanceToNow } from 'date-fns';

	const { data } = $props();
	const { conversation, invites } = $derived(data);

	const openInvites = $derived(invites.filter((invite) => invite.inviteType === 'open'));
</script>

<div class="space-y-4">
	<div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
		<p class="text-muted-foreground max-w-prose">
			Create invite links to share on social media or send directly to people.
		</p>
		<InviteLabelDialog type="new" conversationId={conversation.id}>
			<Button>New Invite Link</Button>
		</InviteLabelDialog>
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
						<Table.Head class="min-w-40">Label</Table.Head>
						<Table.Head class="w-18">Link</Table.Head>
						<Table.Head class="w-32.5">Created</Table.Head>
						<Table.Head class="w-32.5">Expires</Table.Head>
						<Table.Head class="min-w-65">Stats</Table.Head>
						<Table.Head class="w-22.5 text-center">Accepted</Table.Head>
						<Table.Head class="w-37.5 text-center">QR code</Table.Head>
						<Table.Head class="w-37.5 text-center">Embed QR code</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each openInvites as invite (invite.id)}
						<Table.Row>
							<Table.Cell class="font-medium">
								<InviteLabelDialog
									type="update"
									inviteId={invite.id}
									conversationId={conversation.id}
								>
									<Button
										variant="ghost"
										size="sm"
										class="h-auto p-1 font-normal"
									>
										{invite.label || '(click to add label)'}
									</Button>
								</InviteLabelDialog>
							</Table.Cell>

							<Table.Cell>
								<InviteLink
									inviteId={invite.id}
									conversationId={conversation.id}
									label="Link"
								/>
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
								<InviteStatsBarChart
									conversation_id={conversation.id}
									invite_id={invite.id}
								/>
							</Table.Cell>

							<Table.Cell class="text-center font-medium tabular-nums">
								{invite.acceptCount}
							</Table.Cell>

							<Table.Cell>
								<QrCode
									value={inviteUrl(page.url, invite.id, conversation.id)}
									size="512"
									padding={null}
									errorCorrection="M"
									className="h-28 w-28 max-w-none"
								/>
							</Table.Cell>
							<Table.Cell>
								<QrCode
									value={embedInviteUrl(page.url, invite.id, conversation.id)}
									size="512"
									padding={null}
									errorCorrection="M"
									className="h-28 w-28 max-w-none"
								/>
							</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
		</div>
	{/if}
</div>
