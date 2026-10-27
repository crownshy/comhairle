<!--
Component to show the raw responses to long text questions
-->
<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { ChevronDown, ChevronUp } from 'lucide-svelte';

	const LIMIT = 4;

	interface Props {
		data: string[];
	}

	let { data: rawData }: Props = $props();

	let show = $state<boolean>(false);
	let data = $derived(rawData.slice(0, show ? undefined : LIMIT));
</script>

<section class="pt-2">
	<ul class="max-w-prose">
		{#each data as response, i (i)}
			<li class="border-b py-3 leading-relaxed last:border-b-0">{response}</li>
		{/each}
	</ul>
	{#if rawData.length > LIMIT}
		<Button
			onclick={() => (show = !show)}
			class="w-full rounded-t-none rounded-b-xl py-6"
			variant="secondary"
		>
			{#if show}
				<ChevronUp /> Show less
			{:else}
				<ChevronDown /> Show all {rawData.length} responses
			{/if}</Button
		>
	{/if}
</section>
