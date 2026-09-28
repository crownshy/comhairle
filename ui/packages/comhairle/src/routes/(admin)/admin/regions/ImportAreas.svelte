<script lang="ts">
	import { apiClient } from '@crownshy/api-client/client';
	import type { CreateRegionArea } from '@crownshy/api-client/api';
	import { Upload, X } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Progress } from '$lib/components/ui/progress';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { importRecords, readBoundaryFile, type BoundaryDataset } from './import';

	let { onimport, onclose }: { onimport: () => Promise<void>; onclose: () => void } = $props();
	// IMPORTANT: Uses .raw to prevent millions of coordinate Proxies from crashing the browser
	let dataset = $state.raw<BoundaryDataset | null>(null);
	let nameField = $derived(dataset?.nameField ?? '');
	let tags = $derived(dataset?.tags.join(', ') ?? '');
	let busy = $state(false);
	let failure = $state('');
	// IMPORTANT: Uses .raw to prevent millions of coordinate Proxies from crashing the browser
	let records = $state.raw<CreateRegionArea[]>([]);
	let confirmed = $state('');
	let imported = $state(0);
	let importTotal = $state(0);
	const mapping = $derived(JSON.stringify([nameField, tags]));
	const previewCurrent = $derived(!!records.length && confirmed === mapping);

	/** Keeps each request well inside the API's payload limit on large national datasets. */
	const IMPORT_BATCH_SIZE = 50;

	async function loadFile(file: File | undefined) {
		if (!file) return;
		busy = true;
		failure = '';
		records = [];
		dataset = null;
		const response = await tryCatchAsync(() => readBoundaryFile(file));
		busy = false;
		if (response.err !== null) {
			failure = response.err instanceof Error ? response.err.message : 'Could not parse file';
			return;
		}
		dataset = response.ok;
		await preview(response.ok, response.ok.nameField, response.ok.tags.join(', '));
	}

	async function preview(source = dataset, name = nameField, labels = tags) {
		if (!source) return;
		const response = await tryCatchAsync(async () =>
			importRecords(source, name, labels.split(','))
		);
		if (response.err !== null) {
			failure =
				response.err instanceof Error ? response.err.message : 'Invalid field mapping';
			records = [];
			return;
		}
		records = response.ok;
		confirmed = JSON.stringify([name, labels]);
		failure = '';
	}

	async function commit() {
		if (!previewCurrent) return;
		busy = true;
		failure = '';
		imported = 0;
		importTotal = records.length;
		for (let start = 0; start < records.length; start += IMPORT_BATCH_SIZE) {
			const batch = records.slice(start, start + IMPORT_BATCH_SIZE);
			const response = await tryCatchAsync(() =>
				apiClient.ImportRegionAreas({ areas: batch })
			);
			if (response.err !== null) {
				failure = `Import stopped after ${imported} of ${importTotal} areas. Check boundary validity, then re-import the remaining areas.`;
				if (imported) await onimport();
				busy = false;
				importTotal = 0;
				return;
			}
			imported += batch.length;

			// Yield to the event loop so the Garbage Collector can clean up
			// the memory from the previous request's response body.
			await new Promise((resolve) => setTimeout(resolve, 5));
		}
		await onimport();
		busy = false;
		importTotal = 0;
		onclose();
	}
</script>

<section class="bg-background border-b p-4 md:p-6" aria-label="Import boundaries">
	<div class="mb-4 flex items-center justify-between gap-4">
		<h2 class="text-xl font-semibold">Import boundaries</h2>
		<Button
			variant="ghost"
			size="icon"
			title="Close import"
			aria-label="Close import"
			disabled={busy}
			onclick={onclose}><X /></Button
		>
	</div>
	<label class="grid gap-2 text-base"
		>Boundary file (.geojson, .json, .zip)
		<Input
			type="file"
			accept=".geojson,.json,.zip"
			disabled={busy}
			onchange={(event) => loadFile(event.currentTarget.files?.[0])}
		/>
	</label>
	{#if dataset}
		<div class="my-4 grid gap-4 md:grid-cols-3">
			<label class="grid gap-2"
				>Name field
				<select
					class="bg-background h-10 rounded border px-3"
					bind:value={nameField}
					disabled={busy}
				>
					<option value="">No name</option>
					{#each dataset.fields as field}<option value={field}>{field}</option>{/each}
				</select>
			</label>
			<label class="grid gap-2"
				>Tags, comma separated<Input bind:value={tags} disabled={busy} /></label
			>
		</div>
		<div class="my-4 max-h-56 overflow-auto border">
			<table class="w-full text-left text-base">
				<thead class="bg-muted sticky top-0"
					><tr><th class="p-2">Name</th><th class="p-2">Tags</th></tr></thead
				>
				<tbody
					>{#each records.slice(0, 20) as record}<tr class="border-t"
							><td class="p-2">{record.name ?? ''}</td><td class="p-2 break-words"
								>{record.tags?.join(', ')}</td
							></tr
						>{/each}</tbody
				>
			</table>
		</div>
		<div class="flex flex-wrap items-center gap-3">
			<span class="text-muted-foreground text-base">{dataset.features.length} areas</span>
			<Button variant="outline" disabled={busy} onclick={() => preview()}
				>Refresh preview</Button
			>
			<Button disabled={busy || !previewCurrent} onclick={commit}
				><Upload class="size-4" />Import {records.length} areas</Button
			>
		</div>
	{/if}
	{#if importTotal}
		<div class="mt-3 grid gap-2">
			<p role="status">Imported {imported} of {importTotal} areas...</p>
			<Progress value={imported} max={importTotal} />
		</div>
	{:else if busy}<p role="status" class="mt-3">Processing boundaries...</p>{/if}
	{#if failure}<p role="alert" class="text-destructive mt-3 break-words">{failure}</p>{/if}
</section>
