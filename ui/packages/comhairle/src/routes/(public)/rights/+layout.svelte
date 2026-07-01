<script lang="ts">
	import { page } from '$app/state';
	import { cn } from '$lib/utils.js';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as m from '$lib/paraglide/messages';
	import type { LayoutProps } from './$types';

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

<div
	class="mx-auto flex w-full max-w-[1280px] flex-col gap-12 pt-10 pb-16 md:gap-20 md:pt-20 md:pb-28"
>
	<h1 class="text-foreground text-4xl font-bold">
		{m.your_rights()}
	</h1>

	<div class="flex flex-col gap-10 md:flex-row md:items-start md:gap-20">
		<nav class="flex w-full flex-col items-start gap-2 md:w-auto md:shrink-0">
			{#each sections as section (section.href)}
				<Button
					variant="ghost"
					size="sm"
					href={section.href}
					class={cn(
						'px-4 font-medium',
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
