<script lang="ts">
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<svelte:head>
	<title>{data.topic.title} - Comhairle Admin Guide</title>
</svelte:head>

<main class="bg-muted w-full max-w-6xl min-w-0 rounded-xl p-6">
	<h1 class="text-primary text-4xl font-bold">
		{data.topic.title}
	</h1>

	<p class="text-foreground mt-6 max-w-3xl text-lg leading-8 whitespace-pre-line">
		{data.topic.content}
	</p>
</main>
