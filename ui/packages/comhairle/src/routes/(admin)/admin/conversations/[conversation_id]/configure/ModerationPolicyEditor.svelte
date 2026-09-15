<script lang="ts">
	import { onDestroy, tick } from 'svelte';
	import { invalidate } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import { Button } from '$lib/components/ui/button';
	import { Plus, RotateCcw, Trash2 } from 'lucide-svelte';
	import { cn } from '$lib/utils';
	import { guardUnsavedChanges } from '$lib/utils/unsavedChangesGuard.svelte';
	import {
		DEFAULT_REJECT_REASONS,
		MODERATION_POLICY_METADATA_KEY,
		rejectReasonLabelProblems,
		toStoredModerationPolicy,
		type ModerationPolicy,
		type RejectReason
	} from '$lib/moderation/moderationPolicy';
	import { Autosave } from './autosave.svelte';
	import SaveStatusPill from './SaveStatusPill.svelte';

	type Props = {
		conversationId: string;
		/** The policy stored on the conversation, or the default list standing in for one. */
		initial: ModerationPolicy;
	};

	let { conversationId, initial }: Props = $props();

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

	// Why each row's label won't be saved, by position. Blank rows are still being typed, so
	// they aren't flagged.
	const labelProblems = $derived(rejectReasonLabelProblems(rows.map((row) => row.label)));
	const flaggedRows = $derived(
		labelProblems.map((problem) => problem === 'duplicate' || problem === 'contains-separator')
	);

	const autosave = new Autosave(() =>
		apiClient.PatchConversationMetadata(
			{
				[MODERATION_POLICY_METADATA_KEY]: usingDefault
					? null
					: toStoredModerationPolicy(rows)
			},
			{ params: { conversation_id: conversationId } }
		)
	);

	guardUnsavedChanges(() => autosave.dirty);

	// The Moderation tab reads the policy from the conversation layout's data, so once a save
	// lands that data is refreshed when this editor goes away.
	onDestroy(() => {
		autosave.flush();
		autosave.settled().then(() => {
			if (autosave.hasSaved) invalidate('conversation:meta');
		});
	});

	function editRow(id: number, patch: Partial<Omit<Row, 'id'>>) {
		rows = rows.map((row) => (row.id === id ? { ...row, ...patch } : row));
		usingDefault = false;
		autosave.schedule();
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
		autosave.schedule();
	}

	function resetToDefault() {
		rows = toRows(DEFAULT_REJECT_REASONS);
		usingDefault = true;
		autosave.schedule();
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

		<SaveStatusPill status={autosave.status} />
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
					{#each rows as row, index (row.id)}
						<div
							class={cn(
								'focus-within:bg-muted/30 hover:bg-muted/20 grid items-center',
								gridClass
							)}
						>
							<!-- Plain inputs, not the shadcn Input: these are borderless spreadsheet
								cells, matching the glossary editor. -->
							<input
								id="reject-reason-label-{row.id}"
								value={row.label}
								oninput={(event) =>
									editRow(row.id, { label: event.currentTarget.value })}
								placeholder="Off-topic"
								aria-label="Reason"
								aria-invalid={flaggedRows[index]}
								title={row.label}
								class="text-foreground placeholder:text-muted-foreground/60 aria-invalid:text-destructive h-10 truncate bg-transparent px-3 text-base font-medium outline-none"
							/>
							<input
								value={row.description}
								oninput={(event) =>
									editRow(row.id, { description: event.currentTarget.value })}
								placeholder="Shown to moderators when they pick this reason"
								aria-label="What counts under this reason"
								title={row.description}
								class="text-foreground placeholder:text-muted-foreground/60 h-10 truncate bg-transparent px-3 text-base outline-none"
							/>
							<Button
								variant="ghost"
								size="icon"
								onclick={() => removeRow(row.id)}
								aria-label="Remove reason"
								class="text-muted-foreground hover:text-destructive size-10 rounded-none"
							>
								<Trash2 class="size-4" />
							</Button>
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

	{#if labelProblems.includes('duplicate')}
		<p class="text-destructive text-base">
			Some reasons appear more than once. Only the first of each is saved.
		</p>
	{/if}
	{#if labelProblems.includes('contains-separator')}
		<p class="text-destructive text-base">
			A reason can't contain a colon followed by a space, because that separates the reason
			from the note. Those reasons aren't saved.
		</p>
	{/if}
</div>
