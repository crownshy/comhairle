<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<section class="pt-page-top px-gutter grow pb-8 sm:pr-8 sm:pb-12 lg:pr-16">
	{@render children()}
</section>
