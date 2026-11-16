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
		type LabConversation
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

	type Fact = { label: string; value: string };

	// The third fact in the info bar: when it closes, or opens, or closed.
	function timing(c: LabConversation): Fact {
		const days = (n: number) => `${n} day${n === 1 ? '' : 's'}`;
		if (c.state === 'upcoming') return { label: 'Opens', value: `In ${days(c.days)}` };
		if (c.state === 'results') return { label: 'Closed', value: `${days(c.days)} ago` };
		return { label: 'Closes', value: c.days === 0 ? 'Today' : `In ${days(c.days)}` };
	}

	const facts = $derived<Fact[]>(
		conversation
			? [
					{ label: 'Run by', value: conversation.organiser },
					{ label: 'Time needed', value: `About ${conversation.estimatedMinutes} min` },
					timing(conversation)
				]
			: []
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
			? 'h-16 w-full rounded-full px-10 text-xl font-bold'
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
	<!-- Top band on white, full screen width: status, title, a short description, then the
		info bar, which sits across the join with the image below. -->
	<div class="bg-background ml-[calc(50%-50vw)] w-screen">
		<div class="mx-auto flex w-full max-w-5xl flex-col gap-5 px-4 pt-6 md:px-8 md:pt-10">
			<a
				href={homeHref}
				class="text-muted-foreground hover:text-foreground -ml-1 flex w-fit items-center gap-1 text-base font-medium"
			>
				<ChevronLeft class="size-5" /> All conversations
			</a>
			<div class="flex flex-col items-start gap-4 md:px-6">
				<StateBadge state={conversation.state} />
				<h1
					class="text-3xl leading-tight font-bold tracking-tight text-balance sm:text-4xl lg:text-5xl"
				>
					{conversation.title}
				</h1>
				<p class="line-clamp-3 max-w-3xl text-lg">{conversation.description}</p>
			</div>

			<!-- Pulled down over the image. Phones stack the facts; wider screens put them in a row
				with the button at the end. -->
			<div
				class="bg-card ring-border/60 relative z-10 -mb-24 grid gap-4 rounded-3xl p-5 shadow-lg ring-1 sm:-mb-20 md:grid-cols-[1fr_1fr_1fr_auto] md:items-center md:gap-0 md:p-6"
			>
				{#each facts as fact, i (fact.label)}
					<div
						class={i === 0
							? 'flex items-baseline justify-between gap-4 md:flex-col md:justify-start md:gap-1 md:pr-6 md:pl-2'
							: 'border-border flex items-baseline justify-between gap-4 border-t pt-4 md:flex-col md:justify-start md:gap-1 md:border-t-0 md:border-l md:px-6 md:pt-0'}
					>
						<span class="text-muted-foreground text-base">{fact.label}</span>
						<span class="text-lg font-semibold">{fact.value}</span>
					</div>
				{/each}
				<div bind:this={mainButton} class="pt-2 md:pt-0 md:pl-4">
					{@render takePart('big')}
				</div>
			</div>
		</div>
	</div>

	<!-- Full screen width image under the band. -->
	<div
		class="bg-primary/10 relative ml-[calc(50%-50vw)] h-72 w-screen overflow-hidden sm:h-96 lg:h-[32rem]"
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
	</div>

	<div class="mx-auto flex w-full max-w-3xl flex-col gap-8 pt-10 md:pt-14">
		{#if started}
			<p class="bg-accent text-accent-foreground rounded-2xl p-4 text-base" role="status">
				In the real product this opens step 1 straight away, with no extra screens.
			</p>
		{/if}

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
