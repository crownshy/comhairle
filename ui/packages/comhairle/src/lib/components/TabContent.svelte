<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();
</script>

<section class="pt-page-top px-gutter grow pb-8 sm:pb-12">
	<div class="max-w-admin mx-auto w-full">
		{@render children()}
	</div>
</section>
