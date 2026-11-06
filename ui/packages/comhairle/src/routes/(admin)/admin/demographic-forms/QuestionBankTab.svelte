<script lang="ts">
	import { Copy } from 'lucide-svelte';
	import { Badge } from '$lib/components/ui/badge';
	import TagBadge from './TagBadge.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import * as Table from '$lib/components/ui/table';
	import { SPECIAL_CATEGORY_TAG } from './demographicModel';
	import type { QuestionLatestDto } from '@crownshy/api-client/api';

	type Props = {
		questions: QuestionLatestDto[];
		canEdit: boolean;
		onEdit: (questionId: string) => void;
		onDuplicate: (questionId: string) => void;
	};

	let { questions, canEdit, onEdit, onDuplicate }: Props = $props();

	let search = $state('');
	let filter = $state<string | null>(null);

	const tagNames = (question: QuestionLatestDto) => question.tags.map((tag) => tag.name);

	const matchesFilter = (question: QuestionLatestDto, option: string) =>
		option === SPECIAL_CATEGORY_TAG
			? question.specialCategory
			: tagNames(question).includes(option);

	const filterOptions = $derived([...new Set(questions.flatMap(tagNames)), SPECIAL_CATEGORY_TAG]);
	const countFor = (option: string) =>
		questions.filter((question) => matchesFilter(question, option)).length;
	const visible = $derived(
		questions.filter((question) => {
			const passesFilter = filter === null || matchesFilter(question, filter);
			return (
				passesFilter && question.text.toLowerCase().includes(search.trim().toLowerCase())
			);
		})
	);
</script>

<div class="flex flex-col gap-4">
	<p class="bg-primary/10 rounded-lg px-4 py-3 text-base">
		Build your demographic questions by adding your own or duplicating existing ones. Use them
		in any of your demographic forms. Default questions must be duplicated before editing.
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
					<Table.Head>Version</Table.Head>
					<Table.Head>Demographic category</Table.Head>
					{#if canEdit}
						<Table.Head>Actions</Table.Head>
					{/if}
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each visible as question (question.id)}
					<Table.Row>
						<Table.Cell class="font-semibold">
							{question.text}
						</Table.Cell>
						<Table.Cell class="text-muted-foreground text-sm">
							{question.question_type}
						</Table.Cell>
						<Table.Cell>
							<Badge variant={question.publishedVersion === 0 ? 'draft' : 'primary'}>
								{question.publishedVersion === 0
									? 'Unpublished'
									: `v${question.publishedVersion}`}
								{#if question.draftVersion !== 0}
									, draft v{question.draftVersion}
								{/if}
							</Badge>
						</Table.Cell>
						<Table.Cell>
							<div class="flex flex-wrap gap-1">
								{#each tagNames(question) as tag (tag)}
									<TagBadge {tag} />
								{/each}
								{#if question.specialCategory}
									<TagBadge tag={SPECIAL_CATEGORY_TAG} />
								{/if}
							</div>
						</Table.Cell>
						{#if canEdit}
							<Table.Cell>
								<div class="flex gap-2">
									{#if question.isDefault}
										<Badge variant="secondary" class="text-sm">Default</Badge>
									{:else}
										<Button
											variant="outline"
											size="sm"
											onclick={() => onEdit(question.id)}>Edit</Button
										>
									{/if}
									<Button
										variant="outline"
										size="icon"
										class="size-8"
										title="Duplicate question"
										aria-label={`Duplicate ${question.text}`}
										onclick={() => onDuplicate(question.id)}
									>
										<Copy class="size-4" />
									</Button>
								</div>
							</Table.Cell>
						{/if}
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
</div>
