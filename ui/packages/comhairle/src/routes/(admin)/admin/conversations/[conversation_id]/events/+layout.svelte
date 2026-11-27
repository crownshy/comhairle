<script lang="ts">
	import TabContent from '$lib/components/TabContent.svelte';
	import type { Snippet } from 'svelte';

	type Props = {
		children: Snippet;
	};

	let { children }: Props = $props();
</script>

<TabContent>
	{@render children()}
</TabContent>
