<script lang="ts">
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

	let url = $derived(`${polis_url}/m/${polis_id}/comments`);
	let iframe = $state();
	let firstLoad = $state(true);

	function tryLogin() {
		if (firstLoad) {
			iframe.contentWindow.postMessage(
				{ user: admin_user, password: admin_password, type: 'POLIS_LOGIN' },
				'https://polis.comhairle.scot'
			);
			firstLoad = false;
		}
	}
</script>

<h1 class="mb-10 text-2xl">Moderate Polis Conversation</h1>
<iframe
	src={url}
	bind:this={iframe}
	onload={tryLogin}
	title="Polis poll"
	style="width:100%;height:100%"
></iframe>
