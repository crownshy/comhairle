<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import Button from '$lib/components/ui/button/button.svelte';

	interface Props {
		open: boolean;
		timeLeftFormatted: string;
		onClose: () => void;
		onAddTime: (minutes: number) => void;
	}

	let { open = $bindable(), timeLeftFormatted, onClose, onAddTime }: Props = $props();

	function handleAdd(minutes: number) {
		onAddTime(minutes);
		open = false;
	}
</script>

<Dialog.Root bind:open onOpenChange={(v) => !v && onClose()}>
	<Dialog.Content class="max-w-xl rounded-3xl p-9">
		<div class="flex flex-col items-center gap-7">
			<h2 class="text-foreground text-2xl leading-7 font-semibold">Add time</h2>

			<span class="text-muted-foreground text-xs leading-6 font-medium">
				Time left {timeLeftFormatted}
			</span>

			<div class="flex w-full items-center justify-center gap-4">
				<Button
					variant="primaryDark"
					class="h-12 px-5 text-base font-medium"
					onclick={() => handleAdd(1)}
				>
					+1 minute
				</Button>
				<Button
					variant="primaryDark"
					class="h-12 px-5 text-base font-medium"
					onclick={() => handleAdd(2)}
				>
					+2 minutes
				</Button>
				<Button
					variant="primaryDark"
					class="h-12 px-5 text-base font-medium"
					onclick={() => handleAdd(5)}
				>
					+5 minutes
				</Button>
			</div>
		</div>
	</Dialog.Content>
</Dialog.Root>
