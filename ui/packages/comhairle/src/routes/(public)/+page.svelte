<script lang="ts">
	import * as m from '$lib/paraglide/messages';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { Info } from 'lucide-svelte';

	let { data } = $props();
	let { isCommunity } = data;

	let topPadding = $derived(isCommunity ? 'pt-6 sm:pt-36' : 'pt-26 sm:pt-50');
</script>

<svelte:head>
	<title>Comhairle</title>
</svelte:head>

<div class={`flex h-full flex-col items-center ${topPadding}`}>
	<div class="my-auto flex flex-col items-center justify-center gap-y-10 p-6 sm:p-10">
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
						Means "Consult" in Gaelic
					</Tooltip.Content>
				</Tooltip.Root>
			</Tooltip.Provider>

			{#if isCommunity}
				{m.community_server()}
			{/if}
		</h1>

		{#if isCommunity}
			{#each [m.community_server_description_1(), m.community_server_description_2()] as description}
				<p
					class="text-subtle-foreground w-full text-left text-xl font-semibold sm:text-2xl"
				>
					{description}
				</p>
			{/each}
		{:else}
			<p
				class="text-subtle-foreground max-w-xl text-center text-xl font-semibold sm:text-2xl"
			>
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
