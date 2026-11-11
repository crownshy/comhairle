<script lang="ts">
	import { page } from '$app/state';
	import { resolve } from '$app/paths';
	import { themeStore } from '$lib/stores/theme.svelte';
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
	import {
		orgDisplayName,
		readScenario,
		scenarioFields,
		scenarioQuery,
		withListParams
	} from './scenario';
	import FeaturedHero from './components/FeaturedHero.svelte';
	import SearchBox from './components/SearchBox.svelte';
	import ConversationRow from './components/ConversationRow.svelte';
	import LabConversationCard from './components/LabConversationCard.svelte';
	import PersonalStrip from './components/PersonalStrip.svelte';
	import EmptyState from './components/EmptyState.svelte';

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
	const recommended = $derived(
		all
			.filter((c) => SAMPLE_USER.interests.includes(c.topic) && c.state !== 'results')
			.filter((c) => c.id !== SAMPLE_USER.inProgress.conversationId)
			.slice(0, 4)
	);

	const orgName = $derived(orgDisplayName(scenario.org, themeStore.name));

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
		<div class="-mb-4 flex flex-col gap-1">
			<h1 class="text-2xl font-bold sm:text-3xl">Welcome back, {SAMPLE_USER.firstName}</h1>
			<p class="text-muted-foreground text-base">
				You’ve taken part in {SAMPLE_USER.joinedCount} conversations. Here’s what’s new.
			</p>
		</div>
	{:else}
		<h1 class="sr-only">{orgName} conversations</h1>
	{/if}

	{#if featured.length > 0}
		<FeaturedHero conversations={featured} hrefFor={conversationHref} {actionFor} />
	{/if}

	{#if showPersonal}
		<PersonalStrip
			{inProgress}
			inProgressHref={inProgress && conversationHref(inProgress.id)}
			stepIndex={SAMPLE_USER.inProgress.stepIndex}
			stepCount={SAMPLE_USER.inProgress.stepCount}
			{reportReady}
			reportHref={reportReady && conversationHref(reportReady.id)}
			yourView={SAMPLE_USER.reportReady.yourView}
		/>
	{/if}

	{#if all.length === 0}
		<EmptyState {orgName} />
		<section class="flex flex-col gap-4">
			<h2 class="text-xl font-bold sm:text-2xl">See what changed from past conversations</h2>
			{#each PAST_REPORTS as conversation (conversation.id)}
				<ConversationRow {conversation} href={conversationHref(conversation.id)} />
			{/each}
		</section>
	{:else}
		{#if showPersonal && recommended.length > 0 && !search && !statusFilter}
			<section class="flex flex-col gap-4">
				<div class="flex flex-col gap-1">
					<h2 class="text-xl font-bold sm:text-2xl">Picked for you</h2>
					<span class="text-muted-foreground text-base">
						Based on topics you’ve joined: {SAMPLE_USER.interests
							.join(', ')
							.toLowerCase()}
					</span>
				</div>
				<div class="grid gap-3 md:grid-cols-2">
					{#each recommended as conversation (conversation.id)}
						<ConversationRow {conversation} href={conversationHref(conversation.id)} />
					{/each}
				</div>
			</section>
		{/if}

		<section id="all" class="flex scroll-mt-28 flex-col gap-6 md:gap-8">
			<h2 class="text-2xl font-bold sm:text-3xl">
				{search ? `Results for “${search}”` : 'All conversations'}
			</h2>

			<div class="flex flex-col gap-4">
				<div class="max-w-2xl">
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
								<a
									href={withListParams(page.url, { status: filter.value }) +
										'#all'}
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
