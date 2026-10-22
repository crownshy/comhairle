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
			{#if emailConfig.conversationTitle}
				<div class="flex flex-col gap-1 px-6 pb-6">
					<span class="text-sm">Conversation:</span>
					<p class="text-lg font-bold">{emailConfig.conversationTitle}</p>
				</div>
			{/if}
		</div>
	</article>
</a>
