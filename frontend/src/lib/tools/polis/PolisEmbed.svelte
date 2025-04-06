<script lang="ts">
	import { Button } from '$lib/components/ui/button';

	type Props = {
		polis_id: string;
		polis_url: string;
		user_id: string;
		onDone: () => void;
	};
	let { polis_id, polis_url, user_id, onDone }: Props = $props();
	let url = $derived(`${polis_url}/${polis_id}?xid=${user_id}`);
</script>

<iframe src={url} title="Polis poll" style="width:100%;height:100%"></iframe>

<Button variant="default" onclick={onDone}>Done</Button>
