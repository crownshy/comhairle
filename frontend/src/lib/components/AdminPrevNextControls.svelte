<script lang="ts">
	import { ArrowLeft, ArrowRight } from 'lucide-svelte';
	import { Button } from './ui/button';
	import { Separator } from '$lib/components/ui/separator/index.js';

	type NamedLink = {
		name: string;
		url: string;
	};
	type Props = {
		prev?: NamedLink;
		next?: NamedLink;
	};
	let { prev, next }: Props = $props();
</script>

<div class="flex flex-row gap-4">
	{#if prev}
		<div class="flex flex-row items-center gap-2">
			<div class="flex flex-col items-end">
				<h3 class="text-primary text-xs">Previous</h3>
				<h3>{prev.name}</h3>
			</div>

			<Button href={prev.url} class="rounded-full bg-white text-black ring-1 ring-[#E5E7EB] "
				><ArrowLeft /></Button
			>
		</div>
	{/if}
	{#if prev && next}
		<Separator class="text-black" decorative orientation="vertical" />
	{/if}
	{#if next}
		<div class="flex flex-row items-center gap-2">
			<Button href={next.url} class="rounded-full bg-white text-black ring-1 ring-[#E5E7EB] "
				><ArrowRight /></Button
			>
			<div class="flex flex-col items-start">
				<h3 class="text-primary text-xs">Next</h3>
				<h3>{next.name}</h3>
			</div>
		</div>
	{/if}
</div>
