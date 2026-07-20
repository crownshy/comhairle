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

<!-- The form renderer is a white, self-themed UI inside a cross-origin iframe: we can't restyle its
	internals or match comhairle's light/dark per viewer. So we frame it as a centered white card
	(bg-white is deliberate, matching the form's own paper) rather than a full-bleed slab, so it
	reads as an intentional embedded form on any background, dark mode included. Bounding the height
	keeps a short form from showing a large empty area; longer forms scroll inside the card. The
	max-width and height are tunable. -->
<div class="mx-auto mt-1 w-full max-w-2xl overflow-hidden rounded-xl">
	<iframe
		src={fullUrl}
		title="survey"
		allow="microphone; camera"
		class="h-[60dvh] max-h-[700px] min-h-[440px] w-full border-none"
	></iframe>
</div>
