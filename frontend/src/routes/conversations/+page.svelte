<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card/index';
	import ConversationCard from '$lib/components/ConversationCard.svelte';

	import type { Conversation } from '$lib/mock_data';
	let { data }: { data: { conversations: Array<Conversation> } } = $props();
</script>

<header class="mb-20">
	<h1 class="mb-4 text-4xl font-bold">Conversations</h1>
	<p>
		Find open consultations that you can participate, or search topics that interest you the most.
	</p>
</header>

<div
	class="grid w-full auto-rows-auto grid-cols-[repeat(auto-fill,minmax(350px,1fr))] items-center gap-4"
>
	{#each data.conversations as conversation}
		<ConversationCard {conversation} />
	{/each}
</div>
