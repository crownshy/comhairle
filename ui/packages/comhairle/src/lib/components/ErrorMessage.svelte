<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<span class="text-destructive font-semibold" aria-live="polite">
	{@render children()}
</span>
