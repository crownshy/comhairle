<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<span class="text-destructive pt-5 font-semibold" aria-live="polite">
	{@render children()}
</span>
