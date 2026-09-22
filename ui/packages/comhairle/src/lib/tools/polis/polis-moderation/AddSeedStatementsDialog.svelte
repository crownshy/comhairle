<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Spinner } from '$lib/components/ui/spinner';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import { Plus, Upload } from '@lucide/svelte';
	import SeedStatementsPreview, { type SeedDraft } from './SeedStatementsPreview.svelte';
	import { parseSeedCsv } from './seedCsv';

	type Props = {
		workflowStepId: string;
		/** Every statement already in the step, rejected ones included, for duplicate flags. */
		existingStatements: string[];
		// Called after seeds are posted + synced so the parent can refresh its data.
		onSeeded: () => void | Promise<void>;
	};

	let { workflowStepId, existingStatements, onSeeded }: Props = $props();

	let open = $state(false);
	let draftText = $state('');
	let addingSeed = $state(false);
	let fileInput = $state<HTMLInputElement>();

	// The parsed file waiting on the admin's confirmation. Nothing is posted while this is
	// on screen, which is the point of #1218: the old importer posted straight from the file.
	let drafts = $state<SeedDraft[]>([]);
	let skippedHeader = $state<string | null>(null);
	let parseProblems = $state<string[]>([]);
	let previewing = $state(false);
	let importing = $state(false);
	let nextDraftId = 0;
	const postable = $derived(
		drafts.map((draft) => draft.text.trim()).filter((text) => text.length > 0)
	);

	// Number of statements posted so far / in the current batch, for the progress
	// readout while a post or import is in flight.
	let postedCount = $state(0);
	let totalCount = $state(0);

	// A post is in flight: gate the whole dialog until it settles.
	let busy = $derived(addingSeed || importing);

	type SeedPostOutcome = {
		posted: number;
		/** The statement that failed, or null when the whole batch landed. */
		failedText: string | null;
	};

	/**
	 * Posts each statement in order, stopping at the first failure. Seeds go to the active
	 * poll server-side via PolisPostSeed (no browser-side Polis auth / CORS). Posting is
	 * one-way, so a partial batch is a real state the admin has to be told about rather than
	 * something to roll back.
	 */
	async function postSeeds(texts: string[]): Promise<SeedPostOutcome> {
		postedCount = 0;
		totalCount = texts.length;

		for (const statement_text of texts) {
			const result = await tryCatchAsync(() =>
				apiClient.PolisPostSeed({ workflow_step_id: workflowStepId, statement_text })
			);
			if (result.err !== null) {
				console.error('PolisPostSeed failed', result.err);
				return { posted: postedCount, failedText: statement_text };
			}
			postedCount += 1;
		}

		return { posted: postedCount, failedText: null };
	}

	/**
	 * Re-syncs so the new comments come back with their real Polis-issued ids, then hands off
	 * to the parent to refresh. Runs after a partial batch too, so whatever landed is visible.
	 */
	async function refreshAfterSeeding() {
		const synced = await tryCatchAsync(() =>
			apiClient.PolisSyncStatementAux({ workflow_step_id: workflowStepId })
		);
		if (synced.err !== null) {
			console.error('PolisSyncStatementAux failed after seeding', synced.err);
			notifications.send({
				priority: 'ERROR',
				message:
					'Statements were posted but the list could not be refreshed. Use Sync from Polis.'
			});
			return;
		}
		await onSeeded();
	}

	function truncate(text: string): string {
		return text.length > 60 ? `${text.slice(0, 60)}…` : text;
	}

	async function addSeed() {
		const text = draftText.trim();
		if (!text || busy) return;
		addingSeed = true;

		const outcome = await postSeeds([text]);
		if (outcome.posted > 0) await refreshAfterSeeding();
		addingSeed = false;

		if (outcome.failedText !== null) {
			notifications.send({ priority: 'ERROR', message: 'Failed to add statement' });
			return;
		}

		draftText = '';
		open = false;
		notifications.send({ priority: 'INFO', message: 'Seed statement added' });
	}

	/** Reads and parses the picked file into the preview. Posts nothing. */
	async function previewCsv(event: Event) {
		const input = event.currentTarget as HTMLInputElement;
		const file = input.files?.[0];
		// Reset so picking the same file again fires onchange.
		input.value = '';
		if (!file || busy) return;

		if (!file.name.toLowerCase().endsWith('.csv')) {
			notifications.send({ priority: 'ERROR', message: 'Only CSV files are allowed' });
			return;
		}

		const result = await tryCatchAsync(async () => parseSeedCsv(await file.text()));
		if (result.err !== null) {
			console.error('Reading the seed CSV failed', result.err);
			notifications.send({ priority: 'ERROR', message: 'Could not read that CSV' });
			return;
		}

		const parsed = result.ok;
		if (parsed.statements.length === 0) {
			notifications.send({
				priority: 'ERROR',
				message: 'No statements found in that file'
			});
			return;
		}

		drafts = parsed.statements.map((text) => ({ id: nextDraftId++, text }));
		skippedHeader = parsed.header;
		parseProblems = parsed.problems;
		previewing = true;
	}

	function discardImport() {
		drafts = [];
		skippedHeader = null;
		parseProblems = [];
		previewing = false;
	}

	async function confirmImport() {
		const texts = postable;
		if (texts.length === 0 || busy) return;
		importing = true;

		const outcome = await postSeeds(texts);
		if (outcome.posted > 0) await refreshAfterSeeding();
		importing = false;

		if (outcome.failedText !== null) {
			// Name the statement and the count: posting is one-way, so the admin needs to know
			// exactly where the batch stopped to pick it up by hand.
			notifications.send({
				priority: 'ERROR',
				message: `Posted ${outcome.posted} of ${texts.length}, then "${truncate(
					outcome.failedText
				)}" failed. The rest were not posted.`
			});
			// Posting walks the non-empty drafts in order, so the first `posted` of them landed.
			// Drop those and leave the rest on screen to retry or remove.
			let landed = outcome.posted;
			drafts = drafts.filter((draft) => {
				if (landed > 0 && draft.text.trim().length > 0) {
					landed -= 1;
					return false;
				}
				return true;
			});
			return;
		}

		discardImport();
		open = false;
		notifications.send({
			priority: 'INFO',
			message: `Imported ${texts.length} statement${texts.length === 1 ? '' : 's'}`
		});
	}

	/** Cancel. Throws the preview away, so the step is left untouched. */
	function closeDialog() {
		discardImport();
		open = false;
	}
</script>

<Button onclick={() => (open = true)} title="Add seed statements as moderator">
	<Plus class="size-4" />
	Add seed statements
</Button>

<!-- onOpenChange covers the close button and Escape; Cancel discards on its own. -->
<Dialog.Root bind:open onOpenChange={(value) => !value && discardImport()}>
	<Dialog.Content
		class="sm:max-w-xl"
		showCloseButton={!busy}
		onInteractOutside={(e) => busy && e.preventDefault()}
		onEscapeKeydown={(e) => busy && e.preventDefault()}
	>
		<Dialog.Header>
			<Dialog.Title>
				{previewing ? 'Check before posting' : 'Add seed statements'}
			</Dialog.Title>
			<Dialog.Description>
				{#if previewing}
					Edit or remove anything that looks wrong. Nothing is posted until you confirm,
					and posting cannot be undone.
				{:else}
					Post statements to seed the conversation, or import many at once from a CSV.
				{/if}
			</Dialog.Description>
		</Dialog.Header>

		<div class="relative flex flex-col gap-3">
			<!-- Dim + block the body while a post is in flight. -->
			{#if busy}
				<div
					class="bg-background/50 absolute inset-0 z-10 flex flex-col items-center justify-center gap-2 backdrop-blur-[1px]"
				>
					<Spinner class="text-muted-foreground size-6" />
					{#if totalCount > 1}
						<p class="text-muted-foreground text-sm">
							Posting {Math.min(postedCount + 1, totalCount)} / {totalCount}
						</p>
					{/if}
				</div>
			{/if}

			{#if previewing}
				<SeedStatementsPreview
					bind:drafts
					header={skippedHeader}
					problems={parseProblems}
					{existingStatements}
					{busy}
				/>
			{:else}
				<label class="text-muted-foreground text-sm font-medium" for="seed-text">
					Write a statement
				</label>
				<Textarea
					id="seed-text"
					bind:value={draftText}
					rows={3}
					placeholder="Write a seed statement…"
					disabled={busy}
				/>

				<div class="text-muted-foreground flex items-center gap-2 text-sm">
					<span>or</span>
					<Button
						variant="secondary"
						size="sm"
						onclick={() => fileInput?.click()}
						disabled={busy}
						title="Import seed statements from a CSV (one statement per line)"
					>
						<Upload class="size-4" />
						Import CSV
					</Button>
					<span>to add many at once</span>
				</div>
				<input
					bind:this={fileInput}
					type="file"
					accept=".csv"
					class="hidden"
					onchange={previewCsv}
				/>
			{/if}
		</div>

		<Dialog.Footer>
			<Button variant="secondary" onclick={closeDialog} disabled={busy}>Cancel</Button>
			{#if previewing}
				<Button onclick={confirmImport} disabled={postable.length === 0 || busy}>
					{importing
						? 'Posting…'
						: `Post ${postable.length} statement${postable.length === 1 ? '' : 's'}`}
				</Button>
			{:else}
				<Button onclick={addSeed} disabled={!draftText.trim() || busy}>
					{addingSeed ? 'Posting…' : 'Post seed'}
				</Button>
			{/if}
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
