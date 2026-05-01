<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import Button from '$lib/components/ui/button/button.svelte';

	interface Props {
		open: boolean;
		onClose: () => void;
		onSend: (message: string) => void;
	}

	let { open = $bindable(), onClose, onSend }: Props = $props();

	let message = $state('');

	function handleSend() {
		if (!message.trim()) return;
		onSend(message.trim());
		message = '';
		open = false;
	}
</script>

<Dialog.Root bind:open onOpenChange={(v) => !v && onClose()}>
	<Dialog.Content class="max-w-xl rounded-3xl p-9">
		<div class="flex flex-col items-center gap-7">
			<h2 class="text-card-foreground text-2xl leading-7 font-semibold">Broadcast message</h2>

			<div class="flex w-full flex-col gap-2">
				<input
					type="text"
					bind:value={message}
					placeholder="Write message"
					onkeydown={(e) => e.key === 'Enter' && handleSend()}
					class="border-input bg-background text-foreground placeholder:text-muted-foreground h-10 w-full rounded-lg border px-3 text-sm shadow-sm"
				/>
			</div>

			<div class="flex w-full items-center justify-center">
				<Button
					variant="primaryDark"
					class="h-12 px-6 text-base font-medium"
					onclick={handleSend}
					disabled={!message.trim()}
				>
					Send
				</Button>
			</div>
		</div>
	</Dialog.Content>
</Dialog.Root>
