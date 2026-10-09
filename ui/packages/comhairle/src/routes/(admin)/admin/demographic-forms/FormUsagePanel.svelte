<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import * as Table from '$lib/components/ui/table';
	import {
		conversationCount,
		usageSummary,
		versionInUse,
		type DemographicForm
	} from './demographicPrototypeData';

	type Props = { form: DemographicForm; hasUnsavedChanges: boolean };

	let { form, hasUnsavedChanges }: Props = $props();

	const stageLabels = { draft: 'Draft', live: 'Live', closed: 'Closed' } as const;
	const hasDraftChanges = $derived(form.hasUnpublishedChanges || hasUnsavedChanges);
</script>

<div
	class="bg-card border-border flex h-full min-h-0 flex-col gap-4 overflow-y-auto rounded-3xl border p-6"
>
	<div class="flex flex-col gap-1">
		<h2 class="text-xl font-semibold">Where this form is used</h2>
		{#if conversationCount(form) === 0}
			<p class="text-muted-foreground text-base">
				No conversation uses this form yet. Add it from a conversation with Add demographic
				step.
			</p>
		{:else}
			<p class="text-muted-foreground text-base">
				{conversationCount(form)}
				{conversationCount(form) === 1 ? 'conversation' : 'conversations'}: {usageSummary(
					form
				)}. Latest published version is v{form.version}.
			</p>
		{/if}
	</div>

	{#if conversationCount(form) > 0}
		<div class="border-border overflow-hidden rounded-xl border [&_tr>*:first-child]:pl-4">
			<Table.Root>
				<Table.Header class="bg-muted">
					<Table.Row>
						<Table.Head>Conversation</Table.Head>
						<Table.Head>Stage</Table.Head>
						<Table.Head>Version used</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each form.usage as usage (usage.conversationId)}
						{@const behind = form.version - versionInUse(form, usage)}
						<Table.Row>
							<Table.Cell class="text-base font-medium">{usage.title}</Table.Cell>
							<Table.Cell>
								<Badge variant={usage.stage === 'live' ? 'primary' : 'draft'}>
									{stageLabels[usage.stage]}
								</Badge>
							</Table.Cell>
							<Table.Cell>
								<div class="flex flex-col items-start gap-1">
									<span class="text-base font-medium"
										>v{versionInUse(form, usage)}</span
									>
									<span class="text-muted-foreground text-sm">
										{usage.pinnedVersion === null
											? 'Follows latest'
											: behind > 0
												? `Pinned, ${behind} ${behind === 1 ? 'version' : 'versions'} behind`
												: 'Pinned, up to date'}
									</span>
								</div>
							</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
		</div>
	{/if}

	{#if hasDraftChanges && conversationCount(form) > 0}
		<p class="bg-primary/10 rounded-lg px-4 py-3 text-base">
			Draft changes are not live. Conversations that follow latest will pick them up when you
			publish v{form.version + 1}. Pinned conversations stay where they are.
		</p>
	{/if}
</div>
