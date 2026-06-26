<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { TriangleAlert } from 'lucide-svelte';

	interface Props {
		open: boolean;
		message: string;
	}

	const { open, message }: Props = $props();
</script>

<Dialog.Root {open}>
	<Dialog.Content class="w-[50vw] overflow-hidden rounded-xl  p-0">
		<Dialog.Header class="shrink-0 border-b p-6 pb-4">
			<Dialog.Title class="flex flex-row items-center gap-2 text-2xl">
				<TriangleAlert class="text-red-500" />Error
			</Dialog.Title>
		</Dialog.Header>
		<Dialog.Description class="p-6 pt-0">{message}</Dialog.Description>
	</Dialog.Content>
</Dialog.Root>
