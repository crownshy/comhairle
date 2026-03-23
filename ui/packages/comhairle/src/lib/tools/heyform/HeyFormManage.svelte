<script lang="ts">
	let {
		survey_id,
		survey_url,
		admin_user,
		admin_password,
		workspace_id,
		project_id
	}: {
		survey_id: string;
		survey_url: string;
		admin_user: string;
		admin_password: string;
		workspace_id: string;
		project_id: string;

		conversation_id: string;
		workflow_id: string;
		workflow_step_id: string;
	} = $props();
	let iframe = $state();
	let firstLoad = $state(true);

	const base_url = $derived.by(() =>
		survey_url.startsWith('https://') ? survey_url : `https://${survey_url}`
	);

	const CREATE_PAGE = $derived(
		`${base_url}/workspace/${workspace_id}/project/${project_id}/form/${survey_id}/create`
	);

	const HOME = $derived(`${base_url}/login`);

	function handleLoad(e) {
		if (firstLoad) {
			setTimeout(() => {
				iframe.contentWindow.postMessage(
					{
						type: 'HEYFORM_LOGIN',
						user: admin_user,
						password: admin_password,
						redirect: CREATE_PAGE
					},
					base_url
				);
			}, 100);

			setTimeout(() => {
				iframe.style.display = 'block';
			}, 1000);

			firstLoad = false;
		}
	}
</script>

<iframe
	bind:this={iframe}
	onload={handleLoad}
	src={HOME}
	title="survey"
	allow="microphone; camera"
	class="h-full w-full border-none"
	style="display:none;"
></iframe>
