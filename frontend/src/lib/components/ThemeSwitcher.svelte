<script lang="ts">
	/**
	 * ThemeSwitcher Component
	 *
	 * A UI component for switching between available themes.
	 * Can be placed in navigation, settings page, or anywhere else.
	 *
	 * @example
	 * <ThemeSwitcher />
	 * <ThemeSwitcher variant="buttons" />
	 * <ThemeSwitcher variant="compact" showLabel={false} />
	 */

	import {
		themeStore,
		setTheme,
		availableThemes,
		type BuiltInTheme
	} from '$lib/stores/theme';

	interface Props {
		variant?: 'dropdown' | 'buttons' | 'compact';
		showLabel?: boolean;
	}

	let { variant = 'dropdown', showLabel = true }: Props = $props();

	let isOpen = $state(false);

	function handleThemeChange(newTheme: BuiltInTheme) {
		setTheme(newTheme);
		isOpen = false;
	}

	function toggleDropdown() {
		isOpen = !isOpen;
	}

	// Close dropdown when clicking outside
	function handleClickOutside(event: MouseEvent) {
		const target = event.target as HTMLElement;
		if (!target.closest('.theme-switcher-dropdown')) {
			isOpen = false;
		}
	}

	$effect(() => {
		if (isOpen && typeof document !== 'undefined') {
			document.addEventListener('click', handleClickOutside);
			return () => document.removeEventListener('click', handleClickOutside);
		}
	});

	const currentThemeData = $derived(availableThemes.find((t) => t.id === $themeStore));
</script>

{#if variant === 'dropdown'}
	<div class="theme-switcher-dropdown relative">
		<button
			onclick={toggleDropdown}
			class="w-full rounded-md border border-input bg-background px-4 py-2 text-left hover:bg-accent focus:outline-none focus:ring-2 focus:ring-ring"
		>
			<div class="flex items-center justify-between">
				<div class="flex items-center gap-2">
					<div
						class="h-4 w-4 rounded-full border border-border"
						style="background-color: {currentThemeData?.primaryColor};"
					></div>
					<span class="text-sm">{currentThemeData?.name}</span>
				</div>
				<svg
					class="h-4 w-4 transition-transform"
					class:rotate-180={isOpen}
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"
					></path>
				</svg>
			</div>
		</button>

		{#if isOpen}
			<div
				class="absolute z-50 mt-2 w-full rounded-md border border-border bg-popover shadow-lg"
			>
				{#each availableThemes as themeOption}
					<button
						onclick={() => handleThemeChange(themeOption.id as BuiltInTheme)}
						class="flex w-full items-center gap-3 px-4 py-2 text-left hover:bg-accent"
						class:bg-accent={$themeStore === themeOption.id}
					>
						<div
							class="h-4 w-4 rounded-full border border-border"
							style="background-color: {themeOption.primaryColor};"
						></div>
						<div class="flex-1">
							<div class="text-sm font-medium">{themeOption.name}</div>
							<div class="text-xs text-muted-foreground">{themeOption.description}</div>
						</div>
						{#if $themeStore === themeOption.id}
							<svg class="h-4 w-4 text-primary" fill="currentColor" viewBox="0 0 20 20">
								<path
									fill-rule="evenodd"
									d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
									clip-rule="evenodd"
								></path>
							</svg>
						{/if}
					</button>
				{/each}
			</div>
		{/if}
	</div>
{:else if variant === 'buttons'}
	<div>
		{#if showLabel}
			<label class="mb-2 block text-sm font-medium text-foreground">Theme</label>
		{/if}
		<div class="grid grid-cols-2 gap-2 md:grid-cols-3">
			{#each availableThemes as themeOption}
				<button
					onclick={() => handleThemeChange(themeOption.id as BuiltInTheme)}
					class="rounded-md border p-3 text-left transition-all hover:shadow-md {$themeStore === themeOption.id ? 'border-primary bg-primary/5' : 'border-border'}"
				>
					<div class="mb-1 flex items-center gap-2">
						<div
							class="h-4 w-4 rounded-full border border-border"
							style="background-color: {themeOption.primaryColor};"
						></div>
						<span class="text-sm font-medium">{themeOption.name}</span>
					</div>
					<p class="text-xs text-muted-foreground">
						{themeOption.description}
					</p>
				</button>
			{/each}
		</div>
	</div>
{:else if variant === 'compact'}
	<div class="flex items-center gap-2">
		{#if showLabel}
			<span class="text-sm text-muted-foreground">Theme:</span>
		{/if}
		<select
			onchange={(e) => handleThemeChange(e.currentTarget.value as BuiltInTheme)}
			value={$themeStore}
			class="rounded-md border border-input bg-background px-3 py-1 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
		>
			{#each availableThemes as themeOption}
				<option value={themeOption.id}>{themeOption.name}</option>
			{/each}
		</select>
	</div>
{/if}
