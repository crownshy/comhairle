<script lang="ts">
	import * as Drawer from '$lib/components/ui/drawer';
	import * as Tabs from '$lib/components/ui/tabs';
	import { LucideChevronRight } from 'lucide-svelte';
	import CircleQuestionMark from '$lib/components/icons/CircleQuestionMark.svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import type { LocalizedConversationDto, UserDto } from '@crownshy/api-client/api';
	import ComhairlePrivacyPolicy from './ComhairlePrivacyPolicy.svelte';
	import ComhairleFAQs from './ComhairleFAQs.svelte';
	import ChatBot from './Chatbot/ChatBot.svelte';

	let { conversation, user }: { conversation: LocalizedConversationDto; user: UserDto } =
		$props();

	let activeTab = $state(
		conversation?.chatBotId && conversation.enableQaChatBot ? 'tutorBot' : 'faqs'
	);

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
		class="bg-primary/50 hover:bg-primary lg:bg-primary fixed top-1/5 right-0 translate-x-12 -rotate-90 p-3 font-bold text-white transition-colors duration-300 ease-in-out"
		><span>Find out more</span></Drawer.Trigger
	>
	<Drawer.Content class="flex w-screen! max-w-[100vw]! flex-col px-8 py-12 lg:max-w-[50vw]!">
		<Drawer.Close class="absolute top-0 left-0 p-3 focus:border-none"
			><span><LucideChevronRight class="stroke-foreground" /></span></Drawer.Close
		>
		<Tabs.Root bind:value={activeTab} class="flex min-h-0 flex-1 flex-col">
			<div class="bg-sidebar mb-4 flex shrink-0 flex-row gap-0.5 rounded-xl p-1">
				{#if conversation?.chatBotId && conversation.enableQaChatBot}
					<Tabs.Trigger
						value="tutorBot"
						class="text-sidebar-foreground data-[state=active]:text-foreground border-none"
						>Tutor bot</Tabs.Trigger
					>
				{/if}
				{#each tabs as tab (tab.value)}
					<Tabs.Trigger
						value={tab.value}
						class="text-sidebar-foreground data-[state=active]:text-foreground border-none"
						>{tab.label}</Tabs.Trigger
					>
				{/each}
			</div>
			<div class="flex min-h-0 flex-1 flex-col">
				{#each tabs as tab (tab.value)}
					<Tabs.Content value={tab.value} class="overflow-y-auto">
						{#if tab.content}
							<ContentRenderer content={tab.content} />
						{:else}
							{@const Component = tab.fallback}
							<Component
								class="[&_h1]:text-primary [&_h2]:text-primary flex flex-col gap-4 [&_h1,&_h2,&_h3,&_h4,&_h5,&_h6]:font-bold [&_ul]:list-inside [&_ul]:list-[square]!"
							/>
						{/if}
					</Tabs.Content>
				{/each}
				{#if conversation?.chatBotId && conversation.enableQaChatBot}
					<Tabs.Content value="tutorBot" class="flex min-h-0 flex-1 flex-col">
						<div class="flex min-h-0 flex-1 flex-col">
							<ChatBot
								chatId={conversation.chatBotId}
								conversationId={conversation.id}
								userId={user?.id}
								botName="Tutor bot"
								botSubtitle="Ask questions"
								active={activeTab === 'tutorBot'}
							/>
						</div>
					</Tabs.Content>
				{/if}
			</div>
		</Tabs.Root>
	</Drawer.Content>
</Drawer.Root>
