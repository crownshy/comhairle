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
	import { Textarea } from '$lib/components/ui/textarea';
	import { Trash2, TriangleAlert } from '@lucide/svelte';
	import { findSeedIssues, type SeedStatementIssue } from './seedCsv';

	type Props = {
		/** The parsed statements. Bound so edits and removals reach the posting dialog. */
		drafts: SeedDraft[];
		/** The heading row that was skipped, or null when the file had none. */
		header: string | null;
		/** What the parser objected to in the file, if anything. */
		problems: string[];
		/** Every statement already in the step, rejected ones included, for duplicate flags. */
		existingStatements: string[];
		/** Posting is in flight: freeze the list so it matches what is being posted. */
		busy: boolean;
	};

	let { drafts = $bindable(), header, problems, existingStatements, busy }: Props = $props();

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

<div class="flex flex-col gap-3">
	<div class="flex flex-wrap items-baseline justify-between gap-x-4 gap-y-1">
		<p class="font-medium">
			{drafts.length} statement{drafts.length === 1 ? '' : 's'} read from the file
		</p>
		{#if flaggedCount > 0}
			<p class="text-muted-foreground text-sm">
				{flaggedCount} worth checking
			</p>
		{/if}
	</div>

	{#if header !== null}
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
		<!-- Capped so a long import scrolls inside the dialog rather than off the screen. -->
		<ul class="flex max-h-72 flex-col gap-3 overflow-y-auto pr-1">
			{#each drafts as draft, index (draft.id)}
				{@const issue = issues[index]}
				<li class="flex items-start gap-2">
					<div class="flex flex-1 flex-col gap-1">
						<Textarea
							bind:value={draft.text}
							rows={2}
							disabled={busy}
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
						disabled={busy}
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
