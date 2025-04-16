<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Label } from '$lib/components/ui/label';
	import { Switch } from '$lib/components/ui/switch';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '$lib/api/client';
	import { invalidateAll } from '$app/navigation';

	let { data } = $props();
	let local_conversation = $derived(data.conversation);

	let conversation = $state(local_conversation);

	$effect(() => {
		conversation = local_conversation;
	});

	$effect(() => {
		apiClient
			.UpdateConversation(conversation, { params: { conversation_id: conversation.id } })
			.then((response) => {
				console.log(response);
			})
			.catch((e) => {
				notifications.send({ message: 'Failed to save changes' });
				console.warn(e);
			});
	});
</script>

<div class="flex flex-col gap-4">
	<div>
		<Label for="title">Title</Label>
		<Input bind:value={conversation.title} name="title" />
	</div>

	<div>
		<Label for="short_description">Short Description</Label>
		<Textarea bind:value={conversation.short_description} name="title" />
	</div>
	<div>
		<Label for="description">Description</Label>
		<Textarea bind:value={conversation.description} name="description" />
	</div>

	<div class="flex flex-row gap-4">
		<div class="grow">
			<Label for="image">Banner Image</Label>
			<Input bind:value={conversation.image_url} name="image_url" />
		</div>
		<img width="200px" alt="Conversation Banner" src={conversation.image_url} />
	</div>

	<div>
		<Label for="is_public">Show conversation publically</Label>
		<Switch bind:checked={conversation.is_public} name="is_public" />
	</div>

	<div>
		<Label for="is_invite_only">Only allow participation by invite</Label>
		<Switch bind:checked={conversation.is_invite_only} name="is_invite_only" />
	</div>
</div>
