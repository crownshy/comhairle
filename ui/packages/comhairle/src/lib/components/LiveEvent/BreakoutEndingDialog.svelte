<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import Button from '$lib/components/ui/button/button.svelte';
	import { X } from 'lucide-svelte';

	interface Props {
		open: boolean;
		secondsLeft: number;
		onGoBack: () => void;
	}

	let { open = $bindable(), secondsLeft, onGoBack }: Props = $props();
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="max-w-md rounded-3xl p-9 sm:max-w-md" showCloseButton={false}>
		<button
			class="text-muted-foreground hover:text-foreground absolute top-6 right-6"
			onclick={onGoBack}
		>
			<X class="h-5 w-5" />
		</button>

		<div class="flex flex-col items-center gap-7">
			<h2 class="text-foreground text-2xl leading-7 font-semibold">Notice</h2>

			<p class="text-foreground text-center text-lg leading-7 font-medium">
				This breakout room has ended.<br />
				Go back to the plenary room.
			</p>

			<p class="text-foreground text-center text-lg leading-7 font-medium">
				in {secondsLeft} second{secondsLeft !== 1 ? 's' : ''}..
			</p>

			<div class="flex w-full justify-center">
				<Button
					variant="primaryDark"
					class="h-12 px-6 text-base font-medium"
					onclick={onGoBack}
				>
					Go back
				</Button>
			</div>
		</div>
	</Dialog.Content>
</Dialog.Root>
