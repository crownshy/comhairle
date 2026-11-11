<script lang="ts">
	import { page } from '$app/state';
	import { Menu } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import ComhairleLogo from '$lib/components/ComhairleLogo.svelte';
	import { cn } from '$lib/utils';
	import { THEME_ORG_ID, scenarioQuery, type Organisation } from '../scenario';

	type Props = { org: Organisation; orgName: string; loggedIn: boolean; userInitials: string };
	let { org, orgName, loggedIn, userInitials }: Props = $props();

	const LINKS = ['Home', 'About', 'Conversations', 'Your rights'];
	const homeHref = $derived(page.url.pathname.split('/c/')[0] + scenarioQuery(page.url));
	const loginHref = $derived(page.url.pathname + scenarioQuery(page.url, { loggedIn: true }));

	let scrollY = $state(0);
	const scrolled = $derived(scrollY > 8);
</script>

<svelte:window bind:scrollY />

<!-- Links are resolve() plus the scenario query string, which this rule can't see. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->

<!-- White bar so each organisation's logo shows in its own colours. Once the page scrolls it
	turns see-through and blurs what passes underneath. Links are visual only. -->
<header
	class={cn(
		'sticky top-0 z-30 w-full border-b transition-[background-color,border-color,box-shadow] duration-300',
		scrolled
			? 'bg-background/70 border-border/60 shadow-sm backdrop-blur-xl backdrop-saturate-150'
			: 'bg-background border-transparent'
	)}
>
	<div
		class="mx-auto flex h-16 w-full max-w-[1300px] items-center justify-between gap-4 px-4 md:h-20 md:px-8"
	>
		{#if org.id === THEME_ORG_ID}
			<ComhairleLogo href={homeHref} surface="page" color="text-primary" ariaLabel="Home" />
		{:else}
			<a href={homeHref} class="flex min-w-0 items-center gap-3">
				<span
					class="bg-primary text-primary-foreground flex size-10 shrink-0 items-center justify-center rounded-lg text-base font-extrabold"
				>
					{org.initials}
				</span>
				<span class="truncate text-lg font-bold">{orgName}</span>
			</a>
		{/if}

		<nav aria-label="Main" class="hidden items-center gap-1 lg:flex">
			{#each LINKS as link, i (link)}
				<a
					href={i === 0 ? homeHref : undefined}
					class="text-foreground hover:bg-accent flex h-10 items-center rounded-full px-4 text-base font-medium"
					>{link}</a
				>
			{/each}
		</nav>

		<div class="flex shrink-0 items-center gap-2">
			{#if loggedIn}
				<span
					aria-label="Your account"
					class="bg-primary text-primary-foreground flex size-10 items-center justify-center rounded-full text-base font-bold"
				>
					{userInitials}
				</span>
			{:else}
				<a
					href={loginHref}
					class="text-foreground hidden h-10 items-center px-3 text-base font-semibold sm:flex"
					>Log in</a
				>
				<Button
					href={loginHref}
					variant="primaryDark"
					class="h-10 rounded-lg px-4 text-base font-semibold">Sign up</Button
				>
			{/if}
			<button
				aria-label="Menu"
				class="hover:bg-accent flex size-11 items-center justify-center rounded-full lg:hidden"
			>
				<Menu class="size-6" />
			</button>
		</div>
	</div>
</header>
