<script lang="ts">
	import { resolve } from '$app/paths';
	import type { ComponentType, Snippet } from 'svelte';
	import { BellRing, UserRound, type Icon } from 'lucide-svelte';
	import { page } from '$app/state';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<svelte:head>
	<title>Settings - Comhairle</title>
</svelte:head>

{#snippet MenuItem(Icon: ComponentType<Icon>, label: string, href: 'profile' | 'notifications')}
	{@const active = page.url.pathname.endsWith(href)}
	<li>
		<a
			class="mb-3 flex items-center gap-2 rounded-full p-2 pr-5 pl-3 transition-colors {active
				? 'bg-primary/30'
				: ''}"
			href={resolve(`/(public)/settings/${href}`)}
		>
			<Icon class="size-8 rounded-full p-2 {active ? 'bg-primary' : 'bg-slate-500'}" />
			<span class="text-l font-bold">{label}</span>
		</a>
	</li>
{/snippet}

<section class="my-10 grid grid-cols-[auto_1fr]">
	<nav class="mr-12">
		<span class="text-base font-semibold">Settings</span>
		<ul class="mt-5">
			{@render MenuItem(UserRound, 'Your profile', 'profile')}
			{@render MenuItem(BellRing, 'Notifications', 'notifications')}
		</ul>
	</nav>
	<div class="bg-card rounded-lg p-6">
		{@render children()}
	</div>
</section>
