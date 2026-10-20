<script lang="ts">
	import { resolve } from '$app/paths';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import EndConversationModal from '$lib/components/EndConversationModal.svelte';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { permissions } from '$lib/permissions.svelte';
	import ChecklistItem from '../ChecklistItem.svelte';
	import ConsequencesAlert from '../ConsequencesAlert.svelte';
	import { CircleCheck, CircleX, Lock } from 'lucide-svelte';

	let { data } = $props();
	let { conversation } = $derived(data);
	let canClose = $derived(
		permissions.can('conversation', 'conversation_launch', conversation.id)
	);
	let canAdmin = $derived(permissions.can('conversation', 'conversation_admin', conversation.id));
	let endModalOpen = $state(false);
</script>

<PageHeader
	title={conversation.isComplete ? 'Closed' : 'Close'}
	description={conversation.isComplete
		? 'Participants can no longer take part in this conversation.'
		: 'Wrap up the conversation when participation is finished.'}
/>

{#if !conversation.isLive}
	<Card.Root>
		<Card.Content class="flex items-center gap-3">
			<Lock class="text-muted-foreground size-5 shrink-0" />
			<p>You can close a conversation once it has been launched.</p>
		</Card.Content>
	</Card.Root>
{:else if conversation.isComplete}
	<Card.Root>
		<Card.Content class="flex flex-col items-start gap-4">
			<div class="flex items-center gap-3">
				<CircleCheck class="text-primary size-6 shrink-0" />
				<p class="text-lg font-semibold">This conversation is closed.</p>
			</div>
			<p class="text-muted-foreground">
				Your results and reports are still available. If you need more input, you can
				re-open it for participants.
			</p>
			<EndConversationModal {conversation} />
		</Card.Content>
	</Card.Root>
{:else}
	{@const steps = [
		{
			title: 'Let participants know',
			detail: 'Send a notification saying when the conversation ends and what happens next.',
			href: canAdmin ? 'notifications' : null
		},
		{
			title: 'Check final participation',
			detail: 'Look at Monitor for the final numbers and export the data you need.',
			href: 'monitor'
		},
		{
			title: 'Start your report',
			detail: 'Results and reports stay available after you close.',
			href: 'report'
		}
	] as const}
	<div class="flex max-w-3xl flex-col gap-6">
		<section class="flex flex-col gap-3">
			<h2 class="text-lg font-semibold">Before you close</h2>
			<ol class="bg-background divide-border border-border divide-y rounded-xl border">
				{#each steps as step, index (step.title)}
					<ChecklistItem
						{...step}
						href={step.href
							? resolve(
									`/(admin)/admin/conversations/[conversation_id]/${step.href}`,
									{ conversation_id: conversation.id }
								)
							: null}
						number={index + 1}
					/>
				{/each}
			</ol>
		</section>

		<ConsequencesAlert
			title="What happens when you close"
			consequences={[
				'Participants can no longer take part or submit responses.',
				'Everything collected so far stays available in Monitor and Report.',
				'You can re-open the conversation later if you need to.'
			]}
		/>

		<div class="flex flex-col items-start gap-2">
			<Button
				variant="outline"
				class="text-destructive border-destructive hover:bg-destructive/10 hover:text-destructive! h-10 rounded-full px-5"
				disabled={!canClose}
				onclick={() => (endModalOpen = true)}
			>
				<CircleX class="size-4" />
				Close conversation
			</Button>
			{#if !canClose}
				<p class="text-muted-foreground text-sm">
					You need launch permission for this conversation to close it.
				</p>
			{/if}
		</div>
	</div>

	<EndConversationModal {conversation} hideTrigger bind:open={endModalOpen} />
{/if}
