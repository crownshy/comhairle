<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import { goto, invalidateAll } from '$app/navigation';
	import { Trash2, LoaderCircle } from 'lucide-svelte';
	import { notifications } from '$lib/notifications.svelte';
	import type {
		ComhairleDocument,
		ConversationWithTranslations,
		WorkflowStepWithTranslations
	} from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { Switch } from '../ui/switch';
	import { Label } from '../ui/label';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import { useDebounce } from 'runed';
	import { getTextInLocale } from '$lib/components/Translation/translationUtils';
	import { camelToSnakeCase } from '$lib/utils/casingUtils';

	type Props = {
		conversation_id: string;
		conversation: ConversationWithTranslations;
		step: WorkflowStepWithTranslations;
		headerless?: boolean;
		open?: boolean;
		inline?: boolean;
	};

	let {
		step,
		conversation_id,
		conversation,
		headerless = false,
		open = $bindable(false),
		inline = false
	}: Props = $props();

	let primaryLocale = $derived(conversation?.primaryLocale ?? 'en');
	let supportedLanguages = $derived(conversation?.supportedLanguages ?? ['en']);

	let sourceName = $derived.by(() => {
		return getTextInLocale(step?.translations?.name, primaryLocale, step?.name ?? '');
	});

	let sourceDescription = $derived.by(() => {
		return getTextInLocale(
			step?.translations?.description,
			primaryLocale,
			step?.description ?? ''
		);
	});

	let name = $state(step?.name ?? '');
	let description = $state('');
	let availableDocuments = $state<ComhairleDocument[]>([]);

	$effect(() => {
		if (!conversation_id) return;
		apiClient
			.ListDocuments({ params: { conversation_id } })
			.then((docs) => {
				availableDocuments = docs.filter((d) => d.parse_status === 'DONE');
			})
			.catch(() => {
				availableDocuments = [];
			});
	});
	let required = $derived(step?.required ?? false);
	let revisitable = $derived(step?.canRevisit ?? false);
	let requestUserSharePermission = $derived(step?.requestUserSharePermission ?? false);

	$effect(() => {
		name = getTextInLocale(step?.translations?.name, primaryLocale, step?.name ?? '');
	});

	$effect(() => {
		description = getTextInLocale(
			step?.translations?.description,
			primaryLocale,
			step?.description ?? ''
		);
	});

	const debouncedUpdateRequired = useDebounce(async (checked: boolean, field: string) => {
		try {
			await apiClient.UpdateConversationWorkflowStep(
				{ [camelToSnakeCase(field)]: checked },
				{
					params: {
						conversation_id,
						workflow_id: step.workflowId,
						workflow_step_id: step.id
					}
				}
			);
			await invalidateAll();
		} catch (e) {
			notifications.send({ message: `Failed to update ${field} status`, priority: 'ERROR' });
		}
	}, 500);

	function handleSwitchChange(checked: boolean, field: string) {
		debouncedUpdateRequired(checked, field);
	}

	let deleteOpen = $state(false);
	let deleting = $state(false);
	let deleteError = $state<string | null>(null);

	async function deleteStep() {
		deleting = true;
		deleteError = null;
		try {
			await apiClient.DeleteConversationWorkflowStep(undefined, {
				params: {
					conversation_id,
					workflow_id: step.workflowId,
					workflow_step_id: step.id
				}
			});
			notifications.send({ priority: 'INFO', message: 'Step deleted' });
			deleteOpen = false;
			await goto(`/admin/conversations/${conversation_id}/design`, {
				invalidate: ['conversation:workflow']
			});
		} catch (e) {
			console.error(e);
			deleteError = 'Something went wrong while deleting this step. Please try again.';
			notifications.send({ priority: 'ERROR', message: 'Failed to delete step' });
		} finally {
			deleting = false;
		}
	}
</script>

{#snippet fields()}
	<!-- Name field -->
	<div class="flex flex-col gap-1">
		<span class="text-lg font-semibold">Name</span>
		<p class="text-muted-foreground mb-2 text-sm">
			The name of the step that will be shown to participants.
		</p>
		<TranslatableField
			value={name}
			onValueChange={(v) => (name = v)}
			translation={step.translations?.name}
			{primaryLocale}
			{supportedLanguages}
		/>
	</div>

	<!-- Description field -->
	<div class="pt-4">
		<div class="flex flex-col gap-1">
			<span class="text-lg font-semibold">Description</span>
			<p class="text-muted-foreground text-sm">
				A description of this step that will inform users of its intent.
			</p>
		</div>
		<div class="pt-4">
			<TranslatableField
				value={description}
				onValueChange={(v) => (description = v)}
				translation={step.translations?.description}
				{primaryLocale}
				{supportedLanguages}
				{availableDocuments}
				conversationId={conversation_id}
				editorType="rich"
				minHeight="100px"
				maxHeight="150px"
			/>
		</div>
	</div>
{/snippet}

{#snippet switches()}
	<div class="flex items-center gap-2">
		<Switch
			checked={revisitable}
			onCheckedChange={(value) => handleSwitchChange(value, 'canRevisit')}
		/>
		<Label class="text-base">Revisitable step</Label>
		<span class="text-muted-foreground ml-2 text-sm">(Can users revisit this step?)</span>
	</div>
	<div class="flex items-center gap-2">
		<Switch
			checked={required}
			onCheckedChange={(value) => handleSwitchChange(value, 'required')}
		/>
		<Label class="text-base">Required step</Label>
		<span class="text-muted-foreground ml-2 text-sm">(Can users skip this step?)</span>
	</div>
	<div class="flex items-center gap-2">
		<Switch
			checked={requestUserSharePermission}
			onCheckedChange={(value) => handleSwitchChange(value, 'requestUserSharePermission')}
		/>
		<Label class="text-base">Ask for sharing consent</Label>
		<span class="text-muted-foreground ml-2 text-sm">
			(Prompt participants to opt in or out of sharing their responses with organizers.)
		</span>
	</div>
{/snippet}

{#snippet dangerZone()}
	<div class="border-destructive/30 flex flex-col gap-4 rounded-lg border p-6">
		<div class="flex flex-col gap-1">
			<span class="text-destructive text-lg font-semibold">Danger zone</span>
			<p class="text-muted-foreground text-sm">
				Deleting this step permanently removes it and its configuration. The remaining steps
				will be renumbered. This action cannot be undone.
			</p>
		</div>
		<div>
			<Button
				variant="destructive"
				disabled={deleting}
				onclick={() => {
					deleteError = null;
					deleteOpen = true;
				}}
			>
				<Trash2 class="mr-2 h-4 w-4" />
				Delete step
			</Button>
		</div>
	</div>

	<AlertDialog.Root bind:open={deleteOpen}>
		<AlertDialog.Content>
			<AlertDialog.Header>
				<AlertDialog.Title>Delete “{name || sourceName || 'this step'}”?</AlertDialog.Title>
				<AlertDialog.Description>
					This permanently removes the step and its configuration, and renumbers the
					remaining steps. This action cannot be undone.
				</AlertDialog.Description>
			</AlertDialog.Header>

			{#if deleteError}
				<p
					class="border-destructive/30 bg-destructive/10 text-destructive rounded-md border p-3 text-sm"
					role="alert"
				>
					{deleteError}
				</p>
			{/if}
			<AlertDialog.Footer class="flex-col-reverse sm:flex-row">
				<AlertDialog.Cancel class="w-full sm:w-auto" disabled={deleting}>
					Cancel
				</AlertDialog.Cancel>
				<AlertDialog.Action
					class="bg-destructive hover:bg-destructive/90 w-full text-white sm:w-auto"
					disabled={deleting}
					onclick={(e) => {
						e.preventDefault();
						deleteStep();
					}}
				>
					{#if deleting}
						<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
					{/if}
					Delete step
				</AlertDialog.Action>
			</AlertDialog.Footer>
		</AlertDialog.Content>
	</AlertDialog.Root>
{/snippet}

{#if inline}
	<div class="flex flex-col gap-6">
		{@render fields()}
		<div class="border-border flex flex-col gap-4 border-t pt-6">
			{@render switches()}
		</div>
		<div class="border-border border-t pt-6">
			{@render dangerZone()}
		</div>
	</div>
{:else}
	{#if !headerless}
		<div class="mb-10 flex flex-row items-start justify-between">
			<div class="flex flex-col gap-2">
				<div class="flex flex-row items-end gap-2">
					<h2 class="text-2xl">{name || sourceName || 'Unnamed Step'}</h2>
					{#if step?.required}
						<p class="text-red-900">(Required)</p>
					{:else}
						<p class="text-green-900">(Skippable)</p>
					{/if}
				</div>
				<ContentRenderer
					content={description || sourceDescription}
					class="text-muted-foreground text-sm"
					{availableDocuments}
					conversationId={conversation_id}
				/>
			</div>
			<Button variant="default" onclick={() => (open = true)}>Edit Metadata</Button>
		</div>
	{/if}

	<Dialog.Root
		bind:open
		onOpenChange={(isOpen) => {
			if (!isOpen) invalidateAll();
		}}
	>
		<Dialog.Content class="flex max-h-[90vh] min-w-[70vw] flex-col rounded-xl p-0">
			<Dialog.Header class="shrink-0 border-b p-6 pb-4">
				<Dialog.Title class="text-2xl">Edit Step Metadata</Dialog.Title>
				<Dialog.Description>
					Configure the name and description shown to participants.
				</Dialog.Description>
			</Dialog.Header>

			<ScrollArea.Root class="min-h-0 flex-1">
				<div class="px-6 pb-6">
					{@render fields()}
				</div>
			</ScrollArea.Root>

			<div class="bg-muted/30 flex shrink-0 flex-col gap-4 border-t p-6">
				{@render switches()}
			</div>
		</Dialog.Content>
	</Dialog.Root>
{/if}
