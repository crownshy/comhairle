<script lang="ts">
	let { polis_id }: { polis_id: string } = $props();
	let url = $derived(`https://poliscommunity.crown-shy.com/${polis_id}`);
</script>

<iframe src={url} title="Polis poll" style="width:100%;height:100%"></iframe>
