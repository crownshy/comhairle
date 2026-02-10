<script lang="ts">
	import { LocalizedConversationDto } from '$lib/api/api';
	import { apiClient } from '$lib/api/client';
	import Spinner from './ui/spinner/spinner.svelte';

	type Props = {
		conversation_id: string;
	};

	let { conversation_id }: Props = $props();

	let conversation: LocalizedConversationDto | undefined = $state();

	$effect(() => {
		apiClient.GetConversation({ params: { conversation_id } }).then((convo) => {
			conversation = convo;
		});
	});
</script>

<div class="">
	{#if conversation}
		<div class="relative mx-auto mt-20 max-w-xl">
			<img
				alt={conversation.title}
				class="h-64 w-full rounded-md object-cover"
				src={conversation.imageUrl}
			/>
			<div class="absolute inset-0 rounded-md bg-gray-700 opacity-60"></div>
			<div class="absolute inset-0 flex items-center justify-center">
				<h2 class="text-3xl font-bold text-white">conversation.title</h2>
			</div>
		</div>
		<div></div>
	{:else}
		<Spinner />
	{/if}
</div>
