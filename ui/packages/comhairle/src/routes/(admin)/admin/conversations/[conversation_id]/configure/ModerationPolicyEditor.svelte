<script lang="ts">
	import { tick } from 'svelte';
	import { invalidate } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import { Button } from '$lib/components/ui/button';
	import { Spinner } from '$lib/components/ui/spinner';
	import { Check, Plus, RotateCcw, Trash2, TriangleAlert } from 'lucide-svelte';
	import { cn } from '$lib/utils';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { guardUnsavedChanges } from '$lib/utils/unsavedChangesGuard.svelte';
	import {
		DEFAULT_REJECT_REASONS,
		MODERATION_POLICY_METADATA_KEY,
		toStoredModerationPolicy,
		type ModerationPolicy,
		type RejectReason
	} from '$lib/moderation/moderationPolicy';

	let {
		conversationId,
		initial
	}: {
		conversationId: string;
		/** The policy stored on the conversation, or the default list standing in for one. */
		initial: ModerationPolicy;
	} = $props();

	// `id` is a stable {#each} key: labels can be blank or repeated while editing.
	type Row = { id: number; label: string; description: string };

	let nextId = 0;
	const toRows = (reasons: RejectReason[]): Row[] =>
		reasons.map((reason) => ({
			id: nextId++,
			label: reason.label,
			description: reason.description ?? ''
		}));

	let rows = $derived<Row[]>(toRows(initial.rejectReasons));
	// While true the conversation follows DEFAULT_REJECT_REASONS and saves store null, so a
	// later change to the default reaches it. Any edit makes the list the conversation's own.
	let usingDefault = $derived(initial.isDefault);

	// Repeated labels (case-insensitive) after the first. Saving keeps only the first.
	const duplicateIds = $derived.by(() => {
		const seen: Record<string, true> = {};
		const ids: number[] = [];
		for (const row of rows) {
			const key = row.label.trim().toLowerCase();
			if (!key) continue;
			if (seen[key]) ids.push(row.id);
			seen[key] = true;
		}
		return ids;
	});

	// --- Autosave, same shape as the glossary editor.
	let saveState = $state<'idle' | 'saving' | 'saved' | 'error'>('idle');
	let saveTimer: ReturnType<typeof setTimeout> | undefined;
	let dirty = $state(false);
	// The Moderation tab reads the policy from the conversation layout's data, so once a save
	// lands that data is refreshed when this editor goes away.
	let savedOnce = false;

	guardUnsavedChanges(() => dirty);

	async function commit() {
		saveTimer = undefined;
		saveState = 'saving';
		const startedAt = performance.now();
		const result = await tryCatchAsync(() =>
			apiClient.PatchConversationMetadata(
				{
					[MODERATION_POLICY_METADATA_KEY]: usingDefault
						? null
						: toStoredModerationPolicy(rows)
				},
				{ params: { conversation_id: conversationId } }
			)
		);
		if (result.err !== null) {
			saveState = 'error';
			return; // stay dirty so the guard still warns
		}
		savedOnce = true;
		// Keep "Saving…" up long enough to register on a fast local save.
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

	$effect(() => {
		return () => {
			if (saveTimer) {
				clearTimeout(saveTimer);
				commit().then(() => invalidate('conversation:meta'));
			} else if (savedOnce) {
				invalidate('conversation:meta');
			}
		};
	});

	function editRow(id: number, patch: Partial<Omit<Row, 'id'>>) {
		rows = rows.map((row) => (row.id === id ? { ...row, ...patch } : row));
		usingDefault = false;
		scheduleSave();
	}

	async function addRow() {
		const row: Row = { id: nextId++, label: '', description: '' };
		rows = [...rows, row];
		await tick();
		document.querySelector<HTMLInputElement>(`#reject-reason-label-${row.id}`)?.focus();
	}

	function removeRow(id: number) {
		rows = rows.filter((row) => row.id !== id);
		usingDefault = false;
		scheduleSave();
	}

	function resetToDefault() {
		rows = toRows(DEFAULT_REJECT_REASONS);
		usingDefault = true;
		scheduleSave();
	}

	const gridClass = 'grid-cols-[minmax(10rem,16rem)_minmax(12rem,1fr)_auto]';
</script>

<div class="flex flex-col gap-6">
	<div class="flex flex-wrap items-center gap-2">
		<Button variant="outline" onclick={addRow}>
			<Plus class="mr-1.5 size-4" /> Add reason
		</Button>
		{#if !usingDefault}
			<Button
				variant="ghost"
				onclick={resetToDefault}
				title="Replace this list with the default reasons"
			>
				<RotateCcw class="mr-1.5 size-4" /> Reset to default
			</Button>
		{/if}

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

	{#if usingDefault}
		<p class="text-muted-foreground text-base">
			This conversation uses the default reasons. Edit, add or remove one and the list becomes
			this conversation's own.
		</p>
	{/if}

	<div class="border-border overflow-hidden rounded-lg border">
		<div class="overflow-x-auto">
			<div class="min-w-120">
				<div
					class={cn(
						'bg-muted/50 text-muted-foreground border-border grid border-b text-sm font-medium',
						gridClass
					)}
				>
					<div class="px-3 py-2">Reason</div>
					<div class="px-3 py-2">What counts (optional)</div>
					<div class="w-10"></div>
				</div>

				<div class="divide-border bg-background divide-y">
					{#each rows as row (row.id)}
						<div
							class={cn(
								'focus-within:bg-muted/30 hover:bg-muted/20 grid items-center',
								gridClass
							)}
						>
							<input
								id="reject-reason-label-{row.id}"
								value={row.label}
								oninput={(e) => editRow(row.id, { label: e.currentTarget.value })}
								placeholder="Off-topic"
								aria-label="Reason"
								aria-invalid={duplicateIds.includes(row.id)}
								title={row.label}
								class="text-foreground placeholder:text-muted-foreground/60 aria-invalid:text-destructive h-10 truncate bg-transparent px-3 text-base font-medium outline-none"
							/>
							<input
								value={row.description}
								oninput={(e) =>
									editRow(row.id, { description: e.currentTarget.value })}
								placeholder="Shown to moderators when they pick this reason"
								aria-label="What counts under this reason"
								title={row.description}
								class="text-foreground placeholder:text-muted-foreground/60 h-10 truncate bg-transparent px-3 text-base outline-none"
							/>
							<button
								type="button"
								onclick={() => removeRow(row.id)}
								aria-label="Remove reason"
								class="text-muted-foreground hover:text-destructive flex h-10 w-10 items-center justify-center"
							>
								<Trash2 class="size-4" />
							</button>
						</div>
					{:else}
						<p class="text-muted-foreground px-3 py-8 text-center text-base">
							No reasons. Moderators can still explain a rejection in a note.
						</p>
					{/each}
				</div>
			</div>
		</div>
	</div>

	{#if duplicateIds.length > 0}
		<p class="text-destructive text-sm">
			Some reasons appear more than once. Only the first of each is saved.
		</p>
	{/if}
</div>
