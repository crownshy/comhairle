<script lang="ts">
	import { resolve } from '$app/paths';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import EndConversationModal from '$lib/components/EndConversationModal.svelte';
	import * as Card from '$lib/components/ui/card';
	import { Alert, AlertTitle, AlertDescription } from '$lib/components/ui/alert';
	import { Button } from '$lib/components/ui/button';
	import { permissions } from '$lib/permissions.svelte';
	import { ChevronRight, CircleCheck, CircleX, Lock, TriangleAlert } from 'lucide-svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	let conversation = $derived(data.conversation);
	let canClose = $derived(
		permissions.can('conversation', 'conversation_launch', conversation.id)
	);
	let canAdmin = $derived(permissions.can('conversation', 'conversation_admin', conversation.id));
	let endModalOpen = $state(false);

	let params = $derived({ conversation_id: conversation.id });
	let steps = $derived([
		{
			title: 'Let participants know',
			detail: 'Send a notification saying when the conversation ends and what happens next.',
			href: canAdmin
				? resolve('/(admin)/admin/conversations/[conversation_id]/notifications', params)
				: null
		},
		{
			title: 'Check final participation',
			detail: 'Look at Monitor for the final numbers and export the data you need.',
			href: resolve('/(admin)/admin/conversations/[conversation_id]/monitor', params)
		},
		{
			title: 'Start your report',
			detail: 'Results and reports stay available after you close.',
			href: resolve('/(admin)/admin/conversations/[conversation_id]/report', params)
		}
	]);
</script>

<!-- Checklist hrefs are built with resolve() in the script above. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->

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
	<div class="flex max-w-3xl flex-col gap-6">
		<section class="flex flex-col gap-3">
			<h2 class="text-lg font-semibold">Before you close</h2>
			<ol class="bg-background divide-border border-border divide-y rounded-xl border">
				{#each steps as step, index (step.title)}
					<li>
						{#if step.href}
							<a
								href={step.href}
								class="hover:bg-muted/60 flex items-center gap-4 px-5 py-4"
							>
								{@render stepBody(step, index)}
								<ChevronRight class="text-muted-foreground size-4 shrink-0" />
							</a>
						{:else}
							<div class="flex items-center gap-4 px-5 py-4">
								{@render stepBody(step, index)}
							</div>
						{/if}
					</li>
				{/each}
			</ol>
		</section>

		<Alert
			class="border-orange-300 bg-orange-50 dark:border-orange-400/40 dark:bg-orange-400/10"
		>
			<TriangleAlert class="text-orange-500" />
			<AlertTitle>What happens when you close</AlertTitle>
			<AlertDescription>
				<ul class="list-disc space-y-1 pl-4">
					<li>Participants can no longer take part or submit responses.</li>
					<li>Everything collected so far stays available in Monitor and Report.</li>
					<li>You can re-open the conversation later if you need to.</li>
				</ul>
			</AlertDescription>
		</Alert>

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

{#snippet stepBody(step: { title: string; detail: string }, index: number)}
	<span
		class="bg-muted text-foreground flex size-7 shrink-0 items-center justify-center rounded-full text-sm font-semibold"
		>{index + 1}</span
	>
	<span class="flex flex-1 flex-col gap-0.5">
		<span class="font-medium">{step.title}</span>
		<span class="text-muted-foreground text-sm">{step.detail}</span>
	</span>
{/snippet}
