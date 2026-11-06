<script lang="ts">
	import type { Snippet } from 'svelte';

	type Props = {
		children: Snippet;
	};

	let { children }: Props = $props();
</script>

<svelte:head>
	<title>Demographic forms - Comhairle Admin</title>
</svelte:head>

{@render children()}
