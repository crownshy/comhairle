<script lang="ts" module>
	let isOpen = $state<boolean>(false);

	export function openDeleteDialog() {
		isOpen = true;
	}
</script>

<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Dialog from '$lib/components/ui/dialog';

	export interface Props {
		count: number;
		onconfirm: () => void;
	}

	const { count, onconfirm }: Props = $props();
</script>

<Dialog.Root open={isOpen}>
	<Dialog.Content class="rounded-xl p-0">
		<Dialog.Header class="shrink-0 border-b p-6 pb-4">
			<Dialog.Title class="text-2xl">Delete {count} items?</Dialog.Title>
			<Dialog.Description>This is permanent</Dialog.Description>
			<Button onclick={onconfirm}>Yes</Button>
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>
