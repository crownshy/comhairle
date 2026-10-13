<script lang="ts">
	import {
		Dialog,
		DialogTrigger,
		DialogContent,
		DialogHeader,
		DialogTitle,
		DialogFooter
	} from '$lib/components/ui/dialog';
	import { useLoading } from '$lib/hooks/use-loading.svelte';
	import { Alert, AlertTitle, AlertDescription } from '$lib/components/ui/alert';
	import { Button } from '$lib/components/ui/button';
	import { apiClient } from '@crownshy/api-client/client';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';
	import LoadingButton from './ui/button/loading-button.svelte';
	import type { ConversationDto } from '@crownshy/api-client/api';
	import { notifications } from '$lib/notifications.svelte';
	import { LucideCircleX } from 'lucide-svelte';
	import { permissions } from '$lib/permissions.svelte';

	type Props = {
		conversation: ConversationDto;
		open?: boolean;
		hideTrigger?: boolean;
	};

	let { conversation, open = $bindable(false), hideTrigger = false }: Props = $props();
	const loader = useLoading();
	const canLaunch = $derived(
		permissions.can('conversation', 'conversation_launch', conversation.id)
	);

	async function toggleComplete() {
		if (!canLaunch) return;
		await loader.run(async () => {
			const isComplete = !conversation.isComplete;
			try {
				await apiClient.UpdateConversation(
					{ is_complete: isComplete },
					{ params: { conversation_id: conversation.id } }
				);
				open = false;
				await invalidate(key('admin/conversation'));

				notifications.send({
					message: isComplete ? 'Conversation now ended' : 'Conversation re-opened',
					priority: 'INFO'
				});
			} catch (e) {
				console.error(e);
				open = false;
				notifications.send({
					message: `Failed to ${isComplete ? 'end' : 'reopen'} conversation`,
					priority: 'ERROR'
				});
			}
		});
	}

	function cancel() {
		open = false;
	}
</script>

{#if conversation.isComplete}
	{#if !hideTrigger}
		<Button variant="outline" onclick={toggleComplete} disabled={!canLaunch}
			>Re-open Conversation</Button
		>
	{/if}

	<Dialog bind:open>
		<DialogContent>
			<DialogHeader>
				<DialogTitle>Re-open this conversation?</DialogTitle>
			</DialogHeader>

			<Alert>
				<AlertTitle>Re-open</AlertTitle>
				<AlertDescription>
					This will re-open the conversation so participants can take part again.
				</AlertDescription>
			</Alert>

			<DialogFooter>
				<LoadingButton
					variant="default"
					onclick={toggleComplete}
					loading={loader.loading}
					disabled={!canLaunch}
				>
					Re-open
				</LoadingButton>
				<Button onclick={cancel} variant="outline">cancel</Button>
			</DialogFooter>
		</DialogContent>
	</Dialog>
{:else}
	<Dialog bind:open>
		{#if !hideTrigger}
			<DialogTrigger disabled={!canLaunch}>
				<Button
					variant="outline"
					disabled={!canLaunch}
					class="text-destructive border-destructive hover:bg-destructive/10 hover:text-destructive!"
					><LucideCircleX /> End Conversation</Button
				>
			</DialogTrigger>
		{/if}

		<DialogContent>
			<DialogHeader>
				<DialogTitle>Are you sure you want to end the conversation?</DialogTitle>
			</DialogHeader>

			<Alert variant="destructive">
				<AlertTitle>Warning</AlertTitle>
				<AlertDescription>
					This will end the conversation. Doing so, the conversation will not be live for
					participants. Participants will no longer be able to take part in the
					conversation.
					<br />
					<br />
					This action is reversable.
				</AlertDescription>
			</Alert>

			<DialogFooter>
				<LoadingButton
					variant="default"
					onclick={toggleComplete}
					loading={loader.loading}
					disabled={!canLaunch}
				>
					End
				</LoadingButton>
				<Button onclick={cancel} variant="outline">cancel</Button>
			</DialogFooter>
		</DialogContent>
	</Dialog>
{/if}
