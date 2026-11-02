<script lang="ts">
	import FeedbackModal from '$lib/components/FeedbackModal.svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import { m } from '$lib/paraglide/messages';
	import { cn } from '$lib/utils';
	import type { PageProps } from './$types';
	import GuestIdCard from './GuestIdCard.svelte';
	import ParticipationCount from './ParticipationCount.svelte';
	import ProjectUpdatesDialog from './ProjectUpdatesDialog.svelte';
	import RevisitStepCards from '../RevisitStepCards.svelte';

	let { data }: PageProps = $props();
	let user = $derived(data.user);
	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflow);
	let revisitableSteps = $derived(data.revisitableSteps);
	let isPreview = $derived(data.preview);
	let availableDocuments = $derived(data.availableDocuments);
	let participantCount = $derived(data.participantCount);

	let isGuest = $derived(user.authType === 'guest');
	let showReturnInstructions = $derived(conversation.showThankYouPageAnnonInstructions);
	let showGuestId = $derived(isGuest && showReturnInstructions && !!user.guestCode);

	let nextSteps = $derived.by(() => {
		const steps = [m.thank_you_next_still_open(), m.thank_you_next_organiser_reviews()];
		if (showGuestId) steps.push(m.thank_you_next_guest_return());
		else if (!isGuest && showReturnInstructions) steps.push(m.thank_you_next_email_result());
		return steps;
	});

	const ACTION_BUTTON_CLASS = 'w-full sm:w-56 sm:shrink-0';
</script>

<svelte:head>
	<title>{m.thank_you_page_title()} - Comhairle</title>
</svelte:head>

<div
	class={cn(
		'mx-auto flex w-full flex-col gap-10 px-4 py-10 sm:px-6 md:gap-12 md:py-16',
		showGuestId ? 'max-w-6xl lg:px-20' : 'max-w-5xl'
	)}
>
	<div
		class={cn(
			'grid items-start gap-8',
			showGuestId && 'lg:grid-cols-[minmax(0,1fr)_24rem] lg:gap-12'
		)}
	>
		<header class={cn('flex flex-col gap-5', showGuestId && 'max-w-2xl')}>
			<h1 class="text-3xl leading-tight font-bold md:text-4xl lg:text-5xl">
				{m.thank_you_heading()}
			</h1>
			<p class="flex flex-col gap-1">
				<span class="text-secondary-foreground text-base font-medium">
					{m.thank_you_conversation_label()}
				</span>
				<span class="text-xl font-semibold md:text-2xl">{conversation.title}</span>
			</p>
			{#if conversation.thankYouMessage}
				<ContentRenderer
					content={conversation.thankYouMessage}
					{availableDocuments}
					conversationId={conversation.id}
				/>
			{:else}
				<p class="text-base md:text-lg">{m.thank_you_intro()}</p>
			{/if}
		</header>

		{#if showGuestId}
			<GuestIdCard {user} />
		{/if}
	</div>

	{#if participantCount !== null}
		<ParticipationCount {participantCount} />
	{/if}

	<section
		aria-labelledby="next-steps-heading"
		class="flex flex-col gap-6 lg:flex-row lg:items-center lg:justify-between lg:gap-8 lg:px-4"
	>
		<div class="flex flex-col gap-5">
			<h2 id="next-steps-heading" class="text-2xl font-semibold">
				{m.thank_you_next_heading()}
			</h2>
			<ol class="list-decimal space-y-1 ps-6 text-base md:text-lg">
				{#each nextSteps as step (step)}
					<li>{step}</li>
				{/each}
			</ol>
		</div>
		{#if conversation.enableSignupPrompts}
			<ProjectUpdatesDialog
				{user}
				conversationId={conversation.id}
				class={ACTION_BUTTON_CLASS}
			/>
		{/if}
	</section>

	{#if revisitableSteps.length > 0}
		<section
			aria-labelledby="revisit-heading"
			class="flex flex-col gap-5 rounded-md border px-4 py-3.5 lg:px-6 lg:py-5"
		>
			<div class="flex flex-col gap-2">
				<h2 id="revisit-heading" class="text-2xl font-semibold">
					{m.thank_you_revisit_heading()}
				</h2>
				<p class="text-base md:text-lg">{m.thank_you_revisit_body()}</p>
			</div>
			<RevisitStepCards
				steps={revisitableSteps}
				conversationId={conversation.id}
				workflowId={workflow.id}
				{isPreview}
			/>
		</section>
	{/if}

	{#if conversation.showThankyouPageFeedbackButton}
		<section
			aria-labelledby="feedback-heading"
			class="bg-admin-background flex flex-col gap-4 rounded-lg p-5 lg:flex-row lg:items-center lg:justify-between lg:gap-8"
		>
			<div class="flex flex-col gap-1">
				<h2 id="feedback-heading" class="text-2xl font-semibold">
					{m.thank_you_feedback_heading()}
				</h2>
				<p class="text-base md:text-lg">
					{m.we_are_interested_in_hearing_what_is_good_and_bad_about_this_process()}
				</p>
			</div>
			<FeedbackModal conversationId={conversation.id} class={ACTION_BUTTON_CLASS} />
		</section>
	{/if}
</div>
