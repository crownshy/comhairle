<script lang="ts">
	import type { Snippet } from 'svelte';
	import PanelTabs from './PanelTabs.svelte';
	import type { PanelTab } from './types';

	interface Props {
		activeTab: PanelTab;
		showTabs: boolean;
		onTabChange: (tab: PanelTab) => void;
		children: Snippet;
	}

	let { activeTab, showTabs, onTabChange, children }: Props = $props();
</script>

<div
	class="bg-muted flex h-full flex-col overflow-hidden rounded-2xl shadow-lg {showTabs
		? ''
		: 'pt-5'}"
>
	<PanelTabs
		{activeTab}
		{showTabs}
		{onTabChange}
		listClass="bg-muted-foreground/10 m-5 w-auto rounded-2xl p-1.5"
		triggerClass="h-10 py-2 text-sm"
		contentClass="flex flex-1 flex-col overflow-y-auto"
	>
		{@render children()}
	</PanelTabs>
</div>
