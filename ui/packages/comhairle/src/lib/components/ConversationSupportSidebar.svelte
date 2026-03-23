<script lang="ts">
	import * as Drawer from '$lib/components/ui/drawer';
	import * as Tabs from '$lib/components/ui/tabs';
	import { LucideChevronRight } from 'lucide-svelte';
	import CircleQuestionMark from '$lib/components/icons/CircleQuestionMark.svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import CollapsibleChat from '$lib/components/Chatbot/CollapsibleChat.svelte';
	import type { LocalizedConversationDto, UserDto } from '@crownshy/api-client/api';
	import ComhairlePrivacyPolicy from './ComhairlePrivacyPolicy.svelte';
	import ComhairleFAQs from './ComhairleFAQs.svelte';

	let { conversation, user }: { conversation: LocalizedConversationDto; user: UserDto } =
		$props();

	let tabs = [
		{
			value: 'faqs',
			label: 'FAQs',
			content: conversation.faqs,
			fallback: ComhairleFAQs
		},
		{
			value: 'privacyPolicy',
			label: 'Privacy Policy',
			content: conversation.privacyPolicy,
			fallback: ComhairlePrivacyPolicy
		}
	];
</script>

<Drawer.Root direction="right">
	<Drawer.Trigger
		class="bg-primary/50 hover:bg-primary lg:bg-primary fixed top-1/5 right-0 p-3 transition-colors duration-300 ease-in-out"
		><span><CircleQuestionMark class="stroke-foreground" /></span></Drawer.Trigger
	>
	<Drawer.Content
		class="flex w-screen! max-w-[100vw]! flex-col justify-between px-8 py-12 lg:max-w-[50vw]!"
	>
		<Drawer.Close class="absolute top-0 left-0 p-3"
			><span><LucideChevronRight class="stroke-foreground" /></span></Drawer.Close
		>
		<Tabs.Root value="faqs">
			<div class="bg-sidebar mb-4 flex flex-row gap-0.5 rounded-lg p-1">
				{#each tabs as tab (tab.value)}
					<Tabs.Trigger value={tab.value} class="border-none">{tab.label}</Tabs.Trigger>
				{/each}
			</div>
			<div class="max-h-[50vh] overflow-y-auto">
				{#each tabs as tab (tab.value)}
					<Tabs.Content value={tab.value}>
						{#if tab.content}
							<ContentRenderer content={tab.content} />
						{:else}
							{@const Component = tab.fallback}
							<Component
								class="[&_h1]:text-primary flex flex-col gap-4 [&_h1]:font-bold [&_h2]:font-bold [&_ul]:list-inside [&_ul]:list-[square]!"
							/>
						{/if}
					</Tabs.Content>
				{/each}
			</div>
		</Tabs.Root>

		{#if conversation?.chatBotId && conversation.enableQaChatBot}
			<CollapsibleChat
				chatId={conversation.chatBotId}
				conversationId={conversation.id}
				userId={user?.id}
			/>
		{/if}
	</Drawer.Content>
</Drawer.Root>
