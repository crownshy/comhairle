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
	import LoadingButton from './ui/button/loading-button.svelte';
	import { key } from '$lib/utils/invalidationKey';

	type Props = {
		conversation_id: string;
		hideTrigger?: boolean;
		open?: boolean;
	};

	let { conversation_id, hideTrigger = false, open = $bindable(false) }: Props = $props();
	const loader = useLoading();

	async function launch() {
		await loader.run(async () => {
			try {
				await apiClient.LaunchConversation(undefined, { params: { conversation_id } });
				open = false;
				await invalidate(key('conversation'));
			} catch (e) {
				console.error(e);
				open = false;
			}
		});
	}

	function cancel() {
		open = false;
	}
</script>

<Dialog bind:open>
	{#if !hideTrigger}
		<DialogTrigger>
			<Button variant="default" class="h-[40px]">Launch Conversation</Button>
		</DialogTrigger>
	{/if}

	<DialogContent>
		<DialogHeader>
			<DialogTitle>Are you sure you want to lanch the conversation</DialogTitle>
		</DialogHeader>

		<Alert variant="destructive">
			<AlertTitle>Warning</AlertTitle>
			<AlertDescription>
				This will make the conversation live for participants and you will no longer be able
				to modify the conversation.
			</AlertDescription>
		</Alert>

		<DialogFooter>
			<LoadingButton variant="default" onclick={launch} loading={loader.loading}>
				Launch
			</LoadingButton>
			<Button onclick={cancel} variant="outline">cancel</Button>
		</DialogFooter>
	</DialogContent>
</Dialog>
