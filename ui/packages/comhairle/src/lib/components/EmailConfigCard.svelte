<script lang="ts">
	import { isConversationEmail, isEventEmail } from '$lib/email-template-configs';
	import { snakeToStartCase } from '$lib/utils/casingUtils';
	import type { EmailTemplateConfigDto } from '@crownshy/api-client/api';

	type Props = {
		emailConfig: EmailTemplateConfigDto;
	};

	let { emailConfig }: Props = $props();

	const isConversation = $derived(isConversationEmail(emailConfig.emailType));
	const isEvent = $derived(isEventEmail(emailConfig.emailType));
</script>

{#snippet scopeTitle(content: string, heading?: string)}
	<div class="flex flex-col gap-1 px-6 pb-6">
		{#if heading}
			<span class="text-right">{heading}</span>
		{/if}
		<p class="max-w-40 text-right text-lg">{content}</p>
	</div>
{/snippet}

<a href={`/admin/email-template-configs/${emailConfig.id}`}>
	<article class="flex flex-col">
		<div class="flex justify-between">
			<div class="flex flex-col gap-1 px-6 pb-6">
				<span class="text-sm">Email type:</span>
				<p class="text-lg font-bold">{snakeToStartCase(emailConfig.emailType)}</p>
			</div>
			{#if isConversation && !emailConfig.conversationId}
				{@render scopeTitle('Fallback for all conversations')}
			{:else if isConversation && emailConfig.conversationTitle}
				{@render scopeTitle(emailConfig.conversationTitle, 'Conversation:')}
			{:else if isEvent && !emailConfig.eventId}
				{@render scopeTitle('Fallback for all events')}
			{:else if isEvent && emailConfig.eventTitle}
				{@render scopeTitle(emailConfig.eventTitle, 'Event:')}
			{/if}
		</div>
	</article>
</a>
