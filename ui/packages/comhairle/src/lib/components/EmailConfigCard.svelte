<script lang="ts">
	import { snakeToStartCase } from '$lib/utils/casingUtils';
	import type { EmailTemplateConfigDto } from '@crownshy/api-client/api';

	type Props = {
		emailConfig: EmailTemplateConfigDto;
	};

	let { emailConfig }: Props = $props();
</script>

<a href={`/admin/email-template-configs/${emailConfig.id}`}>
	<article class="flex flex-col">
		<div class="flex justify-between">
			<div class="flex flex-col gap-1 px-6 pb-6">
				<span class="text-sm">Email type:</span>
				<p class="text-lg font-bold">{snakeToStartCase(emailConfig.emailType)}</p>
			</div>
			{#if emailConfig.emailType === 'conversation_invite' && !emailConfig.conversationId}
				<div class="flex flex-col gap-1 px-6 pb-6">
					<p class="max-w-40 text-right text-lg">Fallback for all conversations</p>
				</div>
			{:else if emailConfig.emailType === 'conversation_invite' && emailConfig.conversationTitle}
				<div class="flex flex-col gap-1 px-6 pb-6">
					<span class="text-right">Conversation:</span>
					<p class="text-lg font-bold">{emailConfig.conversationTitle}</p>
				</div>
			{/if}
		</div>
	</article>
</a>
