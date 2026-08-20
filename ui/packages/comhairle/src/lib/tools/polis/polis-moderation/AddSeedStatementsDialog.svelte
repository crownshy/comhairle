<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Spinner } from '$lib/components/ui/spinner';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import { Plus, Upload } from '@lucide/svelte';

	type Props = {
		workflowStepId: string;
		// Called after seeds are posted + synced so the parent can refresh its data.
		onSeeded: () => void | Promise<void>;
	};

	let { workflowStepId, onSeeded }: Props = $props();

	let open = $state(false);
	let draftText = $state('');
	let addingSeed = $state(false);
	let csvImporting = $state(false);
	let fileInput = $state<HTMLInputElement>();

	// Number of statements posted so far / in the current batch, for the progress
	// readout while a post or import is in flight.
	let postedCount = $state(0);
	let totalCount = $state(0);

	// A post (single or CSV) is in flight: gate the whole dialog until it settles.
	let busy = $derived(addingSeed || csvImporting);

	// Seeds are posted server-side to the active poll via PolisPostSeed (no
	// browser-side Polis auth / CORS), then we re-sync so the new comment comes
	// back with its real Polis-issued ids, and hand off to the parent to refresh.
	async function postSeeds(texts: string[]) {
		postedCount = 0;
		totalCount = texts.length;
		for (const statement_text of texts) {
			await apiClient.PolisPostSeed({ workflow_step_id: workflowStepId, statement_text });
			postedCount += 1;
		}
		await apiClient.PolisSyncStatementAux({ workflow_step_id: workflowStepId });
		await onSeeded();
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
		open = false;
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
		open = false;
	}
</script>

<Button onclick={() => (open = true)} title="Add seed statements as moderator">
	<Plus class="size-4" />
	Add seed statements
</Button>

<Dialog.Root bind:open>
	<Dialog.Content
		class="sm:max-w-xl"
		showCloseButton={!busy}
		onInteractOutside={(e) => busy && e.preventDefault()}
		onEscapeKeydown={(e) => busy && e.preventDefault()}
	>
		<Dialog.Header>
			<Dialog.Title>Add seed statements</Dialog.Title>
			<Dialog.Description>
				Post statements to seed the conversation, or import many at once from a CSV.
			</Dialog.Description>
		</Dialog.Header>

		<div class="relative flex flex-col gap-3">
			<!-- Dim + block the body while a post or import is in flight. -->
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
					{csvImporting ? 'Importing…' : 'Import CSV'}
				</Button>
				<span>to add many at once</span>
			</div>
			<input
				bind:this={fileInput}
				type="file"
				accept=".csv"
				class="hidden"
				onchange={importCsv}
			/>
		</div>

		<Dialog.Footer>
			<Button variant="secondary" onclick={() => (open = false)} disabled={busy}
				>Cancel</Button
			>
			<Button onclick={addSeed} disabled={!draftText.trim() || busy}>
				{addingSeed ? 'Posting…' : 'Post seed'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
