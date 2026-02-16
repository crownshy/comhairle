<script lang="ts">
	import { themeStore } from '$lib/stores/theme.svelte';
	import type { ThemeName } from '$lib/types/theme';
	import { Moon, Sun } from 'lucide-svelte';

	const themes: Array<{ name: ThemeName; label: string }> = [
		{ name: 'comhairle', label: 'Comhairle' },
		{ name: 'bloom', label: 'Bloom' },
		{ name: 'scotgov', label: 'Scottish Gov' }
	];

	function handleThemeChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		themeStore.setTheme(target.value as ThemeName);
	}
</script>

<div class="flex items-center gap-4">
	<div class="flex items-center gap-2">
		<label for="theme-select" class="text-sm font-medium text-foreground">Theme:</label>
		<select
			id="theme-select"
			value={themeStore.name}
			onchange={handleThemeChange}
			class="rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground shadow-sm focus:outline-none focus:ring-2 focus:ring-ring"
		>
			{#each themes as theme}
				<option value={theme.name}>{theme.label}</option>
			{/each}
		</select>
	</div>

	<button
		onclick={() => themeStore.toggleMode()}
		class="rounded-md border border-input bg-background p-2 text-foreground shadow-sm transition-colors hover:bg-accent hover:text-accent-foreground focus:outline-none focus:ring-2 focus:ring-ring"
		aria-label={themeStore.isDark ? 'Switch to light mode' : 'Switch to dark mode'}
	>
		{#if themeStore.isDark}
			<Sun class="h-5 w-5" />
		{:else}
			<Moon class="h-5 w-5" />
		{/if}
	</button>
</div>
