<script lang="ts">
	import { Plus } from 'lucide-svelte';
	import TagBadge from './TagBadge.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Input } from '$lib/components/ui/input';
	import { QUESTION_KIND_LABELS, type DemographicQuestion } from './demographicPrototypeData';

	type Props = {
		questions: DemographicQuestion[];
		inFormIds: string[];
		onAdd: (ids: string[]) => void;
		onCreateQuestion: () => void;
		onClose: () => void;
	};

	let { questions, inFormIds, onAdd, onCreateQuestion, onClose }: Props = $props();

	let search = $state('');
	let selected = $state<string[]>([]);

	const visible = $derived(
		questions.filter((q) => q.text.toLowerCase().includes(search.trim().toLowerCase()))
	);
	const selectedSpecial = $derived(
		questions.filter((q) => selected.includes(q.id) && q.specialCategory)
	);

	function toggle(id: string, checked: boolean) {
		selected = checked ? [...selected, id] : selected.filter((value) => value !== id);
	}
</script>

<Dialog.Root open onOpenChange={(open) => !open && onClose()}>
	<Dialog.Content class="flex max-h-[85vh] flex-col gap-4 sm:max-w-2xl">
		<Dialog.Header>
			<Dialog.Title>Add questions</Dialog.Title>
			<Dialog.Description>
				Choose from your question bank. Questions already in this form are greyed out.
			</Dialog.Description>
		</Dialog.Header>

		<Input bind:value={search} placeholder="Search questions" aria-label="Search questions" />

		<ul class="border-border min-h-0 flex-1 divide-y overflow-y-auto rounded-lg border">
			{#each visible as question (question.id)}
				{@const inForm = inFormIds.includes(question.id)}
				<li
					class="flex items-center gap-3 px-4 py-3 {inForm
						? 'bg-muted text-muted-foreground'
						: selected.includes(question.id)
							? 'bg-primary/10'
							: ''}"
				>
					<Checkbox
						id={`add-${question.id}`}
						checked={inForm || selected.includes(question.id)}
						disabled={inForm}
						onCheckedChange={(checked) => toggle(question.id, checked === true)}
						aria-label={question.text}
					/>
					<label for={`add-${question.id}`} class="flex min-w-0 flex-1 flex-col">
						<span class="text-base font-medium">{question.text}</span>
						<span class="text-sm">{QUESTION_KIND_LABELS[question.kind]}</span>
					</label>
					<span class="flex items-center gap-1">
						{#each question.tags as tag (tag)}
							<TagBadge {tag} />
						{/each}
						{#if question.specialCategory}
							<TagBadge tag="Special category" />
						{/if}
					</span>
					{#if inForm}
						<span class="text-sm">In this form</span>
					{/if}
				</li>
			{/each}
		</ul>

		{#if selectedSpecial.length > 0}
			<p class="bg-primary/10 rounded-lg px-4 py-3 text-sm">
				{selectedSpecial.map((q) => q.text).join(', ')}
				{selectedSpecial.length === 1 ? 'is' : 'are'} special category data. Participants are
				asked for explicit consent before answering.
			</p>
		{/if}

		<Dialog.Footer class="items-center sm:justify-between">
			<Button variant="link" class="px-0" onclick={onCreateQuestion}>
				<Plus class="size-4" />Create a new question
			</Button>
			<div class="flex gap-2">
				<Button variant="outline" onclick={onClose}>Cancel</Button>
				<Button disabled={selected.length === 0} onclick={() => onAdd(selected)}>
					Add {selected.length}
					{selected.length === 1 ? 'question' : 'questions'}
				</Button>
			</div>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
