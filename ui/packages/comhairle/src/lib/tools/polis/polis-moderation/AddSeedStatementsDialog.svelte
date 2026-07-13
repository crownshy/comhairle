<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import * as Dialog from '$lib/components/ui/dialog';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import { Plus, Upload } from '@lucide/svelte';

	let {
		workflowStepId,
		onSeeded
	}: {
		workflowStepId: string;
		// Called after seeds are posted + synced so the parent can refresh its data.
		onSeeded: () => void | Promise<void>;
	} = $props();

	let open = $state(false);
	let draftText = $state('');
	let addingSeed = $state(false);
	let csvImporting = $state(false);
	let fileInput = $state<HTMLInputElement>();

	// Seeds are posted server-side to the active poll via PolisPostSeed (no
	// browser-side Polis auth / CORS), then we re-sync so the new comment comes
	// back with its real Polis-issued ids, and hand off to the parent to refresh.
	async function postSeeds(texts: string[]) {
		for (const statement_text of texts) {
			await apiClient.PolisPostSeed({ workflow_step_id: workflowStepId, statement_text });
		}
		await apiClient.PolisSyncStatementAux({ workflow_step_id: workflowStepId });
		await onSeeded();
	}

	async function addSeed() {
		const text = draftText.trim();
		if (!text || addingSeed) return;
		addingSeed = true;
		const { err } = await tryCatchAsync(() => postSeeds([text]));
		addingSeed = false;
		if (err) {
			console.error('PolisPostSeed failed', err);
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

		const { ok: count, err } = await tryCatchAsync(async () => {
			// One statement per line; strip wrapping quotes and a leading header row.
			const lines = (await file.text())
				.split(/\r?\n/)
				.map((l) => l.replace(/^"(.*)"$/, '$1').trim())
				.filter(Boolean);
			if (['statement', 'statements', 'text'].includes(lines[0]?.toLowerCase())) {
				lines.shift();
			}
			if (lines.length) await postSeeds(lines);
			return lines.length;
		});
		csvImporting = false;
		input.value = '';

		if (err) {
			console.error('CSV import failed', err);
			notifications.send({ priority: 'ERROR', message: 'CSV import failed' });
			return;
		}
		if (count) {
			notifications.send({
				priority: 'INFO',
				message: `Imported ${count} statement${count === 1 ? '' : 's'}`
			});
		}
		open = false;
	}
</script>

<Button onclick={() => (open = true)} title="Add seed statements as moderator">
	<Plus class="size-4" />
	Add seed statements
</Button>

<Dialog.Root bind:open>
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
			<Button variant="secondary" onclick={() => (open = false)}>Cancel</Button>
			<Button onclick={addSeed} disabled={!draftText.trim() || addingSeed}>
				{addingSeed ? 'Posting…' : 'Post seed'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
