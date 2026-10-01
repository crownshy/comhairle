<script lang="ts">
	import { fade } from 'svelte/transition';
	import HeyFormBuilderSkeleton from './HeyFormBuilderSkeleton.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import type { Form, FormField } from '@crownshy/api-client/api';
	import { permissions } from '$lib/permissions.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { z } from 'zod';

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
	let {
		survey_id,
		survey_url,
		admin_user,
		admin_password,
		workspace_id,
		project_id,
		conversation_id,
		workflow_step_id
	}: Props = $props();
	const canEdit = $derived(
		permissions.can('conversation', 'conversation_update', conversation_id)
	);
	let surveyForm = $state<Form | null>(null);
	let formLoading = $state(true);
	let formError = $state(false);
	const choicesSchema = z.array(z.object({ id: z.string(), label: z.string() }));

	function fieldContent(value: unknown): string {
		if (typeof value === 'string') return value;
		return value !== null && typeof value === 'object' ? JSON.stringify(value) : '';
	}

	function fieldChoices(field: FormField) {
		const result = choicesSchema.safeParse(field.properties?.choices);
		return result.success ? result.data : [];
	}

	async function loadForm(stepId: string) {
		formLoading = true;
		formError = false;
		const result = await tryCatchAsync(() =>
			apiClient.HeyFormGetForm({ params: { workflow_step_id: stepId } })
		);
		if (stepId !== workflow_step_id) return;
		formLoading = false;
		if (result.err !== null) {
			formError = true;
			return;
		}
		surveyForm = result.ok;
	}

	$effect(() => {
		if (!canEdit) void loadForm(workflow_step_id);
	});

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
		if (!canEdit || !firstLoad) return;
		firstLoad = false;

		setTimeout(() => {
			if (!canEdit) return;
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

{#if canEdit}
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
			class="border-none transition-opacity duration-300 {ready
				? 'opacity-100'
				: 'opacity-0'}"
			style="width: {logicalWidth}px; height: {logicalHeight}px; transform: scale({scale}); transform-origin: top left;"
		></iframe>
	</div>
{:else}
	<div class="flex flex-col gap-6">
		<h2 class="text-2xl font-bold">{surveyForm?.name || 'Survey'}</h2>
		{#if formLoading}
			<p class="text-muted-foreground text-base">Loading survey...</p>
		{:else if formError}
			<p class="text-destructive text-base" role="alert">Could not load the survey.</p>
		{:else}
			{#if surveyForm?.description}
				<div class="bg-card border-border rounded-lg border p-4">
					<ContentRenderer content={surveyForm.description} />
				</div>
			{/if}
			{#each surveyForm?.fields ?? [] as field, index (field.id)}
				<section class="bg-card border-border flex flex-col gap-3 rounded-lg border p-4">
					<h3 class="text-lg font-semibold">Question {index + 1}</h3>
					<ContentRenderer content={fieldContent(field.title)} />
					{#if field.description}
						<ContentRenderer content={fieldContent(field.description)} />
					{/if}
					<p class="text-muted-foreground text-base">{field.kind.replaceAll('_', ' ')}</p>
					{#if fieldChoices(field).length > 0}
						<ul class="list-inside list-disc text-base">
							{#each fieldChoices(field) as choice (choice.id)}
								<li>{choice.label}</li>
							{/each}
						</ul>
					{/if}
				</section>
			{:else}
				<p class="text-muted-foreground text-base">No questions yet.</p>
			{/each}
		{/if}
	</div>
{/if}
