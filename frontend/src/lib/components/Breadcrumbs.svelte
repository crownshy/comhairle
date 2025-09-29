<script lang="ts">
	import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
	import type { Conversation, WorkflowStep } from '$lib/api/api';
	let {
		conversation,
		workflow_step
	}: { conversation?: Conversation; workflow_step?: WorkflowStep } = $props();
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
		{#if workflow_step}
			<Breadcrumb.Separator class="text-black" />
			<Breadcrumb.Item class="text-black">
				<Breadcrumb.Page>{workflow_step.name}</Breadcrumb.Page>
			</Breadcrumb.Item>
		{/if}
	</Breadcrumb.List>
</Breadcrumb.Root>
