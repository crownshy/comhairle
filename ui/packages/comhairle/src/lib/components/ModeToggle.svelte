<script lang="ts">
	import { themeStore } from '$lib/stores/theme.svelte';
	import { Moon, Sun } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/form';

	type Props = {
		iconOnly?: boolean;
		class?: string;
	};
	let { iconOnly = false, class: className = '' }: Props = $props();

	function toggleMode() {
		themeStore.toggleMode();
	}

	let label = $derived(themeStore.isDark ? 'Light Mode' : 'Dark Mode');
</script>

{#if iconOnly}
	<Button
		onclick={toggleMode}
		variant="nav"
		size="icon"
		aria-label={label}
		title={label}
		class={`rounded-full hover:bg-white/10 ${className}`}
	>
		{#if themeStore.isDark}
			<Sun class="h-5 w-5" />
		{:else}
			<Moon class="h-5 w-5" />
		{/if}
	</Button>
{:else}
	<Button onclick={toggleMode} variant="ghost" class={`w-full justify-start ${className}`}>
		{#if themeStore.isDark}
			<Sun class="h-4 w-4" />
			Light Mode
		{:else}
			<Moon class="h-4 w-4" />
			Dark Mode
		{/if}
	</Button>
{/if}
