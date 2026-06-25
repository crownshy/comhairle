<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';

	let isOpen = $state<boolean>(false);

	export function open() {
		isOpen = true;
	}
</script>

<Dialog.Root open={isOpen}>
	<Dialog.Content class="rounded-xl p-0">
		<Dialog.Header class="shrink-0 border-b p-6 pb-4">
			<Dialog.Title class="text-2xl">Confirmation</Dialog.Title>
			<Dialog.Description>Are you sure?</Dialog.Description>
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>
