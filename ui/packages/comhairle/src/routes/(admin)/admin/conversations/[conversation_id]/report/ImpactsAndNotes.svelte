<script lang="ts">
	import { untrack } from 'svelte';
	import Pencil from 'lucide-svelte/icons/pencil';
	import Plus from 'lucide-svelte/icons/plus';
	import Trash from 'lucide-svelte/icons/trash-2';
	import { apiClient } from '@crownshy/api-client/client';
	import type {
		CreateImpactDTO,
		FeedbackDto,
		FullReportDto,
		ReportImpactDto
	} from '@crownshy/api-client/api';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import ImpactDialog from './ImpactDialog.svelte';
	import { impactKindLabel } from './impactKinds';

	type Props = { report: FullReportDto; canEdit: boolean };
	let { report, canEdit }: Props = $props();

	// The generated client drops these because the report DTO flattens an untagged union.
	// The API only fills them for people who can update the conversation.
	type PrivateFields = { impacts?: ReportImpactDto[]; facilitatorFeedback?: FeedbackDto[] };
	const privateFields = $derived(report as unknown as PrivateFields);

	// Saves update this list in place, so the page never reloads under the editor.
	let impacts = $state(untrack(() => privateFields.impacts ?? []));
	const feedback = $derived(privateFields.facilitatorFeedback ?? []);

	const params = $derived({
		params: { conversation_id: report.conversationId, report_id: report.id }
	});

	let dialogOpen = $state(false);
	let editing = $state<ReportImpactDto | null>(null);
	/** Remounts the dialog per open, so it starts from the impact being edited. */
	let dialogKey = $state(0);
	let deleting = $state<ReportImpactDto | null>(null);

	function openDialog(impact: ReportImpactDto | null) {
		editing = impact;
		dialogKey += 1;
		dialogOpen = true;
	}

	async function saveImpact(draft: CreateImpactDTO): Promise<boolean> {
		const target = editing;
		const result = await tryCatchAsync(() =>
			target
				? apiClient.UpdateImpact(draft, {
						params: { ...params.params, impact_id: target.id }
					})
				: apiClient.CreateImpact(draft, params)
		);
		if (result.err !== null) {
			notifications.send({ message: "Couldn't save the impact", priority: 'ERROR' });
			return false;
		}
		const saved = result.ok;
		impacts = target
			? impacts.map((impact) => (impact.id === saved.id ? saved : impact))
			: [...impacts, saved];
		return true;
	}

	async function deleteImpact(impact: ReportImpactDto) {
		const result = await tryCatchAsync(() =>
			apiClient.DeleteImpact(undefined, {
				params: { ...params.params, impact_id: impact.id }
			})
		);
		if (result.err !== null) {
			notifications.send({ message: "Couldn't delete the impact", priority: 'ERROR' });
			return;
		}
		impacts = impacts.filter((kept) => kept.id !== impact.id);
	}
</script>

<div class="flex flex-col gap-4">
	<Card.Root>
		<Card.Header class="flex flex-row items-start justify-between gap-4">
			<div class="flex flex-col gap-1.5">
				<Card.Title>Impacts</Card.Title>
				<Card.Description class="text-base">
					What has this conversation changed?
				</Card.Description>
			</div>
			{#if canEdit}
				<Button variant="outline" onclick={() => openDialog(null)}>
					<Plus class="size-4" />Add impact
				</Button>
			{/if}
		</Card.Header>
		<Card.Content>
			{#if impacts.length === 0}
				<p class="text-muted-foreground text-base">No impacts recorded yet.</p>
			{:else}
				<ul class="divide-border flex flex-col divide-y">
					{#each impacts as impact (impact.id)}
						<li
							class="flex items-start justify-between gap-4 py-4 first:pt-0 last:pb-0"
						>
							<div class="flex min-w-0 flex-col gap-1">
								<p class="flex flex-wrap items-center gap-2 text-base font-medium">
									{impact.title}
									<span
										class="bg-accent text-accent-foreground rounded-full px-2.5 py-0.5 text-sm font-normal"
									>
										{impactKindLabel(impact.kind)}
									</span>
								</p>
								{#if impact.details}
									<p class="text-muted-foreground text-base whitespace-pre-line">
										{impact.details}
									</p>
								{/if}
							</div>
							{#if canEdit}
								<div class="flex shrink-0 gap-1">
									<Button
										variant="ghost"
										size="icon"
										aria-label="Edit {impact.title}"
										onclick={() => openDialog(impact)}
									>
										<Pencil class="size-4" />
									</Button>
									<Button
										variant="ghost"
										size="icon"
										aria-label="Delete {impact.title}"
										onclick={() => (deleting = impact)}
									>
										<Trash class="size-4" />
									</Button>
								</div>
							{/if}
						</li>
					{/each}
				</ul>
			{/if}
		</Card.Content>
	</Card.Root>

	<Card.Root>
		<Card.Header>
			<Card.Title>Participant feedback</Card.Title>
			<Card.Description class="text-base">
				What participants sent with the feedback button.
			</Card.Description>
		</Card.Header>
		<Card.Content>
			{#if feedback.length === 0}
				<p class="text-muted-foreground text-base">No feedback yet.</p>
			{:else}
				<ul class="divide-border flex flex-col divide-y">
					{#each feedback as entry (entry.id)}
						<li class="py-3 text-base whitespace-pre-line first:pt-0 last:pb-0">
							{entry.content}
						</li>
					{/each}
				</ul>
			{/if}
		</Card.Content>
	</Card.Root>
</div>

{#key dialogKey}
	<ImpactDialog bind:open={dialogOpen} initial={editing ?? undefined} onSave={saveImpact} />
{/key}

<AlertDialog.Root
	open={deleting !== null}
	onOpenChange={(isOpen) => {
		if (!isOpen) deleting = null;
	}}
>
	<AlertDialog.Content>
		<AlertDialog.Header>
			<AlertDialog.Title>Delete this impact?</AlertDialog.Title>
			<AlertDialog.Description class="text-base">
				"{deleting?.title}" will be removed from the report's impacts.
			</AlertDialog.Description>
		</AlertDialog.Header>
		<AlertDialog.Footer>
			<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
			<AlertDialog.Action
				onclick={() => {
					if (deleting) deleteImpact(deleting);
					deleting = null;
				}}
			>
				Delete
			</AlertDialog.Action>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
