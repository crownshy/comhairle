<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Table from '$lib/components/ui/table';
	import {
		specialCategoryCount,
		type DemographicForm,
		type DemographicQuestion
	} from './demographicPrototypeData';

	type Props = {
		forms: DemographicForm[];
		questions: DemographicQuestion[];
		onNew: () => void;
		onEdit: (formId: string) => void;
		onCopy: (formId: string) => void;
	};

	let { forms, questions, onNew, onEdit, onCopy }: Props = $props();
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
							<DropdownMenu.Item onSelect={() => onCopy(form.id)}
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
		<div class="border-border bg-card rounded-xl border">
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>Form</Table.Head>
						<Table.Head>Created by</Table.Head>
						<Table.Head>Country</Table.Head>
						<Table.Head>Questions</Table.Head>
						<Table.Head>Used in</Table.Head>
						<Table.Head>Status</Table.Head>
						<Table.Head class="w-24"><span class="sr-only">Actions</span></Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each forms as form (form.id)}
						{@const special = specialCategoryCount(form, questions)}
						<Table.Row>
							<Table.Cell>
								<div class="flex flex-col">
									<span class="flex items-center gap-2">
										<span class="text-base font-medium">{form.name}</span>
										{#if form.isNewlyCreated}<Badge variant="primary">New</Badge
											>{/if}
									</span>
									<span class="text-muted-foreground text-sm"
										>{form.editedLabel}</span
									>
								</div>
							</Table.Cell>
							<Table.Cell class="text-base">{form.createdBy}</Table.Cell>
							<Table.Cell class="text-base">{form.country}</Table.Cell>
							<Table.Cell>
								<div class="flex flex-col items-start gap-1">
									<span class="text-base">{form.questions.length} questions</span>
									{#if special > 0}
										<Badge variant="destructive"
											>{special} special category</Badge
										>
									{/if}
								</div>
							</Table.Cell>
							<Table.Cell class="text-base">
								{form.usedInConversations === 0
									? 'Not used yet'
									: `${form.usedInConversations} ${form.usedInConversations === 1 ? 'conversation' : 'conversations'}`}
							</Table.Cell>
							<Table.Cell>
								<Badge variant={form.status === 'published' ? 'primary' : 'draft'}>
									{form.status === 'published'
										? `Published v${form.version}`
										: 'Draft'}
								</Badge>
							</Table.Cell>
							<Table.Cell>
								<Button variant="outline" size="sm" onclick={() => onEdit(form.id)}
									>Edit</Button
								>
							</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
		</div>
	</section>
</div>
