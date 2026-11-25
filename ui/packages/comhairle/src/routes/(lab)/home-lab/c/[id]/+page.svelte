<script lang="ts">
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import { ArrowRight, ChevronLeft, MessageSquareText } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Accordion from '$lib/components/ui/accordion';
	import {
		FAQS,
		STEPS,
		SAMPLE_USER,
		findConversation,
		timingLabel,
		type LabConversation
	} from '../../mockData';
	import { readScenario, scenarioQuery } from '../../scenario';
	import StateBadge from '../../components/StateBadge.svelte';

	const conversation = $derived(findConversation(page.params.id ?? ''));
	const scenario = $derived(readScenario(page.url));
	const homeHref = $derived(resolve('/(lab)/home-lab') + scenarioQuery(page.url));
	const bannerUrl = $derived(scenario.bannerImage?.url ?? (conversation?.imageUrl as string));

	const isContinuing = $derived(
		scenario.loggedIn && conversation?.id === SAMPLE_USER.inProgress.conversationId
	);
	const actionLabel = $derived.by(() => {
		if (conversation?.state === 'results') return 'Read the report';
		if (conversation?.state === 'upcoming') return 'Remind me when it opens';
		return isContinuing ? 'Continue where you left off' : 'Take part';
	});

	// Upcoming conversations have nobody yet, so the line only shows once people have joined.
	function joinedLabel(c: LabConversation): string | null {
		if (c.participantCount === 0) return null;
		const people = c.participantCount === 1 ? 'person' : 'people';
		return `${c.participantCount.toLocaleString()} ${people} joined this conversation`;
	}

	const statusLine = $derived(
		conversation ? [joinedLabel(conversation), timingLabel(conversation)].filter(Boolean) : []
	);

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
			? 'h-14 w-full rounded-full px-8 text-lg font-bold sm:w-auto'
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
	<!-- Top band on white: status line, title, a short description and the button. -->
	<div class="bg-background ml-[calc(50%-50vw)] w-screen">
		<div class="mx-auto flex w-full max-w-5xl flex-col gap-6 px-4 pt-6 md:px-8 md:pt-10">
			<a
				href={homeHref}
				class="text-muted-foreground hover:text-foreground -ml-1 flex w-fit items-center gap-1 text-base font-medium"
			>
				<ChevronLeft class="size-5" /> All conversations
			</a>
			<div class="flex flex-wrap items-center gap-x-4 gap-y-2">
				<StateBadge state={conversation.state} />
				<p class="text-muted-foreground text-base">{statusLine.join('. ')}</p>
			</div>
			<h1
				class="text-3xl leading-tight font-bold tracking-tight text-balance sm:text-4xl lg:text-5xl"
			>
				{conversation.title}
			</h1>
			<p class="line-clamp-3 max-w-3xl text-base text-pretty">{conversation.description}</p>
			<div bind:this={mainButton} class="flex sm:justify-end">
				{@render takePart('big')}
			</div>
		</div>
	</div>

	<!-- Rounded image at content width, sitting across the join from white to grey. Crops from
		just above centre because faces tend to sit in the upper third. -->
	<div
		class="from-background to-muted ml-[calc(50%-50vw)] w-screen bg-linear-to-b from-50% to-50% pt-8 pb-12 md:pt-10 md:pb-16"
	>
		<div class="mx-auto w-full max-w-5xl px-4 md:px-8">
			<div
				class="bg-primary/10 relative aspect-[4/3] overflow-hidden rounded-3xl sm:aspect-[2/1]"
			>
				{#if bannerUrl}
					<img
						src={bannerUrl}
						alt=""
						class="absolute inset-0 size-full object-cover object-[center_25%]"
					/>
				{:else}
					<MessageSquareText class="text-primary/30 absolute inset-0 m-auto size-20" />
				{/if}
			</div>
		</div>
	</div>

	<div class="mx-auto flex w-full max-w-3xl flex-col gap-8 pt-10 md:pt-14">
		{#if started}
			<p class="bg-accent text-accent-foreground rounded-2xl p-4 text-base" role="status">
				In the real product this opens step 1 straight away, with no extra screens.
			</p>
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
