<script lang="ts">
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import {
		ArrowRight,
		ChevronLeft,
		Clock,
		CalendarDays,
		MessageSquareText,
		Users
	} from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Accordion from '$lib/components/ui/accordion';
	import {
		FAQS,
		STEPS,
		SAMPLE_USER,
		findConversation,
		participantsLabel,
		timingLabel
	} from '../../mockData';
	import { readScenario, scenarioQuery } from '../../scenario';
	import StateBadge from '../../components/StateBadge.svelte';
	import OpinionHook from '../../components/OpinionHook.svelte';

	const conversation = $derived(findConversation(page.params.id ?? ''));
	const scenario = $derived(readScenario(page.url));
	const homeHref = $derived(resolve('/(lab)/home-lab') + scenarioQuery(page.url));

	const isContinuing = $derived(
		scenario.loggedIn && conversation?.id === SAMPLE_USER.inProgress.conversationId
	);
	const actionLabel = $derived.by(() => {
		if (conversation?.state === 'results') return 'Read the report';
		if (conversation?.state === 'upcoming') return 'Remind me when it opens';
		return isContinuing ? 'Continue where you left off' : 'Take part';
	});

	let started = $state(false);
	let mainButton = $state<HTMLElement>();
	let mainButtonVisible = $state(true);

	// The sticky bar only shows once the big button has scrolled away, so there are never two.
	$effect(() => {
		if (!mainButton) return;
		const observer = new IntersectionObserver(([entry]) => {
			mainButtonVisible = entry.isIntersecting;
		});
		observer.observe(mainButton);
		return () => observer.disconnect();
	});
</script>

<!-- Links are resolve() plus the scenario query string, which this rule can't see. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->

<svelte:head>
	<title>{conversation?.title ?? 'Conversation'} (prototype)</title>
</svelte:head>

{#snippet takePart(size: 'big' | 'bar')}
	<Button
		onclick={() => (started = true)}
		class={size === 'big'
			? 'h-16 w-full rounded-full text-xl font-bold sm:w-auto sm:px-12'
			: 'h-12 grow rounded-full text-lg font-bold'}
	>
		{actionLabel}
		<ArrowRight class="size-5" />
	</Button>
{/snippet}

{#if !conversation}
	<div class="flex flex-col gap-4 py-16">
		<h1 class="text-2xl font-bold">Conversation not found</h1>
		<a href={homeHref} class="text-primary text-base font-semibold">Back to all conversations</a
		>
	</div>
{:else}
	<!-- Full screen width: breaks out of the page column. Dark edges pull the eye to the middle. -->
	<div
		class="bg-primary/10 relative ml-[calc(50%-50vw)] h-56 w-screen overflow-hidden sm:h-80 lg:h-[28rem]"
	>
		{#if conversation.imageUrl}
			<img
				src={conversation.imageUrl as string}
				alt=""
				class="absolute inset-0 size-full object-cover"
			/>
		{:else}
			<MessageSquareText class="text-primary/30 absolute inset-0 m-auto size-20" />
		{/if}
		<div
			class="absolute inset-0 bg-gradient-to-r from-black/75 via-black/0 via-50% to-black/75"
			aria-hidden="true"
		></div>
		<div class="relative mx-auto h-full w-full max-w-[1300px] px-4 pt-4 md:px-8 md:pt-6">
			<a
				href={homeHref}
				class="bg-background/90 text-foreground flex h-11 w-fit items-center gap-1 rounded-full pr-4 pl-3 text-base font-semibold shadow-md backdrop-blur"
			>
				<ChevronLeft class="size-5" /> All conversations
			</a>
		</div>
	</div>

	<div class="mx-auto flex w-full max-w-3xl flex-col gap-8 pt-6 md:pt-8">
		<!-- First screen: just enough to decide, then the action. -->
		<section class="flex flex-col gap-4">
			<div class="flex gap-2"><StateBadge state={conversation.state} /></div>
			<h1 class="text-3xl leading-tight font-extrabold tracking-tight sm:text-4xl">
				{conversation.title}
			</h1>
			<p class="text-lg">{conversation.shortDescription}</p>
			<div class="text-muted-foreground flex flex-wrap gap-x-5 gap-y-2 text-base">
				<span class="flex items-center gap-1.5">
					<Clock class="size-5" /> About {conversation.estimatedMinutes} min
				</span>
				<span class="flex items-center gap-1.5">
					<CalendarDays class="size-5" />
					{timingLabel(conversation)}
				</span>
				{#if conversation.participantCount > 0}
					<span class="flex items-center gap-1.5">
						<Users class="size-5" />
						{participantsLabel(conversation)}
					</span>
				{/if}
			</div>
			<div bind:this={mainButton} class="pt-2">{@render takePart('big')}</div>
			{#if started}
				<p class="bg-accent text-accent-foreground rounded-2xl p-4 text-base" role="status">
					In the real product this opens step 1 straight away, with no extra screens.
				</p>
			{/if}
		</section>

		{#if conversation.quotes.length > 0}
			<section class="flex flex-col gap-4">
				<h2 class="text-xl font-bold">What people are saying</h2>
				<div class="grid gap-3 sm:grid-cols-2">
					{#each conversation.quotes.slice(0, 2) as quote (quote.text)}
						<OpinionHook {quote} />
					{/each}
				</div>
			</section>
		{/if}

		<!-- Everything else is folded away for the few who want it. -->
		<Accordion.Root type="multiple" class="border-border border-t">
			<Accordion.Item value="how">
				<Accordion.Trigger class="text-lg font-bold">How it works</Accordion.Trigger>
				<Accordion.Content>
					<ol class="flex flex-col gap-3">
						{#each STEPS as step, i (step.name)}
							<li class="flex items-center gap-3 text-base">
								<span
									class="bg-primary text-primary-foreground flex size-8 shrink-0 items-center justify-center rounded-full font-bold"
									>{i + 1}</span
								>
								<span class="grow">{step.name}</span>
								<span class="text-muted-foreground">{step.minutes} min</span>
							</li>
						{/each}
					</ol>
				</Accordion.Content>
			</Accordion.Item>
			{#each FAQS as faq (faq.question)}
				<Accordion.Item value={faq.question}>
					<Accordion.Trigger class="text-lg font-bold">{faq.question}</Accordion.Trigger>
					<Accordion.Content class="text-base">{faq.answer}</Accordion.Content>
				</Accordion.Item>
			{/each}
		</Accordion.Root>
	</div>

	{#if !mainButtonVisible}
		<div
			class="bg-background/95 border-border fixed inset-x-0 bottom-0 z-40 flex border-t p-3 backdrop-blur sm:hidden"
		>
			{@render takePart('bar')}
		</div>
	{/if}
{/if}
