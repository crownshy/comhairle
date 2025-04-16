<script lang="ts">
	import Logo from '$lib/assets/comhairle_logo.png';
	import * as Avatar from '$lib/components/ui/avatar';
	import { userInitals } from '$lib/utils';
	import * as Command from '$lib/components/ui/command/index.js';
	import Plus from 'lucide-svelte/icons/plus';
	import MessageCircle from 'lucide-svelte/icons/message-circle';

	let props = $props();
	let user = $derived(props.user);
	let conversations = $derived(props.conversations);

	let user_initals = $derived(userInitals(user?.username));
	// TODO We need to use data-sveltekit-reload as the
	// component isn't relaoading on navigation when we use
	// page.ts for the data fetching
</script>

<nav class="w-2xl flex h-full flex-col bg-[#E4E4E7]">
	<div class="flex flex-row items-center gap-4 p-4">
		<img src={Logo} alt="Comhairle Logo" />
		<h1 class="text-xl font-bold">Comhairle</h1>
	</div>
	<div class="mt-12 flex flex-col items-center">
		<Avatar.Root class="mb-4 h-40 w-40">
			{#if user.avatar_url}
				<Avatar.Image src={user.avatar_url} alt="@shadcn" />
			{/if}
			<Avatar.Fallback>{user_initals}</Avatar.Fallback>
		</Avatar.Root>
		<h2 class="text-2xl capitalize">{user.username}</h2>
	</div>
	<hr class="my-4 border-gray-300 dark:border-white" />
	<Command.Root class="max-w-[450px] grow rounded-lg border bg-[#E4E4E7] ">
		<Command.Input placeholder="Type a command or search..." />

		<Command.List>
			<Command.Empty>No results found.</Command.Empty>
			<Command.Group heading="Conversations">
				{#if conversations}
					{#each conversations.records as conversation}
						<Command.Item class="flex flex-row">
							<a href={`/admin/conversations/${conversation.id}/landing`} class="flex flex-row">
								<MessageCircle class="mr-2" />
								<span>{conversation.title}</span>
							</a>
						</Command.Item>
					{/each}
				{/if}
				<Command.Item>
					<a href="/admin/conversations/new" class="flex flex-row">
						<Plus class="mr-2" />
						<span>New Conversation</span>
					</a>
				</Command.Item>
			</Command.Group>
			<Command.Separator />
			<Command.Group heading="Settings">
				<Command.Item>
					<span>Settings</span>
				</Command.Item>
			</Command.Group>
		</Command.List>
	</Command.Root>
</nav>
