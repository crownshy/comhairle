<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import { Copy, Edit } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Table from '$lib/components/ui/table';
	import TagBadge from '../TagBadge.svelte';
	import { SPECIAL_CATEGORY_TAG } from '../../demographicModel';
	import { specialCategoryCount } from '../../utils';
	import { Skeleton } from '$lib/components/ui/skeleton';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import TabContent from '$lib/components/TabContent.svelte';

	const { data } = $props();
</script>

<TabContent>
	<Table.Root>
		<Table.Header>
			<Table.Row>
				{#if data.canEdit}
					<Table.Head>
						<span class="sr-only">Actions</span>
					</Table.Head>
				{/if}
				<Table.Head>Form</Table.Head>
				<Table.Head>Questions</Table.Head>
				<Table.Head>Status</Table.Head>
			</Table.Row>
		</Table.Header>
		<Table.Body>
			{#await data.streamedForms}
				<Table.Row>
					<Table.Cell class="pr-2">
						<Skeleton class="h-14 w-6" />
					</Table.Cell>
					<Table.Cell>
						<Skeleton class="h-7 w-30" />
					</Table.Cell>
					<Table.Cell>
						<Skeleton class="h-7 w-19" />
					</Table.Cell>
					<Table.Cell>
						<Skeleton class="h-7 w-auto" />
					</Table.Cell>
				</Table.Row>
			{:then forms}
				{#if forms.err !== null}
					<ErrorMessage>Could not load forms, please try again</ErrorMessage>
				{:else}
					{#each forms.ok as form (form.id)}
						<Table.Row>
							{#if data.canEdit}
								<Table.Cell class="pr-2">
									<div class="flex flex-col gap-y-1">
										<Button
											variant="ghost"
											class="size-6"
											title="Edit form"
											aria-label="Edit {form.name}"
										>
											<Edit class="size-4" />
										</Button>
										<Button
											variant="ghost"
											class="size-6"
											title="Duplicate form"
											aria-label="Duplicate {form.name}"
										>
											<Copy class="size-4" />
										</Button>
									</div>
								</Table.Cell>
							{/if}
							<Table.Cell class="text-base font-semibold" title={form.name}>
								<div class="w-28 truncate">
									{form.name}
								</div>
							</Table.Cell>
							<Table.Cell>
								<div class="flex flex-col items-start gap-x-1">
									<span class="text-muted-foreground text-sm font-medium">
										{form.entries.length} questions
									</span>
									{#await data.streamedQuestions}
										<Skeleton class="h-3 w-5" />
									{:then questions}
										{#if questions.ok !== null}
											{@const specialCount = specialCategoryCount(
												form,
												questions.ok
											)}
											{#if specialCount > 0}
												<TagBadge
													tag={SPECIAL_CATEGORY_TAG}
													label="{specialCount} special category"
												/>
											{/if}
										{/if}
									{/await}
								</div>
							</Table.Cell>
							<Table.Cell>
								<span class="flex flex-col items-start gap-1">
									<Badge
										variant={form.publishedVersion === 0 ? 'draft' : 'primary'}
									>
										{form.publishedVersion === 0
											? 'Draft'
											: `Published v${form.publishedVersion}`}
									</Badge>
									{#if form.draftVersion !== 0 && form.publishedVersion !== 0}
										<Badge variant="draft">Draft v{form.draftVersion}</Badge>
									{/if}
								</span>
							</Table.Cell>
						</Table.Row>
					{/each}
				{/if}
			{/await}
		</Table.Body>
	</Table.Root>
</TabContent>
