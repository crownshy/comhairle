<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import * as Table from '$lib/components/ui/table';
	import {
		QUESTION_KIND_LABELS,
		usedInFormsCount,
		type DemographicForm,
		type DemographicQuestion
	} from './demographicPrototypeData';

	type Props = {
		questions: DemographicQuestion[];
		forms: DemographicForm[];
		onEdit: (questionId: string) => void;
	};

	const SPECIAL_FILTER = 'Special category';

	let { questions, forms, onEdit }: Props = $props();

	let search = $state('');
	let filter = $state<string | null>(null);

	const filterOptions = $derived([...new Set(questions.flatMap((q) => q.tags)), SPECIAL_FILTER]);
	const countFor = (option: string) =>
		questions.filter((q) =>
			option === SPECIAL_FILTER ? q.specialCategory : q.tags.includes(option)
		).length;
	const visible = $derived(
		questions.filter((q) => {
			const matchesFilter =
				filter === null ||
				(filter === SPECIAL_FILTER ? q.specialCategory : q.tags.includes(filter));
			return matchesFilter && q.text.toLowerCase().includes(search.trim().toLowerCase());
		})
	);
</script>

<div class="flex flex-col gap-4">
	<p class="bg-primary/10 rounded-lg px-4 py-3 text-base">
		Add your own demographic questions to the question bank, then use them in any of your
		demographic forms. Select a question to edit it.
	</p>

	<div class="flex flex-wrap items-center justify-between gap-3">
		<div class="flex flex-wrap gap-2">
			<button type="button" aria-pressed={filter === null} onclick={() => (filter = null)}>
				<Badge variant={filter === null ? 'primary' : 'default'} class="px-3 py-1">
					All ({questions.length})
				</Badge>
			</button>
			{#each filterOptions as option (option)}
				<button
					type="button"
					aria-pressed={filter === option}
					onclick={() => (filter = option)}
				>
					<Badge variant={filter === option ? 'primary' : 'default'} class="px-3 py-1">
						{option} ({countFor(option)})
					</Badge>
				</button>
			{/each}
		</div>
		<Input
			class="w-64"
			bind:value={search}
			placeholder="Search questions"
			aria-label="Search questions"
		/>
	</div>

	<div class="border-border bg-card rounded-xl border">
		<Table.Root>
			<Table.Header>
				<Table.Row>
					<Table.Head>Question</Table.Head>
					<Table.Head>Question type</Table.Head>
					<Table.Head>Used in forms</Table.Head>
					<Table.Head>Demographic category</Table.Head>
					<Table.Head class="w-24"><span class="sr-only">Actions</span></Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each visible as question (question.id)}
					<Table.Row>
						<Table.Cell class="text-base font-medium">{question.text}</Table.Cell>
						<Table.Cell class="text-base"
							>{QUESTION_KIND_LABELS[question.kind]}</Table.Cell
						>
						<Table.Cell class="text-base"
							>{usedInFormsCount(question.id, forms)}</Table.Cell
						>
						<Table.Cell>
							<div class="flex flex-wrap gap-1">
								{#each question.tags as tag (tag)}
									<Badge variant="secondary">{tag}</Badge>
								{/each}
								{#if question.specialCategory}
									<Badge variant="destructive">Special category</Badge>
								{/if}
							</div>
						</Table.Cell>
						<Table.Cell>
							<Button variant="outline" size="sm" onclick={() => onEdit(question.id)}
								>Edit</Button
							>
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
</div>
