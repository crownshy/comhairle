<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import Button from '$lib/components/ui/button/button.svelte';
	import { X } from 'lucide-svelte';

	interface Props {
		open: boolean;
		participantName: string;
		roomName: string;
		roomIndex: number;
		onClose: () => void;
		onEnterRoom: (roomIndex: number) => void;
	}

	let {
		open = $bindable(),
		participantName,
		roomName,
		roomIndex,
		onClose,
		onEnterRoom
	}: Props = $props();

	function handleEnter() {
		onEnterRoom(roomIndex);
		open = false;
	}
</script>

<Dialog.Root bind:open onOpenChange={(v) => !v && onClose()}>
	<Dialog.Content class="max-w-xl rounded-3xl p-9">
		<div class="flex flex-col items-center gap-7">
			<h2 class="text-foreground text-2xl leading-7 font-semibold">Notice</h2>

			<p class="text-foreground text-center text-lg leading-7 font-medium">
				{participantName} from {roomName} requested help.
			</p>

			<div class="flex w-full items-center justify-center">
				<Button
					variant="primaryDark"
					class="h-12 px-5 text-base font-medium"
					onclick={handleEnter}
				>
					Enter {roomName}
				</Button>
			</div>
		</div>
	</Dialog.Content>
</Dialog.Root>
