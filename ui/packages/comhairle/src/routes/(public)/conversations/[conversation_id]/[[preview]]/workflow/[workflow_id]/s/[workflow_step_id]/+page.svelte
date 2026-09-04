<script lang="ts">
	/* Every destination here is built by the string helpers in $lib/urls, not from a typed
	   route id, so resolve() has nothing to resolve. */
	/* eslint-disable svelte/no-navigation-without-resolve */
	import type { PageProps } from './$types';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import StepShell from '$lib/components/participant/StepShell.svelte';
	import StepToolBody from '$lib/components/participant/StepToolBody.svelte';
	import StepPager from '$lib/components/participant/StepPager.svelte';
	import StepCover from '$lib/components/participant/StepCover.svelte';
	import StepTour from '$lib/components/participant/StepTour.svelte';
	import StepComplete from '$lib/components/participant/StepComplete.svelte';
	import StepProceedBar from '$lib/components/participant/StepProceedBar.svelte';
	import StepBriefOverlay from '$lib/components/participant/StepBriefOverlay.svelte';
	import StepBriefBar from '$lib/components/participant/StepBriefBar.svelte';
	import ListenTransport from '$lib/components/participant/ListenTransport.svelte';
	import { listen } from '$lib/components/participant/listen.svelte';
	import type { StepItem } from '$lib/components/participant/stepItems';
	import type { SlideDirection } from '$lib/components/participant/slideMotion';
	import { haptic } from '$lib/utils/haptics';
	import { splitSlides } from '$lib/step-brief/splitSlides';
	import {
		isFirstRun,
		hasSeenStepTour,
		markStepTourSeen
	} from '$lib/components/participant/stepTour';
	import { touchFlowTiming } from '$lib/components/participant/flowTiming';
	import { toMetaToolConfig } from '$lib/step-brief/slideMeta';
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
	import { page } from '$app/state';
	import { keyboardOpen } from '$lib/utils/keyboardOpen.svelte';
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

	let prevStepHref = $derived.by(() => {
		if (viewedIndex <= 0) return undefined;
		const prevItem = stepItems[viewedIndex - 1];
		if (!prevItem || prevItem.status !== 'completed') return undefined;
		return prevItem.href;
	});

	// The step brief: the description as slides (ADR-0017). A step with no description still
	// gets one slide, so its cover carries the title and the derived meta line.
	let slides = $derived(splitSlides(workflowStep.description));
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
	/** Which way the last page turn went, so the next slide enters from that side. */
	let slideDirection = $derived.by<SlideDirection>(() => {
		void workflowStep.id;
		return 1;
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

	/**
	 * Typing is its own mode. A phone keyboard leaves a strip barely taller than the question
	 * and the box under it, and the shell was spending the bottom of that strip on the pager
	 * and the scroll's own padding, which pushed the question off the top. While the keyboard
	 * is up the bar stands down: the keyboard is the bottom of the screen, and navigating is
	 * something you do once you have stopped writing.
	 */
	let typing = keyboardOpen();

	let canProceed = $state(false);
	let isSubmitting = $state(false);

	$effect(() => {
		void workflowStep.id;
		isSubmitting = false;
	});

	$effect(() => {
		const type = toolConfig.type;
		if (type === 'learn' || type === 'stories') {
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
		haptic('light');
		if (phase === 'cover') {
			if (slideIndex > 0) {
				slideDirection = -1;
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
		slideDirection = -1;
		slideIndex = briefSlides.length - 1;
	}

	function goForward() {
		if (phase === 'cover') {
			slideDirection = 1;
			if (isLastSlide) {
				haptic('medium');
				phase = 'body';
			} else {
				haptic('light');
				slideIndex += 1;
			}
			return;
		}
		if (sequence.next) {
			haptic('light');
			sequence.next();
			return;
		}
		// Skipping is a decision to move on, not something finished, so it goes straight to the
		// next step. The completion screen is for a step the participant actually did.
		if (forwardMode === 'skip') {
			proceed();
			return;
		}
		// No buzz here: the completion screen brings its own.
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
		haptic('medium');
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

<!-- Listen's pause and speed, between Back and Next while a Learn page is being read aloud
     (ADR-0031). Handed to the pager only then, so the bar is navigation only the rest of the
     time. Declared out here: a snippet directly inside StepShell would become one of its props. -->
{#snippet transport()}
	<ListenTransport />
{/snippet}

{#if conversation && workflowStep && user}
	<StepShell
		class="h-[100dvh]"
		chrome={{
			steps: chromeSteps,
			currentIndex: viewedIndex + 1,
			label: stepLabel,
			fill,
			count: phase === 'done' ? undefined : sequence.count,
			assistantAvailable,
			introUrl,
			briefOpen,
			onBrief: canReopenBrief ? toggleBrief : undefined,
			preview: isPreview
		}}
	>
		{#snippet content()}
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
					direction={slideDirection}
				/>
			{:else}
				<StepToolBody
					{toolConfig}
					{conversation}
					{workflowStep}
					userId={user.id}
					{availableDocuments}
					{hasKnowledgeBaseDocs}
					preview={isPreview}
					permissionToShareWithOrganizers={data.permissionToShareWithOrganizers}
					onDone={stepComplete}
					onCanContinueChange={handleCanContinueChange}
					onSequenceChange={handleSequenceChange}
				/>
			{/if}
		{/snippet}

		{#snippet bar()}
			{#if typing.current}
				<!-- The keyboard has the bottom of the screen. -->
			{:else if phase === 'done'}
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
					middle={listen.status === 'idle' ? undefined : transport}
				/>
			{/if}
		{/snippet}
	</StepShell>

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
