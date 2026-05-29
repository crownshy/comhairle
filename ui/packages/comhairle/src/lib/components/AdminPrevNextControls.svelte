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
		hidePrevLabel?: boolean;
	};
	let { prev, next, hidePrevLabel = false }: Props = $props();
</script>

<div class="flex min-w-0 flex-row gap-4">
	{#if prev}
		<div class="flex min-w-0 flex-row items-center gap-2">
			{#if !hidePrevLabel}
				<div class="flex min-w-0 flex-col items-end">
					<h3 class="text-primary text-xs">Previous</h3>
					<h3 class="max-w-[36ch] truncate" title={prev.name}>{prev.name}</h3>
				</div>
			{/if}

			<Button
				href={prev.url}
				class="shrink-0 rounded-full bg-white text-black ring-1 ring-[#E5E7EB]"
			>
				<ArrowLeft />
			</Button>
		</div>
	{/if}
	{#if prev && next}
		<Separator class="text-black" decorative orientation="vertical" />
	{/if}
	{#if next}
		<div class="flex min-w-0 flex-row items-center gap-2">
			<Button
				href={next.url}
				class="shrink-0 rounded-full bg-white text-black ring-1 ring-[#E5E7EB]"
			>
				<ArrowRight />
			</Button>
			<div class="flex min-w-0 flex-col items-start">
				<h3 class="text-primary text-xs">Next</h3>
				<h3 class="max-w-[36ch] truncate" title={next.name}>{next.name}</h3>
			</div>
		</div>
	{/if}
</div>
