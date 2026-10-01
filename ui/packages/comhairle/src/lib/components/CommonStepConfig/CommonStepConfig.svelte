<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import { goto, invalidate } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { key } from '$lib/utils/invalidationKey';
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
	import {
		defaultDataProtocol,
		defaultDataProtocolHtml,
		hasDataProtocolText
	} from '$lib/dataProtocol';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import { useDebounce } from 'runed';
	import { createTextContentSource } from '$lib/components/Translation/translationSource.svelte';
	import { camelToSnakeCase } from '$lib/utils/casingUtils';
	import type { Locale } from '$lib/paraglide/runtime';
	import { permissions } from '$lib/permissions.svelte';

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
	const canEdit = $derived(
		permissions.can('conversation', 'conversation_update', conversation_id)
	);

	let primaryLocale = $derived<Locale>((conversation?.primaryLocale as Locale) ?? 'en');
	let supportedLanguages = $derived<Locale[]>(
		(conversation?.supportedLanguages as Locale[]) ?? ['en']
	);

	const nameSource = createTextContentSource({
		getTranslation: () => step?.translations?.name,
		getPrimaryLocale: () => primaryLocale,
		getSupportedLanguages: () => supportedLanguages,
		getPrimaryFallback: () => step?.name ?? '',
		refresh: () => invalidate(key('admin/conversation/workflow'))
	});
	const descriptionSource = createTextContentSource({
		getTranslation: () => step?.translations?.description,
		getPrimaryLocale: () => primaryLocale,
		getSupportedLanguages: () => supportedLanguages,
		getPrimaryFallback: () => step?.description ?? '',
		refresh: () => invalidate(key('admin/conversation/workflow'))
	});
	const dataProtocolSource = createTextContentSource({
		getTranslation: () => step?.translations?.dataProtocol ?? undefined,
		getPrimaryLocale: () => primaryLocale,
		getSupportedLanguages: () => supportedLanguages,
		getPrimaryFallback: () => step?.dataProtocol ?? '',
		refresh: () => invalidate(key('admin/conversation/workflow')),
		ensureTextContentId: createDataProtocol
	});

	let toolType = $derived(step?.previewToolConfig?.type);
	let dataProtocolIsBlank = $derived(
		!hasDataProtocolText(dataProtocolSource.contents[primaryLocale])
	);

	async function createDataProtocol(content: string): Promise<string | undefined> {
		const textContent = await tryCatchAsync(() =>
			apiClient.CreateTextContent({ content, format: 'rich', primary_locale: primaryLocale })
		);
		if (textContent.err !== null) {
			notifications.send({ message: 'Failed to create data protocol', priority: 'ERROR' });
			return;
		}

		const linked = await tryCatchAsync(() =>
			apiClient.UpdateConversationWorkflowStep(
				{ data_protocol: textContent.ok.id },
				{
					params: {
						conversation_id,
						workflow_id: step.workflowId,
						workflow_step_id: step.id
					}
				}
			)
		);
		if (linked.err !== null) {
			notifications.send({ message: 'Failed to update data protocol', priority: 'ERROR' });
			return;
		}

		return textContent.ok.id;
	}

	// Header / delete-dialog / preview read the live primary content straight from the sources.
	let displayName = $derived(nameSource.contents[primaryLocale] ?? '');
	let displayDescription = $derived(descriptionSource.contents[primaryLocale] ?? '');
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

	// Data protocol maps onto the `requestUserSharePermission` boolean (only Confidential
	// and Restricted are backed today; see tool_meta DATA_PROTOCOLS).
	let dataProtocol = $derived(protocolFromBool(requestUserSharePermission));
	let currentProtocol = $derived(
		DATA_PROTOCOLS.find((d) => d.value === dataProtocol) ?? DATA_PROTOCOLS[0]
	);
	function setDataProtocol(protocol: DataProtocol) {
		if (!canEdit) return;
		if (protocol === dataProtocol) return;
		handleSwitchChange(boolFromProtocol(protocol), 'requestUserSharePermission');
	}

	const debouncedUpdateRequired = useDebounce(async (checked: boolean, field: string) => {
		if (!canEdit) return;
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
			await invalidate(key('admin/conversation/workflow'));
		} catch {
			notifications.send({ message: `Failed to update ${field} status`, priority: 'ERROR' });
		}
	}, 500);

	function handleSwitchChange(checked: boolean, field: string) {
		if (!canEdit) return;
		debouncedUpdateRequired(checked, field);
	}

	let deleteOpen = $state(false);
	let deleting = $state(false);
	let deleteError = $state<string | null>(null);

	async function deleteStep() {
		if (!canEdit || deleting) return;
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
			await goto(
				resolve('/(admin)/admin/conversations/[conversation_id]/design', {
					conversation_id
				}),
				{
					invalidate: [key('admin/conversation/workflow')]
				}
			);
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
	<div class="flex flex-col gap-1">
		<span class="text-lg font-semibold">Name</span>
		<p class="text-muted-foreground mb-2 text-sm">
			The name of the step that will be shown to participants.
		</p>
		<TranslatableField
			source={nameSource}
			{primaryLocale}
			{supportedLanguages}
			disabled={!canEdit}
		/>
	</div>

	<div class="pt-4">
		<div class="flex flex-col gap-1">
			<span class="text-lg font-semibold">Description</span>
			<p class="text-muted-foreground text-sm">
				A description of this step that will inform users of its intent.
			</p>
		</div>
		<div class="pt-4">
			{#if canEdit}
				<TranslatableField
					source={descriptionSource}
					{primaryLocale}
					{supportedLanguages}
					{availableDocuments}
					conversationId={conversation_id}
					editorType="rich"
					minHeight="100px"
					maxHeight="150px"
				/>
			{:else}
				<div class="bg-card border-border rounded-lg border p-4">
					<ContentRenderer
						content={displayDescription}
						{availableDocuments}
						conversationId={conversation_id}
					/>
				</div>
			{/if}
		</div>
	</div>

	<div class="pt-4">
		<div class="flex flex-col gap-1">
			<span class="text-lg font-semibold">Data protocol</span>
			<p class="text-muted-foreground text-sm">
				Tells participants how their answers in this step are used. Leave blank to use the
				default for this tool.
			</p>
		</div>
		<div class="flex flex-col items-start gap-2 pt-4">
			<div class="w-full">
				<TranslatableField
					source={dataProtocolSource}
					{primaryLocale}
					{supportedLanguages}
					{availableDocuments}
					conversationId={conversation_id}
					editorType="rich"
					placeholder={defaultDataProtocol(toolType)}
					minHeight="100px"
					maxHeight="150px"
				/>
			</div>
			{#if dataProtocolIsBlank}
				<Button
					type="button"
					variant="outline"
					onclick={() => dataProtocolSource.saveSource(defaultDataProtocolHtml(toolType))}
				>
					Start from the default
				</Button>
			{/if}
		</div>
	</div>
{/snippet}

{#snippet switches()}
	<div class="flex items-center gap-2">
		<Switch
			checked={revisitable}
			disabled={!canEdit}
			onCheckedChange={(value) => handleSwitchChange(value, 'canRevisit')}
		/>
		<Label class="text-base">Revisitable step</Label>
		<span class="text-muted-foreground ml-2 text-sm">(Can users revisit this step?)</span>
	</div>
	<div class="flex items-center gap-2">
		<Switch
			checked={required}
			disabled={!canEdit}
			onCheckedChange={(value) => handleSwitchChange(value, 'required')}
		/>
		<Label class="text-base">Required step</Label>
		<span class="text-muted-foreground ml-2 text-sm">(Can users skip this step?)</span>
	</div>
	{#if toolType === 'thinkingspace'}
		<div class="flex items-center gap-2">
			<Switch
				checked={requestUserSharePermission}
				onCheckedChange={(value) => handleSwitchChange(value, 'requestUserSharePermission')}
			/>
			<Label class="text-base">Ask before sharing summaries</Label>
			<span class="text-muted-foreground ml-2 text-sm">
				(Participants choose whether organisers see their summary)
			</span>
		</div>
	{/if}
	<div class="flex items-center gap-2">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger
				disabled={!canEdit}
				class="border-input flex h-9 w-full max-w-sm items-center justify-between gap-2 rounded-md border px-3 text-sm"
			>
				<span class="flex items-center gap-2">
					<Database class="size-4" />
					{currentProtocol.label}
				</span>
				<ChevronDown class="size-4 opacity-50" />
			</DropdownMenu.Trigger>
			<DropdownMenu.Content class="max-w-sm">
				{#each DATA_PROTOCOLS as protocol (protocol.value)}
					<DropdownMenu.Item
						disabled={!canEdit || !protocol.enabled}
						onSelect={() => setDataProtocol(protocol.value)}
					>
						<span class="flex w-4 shrink-0 justify-center">
							{#if dataProtocol === protocol.value}
								<Check class="size-3" />
							{/if}
						</span>
						<span class="flex flex-col">
							<span>{protocol.label}{!protocol.enabled ? ' (soon)' : ''}</span>
							<span class="text-muted-foreground text-xs">{protocol.blurb}</span>
						</span>
					</DropdownMenu.Item>
				{/each}
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
{/snippet}

{#snippet dangerZone()}
	{#if canEdit}
		<div class="border-destructive/30 flex flex-col gap-4 rounded-lg border p-6">
			<div class="flex flex-col gap-1">
				<span class="text-destructive text-lg font-semibold">Danger zone</span>
				<p class="text-muted-foreground text-sm">
					Deleting this step permanently removes it and its configuration. The remaining
					steps will be renumbered. This action cannot be undone.
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
					<AlertDialog.Title>Delete “{displayName || 'this step'}”?</AlertDialog.Title>
					<AlertDialog.Description>
						This permanently removes the step and its configuration along with any
						associated data (e.g. user participation data), and renumbers the remaining
						steps. This action cannot be undone.
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
	{/if}
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
					<h2 class="text-2xl">{displayName || 'Unnamed Step'}</h2>
					{#if step?.required}
						<p class="text-red-900">(Required)</p>
					{:else}
						<p class="text-green-900">(Skippable)</p>
					{/if}
				</div>
				<ContentRenderer
					content={displayDescription}
					class="text-muted-foreground text-sm"
					{availableDocuments}
					conversationId={conversation_id}
				/>
			</div>
			<Button variant="default" onclick={() => (open = true)}
				>{canEdit ? 'Edit Metadata' : 'View Metadata'}</Button
			>
		</div>
	{/if}

	<Dialog.Root
		bind:open
		onOpenChange={(isOpen) => {
			if (!isOpen) invalidate(key('admin/conversation/workflow'));
		}}
	>
		<Dialog.Content class="flex max-h-[90vh] min-w-[70vw] flex-col rounded-xl p-0">
			<Dialog.Header class="shrink-0 border-b p-6 pb-4">
				<Dialog.Title class="text-2xl"
					>{canEdit ? 'Edit Step Metadata' : 'Step Metadata'}</Dialog.Title
				>
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
