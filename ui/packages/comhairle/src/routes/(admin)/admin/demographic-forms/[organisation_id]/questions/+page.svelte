<script lang="ts">
	import { Copy } from 'lucide-svelte';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import * as Table from '$lib/components/ui/table';
	import TagBadge from '../TagBadge.svelte';
	import { SPECIAL_CATEGORY_TAG, type TagName } from '../../demographicModel';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import TabContent from '$lib/components/TabContent.svelte';
	import SearchBar from '$lib/components/SearchBar.svelte';

	const { data } = $props();

	let search = $state('');
	let filter = $state<TagName | undefined>();
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
								onclick={(e) => {
									const radio = e.currentTarget;
									console.log(radio);
									console.log(radio.checked);
									if (filter === tag.name) {
										radio.checked = false;
										filter = undefined;
										return;
									}
									filter = tag.name as TagName;
								}}
							/>
							<TagBadge tag={tag.name as TagName} />
						</label>
					{/each}
				{/if}
			{/await}
		</div>
		<SearchBar bind:value={search} placeholder="Search questions..." />
	</div>

	<Table.Root>
		<Table.Header>
			<Table.Row>
				<Table.Head>Question</Table.Head>
				<Table.Head>Question type</Table.Head>
				<Table.Head>Version</Table.Head>
				<Table.Head>Demographic category</Table.Head>
				{#if data.canEdit}
					<Table.Head class="sr-only">Actions</Table.Head>
				{/if}
			</Table.Row>
		</Table.Header>
		<Table.Body>
			{#await data.streamedQuestions}
				<Table.Row>
					<Table.Cell>
						<Skeleton class="h-9 w-50" />
					</Table.Cell>
					<Table.Cell>
						<Skeleton class="h-9 w-10" />
					</Table.Cell>
					<Table.Cell>
						<Skeleton class="h-9 w-10" />
					</Table.Cell>
					<Table.Cell>
						<Skeleton class="h-9 w-10" />
					</Table.Cell>
					<Table.Cell>
						<Skeleton class="h-9 w-auto" />
					</Table.Cell>
				</Table.Row>
			{:then questions}
				{#if questions.err !== null}
					<ErrorMessage>Couldn't load questions, please try again</ErrorMessage>
				{:else}
					{#each questions.ok as question (question.id)}
						<Table.Row>
							<Table.Cell class="w-50 truncate font-semibold" title={question.text}>
								{question.text}
							</Table.Cell>
							<Table.Cell class="text-muted-foreground text-sm">
								{question.question_type}
							</Table.Cell>
							<Table.Cell>
								<Badge
									variant={question.publishedVersion === 0 ? 'draft' : 'primary'}
								>
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
									{#each question.tags as tag (tag)}
										<TagBadge tag={tag.name as TagName} />
									{/each}
									{#if question.specialCategory}
										<TagBadge tag={SPECIAL_CATEGORY_TAG} />
									{/if}
								</div>
							</Table.Cell>
							{#if data.canEdit}
								<Table.Cell>
									<div class="flex gap-2">
										{#if question.isDefault}
											<Badge variant="secondary" class="text-sm"
												>Default</Badge
											>
										{:else}
											<Button variant="outline" size="sm">Edit</Button>
										{/if}
										<Button
											variant="outline"
											size="icon"
											class="size-8"
											title="Duplicate question"
											aria-label={`Duplicate ${question.text}`}
										>
											<Copy class="size-4" />
										</Button>
									</div>
								</Table.Cell>
							{/if}
						</Table.Row>
					{/each}
				{/if}
			{/await}
		</Table.Body>
	</Table.Root>
</TabContent>
