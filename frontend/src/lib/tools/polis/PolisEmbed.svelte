<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';

	type Props = {
		polis_id: string;
		polis_url: string;
		user_id: string;
		onDone: () => void;
	};

	$effect(() => {
		setTimeout(() => {
			showDialog = true;
		}, 120000);
	});
	let showDialog = $state(false);

	let { polis_id, polis_url, user_id, onDone }: Props = $props();
	let url = $derived(`${polis_url}/${polis_id}?xid=${user_id}`);
</script>

<iframe
	src={url}
	title="Polis poll"
	class="min-h-[500px] border-none"
	style="width:100%;height:100%"
></iframe>

<Dialog.Root bind:open={showDialog}>
	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Thanks for taking part in this conversation</Dialog.Title>
			<Dialog.Description>
				This polis conversation is open ended. Feel free to continue adding statements and voting
				until you feel done or come back later to do more.
			</Dialog.Description>
		</Dialog.Header>
		<Dialog.Footer>
			<Button onclick={() => (showDialog = false)} type="submit">Ok</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
