<script lang="ts">
	import {
		Dialog,
		DialogTrigger,
		DialogContent,
		DialogHeader,
		DialogTitle,
		DialogFooter
	} from '$lib/components/ui/dialog';

	import { Alert, AlertTitle, AlertDescription } from '$lib/components/ui/alert';

	import { Button } from '$lib/components/ui/button';
	import { apiClient } from '$lib/api/client';
	import { invalidateAll } from '$app/navigation';

	type Props = {
		conversation_id: string;
	};

	let { conversation_id }: Props = $props();
	let open = $state(false);

	async function launch() {
		try {
			await apiClient.LaunchConversation({}, { params: { conversation_id } });
			open = false;
			invalidateAll();
		} catch (e) {
			console.error(e);
			open = false;
		}
	}

	function cancel() {
		open = false;
	}
</script>

<Dialog {open}>
	<DialogTrigger>
		<Button variant="default">Launch Conversation</Button>
	</DialogTrigger>

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
			<Button onclick={launch} variant="default">Launch</Button>
			<Button onclick={cancel} variant="outline">cancel</Button>
		</DialogFooter>
	</DialogContent>
</Dialog>
