<script lang="ts">
	import {
		Collapsible,
		CollapsibleTrigger,
		CollapsibleContent
	} from '$lib/components/ui/collapsible';
	import ChevronDown from '@lucide/svelte/icons/chevron-down';
	import type { Snippet } from 'svelte';

	interface Props {
		id?: string;
		title: string;
		subtitle?: string;
		open?: boolean;
		children: Snippet;
	}

	let { id, title, subtitle, open = $bindable(true), children }: Props = $props();
</script>

<div
	{id}
	class="bg-card border-primary w-full max-w-[1200px] scroll-mt-4 border-t-4 px-5 py-8 md:px-24 md:py-12"
>
	<Collapsible bind:open>
		<CollapsibleTrigger class=" group flex w-full items-center justify-between gap-4 md:gap-6">
			<h2
				class="text-card-foreground text-2xl leading-8 font-semibold md:text-4xl md:leading-[48px]"
			>
				{title}
			</h2>
			<ChevronDown
				class="text-foreground h-6 w-6 shrink-0 transition-transform group-data-[state=open]:rotate-180"
			/>
		</CollapsibleTrigger>
		<CollapsibleContent>
			{#if subtitle}
				<p class="text-primary mt-4 text-xl leading-6 font-semibold">{subtitle}</p>
			{/if}
			<div class="mt-6 flex flex-col gap-6">
				{@render children()}
			</div>
		</CollapsibleContent>
	</Collapsible>
</div>
