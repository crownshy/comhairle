<script lang="ts">
	import { themeStore } from '$lib/stores/theme.svelte';
	import Moon from '@lucide/svelte/icons/moon';
	import Sun from '@lucide/svelte/icons/sun';
	import { Button } from '$lib/components/ui/form';

	function toggleMode() {
		themeStore.toggleMode();
	}
</script>

<Button onclick={toggleMode} variant="ghost" class="w-full justify-start">
	{#if themeStore.isDark}
		<Sun class="h-4 w-4" />
		Light Mode
	{:else}
		<Moon class="h-4 w-4" />
		Dark Mode
	{/if}
</Button>
