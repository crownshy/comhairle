<script lang="ts">
	import { resolve } from '$app/paths';
	import { Button, LoadingButton } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Label } from '$lib/components/ui/label';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import * as Dialog from '$lib/components/ui/dialog';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { pluralise } from '$lib/utils/pluralise';
	import { apiClient } from '@crownshy/api-client/client';
	import type { PolisStatementAux } from '@crownshy/api-client/api';
	import Plus from '@lucide/svelte/icons/plus';
	import Trash2 from '@lucide/svelte/icons/trash-2';
	import { permissions } from '$lib/permissions.svelte';
	import { cn } from '$lib/utils';
	import { MODERATION_GUIDE_PARAMS } from './moderationGuidance';
	import { isUnchanged, statementChanges } from './statementChanges';

	type Props = {
		conversationId: string;
		open: boolean;
		/** The participant statement being split or reworded. Null while closed. */
		original: PolisStatementAux | null;
		/** What the participant first wrote, when `original` is already a moderator's edit. */
		participantText?: string;
		/**
		 * Text of the statement the participant was viewing when they submitted
		 * `original`, if we could resolve it. Shown as context; omitted when absent.
		 */
		viewedContext?: string;
		/** Called after a successful split so the parent can refresh. */
		onDone: () => void | Promise<void>;
		/** Undo the split just made, offered on the confirmation toast. */
		onUndo: (original: PolisStatementAux) => void;
	};

	let {
		conversationId,
		open = $bindable(),
		original,
		participantText,
		viewedContext,
		onDone,
		onUndo
	}: Props = $props();
	const canModerate = $derived(
		permissions.can('conversation', 'conversation_moderate', conversationId)
	);

	const originalText = $derived(original?.statement_text ?? '');
	// Changes are shown against the participant's words, so repeated edits can't drift unseen.
	const baselineText = $derived(participantText ?? originalText);

	type Mode = 'reword' | 'split';
	const MODES: { value: Mode; title: string; description: string }[] = [
		{
			value: 'reword',
			title: 'Reword it',
			description: 'It makes one point, but a typo or unclear wording gets in the way.'
		},
		{
			value: 'split',
			title: 'Split it',
			description: 'It makes more than one point, so people can’t answer it with one vote.'
		}
	];
	const isMode = (value: string): value is Mode => MODES.some((m) => m.value === value);

	// These are writable deriveds that reset whenever a different statement is opened, so a
	// cancelled draft survives reopening the same one. Derived values aren't deeply reactive,
	// so every edit replaces the array rather than changing an item in place.
	let mode = $derived.by((): Mode | null => {
		void original?.id;
		return null;
	});
	let replacements = $derived(original ? [original.statement_text] : []);
	let attempted = $derived.by(() => {
		void original?.id;
		return false;
	});
	let submitting = $state(false);

	const filled = $derived(replacements.map((r) => r.trim()).filter(Boolean));
	const problem = $derived.by(() => {
		if (!mode) return 'Choose whether to reword or split it first.';
		if (mode === 'reword') {
			if (filled.length === 0) return 'Write the new wording first.';
			if (isUnchanged(originalText, filled[0]))
				return 'Change the wording first, or close this and accept it as written.';
			return null;
		}
		if (filled.length < 2) return 'A split needs at least two statements.';
		if (filled.some((text) => isUnchanged(originalText, text)))
			return 'Cut each statement down to one point.';
		return null;
	});
	const submitLabel = $derived(
		mode === 'split'
			? `Replace with ${Math.max(filled.length, 2)} statements`
			: 'Replace statement'
	);

	function chooseMode(next: Mode) {
		mode = next;
		attempted = false;
		const first = replacements[0] ?? originalText;
		replacements = next === 'split' ? [first, originalText] : [first];
	}
	function updateField(index: number, text: string) {
		replacements = replacements.with(index, text);
	}
	function addField() {
		replacements = [...replacements, ''];
	}
	function removeField(index: number) {
		replacements = replacements.filter((_, i) => i !== index);
	}

	async function submit() {
		if (!canModerate || submitting) return;
		const target = original;
		if (!target) return;
		if (problem) {
			attempted = true;
			const focusId = mode ? 'replacement-0' : 'replace-mode-reword';
			document.getElementById(focusId)?.focus();
			return;
		}

		// Snapshot, so typing while the request runs can't change what the toast reports.
		const submitted = filled;
		submitting = true;
		const res = await tryCatchAsync(() =>
			apiClient.PolisSplitStatement(
				{ replacements: submitted },
				{ params: { id: target.id } }
			)
		);
		submitting = false;

		if (res.err !== null) {
			notifications.send({ priority: 'ERROR', message: 'Failed to replace the statement' });
			return;
		}

		open = false;
		notifications.send({
			priority: 'INFO',
			message: `Replaced with ${submitted.length} ${pluralise(submitted.length, 'statement')}`,
			duration: 6000,
			action: { label: 'Undo', onClick: () => onUndo(target) }
		});
		await onDone();
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="max-h-[calc(100dvh-2rem)] overflow-y-auto sm:max-w-2xl">
		<Dialog.Header>
			<Dialog.Title>Edit statement</Dialog.Title>
			<Dialog.Description class="text-base">
				Participants vote on your version. The original stays in the moderation report.
			</Dialog.Description>
		</Dialog.Header>

		<div class="flex flex-col gap-6">
			<section class="bg-muted/50 flex flex-col gap-1 rounded-lg p-4">
				<h3 class="text-muted-foreground text-sm font-medium">
					What the participant wrote
				</h3>
				<p class="text-base leading-7">{baselineText}</p>
				{#if participantText}
					<p class="text-muted-foreground text-base">
						Current edited version: <span class="italic">{originalText}</span>
					</p>
				{/if}
				{#if viewedContext}
					<p class="text-muted-foreground text-base">
						In response to: <span class="italic">{viewedContext}</span>
					</p>
				{/if}
			</section>

			<fieldset class="flex flex-col gap-3">
				<legend class="mb-3 text-base font-semibold">What needs fixing?</legend>
				<RadioGroup.Root
					value={mode ?? ''}
					onValueChange={(value) => {
						if (isMode(value)) chooseMode(value);
					}}
					class="grid gap-3 sm:grid-cols-2"
				>
					{#each MODES as option (option.value)}
						<Label
							for="replace-mode-{option.value}"
							class={cn(
								'flex cursor-pointer items-start gap-3 rounded-lg border p-4 font-normal transition-colors',
								mode === option.value
									? 'border-primary bg-primary/5'
									: 'hover:bg-muted/50'
							)}
						>
							<RadioGroup.Item
								value={option.value}
								id="replace-mode-{option.value}"
								class="mt-1"
							/>
							<span class="flex flex-col gap-1">
								<span class="text-base font-medium">{option.title}</span>
								<span class="text-muted-foreground text-sm leading-5">
									{option.description}
								</span>
							</span>
						</Label>
					{/each}
				</RadioGroup.Root>
			</fieldset>

			{#if mode}
				<section class="flex flex-col gap-4">
					<div class="flex flex-col gap-1">
						<h3 class="text-base font-semibold">
							{mode === 'split' ? 'Your statements' : 'Your version'}
						</h3>
						<p class="text-muted-foreground text-sm leading-6">
							{mode === 'split'
								? 'Each box starts with the whole statement. Cut each one down to a single point, keeping their words.'
								: 'Fix only what gets in the way. Keep their words, tone and strength of opinion.'}
							<a
								href={resolve(
									'/(admin)/admin/info/how-to/[guide_id]/[topic_id]',
									MODERATION_GUIDE_PARAMS
								)}
								target="_blank"
								rel="noopener"
								class="text-primary whitespace-nowrap underline underline-offset-4"
							>
								Full guide
							</a>
						</p>
					</div>

					{#each replacements as text, index (index)}
						{@const changes = statementChanges(baselineText, text)}
						<div class="flex flex-col gap-2">
							<div
								class={cn(
									'flex items-center justify-between gap-2',
									mode === 'reword' && 'sr-only'
								)}
							>
								<Label for="replacement-{index}" class="text-base font-medium">
									{mode === 'split' ? `Statement ${index + 1}` : 'Your version'}
								</Label>
								{#if replacements.length > 2}
									<Button
										variant="ghost"
										size="icon"
										onclick={() => removeField(index)}
										aria-label="Remove statement {index + 1}"
										title="Remove this statement"
									>
										<Trash2 class="size-4" />
									</Button>
								{/if}
							</div>
							<Textarea
								id="replacement-{index}"
								value={text}
								oninput={(event) => updateField(index, event.currentTarget.value)}
								rows={2}
								class="field-sizing-content min-h-16 text-base"
							/>
							{#if text.trim() && !isUnchanged(baselineText, text)}
								<p class="text-muted-foreground text-base leading-6">
									<span class="font-medium"
										>{participantText
											? 'Changes from what they wrote:'
											: 'Changes:'}</span
									>
									{#each changes as part, partIndex (partIndex)}
										{#if part.kind === 'removed'}
											<del class="text-destructive">{part.text}</del>{changes[
												partIndex + 1
											]?.kind === 'added'
												? ' '
												: ''}
										{:else if part.kind === 'added'}
											<ins
												class="text-foreground bg-primary/15 rounded-sm px-0.5 no-underline"
												>{part.text}</ins
											>
										{:else}
											<span>{part.text}</span>
										{/if}
									{/each}
								</p>
							{/if}
						</div>
					{/each}

					{#if mode === 'split'}
						<Button variant="outline" size="sm" class="w-fit" onclick={addField}>
							<Plus class="size-4" />
							Add another point
						</Button>
					{/if}
				</section>
			{/if}
		</div>

		<Dialog.Footer class="items-center gap-3 border-t pt-4 sm:justify-between">
			<p class="text-destructive text-base" aria-live="polite">
				{attempted && problem ? problem : ''}
			</p>
			<div class="flex gap-2">
				<Button variant="secondary" onclick={() => (open = false)}>Cancel</Button>
				<LoadingButton loading={submitting} onclick={submit}>
					{submitLabel}
				</LoadingButton>
			</div>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
