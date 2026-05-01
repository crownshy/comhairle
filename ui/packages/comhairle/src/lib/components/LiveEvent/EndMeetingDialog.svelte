<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import Button from '$lib/components/ui/button/button.svelte';

	interface Props {
		open: boolean;
		onConfirm: () => void;
		onCancel: () => void;
	}

	let { open = $bindable(), onConfirm, onCancel }: Props = $props();
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="max-w-md rounded-3xl p-9 sm:max-w-md" showCloseButton={false}>
		<div class="flex flex-col items-center gap-7">
			<h2 class="text-foreground text-2xl leading-7 font-semibold">End meeting?</h2>

			<p class="text-muted-foreground text-center text-base leading-6">
				This will end the meeting for all participants. Everyone will be disconnected from
				the call.
			</p>

			<div class="flex w-full flex-col gap-3">
				<Button
					variant="destructive"
					class="h-12 w-full text-base font-medium"
					onclick={onConfirm}
				>
					End meeting for everyone
				</Button>
				<Button
					variant="outline"
					class="h-12 w-full text-base font-medium"
					onclick={onCancel}
				>
					Cancel
				</Button>
			</div>
		</div>
	</Dialog.Content>
</Dialog.Root>
