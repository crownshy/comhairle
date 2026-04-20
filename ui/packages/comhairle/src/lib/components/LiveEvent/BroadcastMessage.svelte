<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Send } from 'lucide-svelte';

	interface Props {
		open: boolean;
		onOpenChange: (open: boolean) => void;
		onSend: (message: string) => void;
	}

	let { open, onOpenChange, onSend }: Props = $props();

	let message = $state('');
	let sending = $state(false);

	async function handleSend() {
		if (!message.trim() || sending) return;
		sending = true;
		onSend(message.trim());
		message = '';
		sending = false;
		onOpenChange(false);
	}
</script>

<Dialog.Root {open} {onOpenChange}>
	<Dialog.Content class="max-w-md">
		<Dialog.Header>
			<Dialog.Title>Broadcast Message</Dialog.Title>
			<Dialog.Description>
				Send a message to all breakout rooms simultaneously.
			</Dialog.Description>
		</Dialog.Header>

		<div class="space-y-3 py-2">
			<textarea
				bind:value={message}
				placeholder="Type your message..."
				rows={3}
				class="border-border bg-background focus:ring-primary w-full rounded-lg border px-3 py-2 text-sm focus:ring-1 focus:outline-none"
				onkeydown={(e) => {
					if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) handleSend();
				}}
			></textarea>
			<p class="text-muted-foreground text-xs">Press Cmd+Enter to send</p>
		</div>

		<Dialog.Footer>
			<Button variant="outline" onclick={() => onOpenChange(false)}>Cancel</Button>
			<Button disabled={!message.trim() || sending} onclick={handleSend}>
				<Send class="mr-1.5 h-3.5 w-3.5" />
				{sending ? 'Sending...' : 'Send to all rooms'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
