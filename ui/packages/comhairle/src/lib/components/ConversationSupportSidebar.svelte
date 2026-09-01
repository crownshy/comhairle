<script lang="ts">
	import * as Drawer from '$lib/components/ui/drawer';
	import * as Tabs from '$lib/components/ui/tabs';
	import { LucideX } from 'lucide-svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import type { ComhairleDocument, LocalizedConversationDto } from '@crownshy/api-client/api';
	import ComhairlePrivacyPolicy from './ComhairlePrivacyPolicy.svelte';
	import ComhairleFAQs from './ComhairleFAQs.svelte';
	import LearningAssistant from './LearningAssistant/LearningAssistant.svelte';
	import { learningAssistantAvailable } from './LearningAssistant/availability';
	import { supportPanel, type SupportPanelTab } from './participant/supportPanel.svelte';

	let {
		conversation,
		hasKnowledgeBaseDocs = false,
		availableDocuments = [],
		currentStepTitle
	}: {
		conversation: LocalizedConversationDto;
		hasKnowledgeBaseDocs?: boolean;
		/** Parsed documents, so source-document badges in the FAQ/privacy tabs resolve + download. */
		availableDocuments?: ComhairleDocument[];
		currentStepTitle?: string;
	} = $props();

	let assistantAvailable = $derived(
		learningAssistantAvailable(conversation, hasKnowledgeBaseDocs)
	);

	// The panel's tab lives in supportPanel, because the step menu opens the panel on a
	// particular one. A click on the tab strip writes back there, so the two agree.
	let activeTab = $derived(
		supportPanel.tab === 'learningAssistant' && !assistantAvailable ? 'faqs' : supportPanel.tab
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
			label: 'Privacy',
			content: conversation.privacyPolicy,
			fallback: ComhairlePrivacyPolicy
		}
	];
</script>

<Drawer.Root direction="right" bind:open={supportPanel.open}>
	<!-- Below lg the way in is the step menu, which can name what is behind it (the assistant,
	     the FAQs) where an icon could not. The rotated tab needs room to sit beside the content,
	     so it stays a desktop affordance. -->
	<Drawer.Trigger
		class="bg-primary text-primary-foreground fixed top-1/5 right-0 hidden translate-x-12 -rotate-90 rounded-t-xl px-4 py-3 text-base font-bold transition-colors duration-300 ease-in-out lg:block"
		><span>Find out more</span></Drawer.Trigger
	>
	<Drawer.Content class="flex w-screen! max-w-[100vw]! flex-col p-0 lg:max-w-[38rem]!">
		<!-- The panel titles itself and puts the way out where a hand expects it, rather than
		     leaving a bare chevron floating in the corner. -->
		<header class="border-border flex shrink-0 items-center gap-4 border-b px-4 py-4 sm:px-6">
			<h2 class="text-foreground min-w-0 flex-1 text-xl font-bold">Find out more</h2>
			<Drawer.Close
				class="text-muted-foreground hover:bg-muted hover:text-foreground grid h-10 w-10 shrink-0 place-items-center rounded-full transition-colors"
				aria-label="Close"
			>
				<LucideX class="h-5 w-5" />
			</Drawer.Close>
		</header>

		<Tabs.Root
			value={activeTab}
			onValueChange={(next) => supportPanel.openAt(next as SupportPanelTab)}
			class="flex min-h-0 flex-1 flex-col"
		>
			<div class="shrink-0 px-4 pt-4 pb-3 sm:px-6">
				<div class="bg-muted flex flex-row gap-1 overflow-x-auto rounded-full p-1">
					{#if assistantAvailable}
						<Tabs.Trigger
							value="learningAssistant"
							class="text-muted-foreground data-[state=active]:text-foreground shrink-0 rounded-full border-none px-3 py-2 text-base font-semibold sm:px-4"
							>Learning assistant</Tabs.Trigger
						>
					{/if}
					{#each tabs as tab (tab.value)}
						<Tabs.Trigger
							value={tab.value}
							class="text-muted-foreground data-[state=active]:text-foreground shrink-0 rounded-full border-none px-3 py-2 text-base font-semibold sm:px-4"
							>{tab.label}</Tabs.Trigger
						>
					{/each}
				</div>
			</div>

			<div class="flex min-h-0 flex-1 flex-col px-4 pt-1 pb-6 sm:px-6">
				{#each tabs as tab (tab.value)}
					<Tabs.Content value={tab.value} class="min-h-0 overflow-y-auto">
						{#if tab.content}
							<ContentRenderer
								content={tab.content}
								{availableDocuments}
								conversationId={conversation.id}
							/>
						{:else}
							{@const Component = tab.fallback}
							<Component
								class="[&_h1]:text-primary [&_h2]:text-primary flex flex-col gap-4 text-base [&_h1,&_h2,&_h3,&_h4,&_h5,&_h6]:font-bold [&_ul]:list-inside [&_ul]:list-[square]!"
							/>
						{/if}
					</Tabs.Content>
				{/each}
				{#if assistantAvailable}
					<Tabs.Content value="learningAssistant" class="flex min-h-0 flex-1 flex-col">
						<LearningAssistant
							conversationId={conversation.id}
							variant="sidebar"
							pageTitle={currentStepTitle}
						/>
					</Tabs.Content>
				{/if}
			</div>
		</Tabs.Root>
	</Drawer.Content>
</Drawer.Root>
