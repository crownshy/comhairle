<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import * as Dialog from '$lib/components/ui/dialog';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import { Plus, Trash2 } from '@lucide/svelte';

	type Props = {
		open: boolean;
		/** The participant statement being split or reworded. Null while closed. */
		original: PolisStatementAux | null;
		/**
		 * Text of the statement the participant was viewing when they submitted
		 * `original`, if we could resolve it. Shown as context; omitted when absent.
		 */
		viewedContext?: string;
		/** Called after a successful split so the parent can refresh. */
		onDone: () => void | Promise<void>;
	};

	let { open = $bindable(), original, viewedContext, onDone }: Props = $props();

	// The replacement statements the admin is authoring. Starts as one blank field;
	// the original text is shown read-only above for reference, and the admin types
	// (or copies) each replacement deliberately. Reset every time the dialog opens.
	let replacements = $state<string[]>(['']);
	let submitting = $state(false);

	const canSubmit = $derived(replacements.some((r) => r.trim().length > 0) && !submitting);

	function addField() {
		replacements = [...replacements, ''];
	}
	function removeField(index: number) {
		replacements = replacements.filter((_, i) => i !== index);
	}

	async function submit() {
		const target = original;
		if (!target || !canSubmit) return;
		const cleaned = replacements.map((r) => r.trim()).filter(Boolean);
		if (!cleaned.length) return;

		submitting = true;
		const res = await tryCatchAsync(() =>
			apiClient.PolisSplitStatement({ replacements: cleaned }, { params: { id: target.id } })
		);
		submitting = false;

		if (res.err !== null) {
			notifications.send({ priority: 'ERROR', message: 'Failed to split statement' });
			return;
		}

		open = false;
		notifications.send({
			priority: 'INFO',
			message: `Replaced with ${cleaned.length} statement${cleaned.length === 1 ? '' : 's'}`
		});
		await onDone();
	}
</script>

<Dialog.Root
	bind:open
	onOpenChange={(v) => {
		// Fresh blank field each time the dialog is opened.
		if (v) replacements = [''];
	}}
>
	<Dialog.Content class="sm:max-w-4xl">
		<Dialog.Header>
			<Dialog.Title>Split or reword statement</Dialog.Title>
			<Dialog.Description>
				Replace this statement with one or more clean, separately votable statements. The
				original is rejected; the replacements are posted as real statements, not seeds.
			</Dialog.Description>
		</Dialog.Header>

		<div class="flex flex-col gap-4">
			<!-- Original statement (read-only) + the context it was submitted against. -->
			<div class="bg-muted/40 flex flex-col gap-2 rounded-lg p-3">
				<span class="text-muted-foreground text-sm font-semibold uppercase">Original</span>
				<p class="text-base leading-7">{original?.statement_text}</p>
				{#if viewedContext}
					<p class="text-muted-foreground text-sm">
						Submitted while viewing: <span class="italic">{viewedContext}</span>
					</p>
				{/if}
			</div>

			<!-- Replacement fields -->
			<div class="flex flex-col gap-3">
				<span class="text-muted-foreground text-sm font-medium">Replacement statements</span
				>
				{#each replacements as _, i (i)}
					<div class="flex items-start gap-2">
						<Textarea
							bind:value={replacements[i]}
							rows={2}
							placeholder="Write a replacement statement…"
						/>
						<Button
							variant="ghost"
							size="icon"
							disabled={replacements.length === 1}
							onclick={() => removeField(i)}
							title="Remove this replacement"
						>
							<Trash2 class="size-4" />
						</Button>
					</div>
				{/each}
				<div>
					<Button variant="secondary" size="sm" onclick={addField}>
						<Plus class="size-4" />
						Add another statement
					</Button>
				</div>
			</div>
		</div>

		<Dialog.Footer>
			<Button variant="secondary" onclick={() => (open = false)}>Cancel</Button>
			<Button onclick={submit} disabled={!canSubmit}>
				{submitting ? 'Splitting…' : 'Split statement'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
