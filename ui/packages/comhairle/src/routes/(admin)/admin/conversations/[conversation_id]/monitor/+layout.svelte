<script lang="ts">
	import { resolve } from '$app/paths';
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import { cn } from '$lib/utils';
	import { key } from '$lib/utils/invalidationKey';
	import { afterNavigate, invalidate } from '$app/navigation';
	import { useInterval, IsDocumentVisible } from 'runed';
	import { formatDistance } from 'date-fns';
	import { MONITOR_PAGES, type MonitorRoute } from './sub-routes';
	import { Minute, Second } from '$lib/utils/units';

	let { children, params }: LayoutProps = $props();

	const isActivePage = (route: MonitorRoute) => page.route.id === route;

	const REFRESH_INTERVAL_MS = 3 * Minute;

	let lastUpdated = $state(new Date());
	let now = $state(new Date());

	// First page load and every sub page navigation runs load functions, so
	// both count as a data refresh
	afterNavigate(() => (lastUpdated = new Date()));

	const visible = new IsDocumentVisible();
	useInterval(REFRESH_INTERVAL_MS, {
		callback: async () => {
			// Don't poll if browser tab hidden
			if (!visible.current) return;
			await invalidate(key('admin/conversation/monitor'));
			lastUpdated = new Date();
		}
	});
	useInterval(30 * Second, { callback: () => (now = new Date()) });

	let updatedLabel = $derived(formatDistance(lastUpdated, now, { addSuffix: true }));
</script>

<svelte:head>
	<title>Monitor - Comhairle Admin</title>
</svelte:head>

<div class="flex flex-col gap-8 pb-16">
	<div class="flex flex-wrap items-center gap-4">
		<ul class="bg-muted w-fit max-w-full rounded-lg p-0.5" aria-label="Monitor sections">
			{#each MONITOR_PAGES as tab (tab.route)}
				<a
					href={resolve(tab.route, { conversation_id: params.conversation_id })}
					aria-current={isActivePage(tab.route) ? 'page' : undefined}
					class={cn(
						'text-muted-foreground inline-block rounded-md px-4 py-0.5 text-base',
						isActivePage(tab.route) &&
							'border-input bg-background text-foreground dark:bg-input/30 border'
					)}>{tab.label}</a
				>
			{/each}
		</ul>
		<span class="text-muted-foreground text-sm">Updated {updatedLabel}</span>
	</div>

	{@render children()}
</div>
