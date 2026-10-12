<script lang="ts">
	import { Copy } from 'lucide-svelte';
	import { Badge } from '$lib/components/ui/badge';
	import TagBadge from './TagBadge.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import * as Table from '$lib/components/ui/table';
	import {
		QUESTION_KIND_LABELS,
		shortId,
		SPECIAL_CATEGORY_TAG,
		type SavedQuestion
	} from './demographicModel';

	type Props = {
		questions: SavedQuestion[];
		canEdit: boolean;
		onEdit: (questionId: string) => void;
		onDuplicate: (questionId: string) => void;
		highlightId?: string | null;
	};

	let { questions, canEdit, onEdit, onDuplicate, highlightId = null }: Props = $props();

	$effect(() => {
		if (!highlightId) return;
		const id = highlightId;
		const timer = setTimeout(() => {
			document
				.querySelector(`[data-row-id="${id}"]`)
				?.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
		}, 50);
		return () => clearTimeout(timer);
	});

	let search = $state('');
	let filter = $state<string | null>(null);

	const tagNames = (question: SavedQuestion) => question.tags.map((tag) => tag.name);
	const matchesFilter = (question: SavedQuestion, option: string) =>
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
					<Table.Head>ID</Table.Head>
					<Table.Head>Question</Table.Head>
					<Table.Head>Question type</Table.Head>
					<Table.Head>Version</Table.Head>
					<Table.Head>Demographic category</Table.Head>
					{#if canEdit}<Table.Head class="w-24"
							><span class="sr-only">Actions</span></Table.Head
						>{/if}
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each visible as question (question.id)}
					<Table.Row
						data-row-id={question.id}
						class={question.id === highlightId
							? 'bg-primary/15 hover:bg-primary/15'
							: ''}
					>
						<Table.Cell class="text-muted-foreground font-mono text-sm leading-8">
							{shortId('Q', question.id)}
						</Table.Cell>
						<Table.Cell class="text-base leading-8 font-medium"
							>{question.text}</Table.Cell
						>
						<Table.Cell class="text-muted-foreground text-sm leading-8"
							>{QUESTION_KIND_LABELS[question.kind]}</Table.Cell
						>
						<Table.Cell>
							<Badge
								variant={question.publishedVersion === null ? 'draft' : 'primary'}
							>
								{question.publishedVersion === null
									? 'Unpublished'
									: `v${question.publishedVersion}`}
								{#if question.draftVersion !== null}
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
