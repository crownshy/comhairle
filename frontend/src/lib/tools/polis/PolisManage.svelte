<script lang="ts">
	let { polis_id, polis_url }: { polis_id: string; polis_url: string } = $props();
	let url = $derived(`${polis_url}/${polis_id}`);
</script>

<iframe src={url} title="Polis poll" style="width:100%;height:100%"></iframe>
