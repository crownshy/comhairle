<script lang="ts">
	import type { Conversation } from '$lib/mock_data';
	import * as Card from '$lib/components/ui/card/index';
	import { Button } from '$lib/components/ui/button';
	let { conversation }: { conversation: Conversation } = $props();
</script>

<Card.Root class="h-[400px] w-[350px]">
	<Card.Header>
		<Card.Title>{conversation.name}</Card.Title>
		<Card.Description>{conversation.short_text}</Card.Description>
	</Card.Header>
	<Card.Content>
		<form>
			<div class="grid w-full items-center gap-4">
				<img src={conversation.banner_image} alt={conversation.short_text} />
			</div>
		</form>
	</Card.Content>
	<Card.Footer class="flex justify-between">
		{#if conversation.is_active}
			<Button href={`/conversations/${conversation.id}/`}>Take Part</Button>
		{:else}
			<Button href={`/conversations/${conversation.id}/report`}>View Report</Button>
		{/if}
	</Card.Footer>
</Card.Root>
