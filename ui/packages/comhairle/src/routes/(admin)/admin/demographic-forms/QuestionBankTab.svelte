<script lang="ts">
	import { Copy } from 'lucide-svelte';
	import { Badge } from '$lib/components/ui/badge';
	import TagBadge from './TagBadge.svelte';
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
		onDuplicate: (questionId: string) => void;
	};

	const SPECIAL_FILTER = 'Special category';

	let { questions, forms, onEdit, onDuplicate }: Props = $props();

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
		Build your demographic questions by adding your own or duplicating existing ones. Use them
		in any of your demographic forms.
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
					<TagBadge
						tag={option}
						label={`${option} (${countFor(option)})`}
						class="px-3 py-1 {filter === option
							? 'ring-foreground font-semibold ring-2 ring-offset-2'
							: 'opacity-80 hover:opacity-100'}"
					/>
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

	<div
		class="border-border bg-card overflow-hidden rounded-xl border [&_td]:py-3 [&_td]:align-middle [&_tr>*:first-child]:pl-4 [&_tr>*:last-child]:pr-4"
	>
		<Table.Root>
			<Table.Header class="bg-muted">
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
						<Table.Cell class="text-base leading-8 font-medium"
							>{question.text}</Table.Cell
						>
						<Table.Cell class="text-muted-foreground text-sm leading-8"
							>{QUESTION_KIND_LABELS[question.kind]}</Table.Cell
						>
						<Table.Cell class="text-muted-foreground text-sm leading-8"
							>{usedInFormsCount(question.id, forms)}</Table.Cell
						>
						<Table.Cell>
							<div class="flex flex-wrap gap-1">
								{#each question.tags as tag (tag)}
									<TagBadge {tag} />
								{/each}
								{#if question.specialCategory}
									<TagBadge tag="Special category" />
								{/if}
							</div>
						</Table.Cell>
						<Table.Cell>
							<div class="flex gap-2">
								<Button
									variant="outline"
									size="sm"
									onclick={() => onEdit(question.id)}>Edit</Button
								>
								<Button
									variant="outline"
									size="sm"
									aria-label={`Duplicate ${question.text}`}
									onclick={() => onDuplicate(question.id)}
								>
									<Copy class="size-4" />Duplicate
								</Button>
							</div>
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
</div>
