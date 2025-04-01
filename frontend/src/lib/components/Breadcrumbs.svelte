<script lang="ts">
	import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
	import type { Conversation, WorkflowStep } from '$lib/types';
	let {
		conversation,
		workflow_step
	}: { conversation?: Conversation; workflow_step?: WorkflowStep } = $props();
</script>

<Breadcrumb.Root class="mb-16">
	<Breadcrumb.List>
		<Breadcrumb.Item>
			<Breadcrumb.Link href="/">Home</Breadcrumb.Link>
		</Breadcrumb.Item>
		{#if conversation}
			<Breadcrumb.Separator />
			<Breadcrumb.Item>
				<Breadcrumb.Link href="/conversations">Conversations</Breadcrumb.Link>
			</Breadcrumb.Item>
			<Breadcrumb.Separator />
			<Breadcrumb.Item>
				<Breadcrumb.Link href={`/conversations/${conversation.id}`}>
					{conversation.title}
				</Breadcrumb.Link>
			</Breadcrumb.Item>
		{/if}
		{#if workflow_step}
			<Breadcrumb.Separator />
			<Breadcrumb.Item>
				<Breadcrumb.Page>{workflow_step.name}</Breadcrumb.Page>
			</Breadcrumb.Item>
		{/if}
	</Breadcrumb.List>
</Breadcrumb.Root>
