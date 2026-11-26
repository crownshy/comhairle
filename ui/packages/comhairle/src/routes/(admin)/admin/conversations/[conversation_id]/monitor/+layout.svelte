<script lang="ts">
	import { resolve } from '$app/paths';
	import type { LayoutProps } from './$types';
	import { page } from '$app/state';
	import { cn } from '$lib/utils';

	let { children, data }: LayoutProps = $props();

	const MONITOR_PAGES = [
		{ value: 'overview', label: 'At a glance' },
		{ value: 'pulse', label: 'Live pulse' },
		{ value: 'journey', label: 'Journey' },
		{ value: 'recruitment', label: 'Recruitment' }
	] as const;
	type MonitorTab = (typeof MONITOR_PAGES)[number]['value'];

	const isActivePage = (segment: MonitorTab) => page.url.pathname.endsWith('/' + segment);
</script>

<svelte:head>
	<title>Monitor prototype - Comhairle Admin</title>
</svelte:head>

<div class="flex flex-col gap-8 pb-16">
	<div class="flex flex-wrap items-center gap-4">
		<ul class="bg-muted w-fit max-w-full rounded-lg p-0.5" aria-label="Monitor sections">
			{#each MONITOR_PAGES as tab (tab.value)}
				<a
					href={resolve(
						`/(admin)/admin/conversations/[conversation_id]/monitor/${tab.value}`,
						{ conversation_id: data.conversationId }
					)}
					class={cn(
						'text-muted-foreground inline-block rounded-md px-4 py-0.5 text-base',
						isActivePage(tab.value) &&
							'border-input bg-background text-foreground dark:bg-input/30 border'
					)}>{tab.label}</a
				>
			{/each}
		</ul>
		<!-- TODO: time of the last data refresh. -->
		<span class="text-muted-foreground text-sm">Updated 2 minutes ago</span>
	</div>

	{@render children()}
</div>
