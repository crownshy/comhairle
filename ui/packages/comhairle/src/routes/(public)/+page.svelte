<script lang="ts">
	import { m } from '$lib/paraglide/messages';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { Info } from 'lucide-svelte';

	let { data } = $props();
	const isCommunity = $derived(data.isCommunity);
</script>

<svelte:head>
	<title>Comhairle</title>
</svelte:head>

<!-- Centred rather than pushed down by a fixed top padding, which grows with browser zoom
	while the viewport shrinks and drops the text below the fold. -->
<div class="flex min-h-[80vh] flex-col items-center justify-center">
	<div class="flex flex-col items-center justify-center gap-y-10 p-6 sm:p-10">
		<h1
			class="text-foreground scroll-m-20 text-4xl font-bold tracking-tight sm:text-5xl {isCommunity
				? 'text-left'
				: 'text-center'}"
		>
			{#if isCommunity}
				{m.welcome_to_the()}
			{:else}
				{m.welcome_to()}
			{/if}

			<Tooltip.Provider delayDuration={0}>
				<Tooltip.Root>
					<Tooltip.Trigger
						class="inline-flex cursor-default items-baseline gap-2"
						onclick={(e) => e.preventDefault()}
					>
						Comhairle<Info class="text-primary inline size-5 self-center" />
					</Tooltip.Trigger>
					<Tooltip.Content side="top" class="text-base">
						{m.comhairle_meaning()}
					</Tooltip.Content>
				</Tooltip.Root>
			</Tooltip.Provider>

			{#if isCommunity}
				{m.community_server()}
			{/if}
		</h1>

		{#if isCommunity}
			{#each [m.community_server_description_1(), m.community_server_description_2()] as description (description)}
				<p class="text-foreground w-full text-left text-xl sm:text-2xl">
					{description}
				</p>
			{/each}
		{:else}
			<p class="text-foreground max-w-xl text-center text-xl sm:text-2xl">
				{m.landing_page_subtitle()}
			</p>
		{/if}

		<Button
			size="lg"
			class="bg-sidebar text-sidebar-foreground px-8 py-6 text-lg {isCommunity
				? 'sm:self-start'
				: ''}"
			href="/conversations"
		>
			{m.explore_conversations()}
		</Button>
	</div>
</div>
