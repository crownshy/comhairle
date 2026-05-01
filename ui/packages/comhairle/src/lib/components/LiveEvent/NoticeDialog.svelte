<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import Button from '$lib/components/ui/button/button.svelte';

	interface Props {
		open: boolean;
		message: string;
		actionLabel?: string;
		onAction?: () => void;
		onDismiss: () => void;
	}

	let { open, message, actionLabel, onAction, onDismiss }: Props = $props();

	function handleAction() {
		onAction?.();
		onDismiss();
	}
</script>

<Dialog.Root {open} onOpenChange={(v) => !v && onDismiss()}>
	<Dialog.Content class="max-w-xl rounded-3xl p-9">
		<div class="flex flex-col items-center gap-7">
			<h2 class="text-foreground text-2xl leading-7 font-semibold">Notice</h2>

			<p class="text-foreground text-center text-lg leading-7 font-medium">
				{message}
			</p>

			<div class="flex w-full items-center justify-center gap-4">
				{#if actionLabel && onAction}
					<Button
						variant="primaryDark"
						class="h-12 px-5 text-base font-medium"
						onclick={handleAction}
					>
						{actionLabel}
					</Button>
				{:else}
					<Button
						variant="primaryDark"
						class="h-12 px-5 text-base font-medium"
						onclick={onDismiss}
					>
						OK
					</Button>
				{/if}
			</div>
		</div>
	</Dialog.Content>
</Dialog.Root>
