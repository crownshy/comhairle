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
	import { invalidateAll } from '$app/navigation';
	import LoadingButton from './ui/button/loading-button.svelte';
	import type { ConversationDto } from '@crownshy/api-client/api';
	import { notifications } from '$lib/notifications.svelte';
	import { LucideCircleX } from 'lucide-svelte';

	type Props = {
		conversation: ConversationDto;
	};

	let { conversation }: Props = $props();
	let open = $state(false);
	const loader = useLoading();

	async function toggleComplete() {
		await loader.run(async () => {
			const isComplete = !conversation.isComplete;
			try {
				await apiClient.UpdateConversation(
					{ is_complete: isComplete },
					{ params: { conversation_id: conversation.id } }
				);
				open = false;
				invalidateAll();

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
	<Button variant="outline" onclick={toggleComplete}>Re-open Conversation</Button>
{:else}
	<Dialog {open}>
		<DialogTrigger>
			<Button variant="outline" class="text-red-400"
				><LucideCircleX /> End Conversation</Button
			>
		</DialogTrigger>

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
				<LoadingButton variant="default" onclick={toggleComplete} loading={loader.loading}>
					End
				</LoadingButton>
				<Button onclick={cancel} variant="outline">cancel</Button>
			</DialogFooter>
		</DialogContent>
	</Dialog>
{/if}
