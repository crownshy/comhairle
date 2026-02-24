<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import { invalidateAll } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte';
	import type { ConversationWithTranslations, WorkflowStepWithTranslations } from '@crown-shy/api-client/api';
	import { apiClient } from '@crown-shy/api-client/client';
	import { Switch } from '../ui/switch';
	import { Label } from '../ui/label';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import { useDebounce } from 'runed';
	import { getTextInLocale } from '$lib/components/Translation/translationUtils';

	type Props = {
		conversation_id: string;
		conversation: ConversationWithTranslations;
		step: WorkflowStepWithTranslations;
	};

	let open = $state(false);
	let { step, conversation_id, conversation }: Props = $props();

	let primaryLocale = $derived(conversation?.primaryLocale ?? 'en');
	let supportedLanguages = $derived(conversation?.supportedLanguages ?? ['en']);

	let sourceName = $derived.by(() => {
		return getTextInLocale(step?.translations?.name, primaryLocale, step?.name ?? '');
	});

	let sourceDescription = $derived.by(() => {
		return getTextInLocale(step?.translations?.description, primaryLocale, step?.description ?? '');
	});

	let name = $state(step?.name ?? '');
	let description = $state('');
	let required = $state(step?.required ?? false);

	$effect(() => {
		name = getTextInLocale(step?.translations?.name, primaryLocale, step?.name ?? '');
	});

	$effect(() => {
		description = getTextInLocale(step?.translations?.description, primaryLocale, step?.description ?? '');
	});

	$effect(() => {
		required = step?.required ?? false;
	});

	const debouncedUpdateRequired = useDebounce(async (checked: boolean) => {
		try {
			await apiClient.UpdateWorkflowStep({ required: checked }, {
				params: {
					conversation_id,
					workflow_id: step.workflowId,
					workflow_step_id: step.id
				}
			});
			await invalidateAll();
		} catch (e) {
			notifications.send({ message: 'Failed to update required status', priority: 'ERROR' });
		}
	}, 500);

	function handleRequiredChange(checked: boolean) {
		required = checked;
		debouncedUpdateRequired(checked);
	}
</script>

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
		<ContentRenderer content={description || sourceDescription} class="text-sm text-muted-foreground" minimal />
	</div>
	<Dialog.Root bind:open onOpenChange={(isOpen) => { if (!isOpen) invalidateAll(); }}>
		<Dialog.Trigger>
			<Button variant="default">Edit Metadata</Button>
		</Dialog.Trigger>

		<Dialog.Content class="scot-gov max-h-[90vh] min-w-[70vw] p-0 flex flex-col rounded-xl">
			<Dialog.Header class="flex-shrink-0 p-6 pb-4 border-b">
				<Dialog.Title class="text-2xl">Edit Step Metadata</Dialog.Title>
				<Dialog.Description>
					Configure the name and description shown to participants.
				</Dialog.Description>
			</Dialog.Header>
			
			<ScrollArea.Root class="flex-1 min-h-0">
				<div class="px-6 pb-6">
					<!-- Name field -->
					<div class="flex flex-col gap-1">
						<span class="text-lg font-semibold">Name</span>
						<p class="text-sm text-muted-foreground mb-2">The name of the step that will be shown to participants.</p>
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
							<p class="text-sm text-muted-foreground">A description of this step that will inform users of its intent.</p>
						</div>
						<div class="pt-4">
							<TranslatableField
								value={description}
								onValueChange={(v) => (description = v)}
								translation={step.translations?.description}
								{primaryLocale}
								{supportedLanguages}
								editorType="rich"
								minHeight="100px"
								maxHeight="150px"
							/>
						</div>
					</div>
				</div>
			</ScrollArea.Root>

			<!-- Fixed footer with required toggle -->
			<div class="flex-shrink-0 border-t bg-muted/30 p-6">
				<div class="flex items-center gap-2">
					<Switch checked={required} onCheckedChange={handleRequiredChange} />
					<Label class="text-base">Required step</Label>
					<span class="text-sm text-muted-foreground ml-2">(Can users skip this step?)</span>
				</div>
			</div>
		</Dialog.Content>
	</Dialog.Root>
</div>
