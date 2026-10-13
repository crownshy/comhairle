<script lang="ts" module>
	/** One parsed statement, editable before it is posted. `id` is a stable `{#each}` key. */
	export type SeedDraft = {
		id: number;
		text: string;
	};
</script>

<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import * as Select from '$lib/components/ui/select';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Trash2, TriangleAlert } from '@lucide/svelte';
	import { pluralise } from '$lib/utils/pluralise';
	import {
		findSeedIssues,
		type ParsedSeedCsv,
		type SeedStatementIssue
	} from '$lib/utils/seedCsv';

	type Props = {
		/** The parsed statements. Bound so edits and removals reach the posting dialog. */
		drafts: SeedDraft[];
		/** The parse the drafts came from: skipped heading, parser problems, columns. */
		parsed: ParsedSeedCsv;
		/** The admin picked another column to read the statements from. */
		onColumnChange: (index: number) => void;
		/** Every statement already in the step, rejected ones included, for duplicate flags. */
		existingStatements: string[];
		/** Posting is in flight: freeze the list so it matches what is being posted. */
		posting: boolean;
	};

	let {
		drafts = $bindable(),
		parsed,
		onColumnChange,
		existingStatements,
		posting
	}: Props = $props();

	const { header, problems, columns, column } = $derived(parsed);
	const columnHeading = $derived(columns.find((option) => option.index === column)?.heading);

	const issues = $derived(
		findSeedIssues(
			drafts.map((draft) => draft.text),
			existingStatements
		)
	);

	const ISSUE_LABELS: Record<SeedStatementIssue, string> = {
		empty: 'Empty, will be skipped',
		'duplicate-in-file': 'Repeated in this file',
		'duplicate-in-step': 'Already in this step'
	};

	const flaggedCount = $derived(issues.filter(Boolean).length);

	function removeDraft(id: number) {
		drafts = drafts.filter((draft) => draft.id !== id);
	}
</script>

<div class="flex min-h-0 flex-1 flex-col gap-3">
	{#if columns.length > 0}
		<div class="flex flex-wrap items-center gap-x-3 gap-y-1">
			<span class="font-medium">Take statements from</span>
			<Select.Root
				type="single"
				value={column === null ? undefined : String(column)}
				onValueChange={(value) => value && onColumnChange(Number(value))}
				disabled={posting}
			>
				<Select.Trigger class="min-w-48">{columnHeading}</Select.Trigger>
				<Select.Content>
					{#each columns as option (option.index)}
						<Select.Item value={String(option.index)}>{option.heading}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
			<p class="text-muted-foreground text-sm">
				The file has {columns.length} columns. Switching reads the list again and drops your edits.
			</p>
		</div>
	{/if}

	<div class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1">
		<p class="font-medium">
			{drafts.length}
			{pluralise(drafts.length, 'statement')} read from the file
		</p>
		{#if flaggedCount > 0}
			<p class="text-muted-foreground text-sm">
				{flaggedCount} worth checking
			</p>
		{/if}
	</div>

	{#if header !== null && columns.length === 0}
		<p class="text-muted-foreground text-sm">
			Skipped the heading row "{header}".
		</p>
	{/if}

	<!-- A malformed file still parses, just wrongly: an unterminated quote swallows everything
	     after it into one statement, which looks plausible in the list below unless we say so. -->
	{#if problems.length > 0}
		<p class="text-destructive text-sm">
			That file is not quite valid CSV, so check the statements carefully: {problems[0]}
		</p>
	{/if}

	{#if drafts.length === 0}
		<p class="text-muted-foreground bg-muted rounded-md p-4 text-center">
			Nothing left to post. Cancel and pick another file.
		</p>
	{:else}
		<!-- Fills the dialog's fixed height so a long import scrolls inside it rather than off the screen. -->
		<ul class="flex min-h-0 flex-1 flex-col gap-3 overflow-y-auto pr-1">
			{#each drafts as draft, index (draft.id)}
				{@const issue = issues[index]}
				<li class="flex items-start gap-2">
					<div class="flex flex-1 flex-col gap-1">
						<Textarea
							bind:value={draft.text}
							rows={2}
							disabled={posting}
							aria-label={`Statement ${index + 1}`}
						/>
						{#if issue}
							<Badge variant="secondary" class="self-start">
								<TriangleAlert />
								{ISSUE_LABELS[issue]}
							</Badge>
						{/if}
					</div>
					<Button
						variant="ghost"
						size="icon"
						disabled={posting}
						onclick={() => removeDraft(draft.id)}
						title={`Remove statement ${index + 1}`}
					>
						<Trash2 class="size-4" />
					</Button>
				</li>
			{/each}
		</ul>
	{/if}
</div>
