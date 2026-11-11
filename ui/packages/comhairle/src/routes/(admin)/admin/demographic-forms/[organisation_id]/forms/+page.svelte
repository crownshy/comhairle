<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import { Copy, Edit } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Table from '$lib/components/ui/table';
	import TagBadge from '../TagBadge.svelte';
	import { SPECIAL_CATEGORY_TAG } from '../../demographicModel';
	import { specialCategoryCount } from '../../utils';
	import TabContent from '$lib/components/TabContent.svelte';
	import { resolve } from '$app/paths';
	import Callout from '$lib/components/Callout.svelte';

	const { data, params } = $props();
</script>

<TabContent>
	{#if data.forms.length === 0}
		<Callout class="mx-auto flex max-w-150 flex-col items-center gap-y-3 p-5">
			No forms exist yet, create a new form
			<Button
				href={resolve(`/(admin)/admin/demographic-forms/[organisation_id]/forms/new`, {
					organisation_id: params.organisation_id
				})}
			>
				Add a form
			</Button>
		</Callout>
	{:else}
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
				{#each data.forms as form (form.id)}
					{@const specialCount = specialCategoryCount(form, data.questions)}
					<Table.Row>
						{#if data.canEdit}
							<Table.Cell class="pr-2">
								<div class="text-foreground/70 flex flex-col gap-y-1">
									<Button
										variant="ghost"
										class="size-6"
										title="Edit form"
										aria-label="Edit {form.name}"
										href={resolve(
											'/(admin)/admin/demographic-forms/[organisation_id]/forms/edit/[form_id]',
											{
												organisation_id: params.organisation_id,
												form_id: form.form_id
											}
										)}
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
								{#if specialCount > 0}
									<TagBadge
										tag={SPECIAL_CATEGORY_TAG}
										label="{specialCount} special category"
									/>
								{/if}
							</div>
						</Table.Cell>
						<Table.Cell>
							<span class="flex flex-col items-start gap-1">
								<Badge variant={form.publishedVersion === 0 ? 'draft' : 'primary'}>
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
			</Table.Body>
		</Table.Root>
	{/if}
</TabContent>
