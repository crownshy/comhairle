<script lang="ts">
	import type { Snippet } from 'svelte';
	type Props = {
		children?: Snippet;
	};
	let { children }: Props = $props();
</script>

<div class="rounded-[10px] border border-gray-300 p-10">
	{#if children}
		{@render children()}
	{/if}
</div>
