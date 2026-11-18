<script lang="ts">
	import { page } from '$app/state';
	import { Menu, Search, X } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import LocaleSwitcher from '$lib/components/LocaleSwitcher.svelte';
	import { cn } from '$lib/utils';
	import { scenarioFields, scenarioQuery, type Organisation } from '../scenario';
	import LabLogo from './LabLogo.svelte';
	import SearchBox from './SearchBox.svelte';

	type Props = {
		org: Organisation;
		loggedIn: boolean;
		userInitials: string;
		/** The page starts with the light gradient hero, which the bar sits on until scrolled. */
		overHero?: boolean;
	};
	let { org, loggedIn, userInitials, overHero = false }: Props = $props();

	// No Home link (the logo does that) and no Conversations link (the home page is the list).
	// They open the real staging pages.
	const LINKS = [
		{ label: 'About', href: '/about' },
		{ label: 'Your rights', href: '/rights' }
	];
	const homePath = $derived(page.url.pathname.split('/c/')[0]);
	const homeHref = $derived(homePath + scenarioQuery(page.url));
	const loginHref = $derived(page.url.pathname + scenarioQuery(page.url, { loggedIn: true }));

	let scrollY = $state(0);
	const scrolled = $derived(scrollY > 8);
	// Over the light hero (and not scrolled, and no search open) the bar is see-through with
	// dark text and logo. Anywhere else it's the theme colour with white text.
	const light = $derived(overHero && !scrolled);
	const hover = $derived(light ? 'hover:bg-black/5' : 'hover:bg-white/10');

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

<!-- The bar is the theme colour, like the staging NavBar. On the visitor home page it starts
	see-through over the light gradient hero, with dark text, and turns to the theme colour
	once the page scrolls. Scrolled, it's slightly see-through and blurs what passes
	underneath. Links are visual only. -->
<header
	class={cn(
		'sticky top-0 z-30 w-full transition-[background-color,box-shadow,color] duration-300',
		light ? 'text-foreground bg-transparent' : 'text-primary-foreground',
		!light &&
			(scrolled
				? 'bg-primary/90 shadow-md backdrop-blur-xl backdrop-saturate-150'
				: 'bg-primary')
	)}
>
	<div
		class="mx-auto flex h-16 w-full max-w-[1300px] items-center justify-between gap-4 px-4 md:h-20 md:px-8"
	>
		<a href={homeHref} aria-label="{org.name} home" class="flex min-w-0 items-center">
			<LabLogo theme={org.theme} surface={light ? 'light' : 'dark'} />
		</a>

		<div class="flex shrink-0 items-center gap-1 sm:gap-2">
			<!-- The links sit with the language picker and account buttons on the right. -->
			<nav aria-label="Main" class="hidden items-center gap-1 lg:flex">
				{#each LINKS as link (link.href)}
					<a
						href={link.href}
						class="flex h-10 items-center rounded-full px-4 text-base font-medium {hover}"
						>{link.label}</a
					>
				{/each}
			</nav>
			<!-- The staging language picker with the staging NavBar's styling. Like there, phones
				would get it in the menu instead. -->
			<LocaleSwitcher
				class="hidden rounded-full border border-none bg-transparent py-5 text-base text-current shadow-xs data-[placeholder]:text-current lg:flex {hover}"
			/>
			<button
				aria-label={searchOpen ? 'Close search' : 'Search conversations'}
				aria-expanded={searchOpen}
				aria-controls="top-bar-search"
				onclick={() => (searchOpen = !searchOpen)}
				class="flex size-11 items-center justify-center rounded-full md:hidden {hover}"
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
					class={cn(
						'flex size-10 items-center justify-center rounded-full text-base font-bold',
						light ? 'bg-primary text-primary-foreground' : 'text-primary bg-white'
					)}
				>
					{userInitials}
				</span>
			{:else}
				<a
					href={loginHref}
					class="hidden h-10 items-center rounded-full px-3 text-base font-semibold sm:flex {hover}"
					>Log in</a
				>
				<Button
					href={loginHref}
					variant="outline"
					class={cn(
						'h-10 rounded-full border-transparent px-5 text-base font-semibold',
						light
							? 'bg-foreground text-background hover:bg-foreground/90 hover:text-background'
							: 'text-primary bg-white hover:bg-white/90'
					)}>Sign up</Button
				>
			{/if}
			<button
				aria-label="Menu"
				class="flex size-11 items-center justify-center rounded-full lg:hidden {hover}"
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
