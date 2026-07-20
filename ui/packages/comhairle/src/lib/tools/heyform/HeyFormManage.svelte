<script lang="ts">
	import { fade } from 'svelte/transition';
	import HeyFormBuilderSkeleton from './HeyFormBuilderSkeleton.svelte';

	type Props = {
		survey_id: string;
		survey_url: string;
		admin_user: string;
		admin_password: string;
		workspace_id: string;
		project_id: string;
		conversation_id: string;
		workflow_id: string;
		workflow_step_id: string;
	};
	let { survey_id, survey_url, admin_user, admin_password, workspace_id, project_id }: Props =
		$props();

	let iframe = $state<HTMLIFrameElement>();
	let firstLoad = $state(true);
	// Kept hidden through the login + redirect hop so the operator doesn't see the bare HeyForm
	// login page flash before the builder lands on the create page.
	let ready = $state(false);

	// The HeyForm builder is a fixed desktop layout; below this width it overflows horizontally.
	// We lay the frame out at this logical width and scale the whole frame down to fit narrower
	// panels, so the width always fits (no horizontal scroll) and the builder keeps its desktop
	// proportions instead of squashing.
	const DESIGN_WIDTH = 1280;

	// `viewport` is the padding-free area the frame must fill; its clientWidth/clientHeight are the
	// true inner size, so the scale honours the available space exactly (mirrors the scale-to-fit
	// pattern in TemplateIllustration.svelte).
	let viewport = $state<HTMLDivElement | null>(null);
	let availableWidth = $state(DESIGN_WIDTH);
	let availableHeight = $state(0);

	// Downscale-only: at or above DESIGN_WIDTH the frame renders natively (scale 1); narrower than
	// that it shrinks to fit.
	let scale = $derived(Math.min(1, availableWidth / DESIGN_WIDTH));
	// Logical size the iframe document lays out at. Visual size = logical * scale, which equals the
	// available size, so the frame fills the region in both axes with no horizontal scroll.
	let logicalWidth = $derived(scale === 1 ? availableWidth : DESIGN_WIDTH);
	let logicalHeight = $derived(scale > 0 ? availableHeight / scale : availableHeight);

	$effect(() => {
		const el = viewport;
		if (!el) return;
		const measure = () => {
			availableWidth = el.clientWidth;
			availableHeight = el.clientHeight;
		};
		const observer = new ResizeObserver(measure);
		observer.observe(el);
		measure();
		return () => observer.disconnect();
	});

	const base_url = $derived.by(() =>
		survey_url.startsWith('https://') ? survey_url : `https://${survey_url}`
	);

	// `partialNav=true` tells our HeyForm fork to hide its own top navbar so the embedded
	// builder shows only the form editor (see FormNavbar in the heyform repo).
	const CREATE_PAGE = $derived(
		`${base_url}/workspace/${workspace_id}/project/${project_id}/form/${survey_id}/create?partialNav=true`
	);

	const HOME = $derived(`${base_url}/login`);

	function handleLoad() {
		if (!firstLoad) return;
		firstLoad = false;

		setTimeout(() => {
			iframe?.contentWindow?.postMessage(
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
			ready = true;
		}, 1000);
	}
</script>

<div bind:this={viewport} class="bg-muted relative h-full w-full overflow-hidden">
	{#if !ready}
		<!-- Covers the login + redirect hop (the iframe renders at opacity 0 behind this), so the
			operator sees the builder skeleton rather than a blank frame or the bare login page. -->
		<div class="absolute inset-0 z-10" out:fade={{ duration: 200 }}>
			<HeyFormBuilderSkeleton />
		</div>
	{/if}
	<iframe
		bind:this={iframe}
		onload={handleLoad}
		src={HOME}
		title="survey"
		allow="microphone; camera"
		class="border-none transition-opacity duration-300 {ready ? 'opacity-100' : 'opacity-0'}"
		style="width: {logicalWidth}px; height: {logicalHeight}px; transform: scale({scale}); transform-origin: top left;"
	></iframe>
</div>
