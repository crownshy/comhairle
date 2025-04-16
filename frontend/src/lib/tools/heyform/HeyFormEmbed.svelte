<script lang="ts">
	let {
		onDone,
		surveyId,
		userId,
		surveyURL,
		extraSurveyParams
	}: {
		onDone: () => void;
		surveyId: string;
		surveyURL: string;
		userId: string;
		extraSurveyParams?: Record<string, string>;
	} = $props();

	function onFrameMessage(e: any) {
		if (e.data.eventName === 'HIDE_EMBED_MODAL') {
			onDone();
		}
	}

	$effect(() => {
		window.addEventListener('message', onFrameMessage);

		return () => {
			window.removeEventListener('message', onFrameMessage);
		};
	});

	let url = `https://forms.crown-shy.com/form/${surveyId}?&amp;id=${surveyId}&amp;type=modal&amp;customUrl=https%3A%2F%2Fforms.crown-shy.com%2Fform%2F&amp;widthType=%25&amp;width=100&amp;heightType=px&amp;height=500&amp;autoResizeHeight=true&polis_id=${userId}&hideAfterSubmit=true&autoClose=1`;
	if (extraSurveyParams) {
		let params = new URLSearchParams(extraSurveyParams).toString();
		url = url + '&' + params;
	}
	console.log('URL IS ', url);
</script>

<iframe src={url} title="survey" allow="microphone; camera" class="h-full w-full border-none"
></iframe>
