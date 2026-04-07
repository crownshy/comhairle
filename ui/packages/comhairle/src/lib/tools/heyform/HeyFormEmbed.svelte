<script lang="ts">
	let {
		onDone,
		surveyId,
		userId,
		surveyURL,
		serverURL,
		extraSurveyParams
	}: {
		onDone: () => void;
		surveyId: string;
		surveyURL: string;
		serverURL: string;
		userId: string;
		extraSurveyParams?: Record<string, string>;
	} = $props();

	function onFrameMessage(e: any) {
		if (e.data.eventName === 'HIDE_EMBED_MODAL') {
			setTimeout(() => {
				onDone();
			}, 2000);
		}
	}

	$effect(() => {
		window.addEventListener('message', onFrameMessage);

		return () => {
			window.removeEventListener('message', onFrameMessage);
		};
	});

	const base_url = $derived.by(() =>
		serverURL.startsWith('https://') ? serverURL : `https://${serverURL}`
	);

	let url = $derived(
		`${base_url}/form/${surveyId}?&amp;id=${surveyId}&amp;type=modal&amp;customUrl=https%3A%2F%2Fforms.crown-shy.com%2Fform%2F&amp;widthType=%25&amp;width=100&amp;heightType=px&amp;height=500&amp;autoResizeHeight=true&polis_id=${userId}&comhairle_user_id=${userId}&hideAfterSubmit=true&autoClose=1`
	);

	let fullUrl = $derived.by(() => {
		if (extraSurveyParams) {
			let params = new URLSearchParams(extraSurveyParams).toString();
			return url + '&' + params;
		}
		return url;
	});
</script>

<iframe
	src={fullUrl}
	title="survey"
	allow="microphone; camera"
	class="h-full min-h-[900px] w-full border-none"
></iframe>
