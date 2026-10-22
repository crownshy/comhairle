<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import { Label } from '$lib/components/ui/label';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabContent from '$lib/components/TabContent.svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import {
		buildEmailTypeGroups,
		filterGroupsForConversation,
		scopedConversations
	} from '$lib/utils/emailTemplateGroups';
	import EmailTypeGroup from './EmailTypeGroup.svelte';
	import { emailTemplateConfigsTour } from './tour';

	useTour(() => emailTemplateConfigsTour);

	let { data } = $props();

	const ALL_CONVERSATIONS = 'all';

	let conversationFilter = $state(ALL_CONVERSATIONS);
	let conversations = $derived(scopedConversations(data.emailConfigs));
	let groups = $derived(
		filterGroupsForConversation(
			buildEmailTypeGroups(data.schemas, data.emailConfigs),
			conversationFilter === ALL_CONVERSATIONS ? null : conversationFilter
		)
	);
	let filterLabel = $derived(
		conversations.find((conversation) => conversation.id === conversationFilter)?.title ??
			'All conversations'
	);
	let summary = $derived(
		`${data.schemas.length} email types, ${data.emailConfigs.length} customised`
	);
</script>

<svelte:head>
	<title>Email templates - Comhairle Admin</title>
</svelte:head>

<SectionHeader
	content={{
		eyebrow: 'Library',
		title: 'Email templates',
		description:
			'The emails Comhairle sends to participants. Change the wording for all of your conversations, or for one conversation in particular.'
	}}
/>
<TabContent>
	<div class="flex max-w-5xl flex-col gap-6">
		<div class="flex flex-wrap items-end justify-between gap-3">
			{#if conversations.length > 0}
				<div class="flex flex-col gap-1.5">
					<Label for="conversation-filter" class="text-sm font-medium">
						Show templates for
					</Label>
					<Select.Root type="single" bind:value={conversationFilter}>
						<Select.Trigger id="conversation-filter" class="min-w-64 text-base">
							{filterLabel}
						</Select.Trigger>
						<Select.Content>
							<Select.Item value={ALL_CONVERSATIONS}>All conversations</Select.Item>
							{#each conversations as conversation (conversation.id)}
								<Select.Item value={conversation.id}>
									{conversation.title}
								</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				</div>
			{/if}
			<p class="text-muted-foreground text-sm">{summary}</p>
		</div>

		<div data-tour="email-configs-list" class="flex flex-col gap-6">
			{#each groups as group (group.emailType)}
				<EmailTypeGroup
					{group}
					showAddForConversation={conversationFilter === ALL_CONVERSATIONS}
				/>
			{/each}
		</div>
	</div>
</TabContent>
