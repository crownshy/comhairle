<script lang="ts">
	import type { Snippet } from 'svelte';
	import * as Table from '$lib/components/ui/table/index.js';
	import * as Card from '$lib/components/ui/card';
	import type { InviteDto } from '@crownshy/api-client/api';
	import { formatDistanceToNow } from 'date-fns';

	type Props = {
		emailInvites: InviteDto[];
		inviteLink: Snippet<[InviteDto, string]>;
	};

	let { emailInvites, inviteLink }: Props = $props();
</script>

<Card.Root>
	<Card.Header>
		<h1 class="text-xl font-bold">Email Invite List</h1>
	</Card.Header>
	<Card.Content>
		<Table.Root>
			<Table.Header>
				<Table.Row>
					<Table.Head class="w-25">Sent to</Table.Head>
					<Table.Head class="w-25">Link</Table.Head>
					<Table.Head class="w-25">At</Table.Head>
					<Table.Head class="w-25">Expires</Table.Head>
					<Table.Head class="w-25">Status</Table.Head>
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
							{@render inviteLink(invite, 'Link')}
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
