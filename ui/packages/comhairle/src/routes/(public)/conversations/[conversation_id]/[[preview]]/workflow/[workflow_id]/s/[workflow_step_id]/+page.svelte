<script lang="ts">
	/* Every destination here is built by the string helpers in $lib/urls, not from a typed
	   route id, so resolve() has nothing to resolve. */
	/* eslint-disable svelte/no-navigation-without-resolve */
	import * as Polis from '$lib/tools/polis/index.js';
	import * as HeyForm from '$lib/tools/heyform/index.js';
	import * as Learn from '$lib/tools/learn/index.js';
	import * as LivedExperience from '$lib/tools/lived_experince/index.js';
	import * as ThinkingSpace from '$lib/tools/thinking_space/index.js';
	import * as ElicitationBot from '$lib/tools/elicitation_bot/index.js';
	import * as Prioritization from '$lib/tools/prioritization/index.js';
	import type { PageProps } from './$types';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import StepChrome from '$lib/components/participant/StepChrome.svelte';
	import StepPager from '$lib/components/participant/StepPager.svelte';
	import StepCover from '$lib/components/participant/StepCover.svelte';
	import StepBriefDialog from '$lib/components/participant/StepBriefDialog.svelte';
	import type { StepItem } from '$lib/components/participant/stepItems';
	import { splitSlides } from '$lib/step-brief/splitSlides';
	import { toMetaToolConfig } from '$lib/step-brief/slideMeta';
	import { onStepPreview } from '$lib/step-brief/livePreview';
	import { segmentFill } from '$lib/step-brief/segmentFill';
	import type { ToolSequence } from '$lib/step-brief/toolSequence';
	import * as m from '$lib/paraglide/messages';

	import { goto } from '$app/navigation';
	import { thank_you_page, next_workflow_step_url, workflow_step_url } from '$lib/urls';
	import { page, navigating } from '$app/state';
	import LearnArticleSkeleton from '$lib/tools/learn/LearnArticleSkeleton.svelte';
	import { delayedFlag } from '$lib/utils/delayedFlag.svelte';
	import LearningAssistantSkeleton from '$lib/components/LearningAssistant/LearningAssistantSkeleton.svelte';

	const url = $derived(page.url);
	const queryString = $derived(url.search);

	let { data }: PageProps = $props();
	let { user, preview: isPreview } = data;
	let workflow_id = $derived(data.workflow_id);
	let workflowStep = $derived(data.workflowStep);
	let conversation = $derived(data.conversation);
	let workflowSteps = $derived(data.workflowSteps);

	let toolConfig = $derived(
		conversation.isLive ? workflowStep.toolConfig : workflowStep.previewToolConfig
	);

	let pageTitle = $derived(workflowStep?.name ?? 'Workflow Step');

	let sortedSteps = $derived([...workflowSteps].sort((a, b) => a.stepOrder - b.stepOrder));

	let actualCurrentStep = $derived(
		conversation.isLive
			? (sortedSteps.find((ws) => ws.progressStatus !== 'done') ?? null)
			: workflowStep
	);

	let isRevisiting = $derived(workflowStep.progressStatus === 'done');

	// Documents (and the derived "does the knowledge base have parsed docs" gate) are hoisted to
	// the workflow +layout.ts so the step page and the support sidebar share one source.
	let availableDocuments = $derived(data.availableDocuments);
	let hasKnowledgeBaseDocs = $derived(data.hasKnowledgeBaseDocs);

	let stepItems = $derived<StepItem[]>(
		sortedSteps.map((ws) => {
			const isCurrent = actualCurrentStep ? ws.id === actualCurrentStep.id : false;
			const isCompleted = ws.progressStatus === 'done';
			const actualCurrentOrder = actualCurrentStep?.stepOrder ?? Infinity;
			const isBefore = ws.stepOrder < actualCurrentOrder;
			const canRevisit = ws.canRevisit;

			const passedThrough = isCompleted || isBefore;

			let status: StepItem['status'];
			if (isCurrent) {
				status = 'current';
			} else if (passedThrough && canRevisit) {
				status = 'completed';
			} else if (passedThrough) {
				status = 'completed-locked';
			} else {
				status = 'upcoming';
			}

			const href =
				status === 'completed'
					? workflow_step_url(conversation.id, workflow_id, ws.id, isPreview) +
						queryString
					: undefined;

			return { id: ws.id, name: ws.name, status, href };
		})
	);

	let viewedIndex = $derived(sortedSteps.findIndex((ws) => ws.id === workflowStep.id));
	let currentStepNumber = $derived(viewedIndex + 1);
	let stepLabel = $derived(
		`${m.step_position_label({ current: currentStepNumber, total: sortedSteps.length })}: ${workflowStep.name}`
	);

	/** The footer no longer renders on workflow routes, so its links live in the dropdown. */
	const legalLinks = [
		{ href: '/rights/privacy', label: m.privacy_policy() },
		{ href: '/rights/tos', label: m.terms_of_service() },
		{ href: '/rights/cookies', label: m.cookies_settings() }
	];

	/**
	 * Tool type of the step being navigated *to*, or undefined when we aren't navigating to a step.
	 * Mid-navigation `data` still describes the step we're leaving, so the loading skeleton has to
	 * be picked from the destination: otherwise a hop into a survey shows an article skeleton and
	 * then flashes white while the form iframe boots.
	 */
	let navigatingToToolType = $derived.by(() => {
		const targetId = navigating.to?.params?.workflow_step_id;
		if (!targetId) return undefined;
		const target = sortedSteps.find((ws) => ws.id === targetId);
		if (!target) return undefined;
		return conversation.isLive ? target.toolConfig?.type : target.previewToolConfig?.type;
	});

	/**
	 * A step hop that resolves quickly never trips this, so the body skeleton stays hidden and
	 * the page just swaps content. Only a genuinely slow load shows it. See delayedFlag.
	 */
	let showNavigationSkeleton = delayedFlag(() => navigating.to !== null, 150);

	let prevStepHref = $derived.by(() => {
		if (viewedIndex <= 0) return undefined;
		const prevItem = stepItems[viewedIndex - 1];
		if (!prevItem || prevItem.status !== 'completed') return undefined;
		return prevItem.href;
	});

	/**
	 * An unsaved description pushed in by the admin's preview panel. Preview only: on a live
	 * conversation the participant always sees the saved description.
	 */
	let previewDescription = $derived.by<string | null>(() => {
		void workflowStep.id;
		return null;
	});

	$effect(() => {
		if (!isPreview) return;
		return onStepPreview(workflowStep.id, (draft) => {
			previewDescription = draft;
		});
	});

	// The step brief: the description as slides (ADR-0017). A step with no description still
	// gets one slide, so its cover carries the title and the derived meta line.
	let slides = $derived(splitSlides(previewDescription ?? workflowStep.description));
	let briefSlides = $derived(slides.length > 0 ? slides : ['']);
	let metaToolConfig = $derived(toMetaToolConfig(toolConfig));

	// Writable $derived rather than $effect: these reset when the step changes, and are also
	// assigned to directly by the pager. See AGENTS.md on mirroring state.
	let phase = $derived.by<'cover' | 'body'>(() => {
		void workflowStep.id;
		return 'cover';
	});
	let slideIndex = $derived.by(() => {
		void workflowStep.id;
		return 0;
	});
	let briefOpen = $derived.by(() => {
		void workflowStep.id;
		return false;
	});
	/** What the mounted tool reports about itself (ADR-0018). Empty until it says otherwise. */
	let sequence = $derived.by<ToolSequence>(() => {
		void workflowStep.id;
		return {};
	});

	let canProceed = $state(false);
	let isSubmitting = $state(false);

	$effect(() => {
		void workflowStep.id;
		isSubmitting = false;
	});

	$effect(() => {
		const type = toolConfig.type;
		if (type === Learn.TOOL_NAME || type === LivedExperience.TOOL_NAME) {
			canProceed = true;
		} else {
			canProceed = false;
		}
	});

	function handleSequenceChange(next: ToolSequence) {
		sequence = next;
	}

	function handleCanContinueChange(value: boolean) {
		canProceed = value;
	}

	let fill = $derived(
		segmentFill({
			phase,
			slideIndex,
			slideCount: briefSlides.length,
			toolProgress: sequence.progress
		})
	);

	let isLastSlide = $derived(slideIndex >= briefSlides.length - 1);

	let canGoBack = $derived(
		phase === 'body' ? true : slideIndex > 0 || prevStepHref !== undefined
	);

	let canGoForward = $derived.by(() => {
		if (phase === 'cover') return true;
		if (sequence.next) return true;
		// An optional step is always leavable, even when its tool says it is not complete.
		return canProceed || !workflowStep.required;
	});

	let forwardMode = $derived.by<'next' | 'skip' | 'start'>(() => {
		if (phase === 'cover') return isLastSlide ? 'start' : 'next';
		if (sequence.next || canProceed) return 'next';
		return workflowStep.required ? 'next' : 'skip';
	});

	function goBack() {
		if (phase === 'cover') {
			if (slideIndex > 0) {
				slideIndex -= 1;
			} else if (prevStepHref) {
				goto(prevStepHref);
			}
			return;
		}
		// Innermost first: back out of the tool's own sequence before leaving the body.
		if (sequence.prev) {
			sequence.prev();
			return;
		}
		phase = 'cover';
		slideIndex = briefSlides.length - 1;
	}

	function goForward() {
		if (phase === 'cover') {
			if (isLastSlide) {
				phase = 'body';
			} else {
				slideIndex += 1;
			}
			return;
		}
		if (sequence.next) {
			sequence.next();
			return;
		}
		stepComplete();
	}

	function goToThankYouPage() {
		goto(thank_you_page(conversation.id, workflow_id, !conversation.isLive) + queryString);
	}

	async function stepComplete() {
		if (isSubmitting) return;
		isSubmitting = true;

		if (isRevisiting) {
			const isPreview = !conversation.isLive;
			const currentIdx = sortedSteps.findIndex((ws) => ws.id === workflowStep.id);
			const nextRevisitable = sortedSteps.slice(currentIdx + 1).find((ws) => ws.canRevisit);
			const target = nextRevisitable ?? actualCurrentStep;
			if (target) {
				goto(
					workflow_step_url(conversation.id, workflow_id, target.id, isPreview) +
						queryString
				);
			} else {
				goToThankYouPage();
			}
			return;
		}

		try {
			if (conversation.isLive) {
				await apiClient.SetUserProgress(
					{ status: 'done' },
					{
						params: {
							workflow_id: workflowStep.workflowId,
							conversation_id: conversation.id,
							workflow_step_id: workflowStep.id
						},
						headers: { 'Content-Type': 'application/json' }
					}
				);

				/** Deliberately no invalidate() in place. Marking the step done changes
				 * what this page's own load does: a completed, non-revisitable step
				 * redirects to /next. Invalidating here re-runs that load, the redirect
				 * rejects the invalidate, and the catch below fires a spurious error
				 * toast while the redirect navigates anyway. Navigating with
				 * invalidateAll refreshes the step list and the participation seal at
				 * the destination instead. */
				await goto(
					next_workflow_step_url(conversation.id, workflowStep.workflowId) + queryString,
					{ invalidateAll: true }
				);
			} else {
				let next = workflowSteps.find((w) => w.stepOrder === workflowStep.stepOrder + 1);
				if (next) {
					let next_step_url = workflow_step_url(
						conversation.id,
						workflow_id,
						next.id,
						!conversation.isLive
					);
					goto(next_step_url + queryString);
				} else {
					goToThankYouPage();
				}
			}
		} catch (e) {
			if (e instanceof Error) {
				console.warn(e.message);
			}
			notifications.send({
				message: 'Something unexpected happened. Try again shortly',
				priority: 'ERROR'
			});
			isSubmitting = false;
		}
	}
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle</title>
</svelte:head>

{#if conversation && workflowStep && user}
	<div class="flex min-h-[100dvh] flex-col">
		<StepChrome
			steps={stepItems}
			currentIndex={viewedIndex}
			label={stepLabel}
			{fill}
			{legalLinks}
			count={sequence.count}
		/>

		<main class="flex w-full grow flex-col">
			{#if phase === 'cover'}
				<StepCover
					slides={briefSlides}
					index={slideIndex}
					title={workflowStep.name}
					toolConfig={metaToolConfig}
					{availableDocuments}
					conversationId={conversation.id}
				/>
			{:else}
				<div class="mx-auto w-full max-w-5xl grow px-4 pb-6 md:px-6">
					{#if showNavigationSkeleton.current}
						{#if navigatingToToolType === HeyForm.TOOL_NAME}
							<HeyForm.UserUISkeleton />
						{:else}
							<LearnArticleSkeleton />
							{#if conversation?.chatBotId && conversation.enableQaChatBot && hasKnowledgeBaseDocs}
								<div class="mx-auto mt-6 w-full max-w-[65ch]">
									<LearningAssistantSkeleton />
								</div>
							{/if}
						{/if}
					{:else if toolConfig.type === Learn.TOOL_NAME}
						{#key workflowStep.id}
							<Learn.UserUI
								pages={toolConfig.pages}
								onSequenceChange={handleSequenceChange}
								{conversation}
								{availableDocuments}
								{hasKnowledgeBaseDocs}
							/>
						{/key}
					{:else if toolConfig?.type === Polis.TOOL_NAME}
						{#key workflowStep.id}
							<Polis.UserUI
								user_id={user.id}
								polis_id={toolConfig.poll_id}
								polis_url={toolConfig.server_url}
								requiredVotes={toolConfig.required_votes}
								workflowStepId={workflowStep.id}
								{isPreview}
								onDone={stepComplete}
								onCanContinueChange={handleCanContinueChange}
								onSequenceChange={handleSequenceChange}
								showRemainingStatementCount={toolConfig.show_remaining_statements}
							/>
						{/key}
					{:else if toolConfig.type === HeyForm.TOOL_NAME}
						{#key workflowStep.id}
							<HeyForm.UserUI
								userId={user.id}
								surveyId={toolConfig.survey_id}
								surveyURL={toolConfig.survey_url}
								serverURL={toolConfig.server_url}
								onDone={stepComplete}
							/>
						{/key}
					{:else if toolConfig.type === LivedExperience.TOOL_NAME}
						{#key workflowStep.id}
							<LivedExperience.UserUI
								onDone={stepComplete}
								onSequenceChange={handleSequenceChange}
							/>
						{/key}
					{:else if toolConfig.type === ThinkingSpace.TOOL_NAME}
						{#key workflowStep.id}
							<ThinkingSpace.UserUI
								workflowStepId={workflowStep.id}
								workflowId={workflowStep.workflowId}
								conversationId={conversation.id}
								userId={user.id}
								topic={toolConfig.topic}
								rootQuestions={toolConfig.root_questions}
								followUpRoundsCount={toolConfig.follow_up_rounds_count}
								requestUserSharePermission={workflowStep.requestUserSharePermission}
								initialPermissionToShareWithOrganizers={data.permissionToShareWithOrganizers}
								progressStatus={workflowStep.progressStatus}
								onDone={stepComplete}
								onCanContinueChange={handleCanContinueChange}
								onSequenceChange={handleSequenceChange}
							/>
						{/key}
					{:else if toolConfig.type === ElicitationBot.TOOL_NAME}
						{#key workflowStep.id}
							<ElicitationBot.UserUI
								conversationId={conversation.id}
								workflowId={workflowStep.workflowId}
								workflowStepId={workflowStep.id}
								userId={user.id}
								topic={toolConfig.topic}
								onDone={stepComplete}
								onCanContinueChange={handleCanContinueChange}
							/>
						{/key}
					{:else if toolConfig.type === Prioritization.TOOL_NAME}
						{#key workflowStep.id}
							<Prioritization.UserUI
								{workflowStep}
								conversation={{
									primaryLocale: conversation.primaryLocale,
									isLive: conversation.isLive,
									supportedLanguages: conversation.supportedLanguages
								}}
								participantId={user.id}
								onDone={stepComplete}
								onCanContinueChange={handleCanContinueChange}
								onSequenceChange={handleSequenceChange}
							/>
						{/key}
					{/if}
				</div>
			{/if}
		</main>

		<StepPager
			{forwardMode}
			{briefOpen}
			{canGoBack}
			{canGoForward}
			loading={isSubmitting}
			onBack={goBack}
			onForward={goForward}
			onBrief={() => (briefOpen = !briefOpen)}
		/>
	</div>

	{#if briefOpen}
		<StepBriefDialog
			slides={briefSlides}
			title={workflowStep.name}
			toolConfig={metaToolConfig}
			{availableDocuments}
			conversationId={conversation.id}
			onClose={() => (briefOpen = false)}
		/>
	{/if}
{:else}
	<h1>Failed to find conversation</h1>
{/if}
