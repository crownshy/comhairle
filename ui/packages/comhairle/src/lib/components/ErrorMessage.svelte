<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
		size?: 'sm' | 'md' | 'lg' | 'xl';
	}

	let { children, size = 'md' }: Props = $props();
</script>

<span class="text-destructive font-semibold text-{size}" aria-live="polite">
	{@render children()}
</span>
