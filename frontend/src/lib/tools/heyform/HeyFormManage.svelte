<script lang="ts">
	import Label from '$lib/components/ui/label/label.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import { apiClient } from '$lib/api/client';
	import { notifications } from '$lib/notifications.svelte';
	let {
		survey_id,
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

	const CREATE_PAGE = `https://forms.comhairle.scot/workspace/${workspace_id}/project/${project_id}/form/${survey_id}/create`;

	const HOME = 'https://forms.comhairle.scot/login';

	let url = $state(HOME);

	function handleLoad(e) {
		setTimeout(() => {
			iframe.contentWindow.postMessage(
				{
					type: 'HEYFORM_LOGIN',
					user: admin_user,
					password: admin_password,
					redirect: CREATE_PAGE
				},
				'https://forms.comhairle.scot'
			);
		}, 100);

		setTimeout(() => {
			iframe.style.display = 'block';
		}, 1000);
	}
</script>

<iframe
	bind:this={iframe}
	onload={handleLoad}
	src={url}
	title="survey"
	allow="microphone; camera"
	class="h-full w-full border-none"
	style="display:none;"
></iframe>
