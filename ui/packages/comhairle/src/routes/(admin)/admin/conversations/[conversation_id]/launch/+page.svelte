<script lang="ts">
	import { resolve } from '$app/paths';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import LaunchConversationModal from '$lib/components/LaunchConversationModal.svelte';
	import { Alert, AlertTitle, AlertDescription } from '$lib/components/ui/alert';
	import { Button } from '$lib/components/ui/button';
	import { permissions } from '$lib/permissions.svelte';
	import { ArrowUpRight, ChevronRight, Rocket, TriangleAlert } from 'lucide-svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
	let conversation = $derived(data.conversation);
	let stepCount = $derived(data.workflowSteps?.length ?? 0);
	let canLaunch = $derived(
		permissions.can('conversation', 'conversation_launch', conversation.id)
	);
	let launchModalOpen = $state(false);
	let hasPolisStep = $derived(
		(data.workflowSteps ?? []).some(
			(step) => step.toolConfig?.type === 'polis' || step.previewToolConfig?.type === 'polis'
		)
	);
	let canAdmin = $derived(permissions.can('conversation', 'conversation_admin', conversation.id));

	let params = $derived({ conversation_id: conversation.id });
	let nextSteps = $derived(
		[
			{
				title: 'Invite participants',
				detail: 'Send email invites or share the conversation link from Recruit.',
				href: resolve('/(admin)/admin/conversations/[conversation_id]/invites', params),
				show: true
			},
			{
				title: 'Keep participants informed',
				detail: 'Send reminders and updates while the conversation runs.',
				href: resolve(
					'/(admin)/admin/conversations/[conversation_id]/notifications',
					params
				),
				show: canAdmin
			},
			{
				title: 'Moderate statements',
				detail: 'Review new statements in your Polis steps as they come in.',
				href: resolve('/(admin)/admin/conversations/[conversation_id]/moderate', params),
				show: hasPolisStep
			},
			{
				title: 'Follow participation',
				detail: 'See who is taking part and how far they get in Monitor.',
				href: resolve('/(admin)/admin/conversations/[conversation_id]/monitor', params),
				show: true
			},
			{
				title: 'Close when you are done',
				detail: 'Wrap up, then start your report from what was collected.',
				href: resolve('/(admin)/admin/conversations/[conversation_id]/close', params),
				show: true
			}
		].filter((step) => step.show)
	);
	let checks = $derived([
		{
			title: 'Preview it as a participant',
			detail: 'Walk through every step the way participants will see it.',
			href: resolve('/(public)/conversations/[conversation_id]/[[preview]]', {
				...params,
				preview: 'preview'
			}),
			external: true
		},
		{
			title: 'Process design',
			detail:
				stepCount === 0
					? 'There are no steps yet. Add at least one before you launch.'
					: `${stepCount} ${stepCount === 1 ? 'step' : 'steps'}. Check the order and content of each one.`,
			href: resolve('/(admin)/admin/conversations/[conversation_id]/design', params),
			external: false
		},
		{
			title: 'Details and access',
			detail: 'Title, description, languages and who can join.',
			href: resolve(
				'/(admin)/admin/conversations/[conversation_id]/configure/details',
				params
			),
			external: false
		},
		{
			title: 'Team',
			detail: 'Make sure the right people can moderate and see results.',
			href: resolve('/(admin)/admin/conversations/[conversation_id]/configure/team', params),
			external: false
		}
	]);
</script>

<!-- Checklist hrefs are built with resolve() in the script above. -->
<!-- eslint-disable svelte/no-navigation-without-resolve -->

<svelte:head>
	<title>{conversation.isLive ? 'Launched' : 'Launch'} - Comhairle Admin</title>
</svelte:head>

<PageHeader
	title={conversation.isLive ? 'Launched' : 'Launch'}
	description={conversation.isLive
		? undefined
		: 'Check everything is ready, then make the conversation live for participants.'}
/>

{#if conversation.isLive}
	<div class="flex max-w-3xl flex-col gap-6">
		<div class="bg-background border-border flex items-start gap-4 rounded-xl border px-5 py-5">
			<span
				class="bg-primary text-primary-foreground flex size-10 shrink-0 items-center justify-center rounded-full"
			>
				<Rocket class="size-5" />
			</span>
			<div class="flex flex-col gap-1">
				<p class="text-lg font-semibold">
					{conversation.isComplete
						? 'This conversation was launched and is now closed'
						: 'Your conversation is launched'}
				</p>
				<p class="text-muted-foreground">
					{conversation.isComplete
						? 'Participants can no longer take part. Your results are in Monitor and Report.'
						: 'It is live, and anyone with access can take part now. Here is what to do next.'}
				</p>
			</div>
		</div>

		{#if !conversation.isComplete}
			<section class="flex flex-col gap-3">
				<h2 class="text-lg font-semibold">Next steps</h2>
				<ol class="bg-background divide-border border-border divide-y rounded-xl border">
					{#each nextSteps as step, index (step.title)}
						<li>
							<a
								href={step.href}
								class="hover:bg-muted/60 flex items-center gap-4 px-5 py-4"
							>
								<span
									class="bg-muted text-foreground flex size-7 shrink-0 items-center justify-center rounded-full text-sm font-semibold"
									>{index + 1}</span
								>
								<span class="flex flex-1 flex-col gap-0.5">
									<span class="font-medium">{step.title}</span>
									<span class="text-muted-foreground text-sm">{step.detail}</span>
								</span>
								<ChevronRight class="text-muted-foreground size-4 shrink-0" />
							</a>
						</li>
					{/each}
				</ol>
			</section>
		{/if}
	</div>
{:else}
	<div class="flex max-w-3xl flex-col gap-6">
		<section class="flex flex-col gap-3">
			<h2 class="text-lg font-semibold">Before you launch</h2>
			<ul class="bg-background divide-border border-border divide-y rounded-xl border">
				{#each checks as check (check.title)}
					<li>
						<a
							href={check.href}
							target={check.external ? '_blank' : undefined}
							class="hover:bg-muted/60 flex items-center gap-4 px-5 py-4"
						>
							<span class="flex flex-1 flex-col gap-0.5">
								<span class="font-medium">{check.title}</span>
								<span class="text-muted-foreground text-sm">{check.detail}</span>
							</span>
							{#if check.external}
								<ArrowUpRight class="text-muted-foreground size-4 shrink-0" />
							{:else}
								<ChevronRight class="text-muted-foreground size-4 shrink-0" />
							{/if}
						</a>
					</li>
				{/each}
			</ul>
		</section>

		<Alert
			class="border-orange-300 bg-orange-50 dark:border-orange-400/40 dark:bg-orange-400/10"
		>
			<TriangleAlert class="text-orange-500" />
			<AlertTitle>What happens when you launch</AlertTitle>
			<AlertDescription>
				<ul class="list-disc space-y-1 pl-4">
					<li>The conversation goes live and anyone with access can take part.</li>
					<li>You will no longer be able to change the conversation's set-up.</li>
					<li>Recruit, Notify, Monitor and Report open, and you can close it later.</li>
					<li>A launched conversation can't go back to draft.</li>
				</ul>
			</AlertDescription>
		</Alert>

		<div class="flex flex-col items-start gap-2">
			<Button
				class="h-10 rounded-full px-5"
				disabled={!canLaunch}
				onclick={() => (launchModalOpen = true)}
			>
				<Rocket class="size-4" />
				Launch conversation
			</Button>
			{#if !canLaunch}
				<p class="text-muted-foreground text-sm">
					You need launch permission for this conversation. Ask a conversation Admin.
				</p>
			{:else if stepCount === 0}
				<p class="text-muted-foreground text-sm">
					There are no steps yet, so participants will see an empty conversation.
				</p>
			{/if}
		</div>
	</div>

	<LaunchConversationModal
		conversation_id={conversation.id}
		hideTrigger
		bind:open={launchModalOpen}
	/>
{/if}
