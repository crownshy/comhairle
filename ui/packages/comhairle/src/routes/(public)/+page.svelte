<script lang="ts">
	import * as m from '$lib/paraglide/messages';
	import Button from '$lib/components/ui/button/button.svelte';

	let { data } = $props();
	let { isCommunity } = data;
</script>

<div class="flex h-full flex-col items-center justify-around pt-50">
	<div class="my-auto flex flex-col items-center justify-center gap-y-10 p-10">
		<h1
			class="scroll-m-20 text-center text-4xl font-bold tracking-tight md:text-left lg:text-5xl"
		>
			{#if isCommunity}
				{m.welcome_to_comhairle_community_server()}
			{:else}
				{m.welcome_to_comhairle()}
			{/if}
		</h1>
		{#if isCommunity}
			<p class="w-full text-center text-2xl md:text-left">
				{m.community_server_description_1()}
			</p>
			<p class="w-full text-center text-2xl md:text-left">
				{m.community_server_description_2()}
			</p>
		{:else}
			<p class="text-center text-2xl md:text-left">{m.a_platfrom_about_what_people_want()}</p>
		{/if}
		<div class="flex flex-row gap-2">
			<Button size="lg" class="bg-sidebar text-sidebar-foreground" href="/conversations"
				>{m.explore_conversations()}</Button
			>
		</div>
	</div>
</div>
