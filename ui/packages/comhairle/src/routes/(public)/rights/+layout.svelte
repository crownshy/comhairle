<script lang="ts">
	import { page } from '$app/state';
	import { cn } from '$lib/utils.js';
	import Button from '$lib/components/ui/button/button.svelte';
	import { m } from '$lib/paraglide/messages';
	import type { LayoutProps } from './$types';
	import PageHero from '$lib/components/PageHero.svelte';

	let { children }: LayoutProps = $props();

	const sections = [
		{ href: '/rights/tos', label: m.terms_of_service() },
		{ href: '/rights/privacy', label: m.privacy_policy() },
		{ href: '/rights/cookies', label: m.cookies_settings() },
		{ href: '/rights/accessibility', label: m.accessibility() }
	];

	const isActive = (href: string) =>
		page.url.pathname === href || page.url.pathname.startsWith(href + '/');
</script>

<PageHero compact>
	<h1 class="text-[1.75rem] leading-tight font-bold sm:text-4xl lg:text-[2.625rem]">
		{m.your_rights()}
	</h1>
</PageHero>

<div class="mx-auto w-full max-w-[1300px] px-4 pt-12 pb-16 md:px-8 md:pt-16 md:pb-28">
	<div class="mx-auto flex w-full max-w-6xl flex-col gap-10 md:flex-row md:items-start md:gap-20">
		<nav
			aria-label={m.your_rights()}
			class="flex w-full flex-col items-start gap-2 md:sticky md:top-32 md:w-auto md:shrink-0"
		>
			{#each sections as section (section.href)}
				<Button
					variant="ghost"
					size="sm"
					href={section.href}
					class={cn(
						'px-4 text-base font-medium',
						isActive(section.href) && 'bg-accent text-accent-foreground'
					)}
				>
					{section.label}
				</Button>
			{/each}
		</nav>

		<div class="min-w-0 flex-1">
			{@render children()}
		</div>
	</div>
</div>
