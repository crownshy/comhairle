<script lang="ts">
	/* The step links are built by $lib/urls, not from a typed route id, so resolve() has
	   nothing to resolve. */
	/* eslint-disable svelte/no-navigation-without-resolve */
	import { onMount } from 'svelte';
	import { page } from '$app/state';
	import { ChevronDown } from 'lucide-svelte';
	import type { PageProps } from './$types';
	import { conversation_url, workflow_step_url } from '$lib/urls';
	import * as Collapsible from '$lib/components/ui/collapsible';
	import StepChrome from '$lib/components/participant/StepChrome.svelte';
	import type { StepItem } from '$lib/components/participant/stepItems';
	import { stepPreviews } from '$lib/components/participant/stepPreview';
	import { minutesInFlow } from '$lib/components/participant/flowTiming';
	import { learningAssistantAvailable } from '$lib/components/LearningAssistant/availability';
	import FeedbackModal from '$lib/components/FeedbackModal.svelte';
	import UserConversationPreferencesForm from '$lib/components/UserConversationPreferencesForm/UserConversationPreferencesForm.svelte';
	import UpgradeAccountModal from '$lib/components/UpgradeAccountModal/UpgradeAccountModal.svelte';
	import EmailRegistrationForm from '$lib/components/EmailRegistrationForm/EmailRegistrationForm.svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import ThankYouStats from './ThankYouStats.svelte';
	import { haptic } from '$lib/utils/haptics';
	import * as m from '$lib/paraglide/messages';

	let { data }: PageProps = $props();
	let user = $derived(data.user);
	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflow);
	let steps = $derived(data.steps);
	let revisitableSteps = $derived(data.revisitableSteps);
	let hasRevisitableSteps = $derived(revisitableSteps.length > 0);
	let isPreview = $derived(data.preview);
	let availableDocuments = $derived(data.availableDocuments);
	let queryString = $derived(page.url.search);

	/** The revisitable steps with their tool icons, the way the landing page lists them. */
	let revisitPreviews = $derived(stepPreviews(revisitableSteps));

	// Preview records no progress, so an admin looking at this page would be told they did
	// nothing. They are being shown what a participant who finished sees, so show that.
	let stepsDone = $derived(
		isPreview ? steps.length : steps.filter((s) => s.progressStatus === 'done').length
	);
	let percentComplete = $derived(
		steps.length ? Math.round((stepsDone / steps.length) * 100) : 100
	);

	// Nothing finished means someone arrived here without doing the flow. There is no
	// achievement to count up, so the row stays away rather than reporting zero.
	let hasStats = $derived(stepsDone > 0);

	/**
	 * Minutes come from a clock kept in the browser, so there is nothing to render on the
	 * server. The whole row waits for the client rather than rendering two tiles and growing
	 * a third under the participant.
	 */
	let minutes = $state<number | null>(null);
	let mounted = $state(false);
	onMount(() => {
		minutes = minutesInFlow(conversation.id);
		mounted = true;
		// Only the finish line buzzes: someone who came back to reread gets nothing.
		if (hasStats) haptic('success');
	});

	let assistantAvailable = $derived(
		learningAssistantAvailable(conversation, data.hasKnowledgeBaseDocs)
	);

	let introUrl = $derived(conversation_url(conversation.id, isPreview) + queryString);

	// Everything is behind the participant now. A step they can still open is offered as
	// completed, one they cannot as completed and locked, which is what the dropdown reads.
	let stepItems = $derived<StepItem[]>(
		steps.map((step) => {
			const canRevisit = revisitableSteps.some((s) => s.id === step.id);
			return {
				id: step.id,
				name: step.name,
				status: canRevisit ? 'completed' : 'completed-locked',
				href: canRevisit
					? workflow_step_url(conversation.id, workflow.id, step.id, isPreview) +
						queryString
					: undefined
			};
		})
	);

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

	/**
	 * The last step's segment, full. There is no thank-you segment of its own: adding one
	 * would change the "Step N of M" every step quotes.
	 */
	let currentIndex = $derived(steps.length);

	let showFeedback = $derived(conversation.showThankyouPageFeedbackButton);
	let showMore = $derived(hasRevisitableSteps || showFeedback);
</script>

<svelte:head>
	<title>{m.thank_you_label()} - Comhairle</title>
</svelte:head>

<!-- The end of the flow is still the flow: same chrome, same one screen, and the extras are
	folded away under it rather than stacked down the page. -->
<div class="grid h-[100dvh] grid-cols-[minmax(0,1fr)] grid-rows-[auto_1fr] overflow-hidden">
	<StepChrome
		steps={chromeSteps}
		{currentIndex}
		label={m.thank_you_label()}
		fill={1}
		{assistantAvailable}
		{introUrl}
		preview={isPreview}
	/>

	<main
		class="flex min-h-0 w-full flex-col overflow-y-auto mask-b-from-[calc(100%-2.5rem)] pb-10"
	>
		<div
			class="mx-auto flex w-full max-w-2xl flex-col items-center gap-8 px-5 pt-[8vh] pb-16 text-center md:px-6"
		>
			<h1
				class="animate-in fade-in slide-in-from-bottom-2 text-3xl font-bold duration-500 md:text-4xl"
			>
				{m.thank_you_title()}
				<span
					class="animate-in zoom-in-50 spin-in-12 fill-mode-both inline-block delay-200 duration-700 motion-reduce:animate-none"
					aria-hidden="true">🎉</span
				>
			</h1>

			{#if mounted && hasStats}
				<ThankYouStats {minutes} {stepsDone} {percentComplete} />
			{/if}

			<div class="prose w-full max-w-none">
				{#if conversation.thankYouMessage}
					<ContentRenderer
						content={conversation.thankYouMessage}
						{availableDocuments}
						conversationId={conversation.id}
					/>
				{:else}
					<p>{m.thank_you_intro({ title: conversation.title })}</p>
				{/if}

				{#if conversation.showThankYouPageAnnonInstructions}
					{#if user.authType === 'guest'}
						<p>{m.thank_you_anonymous_id({ id: user.guestCode })}</p>
					{:else}
						<p>{m.thank_you_results_note()}</p>
					{/if}
				{/if}
			</div>

			{#if showMore}
				<Collapsible.Root class="w-full">
					<Collapsible.Trigger
						class="text-primary group mx-auto flex items-center gap-2 text-base font-medium underline underline-offset-4"
					>
						{m.thank_you_more()}
						<ChevronDown
							class="size-5 transition-transform group-data-[state=open]:rotate-180"
							aria-hidden="true"
						/>
					</Collapsible.Trigger>
					<Collapsible.Content class="flex flex-col gap-6 pt-6">
						{#if hasRevisitableSteps}
							<p class="text-muted-foreground text-base">{m.thank_you_more_body()}</p>
							<ul class="flex flex-col gap-2 text-left">
								{#each revisitPreviews as step (step.id)}
									{@const StepIcon = step.icon}
									<li>
										<a
											class="hover:bg-accent flex items-center gap-4 rounded-2xl px-3 py-2"
											href={workflow_step_url(
												conversation.id,
												workflow.id,
												step.id,
												isPreview
											) + queryString}
										>
											<span
												class="bg-accent text-accent-foreground flex size-10 shrink-0 items-center justify-center rounded-full"
											>
												{#if StepIcon}
													<StepIcon class="size-5" aria-hidden="true" />
												{/if}
											</span>
											<span class="min-w-0 flex-1 text-base">{step.name}</span
											>
										</a>
									</li>
								{/each}
							</ul>
						{/if}

						{#if showFeedback}
							<div class="flex justify-center">
								<FeedbackModal conversationId={conversation.id} />
							</div>
						{/if}
					</Collapsible.Content>
				</Collapsible.Root>
			{/if}

			{#if conversation.enableSignupPrompts}
				<Collapsible.Root class="w-full">
					<Collapsible.Trigger
						class="text-primary group mx-auto flex items-center gap-2 text-base font-medium underline underline-offset-4"
					>
						{m.thank_you_keep_informed()}
						<ChevronDown
							class="size-5 transition-transform group-data-[state=open]:rotate-180"
							aria-hidden="true"
						/>
					</Collapsible.Trigger>
					<Collapsible.Content class="flex flex-col gap-4 pt-6 text-left">
						{#if user.authType === 'guest'}
							<p class="text-muted-foreground text-base">
								{m.thank_you_keep_informed_anonymous()}
							</p>
							<UpgradeAccountModal currentUser={user} />
							<EmailRegistrationForm conversation_id={conversation.id} />
						{:else}
							<p class="text-muted-foreground text-base">
								{m.thank_you_keep_informed_body()}
							</p>
							<UserConversationPreferencesForm
								conversationId={conversation.id}
								isGuest={user.authType === 'guest'}
							/>
						{/if}
					</Collapsible.Content>
				</Collapsible.Root>
			{/if}
		</div>
	</main>
</div>
