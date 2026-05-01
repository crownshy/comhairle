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
		<div class="p-5">
			<div class="bg-muted-foreground/10 flex items-center rounded-2xl p-1.5">
				<button
					class="flex h-10 flex-1 items-center justify-center rounded-2xl px-3 py-2 text-sm transition-all {activeTab ===
					'agenda'
						? 'bg-background text-foreground font-semibold shadow-sm'
						: 'text-muted-foreground font-semibold'}"
					onclick={() => onTabChange('agenda')}
				>
					Agenda
				</button>
				<button
					class="flex h-10 flex-1 items-center justify-center rounded-2xl px-3 py-2 text-sm transition-all {activeTab ===
					'breakoutRooms'
						? 'bg-background text-foreground font-semibold shadow-sm'
						: 'text-muted-foreground font-semibold'}"
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
