<script lang="ts">
	import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
	import type {
		LocalizedConversationDto,
		LocalizedWorkflowStepDto
	} from '@crownshy/api-client/api';
	let {
		conversation,
		workflowStep
	}: { conversation?: LocalizedConversationDto; workflowStep?: LocalizedWorkflowStepDto } =
		$props();
</script>

<Breadcrumb.Root class="mb-16">
	<Breadcrumb.List>
		<Breadcrumb.Item class="text-black">
			<Breadcrumb.Link href="/">Home</Breadcrumb.Link>
		</Breadcrumb.Item>
		{#if conversation}
			<Breadcrumb.Separator class="text-black" />
			<Breadcrumb.Item class="text-black">
				<Breadcrumb.Link href="/conversations">Conversations</Breadcrumb.Link>
			</Breadcrumb.Item>
			<Breadcrumb.Separator class="text-black" />
			<Breadcrumb.Item class="text-black">
				<Breadcrumb.Link href={`/conversations/${conversation.id}`}>
					{conversation.title}
				</Breadcrumb.Link>
			</Breadcrumb.Item>
		{/if}
		{#if workflowStep}
			<Breadcrumb.Separator class="text-black" />
			<Breadcrumb.Item class="text-black">
				<Breadcrumb.Page>{workflowStep.name}</Breadcrumb.Page>
			</Breadcrumb.Item>
		{/if}
	</Breadcrumb.List>
</Breadcrumb.Root>
