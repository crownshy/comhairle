<script lang="ts">
	import * as Drawer from '$lib/components/ui/drawer';
	import * as Tabs from '$lib/components/ui/tabs';
	import { CircleHelp, LucideChevronRight } from 'lucide-svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import type { ComhairleDocument, LocalizedConversationDto } from '@crownshy/api-client/api';
	import ComhairlePrivacyPolicy from './ComhairlePrivacyPolicy.svelte';
	import ComhairleFAQs from './ComhairleFAQs.svelte';
	import LearningAssistant from './LearningAssistant/LearningAssistant.svelte';
	import StepDataProtocol from './StepDataProtocol.svelte';
	import { m } from '$lib/paraglide/messages';
	import { page } from '$app/state';
	import { useSupportDrawer } from './supportDrawerContext.svelte';

	let {
		conversation,
		hasKnowledgeBaseDocs = false,
		availableDocuments = [],
		currentStepTitle,
		stepDataProtocol
	}: {
		conversation: LocalizedConversationDto;
		hasKnowledgeBaseDocs?: boolean;
		/** Parsed documents, so source-document badges in the FAQ/privacy tabs resolve + download. */
		availableDocuments?: ComhairleDocument[];
		currentStepTitle?: string;
		/** Set while the participant is on a step, so the Privacy tab can say how that step's data is used. */
		stepDataProtocol?: { text: string | null | undefined; toolType: string | undefined };
	} = $props();

	// The Learning Assistant only answers from parsed knowledge base documents, so it is hidden
	// entirely when the knowledge base is empty (see workflow +layout.ts for the single source).
	let learningAssistantAvailable = $derived(
		!!conversation?.chatBotId && !!conversation.enableQaChatBot && hasKnowledgeBaseDocs
	);

	let activeTab = $state(
		conversation?.chatBotId && conversation.enableQaChatBot && hasKnowledgeBaseDocs
			? 'learningAssistant'
			: 'faqs'
	);

	const TAB_TRIGGER_CLASS =
		'text-muted-foreground hover:text-foreground hover:bg-muted/60 hover:border-muted-foreground/40 active:bg-muted data-[state=active]:text-foreground data-[state=active]:border-primary dark:data-[state=active]:border-primary data-[state=active]:hover:bg-muted/60 dark:data-[state=active]:hover:bg-muted/60 h-11 flex-none cursor-pointer rounded-none rounded-t-md border-0 border-b-2 border-transparent bg-transparent px-3 text-base shadow-none transition-colors data-[state=active]:bg-transparent data-[state=active]:shadow-none dark:data-[state=active]:bg-transparent';

	const supportDrawer = useSupportDrawer();
	// Embedded pages have no NavBar to hold the phone trigger, so they keep a floating one.
	let isEmbed = $derived(page.url.searchParams.get('embed') === 'true');

	let tabs = [
		{
			value: 'faqs',
			label: m.faq,
			content: conversation.faqs,
			fallback: ComhairleFAQs
		},
		{
			value: 'privacyPolicy',
			label: m.privacy_policy,
			content: conversation.privacyPolicy,
			fallback: ComhairlePrivacyPolicy
		}
	];
</script>

<Drawer.Root direction="right" bind:open={supportDrawer.open}>
	<!-- The sideways tab covers the step header below lg, so smaller screens open the drawer
	     from a NavBar button instead. -->
	<Drawer.Trigger
		class="bg-primary text-primary-foreground fixed top-1/5 right-0 translate-x-12 -rotate-90 p-3 font-bold max-lg:hidden"
		>{m.support_find_out_more()}</Drawer.Trigger
	>
	{#if isEmbed}
		<!-- Raised to clear sticky bottom bars like Prioritization's. -->
		<Drawer.Trigger
			class="bg-primary text-primary-foreground fixed end-4 bottom-20 z-40 inline-flex size-14 items-center justify-center rounded-full shadow-lg lg:hidden"
			aria-label={m.support_find_out_more()}
		>
			<CircleHelp class="size-7" aria-hidden="true" />
		</Drawer.Trigger>
	{/if}
	<Drawer.Content class="flex w-screen! max-w-[100vw]! flex-col pb-12 lg:max-w-[50vw]!">
		<Tabs.Root bind:value={activeTab} class="flex min-h-0 flex-1 flex-col gap-0">
			<div class="border-border flex shrink-0 items-end gap-2 border-b px-4 pt-2">
				<Drawer.Close
					aria-label={m.support_close()}
					class="hover:bg-muted/60 active:bg-muted focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:outline-ring grid h-11 w-11 shrink-0 cursor-pointer place-items-center rounded-t-md border-b-2 border-transparent transition-[color,background-color,box-shadow] focus-visible:ring-[3px] focus-visible:outline-1"
				>
					<LucideChevronRight class="stroke-foreground" aria-hidden="true" />
				</Drawer.Close>
				<Tabs.List
					class="h-auto w-full justify-start gap-1 overflow-x-auto rounded-none bg-transparent p-0"
				>
					{#if learningAssistantAvailable}
						<Tabs.Trigger value="learningAssistant" class={TAB_TRIGGER_CLASS}
							>{m.learning_assistant()}</Tabs.Trigger
						>
					{/if}
					{#each tabs as tab (tab.value)}
						<Tabs.Trigger value={tab.value} class={TAB_TRIGGER_CLASS}
							>{tab.label()}</Tabs.Trigger
						>
					{/each}
				</Tabs.List>
			</div>
			<div class="flex min-h-0 flex-1 flex-col px-6 pt-6 md:px-8">
				{#each tabs as tab (tab.value)}
					<Tabs.Content value={tab.value} class="overflow-y-auto pe-2">
						{#if tab.value === 'privacyPolicy' && stepDataProtocol}
							<StepDataProtocol
								text={stepDataProtocol.text}
								toolType={stepDataProtocol.toolType}
								onOpenFaq={() => (activeTab = 'faqs')}
								{availableDocuments}
								conversationId={conversation.id}
							/>
						{/if}
						{#if tab.content}
							<ContentRenderer
								content={tab.content}
								{availableDocuments}
								conversationId={conversation.id}
							/>
						{:else}
							{@const Component = tab.fallback}
							<Component
								class="[&_h1]:text-primary [&_h2]:text-primary flex flex-col gap-4 [&_h1,&_h2,&_h3,&_h4,&_h5,&_h6]:font-bold [&_ul]:list-inside [&_ul]:list-[square]!"
							/>
						{/if}
					</Tabs.Content>
				{/each}
				{#if learningAssistantAvailable}
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
