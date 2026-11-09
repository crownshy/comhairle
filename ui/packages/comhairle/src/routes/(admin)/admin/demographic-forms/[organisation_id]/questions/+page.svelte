<script lang="ts">
	import { Copy, Edit } from 'lucide-svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import * as Table from '$lib/components/ui/table';
	import TagBadge from '../TagBadge.svelte';
	import { SPECIAL_CATEGORY_TAG, type TagName } from '../../demographicModel';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import TabContent from '$lib/components/TabContent.svelte';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import { updateQueryParam } from '$lib/utils.svelte';
	import { useDebounce } from 'runed';
	import { page } from '$app/state';
	import Demographics from '$lib/interfaces/Demographics';

	const { data } = $props();
</script>

<TabContent>
	<div class="mb-5 flex flex-wrap items-center justify-between gap-3">
		<div class="flex flex-wrap gap-2">
			{#await data.streamedTags}
				{#each { length: 3 }}
					<Skeleton class="h-4 w-10 rounded-xl" />
				{/each}
			{:then tags}
				{#if tags.err !== null}
					<ErrorMessage>Could not load tags, please try again</ErrorMessage>
				{:else}
					{#each tags.ok as tag (tag.id)}
						<label class="cursor-pointer">
							<input
								type="radio"
								name="filter"
								class="hidden"
								defaultChecked={page.url.searchParams.get('filter') === tag.name}
								onclick={(e) => {
									const radio = e.currentTarget;

									updateQueryParam('filter', tag.name, {
										onDelete: () => {
											radio.checked = false;
										}
									});
								}}
							/>
							<TagBadge tag={tag.name as TagName} />
						</label>
					{/each}
				{/if}
			{/await}
		</div>
		<SearchBar
			oninput={useDebounce((e) => {
				const input = e.target as HTMLInputElement | null;
				if (!input) {
					return;
				}
				updateQueryParam('search', input.value);
			}, 250)}
			value={page.url.searchParams.get('search') ?? undefined}
			placeholder="Search questions..."
		/>
	</div>

	<Table.Root>
		<Table.Header>
			<Table.Row>
				{#if data.canEdit}
					<Table.Head class="sr-only">Actions</Table.Head>
				{/if}
				<Table.Head>Question</Table.Head>
				<Table.Head>Question type</Table.Head>
				<Table.Head>Version</Table.Head>
				<Table.Head>Demographic category</Table.Head>
			</Table.Row>
		</Table.Header>
		<Table.Body>
			{#await data.streamedQuestions}
				<Table.Row>
					<Table.Cell class="pr-2">
						<Skeleton class="h-14 w-6" />
					</Table.Cell>
					<Table.Cell>
						<Skeleton class="h-7 w-54" />
					</Table.Cell>
					<Table.Cell>
						<Skeleton class="h-7 w-17" />
					</Table.Cell>
					<Table.Cell>
						<Skeleton class="h-7 w-8" />
					</Table.Cell>
					<Table.Cell>
						<Skeleton class="h-7 w-auto" />
					</Table.Cell>
				</Table.Row>
			{:then questions}
				{#if questions.err !== null}
					<ErrorMessage>Couldn't load questions, please try again</ErrorMessage>
				{:else}
					{#each questions.ok as question (question.id)}
						{console.log(question)}
						<Table.Row>
							{#if data.canEdit}
								<Table.Cell class="pr-2">
									<div class="text-foreground/70 flex flex-col gap-y-1">
										{#if !Demographics.isDefaultQuestion(question)}
											<Button
												variant="ghost"
												class="size-6"
												title="Edit question"
												aria-label="Edit {question.text}"
											>
												<Edit class="size-4" />
											</Button>
										{/if}
										<Button
											variant="ghost"
											class="size-6"
											title="Duplicate question"
											aria-label="Duplicate {question.text}"
										>
											<Copy class="size-4" />
										</Button>
									</div>
								</Table.Cell>
							{/if}
							<Table.Cell class=" font-semibold" title={question.text}>
								<div class="w-50 truncate">
									{question.text}
								</div>
							</Table.Cell>
							<Table.Cell class="text-muted-foreground text-sm">
								{Demographics.getQuestionFromType(question.question_type)}
							</Table.Cell>
							<Table.Cell>
								{#if Demographics.isDefaultQuestion(question)}
									<Badge variant="secondary" class="text-xs">Default</Badge>
								{:else}
									<Badge
										variant={question.publishedVersion === 0
											? 'draft'
											: 'primary'}
									>
										{question.publishedVersion === 0
											? 'Unpublished'
											: `v${question.publishedVersion}`}
										{#if question.draftVersion !== 0}
											, draft v{question.draftVersion}
										{/if}
									</Badge>
								{/if}
							</Table.Cell>
							<Table.Cell>
								<div class="flex flex-wrap gap-1">
									{#each question.tags as tag (tag)}
										<TagBadge tag={tag.name as TagName} />
									{/each}
									{#if question.specialCategory}
										<TagBadge tag={SPECIAL_CATEGORY_TAG} />
									{/if}
								</div>
							</Table.Cell>
						</Table.Row>
					{:else}
						<Table.Row>
							<Table.Cell class="italic font-semibold">No questions found</Table.Cell>
						</Table.Row>
					{/each}
				{/if}
			{/await}
		</Table.Body>
	</Table.Root>
</TabContent>
