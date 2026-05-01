<script lang="ts">
	import type { Snippet } from 'svelte';
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
	<!-- Tab switcher -->
	{#if showTabs}
		<div class="px-5 pt-2.5 pb-2.5">
			<div class="flex h-12 items-center rounded-xl p-1">
				<button
					class="flex flex-1 items-center justify-center rounded-full px-3 py-2 text-sm transition-all {activeTab ===
					'agenda'
						? 'bg-background text-foreground ring-foreground/5 font-bold shadow-[0px_1px_2px_-1px_rgba(0,0,0,0.10),0px_1px_3px_0px_rgba(0,0,0,0.10)] ring-1'
						: 'text-muted-foreground font-medium'}"
					onclick={() => onTabChange('agenda')}
				>
					Agenda
				</button>
				<button
					class="flex flex-1 items-center justify-center rounded-full px-3 py-2 text-sm transition-all {activeTab ===
					'breakoutRooms'
						? 'bg-background text-foreground ring-foreground/5 font-bold shadow-[0px_1px_2px_-1px_rgba(0,0,0,0.10),0px_1px_3px_0px_rgba(0,0,0,0.10)] ring-1'
						: 'text-muted-foreground font-medium'}"
					onclick={() => onTabChange('breakoutRooms')}
				>
					Breakout Session
				</button>
			</div>
		</div>
	{/if}

	<!-- Panel content -->
	<div class="flex flex-1 flex-col overflow-y-auto">
		{@render children()}
	</div>
</div>
