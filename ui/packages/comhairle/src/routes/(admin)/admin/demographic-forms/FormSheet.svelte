<script lang="ts">
	import { EllipsisVertical, Plus, Trash2 } from 'lucide-svelte';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Badge } from '$lib/components/ui/badge';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';
	import * as Sheet from '$lib/components/ui/sheet';
	import DraggableList from '$lib/components/DraggableList.svelte';
	import AddQuestionsDialog from './AddQuestionsDialog.svelte';
	import PhonePreview from './PhonePreview.svelte';
	import QuestionAnswers from './QuestionAnswers.svelte';
	import {
		COUNTRY_OPTIONS,
		type DemographicForm,
		type DemographicQuestion
	} from './demographicPrototypeData';

	type Props = {
		form: DemographicForm;
		isNew: boolean;
		questions: DemographicQuestion[];
		onEditQuestion: (questionId: string) => void;
		onCreateQuestion: () => void;
		hasChanges: boolean;
		onSave: () => void;
		onSaveAsNew: () => void;
		onDelete: () => void;
		onClose: () => void;
	};

	let {
		form = $bindable(),
		isNew,
		questions,
		onEditQuestion,
		onCreateQuestion,
		hasChanges,
		onSave,
		onSaveAsNew,
		onDelete,
		onClose
	}: Props = $props();

	let addOpen = $state(false);
	let previewId = $state<string | null>(null);

	const rows = $derived(
		form.questions.flatMap((formQuestion) => {
			const question = questions.find((q) => q.id === formQuestion.questionId);
			return question ? [{ id: question.id, question, required: formQuestion.required }] : [];
		})
	);
	const previewIndex = $derived(
		Math.max(
			0,
			rows.findIndex((row) => row.question.id === previewId)
		)
	);
	const preview = $derived(rows[previewIndex]?.question);

	function toggleRequired(index: number) {
		form.questions[index].required = !form.questions[index].required;
	}

	let dragRows = $state<typeof rows | null>(null);

	function reorderWhileDragging(next: typeof rows) {
		dragRows = next;
	}

	function commitReorder(next: typeof rows) {
		form.questions = next.map((row) => ({ questionId: row.id, required: row.required }));
		dragRows = null;
	}

	function showNextQuestion() {
		const next = rows[previewIndex + 1];
		if (next) previewId = next.question.id;
	}

	function move(index: number, offset: number) {
		const target = index + offset;
		if (target < 0 || target >= form.questions.length) return;
		[form.questions[index], form.questions[target]] = [
			form.questions[target],
			form.questions[index]
		];
	}

	function addQuestions(ids: string[]) {
		form.questions.push(...ids.map((questionId) => ({ questionId, required: false })));
		addOpen = false;
	}

	function createFromDialog() {
		addOpen = false;
		onCreateQuestion();
	}
</script>

<Sheet.Root open onOpenChange={(open) => !open && onClose()}>
	<Sheet.Content side="right" class="flex w-full flex-col gap-0 p-0 sm:max-w-none lg:w-[64rem]">
		<div class="grid min-h-0 flex-1 gap-6 p-6 lg:grid-cols-[20rem_1fr]">
			<div class="flex min-h-0 flex-col gap-5 overflow-y-auto">
				<Sheet.Header class="border-border border-b p-0 pb-4">
					<Sheet.Title class="text-2xl font-semibold">
						{isNew ? 'Create form' : 'Edit form'}
					</Sheet.Title>
					<Sheet.Description>
						{isNew ? 'Create a form' : 'Update this form'}
					</Sheet.Description>
				</Sheet.Header>

				<div class="flex flex-col gap-2">
					<Label for="form-name" class="text-base">Form name</Label>
					<Input id="form-name" bind:value={form.name} placeholder="Name this form" />
				</div>

				<div class="flex flex-col gap-2">
					<Label for="form-country" class="text-base">Country</Label>
					<Select.Root
						type="single"
						value={form.country}
						onValueChange={(value) => (form.country = value)}
					>
						<Select.Trigger id="form-country" class="w-full"
							>{form.country}</Select.Trigger
						>
						<Select.Content>
							{#each COUNTRY_OPTIONS as country (country)}
								<Select.Item value={country} label={country}>{country}</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>
				</div>

				<div class="border-border flex flex-col gap-3 border-t pt-4">
					<h3 class="text-base font-semibold">Questions in this form</h3>
					{#if rows.length === 0}
						<p class="text-muted-foreground text-base">No questions yet.</p>
					{/if}
					<DraggableList
						items={dragRows ?? rows}
						onReorder={reorderWhileDragging}
						onCommit={commitReorder}
						class="flex flex-col gap-2"
					>
						{#snippet children(row, index)}
							<div
								class="bg-card flex items-center gap-2 rounded-lg border py-1 pr-1 pl-3 {index ===
								previewIndex
									? 'border-primary'
									: 'border-border'}"
							>
								<button
									type="button"
									class="flex min-w-0 flex-1 items-center gap-2 py-2 text-left"
									onclick={() => (previewId = row.question.id)}
								>
									<span
										class="bg-primary text-primary-foreground flex size-6 shrink-0 items-center justify-center rounded-full text-sm font-semibold"
									>
										{index + 1}
									</span>
									<span class="truncate text-base">{row.question.text}</span>
									{#if row.required}
										<span
											class="text-destructive font-semibold"
											title="Required">*</span
										>
										<span class="sr-only">Required</span>
									{/if}
								</button>
								{#if row.question.tags[0]}
									<Badge variant="secondary" class="shrink-0"
										>{row.question.tags[0]}</Badge
									>
								{/if}
								<DropdownMenu.Root>
									<DropdownMenu.Trigger
										class={buttonVariants({ variant: 'ghost', size: 'icon' })}
										aria-label={`Actions for ${row.question.text}`}
									>
										<EllipsisVertical class="size-4" />
									</DropdownMenu.Trigger>
									<DropdownMenu.Content align="end" class="w-52">
										<DropdownMenu.Item onSelect={() => toggleRequired(index)}>
											{row.required ? 'Set to optional' : 'Set to required'}
										</DropdownMenu.Item>
										<DropdownMenu.Item
											onSelect={() => onEditQuestion(row.question.id)}
										>
											Edit question
										</DropdownMenu.Item>
										<DropdownMenu.Item
											disabled={index === 0}
											onSelect={() => move(index, -1)}
										>
											Move up
										</DropdownMenu.Item>
										<DropdownMenu.Item
											disabled={index === rows.length - 1}
											onSelect={() => move(index, 1)}
										>
											Move down
										</DropdownMenu.Item>
										<DropdownMenu.Separator />
										<DropdownMenu.Item
											class="text-destructive"
											onSelect={() => form.questions.splice(index, 1)}
										>
											Remove from form
										</DropdownMenu.Item>
									</DropdownMenu.Content>
								</DropdownMenu.Root>
							</div>
						{/snippet}
					</DraggableList>
					<Button onclick={() => (addOpen = true)}
						><Plus class="size-4" />Add question</Button
					>
				</div>

				<div class="mt-auto flex flex-col gap-1">
					<span class="text-base font-semibold">Created by</span>
					<span class="text-base">{form.createdBy}</span>
				</div>

				{#if !isNew}
					<AlertDialog.Root>
						<AlertDialog.Trigger
							class={buttonVariants({ variant: 'destructiveOutline' })}
						>
							<Trash2 class="size-4" />Delete this form
						</AlertDialog.Trigger>
						<AlertDialog.Content>
							<AlertDialog.Header>
								<AlertDialog.Title>Delete this form?</AlertDialog.Title>
								<AlertDialog.Description>
									{form.usedInConversations > 0
										? `${form.name} is used in ${form.usedInConversations} ${form.usedInConversations === 1 ? 'conversation' : 'conversations'}. Those conversations will lose their demographic step.`
										: `${form.name} will be removed. This cannot be undone.`}
								</AlertDialog.Description>
							</AlertDialog.Header>
							<AlertDialog.Footer>
								<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
								<AlertDialog.Action onclick={onDelete}
									>Delete form</AlertDialog.Action
								>
							</AlertDialog.Footer>
						</AlertDialog.Content>
					</AlertDialog.Root>
				{/if}
			</div>

			<PhonePreview
				step={previewIndex + 1}
				steps={Math.max(rows.length, 1)}
				onNext={showNextQuestion}
				nextDisabled={previewIndex >= rows.length - 1}
			>
				{#if preview}
					<h2 class="text-2xl font-semibold">{preview.text}</h2>
					{#if preview.description}
						<p class="text-muted-foreground text-base">{preview.description}</p>
					{/if}
					<QuestionAnswers question={preview} />
				{:else}
					<p class="text-muted-foreground text-base">
						Add a question to see how participants will see it.
					</p>
				{/if}
			</PhonePreview>
		</div>

		<Sheet.Footer
			class="border-border flex-row items-center justify-end gap-2 border-t px-6 py-4"
		>
			{#if hasChanges && !isNew}
				<span class="text-muted-foreground mr-auto text-sm">
					Saving publishes v{form.version + 1}
				</span>
			{/if}
			<Button variant="outline" onclick={onClose}>Cancel</Button>
			{#if !isNew}
				<Button variant="outline" disabled={!hasChanges} onclick={onSaveAsNew}>
					Save to new form
				</Button>
			{/if}
			<Button disabled={form.name.trim() === '' || !hasChanges} onclick={onSave}>Save</Button>
		</Sheet.Footer>
	</Sheet.Content>
</Sheet.Root>

{#if addOpen}
	<AddQuestionsDialog
		{questions}
		inFormIds={form.questions.map((q) => q.questionId)}
		onAdd={addQuestions}
		onCreateQuestion={createFromDialog}
		onClose={() => (addOpen = false)}
	/>
{/if}
