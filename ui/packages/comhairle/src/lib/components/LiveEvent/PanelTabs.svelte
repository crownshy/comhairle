<script lang="ts">
	import type { Snippet } from 'svelte';
	import * as Tabs from '$lib/components/ui/tabs';
	import { cn } from '$lib/utils';
	import type { PanelTab } from './types';

	type Props = {
		activeTab: PanelTab;
		showTabs: boolean;
		onTabChange: (tab: PanelTab) => void;
		listClass?: string;
		triggerClass?: string;
		contentClass?: string;
		children: Snippet;
	};

	let {
		activeTab,
		showTabs,
		onTabChange,
		listClass,
		triggerClass,
		contentClass,
		children
	}: Props = $props();

	const TABS: { value: PanelTab; label: string }[] = [
		{ value: 'agenda', label: 'Agenda' },
		{ value: 'breakoutRooms', label: 'Breakout Session' }
	];

	const TRIGGER_CLASS =
		'text-muted-foreground data-[state=active]:text-foreground dark:data-[state=active]:bg-background dark:data-[state=active]:border-transparent h-auto rounded-xl px-3 font-semibold transition-all';
</script>

<!-- One panel follows the active tab, so the content stays mounted when the tabs come and go
	(the host's unsaved agenda edits live in it). Without tabs it drops the tabpanel role. -->
<Tabs.Root
	value={activeTab}
	onValueChange={(value) => onTabChange(value as PanelTab)}
	class="contents"
>
	{#if showTabs}
		<Tabs.List class={cn('flex h-auto w-full items-center', listClass)}>
			{#each TABS as tab (tab.value)}
				<Tabs.Trigger value={tab.value} class={cn(TRIGGER_CLASS, triggerClass)}>
					{tab.label}
				</Tabs.Trigger>
			{/each}
		</Tabs.List>
	{/if}
	<Tabs.Content value={activeTab}>
		{#snippet child({ props })}
			<div {...showTabs ? props : {}} class={contentClass}>
				{@render children()}
			</div>
		{/snippet}
	</Tabs.Content>
</Tabs.Root>
