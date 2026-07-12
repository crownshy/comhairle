<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { Button, LoadingButton } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import * as Card from '$lib/components/ui/card';
	import * as Dialog from '$lib/components/ui/dialog';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import { Check, X, Plus, Upload, RefreshCw } from '@lucide/svelte';

	let {
		workflowStepId,
		statements: initialStatements
	}: {
		workflowStepId: string;
		statements: PolisStatementAux[];
	} = $props();

	// Local mutable copy so optimistic accept/reject re-renders without a refetch.
	// Reset whenever the load re-runs (sync/seed invalidate `polis:statement-aux`).
	let statements = $state<PolisStatementAux[]>(initialStatements);
	$effect(() => {
		statements = initialStatements;
	});

	// --- Sync from Polis ---
	// Participant submissions only appear here after a sync — aux rows are synced
	// from Polis, not created live. Existing moderation/themes are preserved.
	let syncing = $state(false);

	async function syncFromPolis() {
		if (syncing) return;
		syncing = true;
		try {
			const res = await apiClient.PolisSyncStatementAux({ workflow_step_id: workflowStepId });
			notifications.send({
				priority: 'INFO',
				message: `Synced ${res.synced} statement${res.synced === 1 ? '' : 's'} from Polis${
					res.skipped_invalid_xid ? ` (${res.skipped_invalid_xid} skipped)` : ''
				}`
			});
			await invalidate('polis:statement-aux');
		} catch (e) {
			console.error('PolisSyncStatementAux failed', e);
			notifications.send({
				priority: 'ERROR',
				message: 'Failed to sync statements from Polis'
			});
		} finally {
			syncing = false;
		}
	}

	// --- Add seed statements (moderator authoring) ---
	// Seeds are posted server-side to the active poll via PolisPostSeed (no
	// browser-side Polis auth / CORS), then we re-sync so the new comment comes
	// back with its real Polis-issued ids.
	let showAddForm = $state(false);
	let draftText = $state('');
	let addingSeed = $state(false);
	let csvImporting = $state(false);
	let fileInput = $state<HTMLInputElement>();

	async function postSeeds(texts: string[]) {
		for (const statement_text of texts) {
			await apiClient.PolisPostSeed({ workflow_step_id: workflowStepId, statement_text });
		}
		await apiClient.PolisSyncStatementAux({ workflow_step_id: workflowStepId });
		await invalidate('polis:statement-aux');
	}

	async function addSeed() {
		const text = draftText.trim();
		if (!text || addingSeed) return;
		addingSeed = true;
		const result = await tryCatchAsync(() => postSeeds([text]));
		addingSeed = false;
		if (result.err !== null) {
			console.error('PolisPostSeed failed', result.err);
			notifications.send({ priority: 'ERROR', message: 'Failed to add statement' });
			return;
		}
		draftText = '';
		showAddForm = false;
		notifications.send({ priority: 'INFO', message: 'Seed statement added' });
	}

	async function importCsv(e: Event) {
		const input = e.currentTarget as HTMLInputElement;
		const file = input.files?.[0];
		if (!file || csvImporting) return;
		csvImporting = true;

		const result = await tryCatchAsync<number, 'INCORRECT_FILE_TYPE' | 'NO_VALUES_FOUND'>(
			async () => {
				if (!file.name.toLowerCase().endsWith('.csv')) throw 'INCORRECT_FILE_TYPE';

				// One statement per line; strip wrapping quotes and a leading header row.
				const lines = (await file.text())
					.split(/\r?\n/)
					.map((l) => l.replace(/^"(.*)"$/, '$1').trim())
					.filter(Boolean);
				if (['statement', 'statements', 'text'].includes(lines[0]?.toLowerCase())) {
					lines.shift();
				}
				if (!lines.length) throw 'NO_VALUES_FOUND';

				await postSeeds(lines);
				return lines.length;
			}
		);
		csvImporting = false;
		input.value = '';

		if (result.err !== null) {
			console.error('CSV import failed', result.err);
			switch (result.err) {
				case 'INCORRECT_FILE_TYPE':
					notifications.send({
						priority: 'ERROR',
						message: 'Only CSV files are allowed'
					});
					break;
				case 'NO_VALUES_FOUND':
					notifications.send({
						priority: 'ERROR',
						message: 'No statements found in that file'
					});
					break;
				default:
					// postSeeds / network failures are not one of the typed strings.
					notifications.send({ priority: 'ERROR', message: 'CSV import failed' });
			}
			return;
		}

		notifications.send({
			priority: 'INFO',
			message: `Imported ${result.ok} statement${result.ok === 1 ? '' : 's'}`
		});
		showAddForm = false;
	}

	// --- Status filters ---
	type Filter = 'all' | 'seeded' | 'accepted' | 'pending' | 'rejected';
	let filter = $state<Filter>('all');

	const counts = $derived({
		all: statements.length,
		seeded: statements.filter((s) => s.is_seed).length,
		accepted: statements.filter((s) => s.moderation_status === 'accepted').length,
		pending: statements.filter((s) => s.moderation_status === 'pending').length,
		rejected: statements.filter((s) => s.moderation_status === 'rejected').length
	});

	const visible = $derived.by(() => {
		let list = statements;
		if (filter === 'seeded') list = list.filter((s) => s.is_seed);
		else if (filter !== 'all') list = list.filter((s) => s.moderation_status === filter);
		return [...list].sort((a, b) => b.polis_statement_id - a.polis_statement_id);
	});

	const filters: { key: Filter; label: string }[] = [
		{ key: 'all', label: 'All' },
		{ key: 'seeded', label: 'Seeded' },
		{ key: 'accepted', label: 'Accepted' },
		{ key: 'pending', label: 'Pending' },
		{ key: 'rejected', label: 'Rejected' }
	];

	// --- Accept / reject ---
	// Track in-flight requests per aux row so the buttons can disable mid-call.
	let pending = $state<Record<string, boolean>>({});

	async function setStatus(row: PolisStatementAux, status: 'accepted' | 'rejected') {
		if (pending[row.id] || row.moderation_status === status) return;
		const decision = status === 'accepted' ? 'accept' : 'reject';
		pending = { ...pending, [row.id]: true };

		// Optimistic update; roll back on failure.
		const prevStatus = row.moderation_status;
		statements = statements.map((s) =>
			s.id === row.id ? { ...s, moderation_status: status } : s
		);

		try {
			const updated = await apiClient.PolisModerateStatementAux(
				{ decision },
				{ params: { id: row.id } }
			);
			statements = statements.map((s) => (s.id === row.id ? updated : s));
		} catch (e) {
			console.error('PolisModerateStatementAux failed', e);
			statements = statements.map((s) =>
				s.id === row.id ? { ...s, moderation_status: prevStatus } : s
			);
			notifications.send({ priority: 'ERROR', message: 'Failed to update statement' });
		} finally {
			pending = { ...pending, [row.id]: false };
		}
	}

	// Left accent bar colour keyed on seed/status. Olive-green primary stands in
	// for "accepted"; there is no dedicated success token in the theme.
	function accentFor(row: PolisStatementAux): string {
		if (row.is_seed) return 'bg-accent';
		if (row.moderation_status === 'accepted') return 'bg-primary';
		if (row.moderation_status === 'rejected') return 'bg-destructive';
		return 'bg-muted-foreground/40';
	}
</script>

<div class="flex flex-col gap-6">
	<!-- Heading + actions -->
	<div class="flex items-start justify-between gap-4">
		<div class="flex max-w-3xl flex-col gap-1">
			<h2 class="text-2xl font-bold">Statements moderation</h2>
			<p class="text-muted-foreground text-sm">Moderate and view all statements.</p>
		</div>
		<div class="flex shrink-0 items-center gap-2">
			<LoadingButton
				loading={syncing}
				variant="outline"
				onclick={syncFromPolis}
				title="Pull the latest submitted statements from Polis"
			>
				<RefreshCw class="size-4" />
				Sync from Polis
			</LoadingButton>
			<Button onclick={() => (showAddForm = true)} title="Add seed statements as moderator">
				<Plus class="size-4" />
				Add seed statements
			</Button>
		</div>
	</div>

	<!-- Add seed statements dialog -->
	<Dialog.Root bind:open={showAddForm}>
		<Dialog.Content class="sm:max-w-xl">
			<Dialog.Header>
				<Dialog.Title>Add seed statements</Dialog.Title>
				<Dialog.Description>
					Post statements to seed the conversation, or import many at once from a CSV.
				</Dialog.Description>
			</Dialog.Header>

			<div class="flex flex-col gap-3">
				<label class="text-muted-foreground text-sm font-medium" for="seed-text">
					Write a statement
				</label>
				<Textarea
					id="seed-text"
					bind:value={draftText}
					rows={3}
					placeholder="Write a seed statement…"
				/>

				<div class="text-muted-foreground flex items-center gap-2 text-sm">
					<span>or</span>
					<Button
						variant="secondary"
						size="sm"
						onclick={() => fileInput?.click()}
						disabled={csvImporting}
						title="Import seed statements from a CSV (one statement per line)"
					>
						<Upload class="size-4" />
						{csvImporting ? 'Importing…' : 'Import CSV'}
					</Button>
					<span>to add many at once</span>
				</div>
				<input
					bind:this={fileInput}
					type="file"
					accept=".csv,.txt"
					class="hidden"
					onchange={importCsv}
				/>
			</div>

			<Dialog.Footer>
				<Button variant="secondary" onclick={() => (showAddForm = false)}>Cancel</Button>
				<Button onclick={addSeed} disabled={!draftText.trim() || addingSeed}>
					{addingSeed ? 'Posting…' : 'Post seed'}
				</Button>
			</Dialog.Footer>
		</Dialog.Content>
	</Dialog.Root>

	<!-- Status filter chips -->
	<div class="flex flex-wrap items-center gap-2">
		{#each filters as f (f.key)}
			<button
				type="button"
				onclick={() => (filter = f.key)}
				class={`inline-flex cursor-pointer items-center rounded-full px-3.5 py-2 text-sm font-medium transition-colors ${
					filter === f.key
						? 'bg-primary text-primary-foreground'
						: 'bg-secondary text-secondary-foreground hover:bg-secondary/80'
				}`}
			>
				{f.label} · {counts[f.key]}
			</button>
		{/each}
	</div>

	<!-- Statements list -->
	<Card.Root class="gap-0 overflow-hidden py-0">
		<div
			class="text-muted-foreground grid grid-cols-[3rem_minmax(0,1fr)_auto] items-center gap-4 border-b px-4 py-3 text-xs font-semibold uppercase"
		>
			<div>#</div>
			<div>Statement</div>
			<div class="pr-2">Action</div>
		</div>

		{#if visible.length === 0}
			<p class="text-muted-foreground px-4 py-6 text-sm italic">
				No statements match this filter.
			</p>
		{:else}
			{#each visible as row (row.id)}
				<div
					class="border-border group hover:bg-muted/40 relative grid grid-cols-[3rem_minmax(0,1fr)_auto] items-center gap-4 border-b py-4 pl-4 transition-colors last:border-b-0"
				>
					<!-- Left accent bar (status colour) -->
					<span
						class={`absolute top-0 bottom-0 left-0 w-1 transition-all group-hover:w-1.5 ${accentFor(row)}`}
					></span>

					<!-- # -->
					<div class="text-muted-foreground text-center text-xs tabular-nums">
						{row.polis_statement_id}
					</div>

					<!-- Statement text -->
					<p class="min-w-0 text-base leading-7">{row.statement_text}</p>

					<!-- Actions -->
					<div class="flex items-center gap-1 self-center pr-2">
						<button
							type="button"
							disabled={pending[row.id] || row.moderation_status === 'accepted'}
							onclick={() => setStatus(row, 'accepted')}
							title="Accept"
							class="text-primary hover:bg-primary/15 inline-flex size-11 cursor-pointer items-center justify-center rounded-full transition-all hover:scale-110 active:scale-95 disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:scale-100 disabled:hover:bg-transparent"
						>
							<Check class="size-6" />
						</button>
						<button
							type="button"
							disabled={pending[row.id] || row.moderation_status === 'rejected'}
							onclick={() => setStatus(row, 'rejected')}
							title="Reject"
							class="text-destructive hover:bg-destructive/15 inline-flex size-11 cursor-pointer items-center justify-center rounded-full transition-all hover:scale-110 active:scale-95 disabled:cursor-not-allowed disabled:opacity-40 disabled:hover:scale-100 disabled:hover:bg-transparent"
						>
							<X class="size-6" />
						</button>
					</div>
				</div>
			{/each}
		{/if}
	</Card.Root>
</div>
