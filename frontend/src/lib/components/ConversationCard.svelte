<script lang="ts">
	import type { Conversation } from '$lib/mock_data';
	import * as Card from '$lib/components/ui/card/index';
	import { Button } from '$lib/components/ui/button';
	let { conversation }: { conversation: Conversation } = $props();
</script>

<Card.Root class="flex h-[400px] w-[350px] flex-col">
	<Card.Header class="grow">
		<Card.Title class="truncate">{conversation.title}</Card.Title>
		<Card.Description class="line-clamp-4">{conversation.description}</Card.Description>
	</Card.Header>
	<Card.Content class="grow">
		<img class="m-auto h-32" src={conversation.image_url} alt={conversation.title} />
	</Card.Content>
	<Card.Footer class="flex grow-0 justify-between">
		{#if conversation.is_complete}
			<Button href={`/conversations/${conversation.id}/report`}>View Report</Button>
		{:else}
			<Button href={`/conversations/${conversation.id}/`}>Take Part</Button>
		{/if}
	</Card.Footer>
</Card.Root>
