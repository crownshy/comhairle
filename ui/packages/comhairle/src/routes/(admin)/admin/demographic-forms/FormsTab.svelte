<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import TagBadge from './TagBadge.svelte';
	import { Copy } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Table from '$lib/components/ui/table';
	import { SPECIAL_CATEGORY_TAG } from './demographicModel';
	import { specialCategoryCount } from './utils';
	import type { DemographicFormLatestDto, QuestionLatestDto } from '@crownshy/api-client/api';

	type Props = {
		forms: DemographicFormLatestDto[];
		questions: QuestionLatestDto[];
		canEdit: boolean;
		onNew: () => void;
		onEdit: (formId: string) => void;
		onCopy: (formId: string) => void;
	};

	let { forms, questions, canEdit, onNew, onEdit, onCopy }: Props = $props();
</script>

<div class="flex flex-col gap-8">
	{#if canEdit}
		<section class="flex flex-col gap-3">
			<h2 class="text-lg font-semibold">Start from</h2>
			<div class="grid gap-4 md:grid-cols-2">
				<div
					class="border-border bg-card flex flex-col items-start gap-2 rounded-xl border p-5"
				>
					<h3 class="text-base font-semibold">Blank form</h3>
					<p class="text-muted-foreground text-base">
						Pick questions from your question bank, or write new ones as you go.
					</p>
					<Button variant="outline" onclick={onNew}>Start blank</Button>
				</div>
				<div
					class="border-border bg-card flex flex-col items-start gap-2 rounded-xl border p-5"
				>
					<h3 class="text-base font-semibold">Copy an existing form</h3>
					<p class="text-muted-foreground text-base">
						Duplicate one of your forms and adjust it, without changing the original.
					</p>
					<DropdownMenu.Root>
						<DropdownMenu.Trigger>
							{#snippet child({ props })}
								<Button variant="outline" {...props}>Choose a form</Button>
							{/snippet}
						</DropdownMenu.Trigger>
						<DropdownMenu.Content align="start">
							{#each forms as form (form.id)}
								<DropdownMenu.Item onSelect={() => onCopy(form.id)}>
									{form.name}
								</DropdownMenu.Item>
							{/each}
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				</div>
			</div>
		</section>
	{/if}

	<section class="flex flex-col gap-3">
		<h2 class="text-lg font-semibold">Your forms</h2>
		<div
			class="border-border bg-card [&_th]:text-muted-foreground overflow-hidden rounded-xl border [&_td]:py-4 [&_tr>*:first-child]:pl-4 [&_tr>*:last-child]:pr-4"
		>
			<Table.Root>
				<Table.Header class="bg-muted">
					<Table.Row>
						<Table.Head>Form</Table.Head>
						<Table.Head>Questions</Table.Head>
						<Table.Head>Status</Table.Head>
						{#if canEdit}
							<Table.Head>
								<span class="sr-only">Actions</span>
							</Table.Head>
						{/if}
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each forms as form (form.id)}
						{@const specialCount = specialCategoryCount(form, questions)}
						<Table.Row>
							<Table.Cell class="w-50 font-semibold">
								{form.name}
							</Table.Cell>
							<Table.Cell>
								<div class="flex flex-col items-start gap-x-1">
									{form.entries.length} questions
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
							{#if canEdit}
								<Table.Cell>
									<div class="flex items-center gap-2">
										<Button
											variant="outline"
											size="sm"
											onclick={() => onEdit(form.id)}>Edit</Button
										>
										<Button
											variant="outline"
											size="icon"
											class="size-8"
											title="Duplicate form"
											aria-label={`Duplicate ${form.name}`}
											onclick={() => onCopy(form.id)}
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
	</section>
</div>
