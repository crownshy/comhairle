<script lang="ts">
	/** Portable entry point. No imports from @crownshy/api-client or from anywhere comhairle-specific. Routes to the right mode and lets each mode pull the adapter + step context from context. */
	import Manage from './modes/Manage.svelte';
	import User from './modes/User.svelte';
	import Report from './modes/Report.svelte';
	import type { Mode } from './types';

	let { mode }: { mode: Mode } = $props();
</script>

{#if mode === 'manage'}
	<Manage />
{:else if mode === 'user'}
	<User />
{:else}
	<Report />
{/if}
