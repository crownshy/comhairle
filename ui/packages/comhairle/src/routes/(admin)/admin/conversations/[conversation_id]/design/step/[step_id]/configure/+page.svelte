<script lang="ts">
	import CommonStepConfig from '$lib/components/CommonStepConfig/CommonStepConfig.svelte';
	import ParticipantViewSplit from '$lib/components/admin/ParticipantViewSplit.svelte';
	import ParticipantScreen from '$lib/components/admin/ParticipantScreen.svelte';
	import StepShell from '$lib/components/participant/StepShell.svelte';
	import StepCover from '$lib/components/participant/StepCover.svelte';
	import StepBriefBar from '$lib/components/participant/StepBriefBar.svelte';
	import type { StepItem } from '$lib/components/participant/stepItems';
	import { splitSlides } from '$lib/step-brief/splitSlides';
	import { toMetaToolConfig } from '$lib/step-brief/slideMeta';
	import { segmentFill } from '$lib/step-brief/segmentFill';
	import * as m from '$lib/paraglide/messages';

	let { data } = $props();

	let conversation = $derived(data.conversation);
	let step = $derived(data.step);

	/**
	 * The description as it is being typed. Null until the editor reports its first value,
	 * so the view opens on the saved text rather than blank for a frame.
	 */
	let typedDescription = $state<string | null>(null);
	let description = $derived(typedDescription ?? step?.description ?? '');

	// A step with no description still gets one slide: its cover carries the title and the
	// derived meta line (ADR-0017).
	let slides = $derived(splitSlides(description));
	let briefSlides = $derived(slides.length > 0 ? slides : ['']);
	let metaToolConfig = $derived(toMetaToolConfig(data.toolConfig));

	// One screen per slide. The cover takes the whole set plus a position, so what the loop
	// needs is the index rather than the slide itself.
	let slideIndexes = $derived(briefSlides.map((_, index) => index));

	let sortedSteps = $derived(
		[...(data.workflowSteps ?? [])].sort((a, b) => a.stepOrder - b.stepOrder)
	);
	let viewedIndex = $derived(sortedSteps.findIndex((s) => s.id === step?.id));

	/**
	 * The chrome's step list. No hrefs: the view is inert, and the only destinations these
	 * would carry are participant routes an admin is not standing in.
	 */
	let chromeSteps = $derived<StepItem[]>([
		{
			id: 'landing',
			name: m.landing_before_you_start(),
			status: 'completed',
			isIntro: true
		},
		...sortedSteps.map((s, index): StepItem => {
			let status: StepItem['status'] = 'upcoming';
			if (index === viewedIndex) status = 'current';
			else if (index < viewedIndex) status = 'completed';
			return { id: s.id, name: s.name, status };
		})
	]);

	let stepLabel = $derived(
		`${m.step_position_label({ current: viewedIndex + 1, total: sortedSteps.length })}: ${step?.name ?? ''}`
	);
</script>

{#if step}
	<ParticipantViewSplit
		description="The step brief, one screen per slide. Slide breaks appear as you type."
	>
		{#snippet editor()}
			<div class="max-w-3xl">
				<CommonStepConfig
					conversation_id={conversation.id}
					{conversation}
					{step}
					inline
					availableDocuments={data.availableDocuments}
					onDraftDescriptionChange={(value) => (typedDescription = value)}
				/>
			</div>
		{/snippet}

		{#snippet screens({ device, scale })}
			{#each slideIndexes as index (index)}
				<ParticipantScreen {device} {scale}>
					<StepShell
						class="h-full"
						chrome={{
							steps: chromeSteps,
							currentIndex: viewedIndex + 1,
							label: stepLabel,
							fill: segmentFill({
								phase: 'cover',
								slideIndex: index,
								slideCount: briefSlides.length
							}),
							showSupport: false
						}}
					>
						{#snippet content()}
							<StepCover
								slides={briefSlides}
								{index}
								title={step.name}
								toolConfig={metaToolConfig}
								availableDocuments={data.availableDocuments}
								conversationId={conversation.id}
							/>
						{/snippet}

						{#snippet bar()}
							<StepBriefBar
								label={index === briefSlides.length - 1
									? m.step_brief_start()
									: m.pager_next()}
								onForward={() => {}}
							/>
						{/snippet}
					</StepShell>
				</ParticipantScreen>
			{/each}
		{/snippet}
	</ParticipantViewSplit>
{/if}
