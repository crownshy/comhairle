<script lang="ts">
	import { tick } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Progress } from '$lib/components/ui/progress';
	import {
		Plus,
		Trash2,
		Upload,
		Search,
		Check,
		TriangleAlert,
		Languages,
		Info
	} from 'lucide-svelte';
	import { Spinner } from '$lib/components/ui/spinner';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { cn } from '$lib/utils';
	import { getLanguageName } from '$lib/config/languages';
	import { aiTranslateContent } from '$lib/components/Translation/translationUtils';
	import { guardUnsavedChanges } from '$lib/utils/unsavedChangesGuard.svelte';
	import { GLOSSARY_METADATA_KEY } from '$lib/glossary/parseGlossary';
	import { parseGlossaryCsv } from '$lib/glossary/glossaryCsv';
	import type { LocalizedGlossary } from '$lib/glossary/types';

	let {
		conversationId,
		primaryLocale,
		supportedLanguages,
		initial = []
	}: {
		conversationId: string;
		primaryLocale: string;
		supportedLanguages: string[];
		/** The translatable glossary already stored on the conversation, if any. */
		initial?: LocalizedGlossary;
	} = $props();

	// A row holds the terms and the explanation per locale, each as a plain string while editing
	// (terms are comma-separated, split into an array on save). `id` is a stable {#each} key.
	type Row = {
		id: number;
		terms: Record<string, string>;
		tooltips: Record<string, string>;
	};

	let nextId = 0;
	const toRow = (terms: Record<string, string>, tooltips: Record<string, string>): Row => ({
		id: nextId++,
		terms,
		tooltips
	});

	let rows = $state<Row[]>(
		initial.length > 0
			? initial.map((entry) =>
					toRow(
						Object.fromEntries(
							Object.entries(entry.text).map(([locale, terms]) => [
								locale,
								terms.join(', ')
							])
						),
						{ ...entry.tooltip }
					)
				)
			: [toRow({}, {})]
	);

	// The language currently being edited. Starts on the primary locale.
	let activeLocale = $state(primaryLocale);
	let activeName = $derived(getLanguageName(activeLocale));
	let primaryName = $derived(getLanguageName(primaryLocale));
	const isPrimary = $derived(activeLocale === primaryLocale);

	let query = $state('');
	let importing = $state(false);
	let fileInput = $state<HTMLInputElement>();

	const termsOf = (row: Row, locale: string) => row.terms[locale] ?? '';
	const tipOf = (row: Row, locale: string) => row.tooltips[locale] ?? '';

	const hasAnyTerms = (row: Row) => Object.values(row.terms).some((t) => t.trim());
	const hasAnyTip = (row: Row) => Object.values(row.tooltips).some((t) => t.trim());
	const isBlank = (row: Row) => !hasAnyTerms(row) && !hasAnyTip(row);
	const willSave = (row: Row) => hasAnyTerms(row) && hasAnyTip(row);

	// A row needs translating when it has primary-language content but is missing the active
	// language's term or explanation.
	const needsTranslation = (row: Row) =>
		!isPrimary &&
		(termsOf(row, primaryLocale).trim() || tipOf(row, primaryLocale).trim()) &&
		(!termsOf(row, activeLocale).trim() || !tipOf(row, activeLocale).trim());

	let filledCount = $derived(rows.filter(willSave).length);
	let missingCount = $derived(rows.filter(needsTranslation).length);

	let filtered = $derived.by(() => {
		const q = query.trim().toLowerCase();
		if (!q) return rows;
		return rows.filter(
			(row) =>
				Object.values(row.terms).some((t) => t.toLowerCase().includes(q)) ||
				Object.values(row.tooltips).some((t) => t.toLowerCase().includes(q))
		);
	});

	// --- Autosave: persist a debounced snapshot on every edit, like the other config fields.
	let saveState = $state<'idle' | 'saving' | 'saved' | 'error'>('idle');
	let saveTimer: ReturnType<typeof setTimeout> | undefined;
	// `dirty` is true from the moment an edit is made until its save lands. Drives both the
	// status pill and the leave-page guard.
	let dirty = $state(false);

	// Warn before leaving (in-app nav or full refresh) while an edit hasn't been saved yet.
	guardUnsavedChanges(() => dirty);

	/** Rows -> a clean translatable glossary, dropping empties and splitting the terms per locale. */
	function toLocalizedGlossary(): LocalizedGlossary {
		const glossary: LocalizedGlossary = [];
		for (const row of rows) {
			const text: Record<string, string[]> = {};
			for (const [locale, value] of Object.entries(row.terms)) {
				const terms = value
					.split(',')
					.map((term) => term.trim())
					.filter(Boolean);
				if (terms.length) text[locale] = terms;
			}
			const tooltip: Record<string, string> = {};
			for (const [locale, value] of Object.entries(row.tooltips)) {
				const trimmed = value.trim();
				if (trimmed) tooltip[locale] = trimmed;
			}
			if (Object.keys(text).length === 0 || Object.keys(tooltip).length === 0) continue;
			glossary.push({ text, tooltip });
		}
		return glossary;
	}

	async function commit() {
		saveTimer = undefined;
		saveState = 'saving';
		const startedAt = performance.now();
		const glossary = toLocalizedGlossary();
		const result = await tryCatchAsync(() =>
			apiClient.PatchConversationMetadata(
				{ [GLOSSARY_METADATA_KEY]: glossary },
				{ params: { conversation_id: conversationId } }
			)
		);
		if (result.err) {
			saveState = 'error';
			return; // stay dirty so the guard still warns
		}
		// Keep "Saving…" on screen long enough to actually register; a fast local save
		// would otherwise flip straight to "Saved" and the spinner would never be seen.
		const elapsed = performance.now() - startedAt;
		if (elapsed < 500) await new Promise((r) => setTimeout(r, 500 - elapsed));
		saveState = 'saved';
		dirty = false;
	}

	function scheduleSave() {
		dirty = true;
		clearTimeout(saveTimer);
		saveTimer = setTimeout(commit, 700);
	}

	// Flush a pending save when leaving the tab so the last keystroke isn't lost.
	$effect(() => {
		return () => {
			if (saveTimer) {
				clearTimeout(saveTimer);
				commit();
			}
		};
	});

	function setTerms(row: Row, value: string) {
		row.terms[activeLocale] = value;
		scheduleSave();
	}

	function setTooltip(row: Row, value: string) {
		row.tooltips[activeLocale] = value;
		scheduleSave();
	}

	async function addRow() {
		const row = toRow({}, {});
		rows = [...rows, row];
		// Clear any filter so the new row is visible, then focus its term cell.
		query = '';
		await tick();
		document.querySelector<HTMLInputElement>(`#glossary-terms-${row.id}`)?.focus();
	}

	function removeRow(id: number) {
		rows = rows.filter((row) => row.id !== id);
		if (rows.length === 0) rows = [toRow({}, {})];
		scheduleSave();
	}

	/**
	 * Imports entries from a CSV and appends them (dropping blanks). The CSV has one term and one
	 * explanation column, so both fill the language currently being edited. Autosaves after.
	 */
	async function importCsv(event: Event) {
		const input = event.currentTarget as HTMLInputElement;
		const file = input.files?.[0];
		if (!file || importing) return;
		importing = true;

		const result = await tryCatchAsync<number, 'INCORRECT_FILE_TYPE' | 'NO_ENTRIES_FOUND'>(
			async () => {
				if (!file.name.toLowerCase().endsWith('.csv')) throw 'INCORRECT_FILE_TYPE';

				const imported = parseGlossaryCsv(await file.text());
				if (imported.length === 0) throw 'NO_ENTRIES_FOUND';

				const existing = rows.filter((row) => !isBlank(row));
				const importedRows = imported.map((entry) =>
					toRow(
						{ [activeLocale]: entry.text.join(', ') },
						{ [activeLocale]: entry.tooltip }
					)
				);
				rows = [...existing, ...importedRows];
				return imported.length;
			}
		);
		importing = false;
		// Reset so re-importing the same file fires onchange again.
		input.value = '';

		if (result.err !== null) {
			const message =
				result.err === 'INCORRECT_FILE_TYPE'
					? 'Only CSV files are allowed'
					: result.err === 'NO_ENTRIES_FOUND'
						? 'No glossary terms found in that file'
						: 'Could not read that CSV';
			notifications.addFlash({ message, priority: 'WARNING' });
			return;
		}

		scheduleSave();
		notifications.addFlash({
			message: `Imported ${result.ok} term${result.ok === 1 ? '' : 's'} into ${activeName}`,
			priority: 'SUCCESS'
		});
	}

	// --- AI translate: fill the active language's empty terms AND explanations from the primary
	// one, the same "translate" affordance the Learn step editor offers. Never overwrites text
	// that's already there, so manual fixes are safe.
	let translating = $state(false);
	let translateDone = $state(0);
	let translateTotal = $state(0);

	async function translateActiveLanguage() {
		if (translating || isPrimary) return;

		const targets = rows.filter(needsTranslation);
		if (targets.length === 0) {
			notifications.addFlash({
				message: `Everything already has ${activeName} text`,
				priority: 'INFO'
			});
			return;
		}

		translating = true;
		translateTotal = targets.length;
		translateDone = 0;
		let failed = 0;

		for (const row of targets) {
			const sourceTerms = termsOf(row, primaryLocale).trim();
			const sourceTip = tipOf(row, primaryLocale).trim();
			let rowFailed = false;

			if (sourceTerms && !termsOf(row, activeLocale).trim()) {
				const result = await tryCatchAsync(() =>
					aiTranslateContent(sourceTerms, activeLocale, primaryLocale)
				);
				if (result.err || !result.ok) rowFailed = true;
				else row.terms[activeLocale] = result.ok;
			}
			if (sourceTip && !tipOf(row, activeLocale).trim()) {
				const result = await tryCatchAsync(() =>
					aiTranslateContent(sourceTip, activeLocale, primaryLocale)
				);
				if (result.err || !result.ok) rowFailed = true;
				else row.tooltips[activeLocale] = result.ok;
			}

			if (rowFailed) failed++;
			translateDone += 1;
		}

		translating = false;
		scheduleSave();

		const ok = targets.length - failed;
		notifications.addFlash({
			message: failed
				? `Translated ${ok} of ${targets.length} — ${failed} failed`
				: `Translated ${ok} term${ok === 1 ? '' : 's'} to ${activeName}`,
			priority: failed ? 'WARNING' : 'SUCCESS'
		});
	}

	// Column template differs per tab: the primary tab is a plain 2-field row; a translation tab
	// shows the read-only source TERM (as the reference key) beside the editable translated term
	// and explanation.
	const gridClass = $derived(
		isPrimary
			? 'grid-cols-[minmax(8rem,14rem)_1fr_auto]'
			: 'grid-cols-[minmax(7rem,12rem)_minmax(7rem,12rem)_minmax(9rem,1fr)_auto]'
	);
</script>

<div class="flex flex-col gap-6">
	{#if supportedLanguages.length > 1}
		<!-- Language switch: terms and explanations are edited per language. The translate button
			fills the active language's empty terms + explanations from the primary one. -->
		<div class="flex flex-wrap items-center justify-between gap-2">
			<div
				class="flex flex-wrap items-center gap-1"
				role="tablist"
				aria-label="Editing language"
			>
				{#each supportedLanguages as locale (locale)}
					<button
						type="button"
						role="tab"
						aria-selected={activeLocale === locale}
						onclick={() => (activeLocale = locale)}
						class={cn(
							'rounded-md px-3 py-1 text-sm font-medium transition-colors',
							activeLocale === locale
								? 'bg-primary text-primary-foreground'
								: 'text-muted-foreground hover:bg-muted'
						)}
					>
						{getLanguageName(locale)}
						{#if locale === primaryLocale}<span class="opacity-70">(primary)</span>{/if}
					</button>
				{/each}
			</div>
			{#if !isPrimary}
				<Button
					variant="outline"
					size="sm"
					onclick={translateActiveLanguage}
					disabled={translating}
					title={`Fill empty ${activeName} terms and explanations by translating from ${primaryName}`}
				>
					{#if translating}
						<Spinner class="mr-1.5 size-4" />
					{:else}
						<Languages class="mr-1.5 size-4" />
					{/if}
					Translate from {primaryName}
				</Button>
			{/if}
		</div>
	{/if}

	<!-- Toolbar: filter the list plus the row-level actions. -->
	<div class="flex flex-wrap items-center gap-2">
		<div class="relative flex-1 md:max-w-xs">
			<Search
				class="text-muted-foreground pointer-events-none absolute top-1/2 left-2.5 size-4 -translate-y-1/2"
			/>
			<Input bind:value={query} placeholder="Search terms" class="pl-8" />
		</div>
		<Button variant="outline" onclick={addRow}>
			<Plus class="mr-1.5 size-4" /> Add term
		</Button>
		<Button
			variant="secondary"
			onclick={() => fileInput?.click()}
			disabled={importing}
			title="Import glossary terms from a CSV"
		>
			<Upload class="mr-1.5 size-4" />
			{importing ? 'Importing…' : 'Import CSV'}
		</Button>
		<!-- CSV format hint, tucked behind an info icon so it stays discreet. -->
		<Tooltip.Root>
			<Tooltip.Trigger>
				{#snippet child({ props })}
					<Info
						{...props}
						class="text-muted-foreground hover:text-foreground size-4 cursor-help"
						aria-label="CSV format"
					/>
				{/snippet}
			</Tooltip.Trigger>
			<Tooltip.Content
				class="bg-card text-card-foreground border-border max-w-xs border text-sm"
				arrowClasses="bg-card border-border"
			>
				Use two columns: term and explanation. A header row is optional. Separate synonyms
				in the term column with a semicolon, for example <code>bus; coach</code>.
			</Tooltip.Content>
		</Tooltip.Root>
		<input
			bind:this={fileInput}
			type="file"
			accept=".csv"
			class="hidden"
			onchange={importCsv}
		/>

		{#if saveState !== 'idle'}
			<span
				class={cn(
					'ml-auto flex items-center gap-1.5 rounded-full px-2.5 py-1 text-sm',
					saveState === 'error'
						? 'bg-destructive/10 text-destructive'
						: saveState === 'saving'
							? 'bg-primary text-primary-foreground'
							: 'bg-muted text-muted-foreground'
				)}
				aria-live="polite"
			>
				{#if saveState === 'saving'}
					<Spinner class="size-3.5" /> Saving…
				{:else if saveState === 'saved'}
					<Check class="size-3.5" /> Saved
				{:else}
					<TriangleAlert class="size-3.5" /> Not saved
				{/if}
			</span>
		{/if}
	</div>

	<!-- Dense, spreadsheet-style grid. On a translation tab the source (primary) term shows
		read-only as the reference key, beside the editable translated term and explanation.
		Truncated cells reveal their full text on hover via the title attribute. -->
	<div class="border-border overflow-hidden rounded-lg border">
		<div class="overflow-x-auto">
			<div class={cn(isPrimary ? 'min-w-full' : 'min-w-[38rem]')}>
				<div
					class={cn(
						'bg-muted/50 text-muted-foreground border-border grid gap-px border-b text-sm font-medium',
						gridClass
					)}
				>
					{#if isPrimary}
						<div class="px-3 py-2">Term (and synonyms)</div>
						<div class="px-3 py-2">Explanation</div>
						<div class="px-2 py-2"></div>
					{:else}
						<div class="px-3 py-2">Term · {primaryName}</div>
						<div class="text-foreground px-3 py-2">Term · {activeName}</div>
						<div class="text-foreground px-3 py-2">Explanation · {activeName}</div>
						<div class="px-2 py-2"></div>
					{/if}
				</div>

				<div class="divide-border bg-background divide-y">
					{#each filtered as row (row.id)}
						<div
							class={cn(
								'group focus-within:bg-muted/30 hover:bg-muted/20 grid items-center gap-px',
								gridClass
							)}
						>
							{#if !isPrimary}
								<!-- Source term (primary), read-only, as the reference key for the translation. -->
								<div
									class="text-muted-foreground truncate px-3 text-base font-medium"
									title={termsOf(row, primaryLocale)}
								>
									{termsOf(row, primaryLocale) || '—'}
								</div>
							{/if}

							<input
								id="glossary-terms-{row.id}"
								value={termsOf(row, activeLocale)}
								oninput={(e) => setTerms(row, e.currentTarget.value)}
								placeholder={isPrimary
									? 'referral, self-referral'
									: `Terms in ${activeName}`}
								aria-label={`Term and synonyms in ${activeName}`}
								title={termsOf(row, activeLocale)}
								class="text-foreground placeholder:text-muted-foreground/60 bg-background h-10 truncate px-3 text-base font-medium outline-none"
							/>
							<input
								value={tipOf(row, activeLocale)}
								oninput={(e) => setTooltip(row, e.currentTarget.value)}
								placeholder={isPrimary
									? 'When your case is passed to another team that can help.'
									: `Explanation in ${activeName}`}
								aria-label={`Explanation in ${activeName}`}
								title={tipOf(row, activeLocale)}
								class="text-foreground placeholder:text-muted-foreground/60 bg-background h-10 truncate px-3 text-base outline-none"
							/>
							<button
								type="button"
								onclick={() => removeRow(row.id)}
								aria-label="Remove term"
								class="bg-background text-muted-foreground hover:text-destructive flex h-10 w-10 items-center justify-center opacity-0 transition-opacity group-hover:opacity-100 focus:opacity-100"
							>
								<Trash2 class="size-4" />
							</button>
						</div>
					{:else}
						<p class="text-muted-foreground px-3 py-8 text-center text-sm">
							No terms match "{query}".
						</p>
					{/each}
				</div>
			</div>
		</div>
	</div>

	<div class="flex flex-col gap-3">
		{#if translating}
			<!-- Translation runs one entry at a time; the bar makes the wait legible. -->
			<div class="flex flex-col gap-1.5">
				<span class="text-muted-foreground flex items-center gap-2 text-sm">
					<Spinner class="size-4" />
					Translating to {activeName}… {translateDone}/{translateTotal}
				</span>
				<Progress
					value={translateDone}
					max={translateTotal}
					aria-label="Translation progress"
				/>
			</div>
		{/if}

		<span class="text-muted-foreground text-sm">
			{filledCount}
			{filledCount === 1 ? 'term' : 'terms'}
			{#if supportedLanguages.length > 1 && missingCount > 0}
				· {missingCount} to translate into {activeName}
			{/if}
		</span>
	</div>
</div>
