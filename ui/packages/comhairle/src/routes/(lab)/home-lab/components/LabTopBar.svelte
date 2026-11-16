<script lang="ts">
	import { page } from '$app/state';
	import { Menu, Search, X } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import LocaleSwitcher from '$lib/components/LocaleSwitcher.svelte';
	import { cn } from '$lib/utils';
	import { scenarioFields, scenarioQuery, type Organisation } from '../scenario';
	import LabLogo from './LabLogo.svelte';
	import SearchBox from './SearchBox.svelte';

	type Props = { org: Organisation; loggedIn: boolean; userInitials: string };
	let { org, loggedIn, userInitials }: Props = $props();

	const LINKS = ['Home', 'About', 'Conversations', 'Your rights'];
	const homePath = $derived(page.url.pathname.split('/c/')[0]);
	const homeHref = $derived(homePath + scenarioQuery(page.url));
	const loginHref = $derived(page.url.pathname + scenarioQuery(page.url, { loggedIn: true }));

	let scrollY = $state(0);
	const scrolled = $derived(scrollY > 8);

	// Phones only: the search icon opens a search row under the bar. It searches the home list
	// from any page.
	let searchOpen = $state(false);
</script>

<svelte:window
	bind:scrollY
	onkeydown={(event) => {
		if (event.key === 'Escape') searchOpen = false;
	}}
/>

<!-- Links are resolve() plus the scenario query string, which this rule can't see. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->

<!-- The bar is the theme colour, like the staging NavBar, and runs straight into the visitor
	pitch below. Once the page scrolls it turns slightly see-through and blurs what passes
	underneath. Links are visual only. -->
<header
	class={cn(
		'text-primary-foreground sticky top-0 z-30 w-full transition-[background-color,box-shadow] duration-300',
		scrolled ? 'bg-primary/90 shadow-md backdrop-blur-xl backdrop-saturate-150' : 'bg-primary'
	)}
>
	<div
		class="mx-auto flex h-16 w-full max-w-[1300px] items-center justify-between gap-4 px-4 md:h-20 md:px-8"
	>
		<a href={homeHref} aria-label="{org.name} home" class="flex min-w-0 items-center">
			<LabLogo theme={org.theme} />
		</a>

		<nav aria-label="Main" class="hidden items-center gap-1 lg:flex">
			{#each LINKS as link, i (link)}
				<a
					href={i === 0 ? homeHref : undefined}
					class="flex h-10 items-center rounded-full px-4 text-base font-medium hover:bg-white/10"
					>{link}</a
				>
			{/each}
		</nav>

		<div class="flex shrink-0 items-center gap-1 sm:gap-2">
			<!-- The staging language picker with the staging NavBar's styling. Like there, phones
				would get it in the menu instead. -->
			<LocaleSwitcher
				class="data-[placeholder]:text-primary-foreground text-primary-foreground hidden rounded-full border border-none bg-transparent py-5 text-base shadow-xs hover:bg-white/10 lg:flex"
			/>
			<button
				aria-label={searchOpen ? 'Close search' : 'Search conversations'}
				aria-expanded={searchOpen}
				aria-controls="top-bar-search"
				onclick={() => (searchOpen = !searchOpen)}
				class="flex size-11 items-center justify-center rounded-full hover:bg-white/10 md:hidden"
			>
				{#if searchOpen}
					<X class="size-6" />
				{:else}
					<Search class="size-6" />
				{/if}
			</button>
			{#if loggedIn}
				<span
					aria-label="Your account"
					class="text-primary flex size-10 items-center justify-center rounded-full bg-white text-base font-bold"
				>
					{userInitials}
				</span>
			{:else}
				<a
					href={loginHref}
					class="hidden h-10 items-center rounded-full px-3 text-base font-semibold hover:bg-white/10 sm:flex"
					>Log in</a
				>
				<Button
					href={loginHref}
					variant="outline"
					class="text-primary h-10 rounded-full border-transparent bg-white px-5 text-base font-semibold hover:bg-white/90"
					>Sign up</Button
				>
			{/if}
			<button
				aria-label="Menu"
				class="flex size-11 items-center justify-center rounded-full hover:bg-white/10 lg:hidden"
			>
				<Menu class="size-6" />
			</button>
		</div>
	</div>

	{#if searchOpen}
		<div id="top-bar-search" class="px-4 pb-4 md:hidden">
			<SearchBox
				value={page.url.searchParams.get('q') ?? ''}
				hiddenFields={scenarioFields(page.url)}
				action="{homePath}#all"
				compact
				autofocus
				onsubmit={() => (searchOpen = false)}
			/>
		</div>
	{/if}
</header>
