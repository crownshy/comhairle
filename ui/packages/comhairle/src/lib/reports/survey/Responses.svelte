<!--
Component to show the raw responses to long text questions
-->
<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { ChevronDown, ChevronUp } from 'lucide-svelte';

	interface Props {
		data: string[];
	}

	let { data: rawData }: Props = $props();

	let show = $state<boolean>(false);
	let data = $derived(rawData.slice(0, show ? undefined : 10));
</script>

<section class="flex flex-col">
	<ul class="{show ? 'overflow-scroll' : 'overflow-auto'} {show ? 'max-h-300' : 'max-h-none'}">
		{#each data as response, i (i)}
			<li class="mb-4 border-b pb-4 last:border-b-0">{response}</li>
		{/each}
	</ul>
	<Button onclick={() => (show = !show)}>
		{#if show}
			<ChevronUp /> Show less
		{:else}
			<ChevronDown /> Show all {rawData.length} responses
		{/if}</Button
	>
</section>
