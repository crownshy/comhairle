<script lang="ts">
	import type { Snippet } from 'svelte';

	type Props = {
		Header: Snippet;
		Body: Snippet;
	};

	let { Header, Body }: Props = $props();
</script>

<header class="border-border flex w-full justify-between border-b pt-6 pb-5">
	{@render Header()}
</header>

<section class="pt-page-top h-full px-5">
	{@render Body()}
</section>
