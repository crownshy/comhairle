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
	import StepTour from '$lib/components/participant/StepTour.svelte';
	import StepComplete from '$lib/components/participant/StepComplete.svelte';
	import StepProceedBar from '$lib/components/participant/StepProceedBar.svelte';
	import StepBriefOverlay from '$lib/components/participant/StepBriefOverlay.svelte';
	import StepBriefBar from '$lib/components/participant/StepBriefBar.svelte';
	import type { StepItem } from '$lib/components/participant/stepItems';
	import { splitSlides } from '$lib/step-brief/splitSlides';
	import {
		isFirstRun,
		hasSeenStepTour,
		markStepTourSeen
	} from '$lib/components/participant/stepTour';
	import { touchFlowTiming } from '$lib/components/participant/flowTiming';
	import { toMetaToolConfig } from '$lib/step-brief/slideMeta';
	import { onStepPreview } from '$lib/step-brief/livePreview';
	import { segmentFill } from '$lib/step-brief/segmentFill';
	import type { ToolSequence } from '$lib/step-brief/toolSequence';
	import * as m from '$lib/paraglide/messages';

	import { goto } from '$app/navigation';
	import {
		thank_you_page,
		next_workflow_step_url,
		conversation_url,
		workflow_step_url
	} from '$lib/urls';
	import { page, navigating } from '$app/state';
	import LearnArticleSkeleton from '$lib/tools/learn/LearnArticleSkeleton.svelte';
	import { delayedFlag } from '$lib/utils/delayedFlag.svelte';
	import LearningAssistantSkeleton from '$lib/components/LearningAssistant/LearningAssistantSkeleton.svelte';
	import { learningAssistantAvailable } from '$lib/components/LearningAssistant/availability';

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
	let assistantAvailable = $derived(
		learningAssistantAvailable(conversation, hasKnowledgeBaseDocs)
	);

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

	/** Where the logo and the menu's first row go: this conversation's Before you start. */
	let introUrl = $derived(conversation_url(conversation.id, isPreview) + queryString);

	/**
	 * Before you start keeps its segment in the chrome once a participant has joined, so the
	 * bar does not lose a segment on the first step and the menu keeps a way back to it
	 * (ADR-0024). It is not one of the workflow's steps, so `isIntro` keeps it out of
	 * "Step N of M" and out of the index the pager walks.
	 */
	let chromeSteps = $derived<StepItem[]>([
		{
			id: 'landing',
			name: m.landing_before_you_start(),
			status: 'completed',
			href: introUrl,
			isIntro: true
		},
		...stepItems
	]);

	let viewedIndex = $derived(sortedSteps.findIndex((ws) => ws.id === workflowStep.id));
	let currentStepNumber = $derived(viewedIndex + 1);
	let stepLabel = $derived(
		`${m.step_position_label({ current: currentStepNumber, total: sortedSteps.length })}: ${workflowStep.name}`
	);

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
	let phase = $derived.by<'cover' | 'body' | 'done'>(() => {
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

	// The clock the thank-you page reports on. Stamped as each step opens, so the number it
	// gives is time this participant spent in the flow rather than the sum of the tools'
	// hardcoded estimates.
	$effect(() => {
		void workflowStep.id;
		touchFlowTiming(conversation.id);
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

	let coverForwardLabel = $derived(isLastSlide ? m.step_brief_start() : m.pager_next());

	let canGoBack = $derived(
		phase === 'body' ? true : slideIndex > 0 || prevStepHref !== undefined
	);

	let canGoForward = $derived.by(() => {
		if (sequence.next) return true;
		// An optional step is always leavable, even when its tool says it is not complete.
		return canProceed || !workflowStep.required;
	});

	let forwardMode = $derived.by<'next' | 'skip'>(() => {
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
		// Skipping is a decision to move on, not something finished, so it goes straight to the
		// next step. The completion screen is for a step the participant actually did.
		if (forwardMode === 'skip') {
			proceed();
			return;
		}
		stepComplete();
	}

	function toggleBrief() {
		briefOpen = !briefOpen;
	}

	/**
	 * The brief is only worth reopening once the tool is up. On the cover it is the screen the
	 * participant is looking at, and a step with no description has no brief at all.
	 */
	let canReopenBrief = $derived(phase === 'body' && briefSlides.length > 0);

	/**
	 * The one-time tour of the chrome. It waits for the body phase because that is the first
	 * screen where all four places it names exist: the cover has no pager and no brief chip.
	 */
	let tourOpen = $state(false);

	$effect(() => {
		if (phase !== 'body') return;
		if (!isFirstRun(workflowSteps)) return;
		if (hasSeenStepTour(conversation.id)) return;
		tourOpen = true;
	});

	function dismissTour() {
		markStepTourSeen(conversation.id);
		tourOpen = false;
	}

	function goToThankYouPage() {
		goto(thank_you_page(conversation.id, workflow_id, !conversation.isLive) + queryString);
	}

	/**
	 * The step's work is finished. Nothing is written yet: the participant sees the completion
	 * screen and the write happens when they proceed, so a tool that finishes on its own last
	 * action does not silently navigate out from under them.
	 */
	function stepComplete() {
		phase = 'done';
	}

	async function proceed() {
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
	<!-- The step is exactly one screen: chrome on top, pager on the bottom, and the tool body
	     takes whatever is left and scrolls inside it. The two chrome rows are laid out, not
	     stuck: nothing can push them off. The column is minmax(0,1fr), not the implicit
	     auto: an auto column floors at the widest row's min-content, so a long header (the
	     opinion count next to an untruncated step label) would widen the whole grid past the
	     viewport and shift every centred row right. -->
	<div
		class="grid h-[100dvh] grid-cols-[minmax(0,1fr)] grid-rows-[auto_1fr_auto] overflow-hidden"
	>
		<StepChrome
			steps={chromeSteps}
			currentIndex={viewedIndex + 1}
			label={stepLabel}
			{fill}
			count={phase === 'done' ? undefined : sequence.count}
			{assistantAvailable}
			{introUrl}
			{briefOpen}
			onBrief={canReopenBrief ? toggleBrief : undefined}
			preview={isPreview}
		/>

		<!-- The bottom of the scroll dissolves into the background instead of stopping at a
		     hard line, so a cut-off paragraph reads as content continuing under the bar. The
		     padding matches the fade, so the mask sits over empty space once the reader is at
		     the end and never dims the last line. -->
		<main
			data-step-scroll
			class="flex min-h-0 w-full flex-col overflow-y-auto mask-b-from-[calc(100%-2.5rem)] pb-10"
		>
			{#if phase === 'done'}
				<StepComplete />
			{:else if phase === 'cover'}
				<!-- No back or skip here: the cover explains the step, and one forward action is
				     the whole decision it asks for (ADR-0024). Both moves are still on the pager
				     once the step itself is open. -->
				<StepCover
					slides={briefSlides}
					index={slideIndex}
					title={workflowStep.name}
					toolConfig={metaToolConfig}
					{availableDocuments}
					conversationId={conversation.id}
				/>
			{:else}
				<div
					class="mx-auto flex min-h-full w-full max-w-5xl flex-col px-4 pb-[clamp(0.5rem,2vh,1.5rem)] md:px-6"
				>
					{#if showNavigationSkeleton.current}
						{#if navigatingToToolType === HeyForm.TOOL_NAME}
							<HeyForm.UserUISkeleton />
						{:else if navigatingToToolType === Polis.TOOL_NAME}
							<Polis.UserUISkeleton />
						{:else}
							<LearnArticleSkeleton />
							{#if assistantAvailable}
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

		{#if phase === 'done'}
			<StepProceedBar loading={isSubmitting} onProceed={proceed} />
		{:else if phase === 'cover'}
			<StepBriefBar label={coverForwardLabel} onForward={goForward} />
		{:else}
			<StepPager
				{forwardMode}
				{canGoBack}
				{canGoForward}
				loading={isSubmitting}
				onBack={goBack}
				onForward={goForward}
			/>
		{/if}
	</div>

	{#if tourOpen}
		<StepTour onDismiss={dismissTour} />
	{/if}

	{#if briefOpen}
		<StepBriefOverlay
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
