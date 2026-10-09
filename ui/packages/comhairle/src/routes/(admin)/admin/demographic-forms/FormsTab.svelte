<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import CreatedBy from './CreatedBy.svelte';
	import { Copy } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Table from '$lib/components/ui/table';
	import {
		conversationCount,
		shortId,
		specialCategoryCount,
		usageSummary,
		type DemographicForm,
		type DemographicQuestion
	} from './demographicPrototypeData';

	type Props = {
		forms: DemographicForm[];
		questions: DemographicQuestion[];
		highlightId?: string | null;
		onNew: () => void;
		onEdit: (formId: string) => void;
		onDuplicate: (formId: string) => void;
	};

	let { forms, questions, highlightId = null, onNew, onEdit, onDuplicate }: Props = $props();
</script>

<div class="flex flex-col gap-8">
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
							<DropdownMenu.Item onSelect={() => onDuplicate(form.id)}
								>{form.name}</DropdownMenu.Item
							>
						{/each}
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</div>
		</div>
	</section>

	<section class="flex flex-col gap-3">
		<h2 class="text-lg font-semibold">Your forms</h2>
		<div
			class="border-border bg-card overflow-hidden rounded-xl border [&_td]:py-3 [&_td]:align-middle [&_tr>*:first-child]:pl-4 [&_tr>*:last-child]:pr-4"
		>
			<Table.Root>
				<Table.Header class="bg-muted">
					<Table.Row>
						<Table.Head>ID</Table.Head>
						<Table.Head>Form</Table.Head>
						<Table.Head>Created by</Table.Head>
						<Table.Head>Questions</Table.Head>
						<Table.Head>Used in</Table.Head>
						<Table.Head>Status</Table.Head>
						<Table.Head class="w-24"><span class="sr-only">Actions</span></Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each forms as form (form.id)}
						{@const special = specialCategoryCount(form, questions)}
						<Table.Row
							class={form.id === highlightId
								? 'bg-primary/15 hover:bg-primary/15'
								: ''}
						>
							<Table.Cell class="text-muted-foreground font-mono text-sm leading-8">
								{shortId('F', form.id)}
							</Table.Cell>
							<Table.Cell>
								<div class="flex flex-col">
									<span class="flex h-8 items-center gap-2">
										{#if form.status === 'draft'}
											<button
												type="button"
												class="text-primary hover:text-primary/80 rounded text-left text-base font-medium hover:underline focus-visible:ring-2 focus-visible:outline-none"
												title="Continue editing this draft"
												onclick={() => onEdit(form.id)}
											>
												{form.name}
											</button>
										{:else}
											<span class="text-base font-medium">{form.name}</span>
										{/if}
									</span>
									<span class="text-muted-foreground text-sm leading-5"
										>{form.editedLabel}</span
									>
								</div>
							</Table.Cell>
							<Table.Cell><CreatedBy name={form.createdBy} /></Table.Cell>
							<Table.Cell>
								<div class="flex flex-col">
									<span class="text-base leading-8"
										>{form.questions.length} questions</span
									>
									{#if special > 0}
										<span class="text-muted-foreground text-sm leading-5">
											{special} special category
										</span>
									{/if}
								</div>
							</Table.Cell>
							<Table.Cell>
								<div class="flex flex-col">
									<span class="text-base leading-8">
										{conversationCount(form) === 0
											? 'Not used yet'
											: `${conversationCount(form)} ${conversationCount(form) === 1 ? 'conversation' : 'conversations'}`}
									</span>
									{#if form.usage.length > 0}
										<span class="text-muted-foreground text-sm leading-5"
											>{usageSummary(form)}</span
										>
									{/if}
								</div>
							</Table.Cell>
							<Table.Cell>
								<div class="flex flex-col items-start">
									<span class="flex h-8 items-center">
										<Badge
											variant={form.status === 'published'
												? 'primary'
												: 'draft'}
										>
											{form.status === 'published'
												? `Published v${form.version}`
												: 'Draft'}
										</Badge>
									</span>
									{#if form.hasUnpublishedChanges}
										<span class="text-muted-foreground text-sm leading-5">
											Unpublished changes
										</span>
									{/if}
								</div>
							</Table.Cell>
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
										onclick={() => onDuplicate(form.id)}
									>
										<Copy class="size-4" />
									</Button>
								</div>
							</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
		</div>
	</section>
</div>
