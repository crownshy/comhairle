<script lang="ts">
	import { apiClient } from '$lib/api/client';

	let {
		polis_id,
		polis_url,
		workflow_step_id,
		admin_password,
		admin_user
	}: {
		polis_id: string;
		polis_url: string;
		workflow_step_id: string;
		admin_user: string;
		admin_password: string;
	} = $props();
	$inspect(workflow_step_id);
	let url = $derived(`${polis_url}/m/${polis_id}`);
</script>

<h2>Polis Setup</h2>

<p>Polis ID: {polis_id}</p>
<p>Polis url: {polis_url}</p>
<p>Admin email : {admin_user}</p>
<p>Adming password: {admin_password}</p>

<iframe class="min-h-[40vh]" src={url} title="Polis poll" style="width:100%;height:100%"></iframe>
