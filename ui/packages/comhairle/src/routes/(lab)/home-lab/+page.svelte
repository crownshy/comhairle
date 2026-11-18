<script lang="ts">
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import { Pager, getPage, calcOffset } from '$lib/pagination';
	import { cn } from '$lib/utils';
	import {
		conversationsFor,
		findConversation,
		PAST_REPORTS,
		SAMPLE_USER,
		type LabConversation,
		type LabState
	} from './mockData';
	import { readScenario, scenarioFields, scenarioQuery, withListParams } from './scenario';
	import FeaturedHero from './components/FeaturedHero.svelte';
	import SearchBox from './components/SearchBox.svelte';
	import LabConversationCard from './components/LabConversationCard.svelte';
	import PersonalStrip from './components/PersonalStrip.svelte';
	import EmptyState from './components/EmptyState.svelte';
	import Pitch from './components/Pitch.svelte';

	const PAGE_SIZE = 20;

	type Chip = LabState | 'popular' | 'latest';
	const FILTERS: { value: Chip | null; label: string }[] = [
		{ value: null, label: 'All' },
		{ value: 'popular', label: 'Popular' },
		{ value: 'latest', label: 'Latest' },
		{ value: 'live', label: 'Live now' },
		{ value: 'closing', label: 'Closing soon' },
		{ value: 'upcoming', label: 'Coming soon' },
		{ value: 'results', label: 'Results ready' }
	];

	const scenario = $derived(readScenario(page.url));
	const query = $derived(scenarioQuery(page.url));
	const conversationHref = (id: string) => resolve('/(lab)/home-lab/c/[id]', { id }) + query;

	const all = $derived(conversationsFor(scenario.dataset));
	const featured = $derived(all.filter((c) => c.isFeatured));

	const statusFilter = $derived(page.url.searchParams.get('status'));
	const search = $derived(page.url.searchParams.get('q') ?? '');
	const matchesSearch = (c: LabConversation) =>
		!search || `${c.title} ${c.topic}`.toLowerCase().includes(search.toLowerCase());

	// Popular ranks the ones you can still join; Latest shows the newest first. The rest filter by status.
	const filtered = $derived.by(() => {
		const found = all.filter(matchesSearch);
		if (statusFilter === 'popular') {
			return found
				.filter((c) => c.state === 'live' || c.state === 'closing')
				.toSorted((a, b) => b.participantCount - a.participantCount);
		}
		if (statusFilter === 'latest') {
			return found
				.filter((c) => c.openedDaysAgo >= 0)
				.toSorted((a, b) => a.openedDaysAgo - b.openedDaysAgo);
		}
		return found.filter((c) => !statusFilter || c.state === statusFilter);
	});
	const offset = $derived(calcOffset({ page: getPage(page.url), pageSize: PAGE_SIZE }));
	const visible = $derived(filtered.slice(offset, offset + PAGE_SIZE));

	const inProgress = $derived(findConversation(SAMPLE_USER.inProgress.conversationId));
	const reportReady = $derived(findConversation(SAMPLE_USER.reportReady.conversationId));
	const showPersonal = $derived(scenario.loggedIn && all.length > 0);

	const orgName = $derived(scenario.org.name);

	function actionFor(conversation: LabConversation): string {
		if (conversation.state === 'results') return 'Read the report';
		if (conversation.state === 'upcoming') return 'Find out more';
		if (showPersonal && conversation.id === SAMPLE_USER.inProgress.conversationId)
			return 'Continue';
		return 'Take part';
	}
</script>

<!-- Links are resolve() plus the scenario query string, which this rule can't see. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->

<svelte:head>
	<title>{orgName} conversations (prototype)</title>
</svelte:head>

<div class="mx-auto flex w-full max-w-6xl flex-col gap-14 pt-6 md:gap-20 md:pt-10">
	{#if scenario.loggedIn}
		<!-- Logged in: the welcome and their own conversations (in progress, results ready)
			sit together at the top as one section, before the featured slides. -->
		<section
			aria-labelledby="welcome-heading"
			class="flex flex-col gap-6 pt-4 pb-2 md:gap-8 md:pt-8 md:pb-4"
		>
			<div class="flex flex-col gap-1">
				<h1 id="welcome-heading" class="text-2xl font-bold sm:text-3xl">
					Welcome back, {SAMPLE_USER.firstName}
				</h1>
				<p class="text-muted-foreground text-base">
					You’ve taken part in {SAMPLE_USER.joinedCount} conversations. Here’s what’s new.
				</p>
			</div>
			{#if showPersonal}
				<PersonalStrip
					{inProgress}
					inProgressHref={inProgress && conversationHref(inProgress.id)}
					stepIndex={SAMPLE_USER.inProgress.stepIndex}
					stepCount={SAMPLE_USER.inProgress.stepCount}
					{reportReady}
					reportHref={reportReady && conversationHref(reportReady.id)}
				/>
			{/if}
		</section>
	{:else}
		<!-- Pulled up under the top bar (its height plus this column's top padding), so the
			gradient runs behind the see-through bar. -->
		<div class="-mt-[5.5rem] md:-mt-[7.5rem]">
			<Pitch showAction={all.length > 0} />
		</div>
	{/if}

	{#if featured.length > 0}
		<FeaturedHero conversations={featured} hrefFor={conversationHref} {actionFor} />
	{/if}

	{#if all.length === 0}
		<EmptyState {orgName} />
		<section class="flex flex-col gap-6 md:gap-8">
			<h2 class="text-2xl font-bold sm:text-3xl">See what changed from past conversations</h2>
			<!-- The same card and grid as the conversation list, so there's one card design. -->
			<div class="grid gap-8 lg:grid-cols-3 lg:gap-x-8 lg:gap-y-10">
				{#each PAST_REPORTS as conversation (conversation.id)}
					<LabConversationCard {conversation} href={conversationHref(conversation.id)} />
				{/each}
			</div>
		</section>
	{:else}
		<section id="all" class="flex scroll-mt-28 flex-col gap-6 md:gap-8">
			<div class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1">
				<h2 id="all-heading" tabindex="-1" class="text-2xl font-bold sm:text-3xl">
					{search ? `Results for “${search}”` : 'All conversations'}
				</h2>
				{#if search}
					<a
						href={withListParams(page.url, { q: null }) + '#all'}
						class="text-primary text-base font-semibold underline">Clear search</a
					>
				{/if}
			</div>

			<div class="flex flex-col gap-4">
				<!-- On phones, search lives in the top bar instead. -->
				<div class="hidden max-w-2xl md:block">
					<SearchBox value={search} hiddenFields={scenarioFields(page.url)} />
				</div>

				<nav
					aria-label="Filter conversations"
					class="-mx-4 overflow-x-auto px-4 sm:mx-0 sm:px-0"
				>
					<ul class="flex w-max gap-2">
						{#each FILTERS as filter (filter.label)}
							{@const active = statusFilter === filter.value}
							<li>
								<!-- noscroll keeps the page where it is when a filter changes;
									keepfocus leaves keyboard focus on the chip just pressed. -->
								<a
									href={withListParams(page.url, { status: filter.value })}
									data-sveltekit-noscroll
									data-sveltekit-keepfocus
									aria-current={active ? 'page' : undefined}
									class={cn(
										'flex h-9 items-center rounded-full border px-3.5 text-base whitespace-nowrap',
										active
											? 'bg-primary text-primary-foreground border-primary'
											: 'border-border bg-card'
									)}>{filter.label}</a
								>
							</li>
						{/each}
					</ul>
				</nav>
			</div>

			{#if filtered.length === 0}
				<div class="bg-card border-border flex flex-col gap-2 rounded-2xl border p-5">
					<span class="text-lg font-bold">Nothing matches</span>
					<span class="text-muted-foreground text-base">
						Try a broader word, or
						<a
							href={withListParams(page.url, { q: null, status: null })}
							class="text-primary underline">clear the filters</a
						>.
					</span>
				</div>
			{:else}
				<div class="grid gap-8 lg:grid-cols-3 lg:gap-x-8 lg:gap-y-10">
					{#each visible as conversation (conversation.id)}
						<LabConversationCard
							{conversation}
							href={conversationHref(conversation.id)}
						/>
					{/each}
				</div>
			{/if}

			{#if filtered.length > PAGE_SIZE}
				<Pager pageSize={PAGE_SIZE} count={filtered.length} url={page.url} />
			{/if}
		</section>
	{/if}
</div>
