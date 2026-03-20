<script lang="ts">
	let {
		polis_id,
		polis_url,
		admin_password,
		admin_user
	}: {
		polis_id: string;
		polis_url: string;
		workflow_step_id: string;
		admin_user: string;
		admin_password: string;
	} = $props();

	let base_url = $derived(polis_url.startsWith('https://') ? polis_url : `https://${polis_url}`);
	let url = $derived(`${base_url}/m/${polis_id}`);
	let iframe = $state();
	let firstLoad = $state(true);

	function tryLogin() {
		if (firstLoad) {
			iframe.contentWindow.postMessage(
				{ user: admin_user, password: admin_password, type: 'POLIS_LOGIN' },
				base_url
			);
			firstLoad = false;
		}
	}
</script>

<h2 class="my-5 text-2xl font-bold">Polis Setup</h2>
<div class="grid grid-cols-[1fr_30vw]">
	<iframe
		bind:this={iframe}
		onload={tryLogin}
		src={url}
		title="Polis poll"
		style="width:100%;height:100%"
	></iframe>
	<div>
		<h2 class="text-xl">Guidance</h2>
		<p class="my-5">
			Polis seed statements are the initial set of statements provided to participants in a
			Polis conversation to help spark discussion and guide the direction of the debate. They
			serve as a starting framework, giving people something concrete to agree or disagree
			with before participants begin contributing their own ideas. Well-crafted seed
			statements cover a broad range of perspectives on the issue at hand, ensuring that
			participants from different backgrounds and viewpoints feel invited to engage. They
			don’t aim to be exhaustive but instead create an entry point that stimulates
			conversation and reveals patterns of agreement and disagreement across diverse groups.
		</p>
		<p>
			When writing good Polis seed statements, clarity and neutrality are key. Avoid jargon,
			complex phrasing, or emotionally charged language that might alienate participants. The
			statements should be concise, specific, and testable—something participants can
			reasonably say “agree” or “disagree” to, rather than vague sentiments. It’s also helpful
			to include a variety of framings: some that reflect common arguments, some that test
			assumptions, and some that raise trade-offs or edge cases. Finally, balance matters; if
			all the seed statements lean heavily toward one viewpoint, the conversation will feel
			biased from the start. A thoughtful mix ensures participants see themselves reflected
			early on, which increases engagement and makes it easier for the group’s collective
			intelligence to emerge.
		</p>
	</div>
</div>
