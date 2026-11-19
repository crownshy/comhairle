<script lang="ts">
	import type { Snippet } from 'svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import { Button } from '$lib/components/ui/button';
	import { Label } from '$lib/components/ui/label';
	import { Textarea } from '$lib/components/ui/textarea';
	import { cn } from '$lib/utils';
	import { Plus } from '@lucide/svelte';
	import { composeReason, type RejectReason } from '$lib/moderation/moderationPolicy';

	type Props = {
		/** The reject button. Spread `props` onto it so it opens the dialog. */
		trigger: Snippet<[{ props: Record<string, unknown> }]>;
		/** Reject, passing the composed reason (undefined when there is none). */
		onConfirm: (reason: string | undefined) => void;
		/** The conversation's moderation policy reasons. Empty leaves only the note. */
		reasons: RejectReason[];
		/** Heading, e.g. "Reject statement" or "Reject 3 statements". */
		heading?: string;
		/** The statement being rejected, quoted so the row hidden behind the dialog isn't needed. */
		statementText?: string;
		/** When true the trigger is inert and the dialog cannot open. */
		disabled?: boolean;
	};

	let {
		trigger,
		onConfirm,
		reasons,
		heading = 'Reject statement',
		statementText,
		disabled = false
	}: Props = $props();

	// Every row renders one of these dialogs, so ids need a per-instance prefix.
	const instanceId = $props.id();
	const noteId = `${instanceId}-note`;

	// Number keys pick a rule; nine covers any real policy.
	const SHORTCUT_LIMIT = 9;

	let open = $state(false);
	let selected = $state('');
	let note = $state('');
	let noteOpen = $state(false);

	// A policy rule is required so every reject can be checked against the rules later.
	const needsReason = $derived(reasons.length > 0);
	const canConfirm = $derived(!needsReason || selected !== '');
	// With no rules to pick, the note is the only thing to fill in, so it starts open.
	const showNote = $derived(noteOpen || !needsReason);

	function reset() {
		selected = '';
		note = '';
		noteOpen = false;
	}

	function confirm() {
		if (!canConfirm) return;
		onConfirm(composeReason(selected || null, note));
		open = false;
	}

	function openNote() {
		noteOpen = true;
		requestAnimationFrame(() => document.getElementById(noteId)?.focus());
	}

	function handleShortcut(event: KeyboardEvent) {
		if (event.target instanceof HTMLTextAreaElement) return;
		if (event.metaKey || event.ctrlKey || event.altKey) return;

		if (event.key === 'Enter') {
			// Radio items are buttons, so Enter on one would only select it, not submit.
			// Cancel, close and the note link keep their own Enter.
			const onOtherButton =
				event.target instanceof HTMLButtonElement &&
				event.target.getAttribute('role') !== 'radio';
			if (onOtherButton) return;
			event.preventDefault();
			confirm();
			return;
		}
		const index = Number(event.key) - 1;
		if (!Number.isInteger(index) || index < 0 || index >= SHORTCUT_LIMIT) return;
		const reason = reasons[index];
		if (!reason) return;
		event.preventDefault();
		selected = reason.label;
	}
</script>

<Dialog.Root
	bind:open
	onOpenChangeComplete={(isOpen) => {
		// Every open starts empty, so a previous statement's reason can't carry over.
		if (!isOpen) reset();
	}}
>
	<Dialog.Trigger {disabled}>
		{#snippet child({ props })}
			{@render trigger({ props })}
		{/snippet}
	</Dialog.Trigger>
	<Dialog.Content
		class="max-h-[calc(100dvh-2rem)] overflow-y-auto sm:max-w-lg"
		onkeydown={handleShortcut}
	>
		<Dialog.Header>
			<Dialog.Title>{heading}</Dialog.Title>
			{#if statementText}
				<Dialog.Description
					class="text-muted-foreground line-clamp-2 text-base italic"
					title={statementText}
				>
					“{statementText}”
				</Dialog.Description>
			{/if}
		</Dialog.Header>

		<form
			class="flex flex-col gap-4"
			onsubmit={(event) => {
				event.preventDefault();
				confirm();
			}}
		>
			{#if needsReason}
				<fieldset class="flex flex-col">
					<legend class="mb-2 text-base font-medium">Which rule does it break?</legend>
					<RadioGroup.Root bind:value={selected} class="gap-0.5">
						{#each reasons as reason, index (reason.label)}
							{@const isSelected = selected === reason.label}
							<Label
								for="{instanceId}-reason-{index}"
								class={cn(
									'flex min-h-11 cursor-pointer items-start gap-3 rounded-md border border-transparent px-2 py-2.5 font-normal',
									isSelected ? 'border-border bg-muted' : 'hover:bg-muted/50'
								)}
							>
								<RadioGroup.Item
									value={reason.label}
									id="{instanceId}-reason-{index}"
									aria-keyshortcuts={index < SHORTCUT_LIMIT
										? String(index + 1)
										: undefined}
									class="border-muted-foreground mt-1"
								/>
								<span class="flex min-w-0 flex-1 flex-col gap-0.5">
									<span class="text-base font-medium">{reason.label}</span>
									{#if isSelected && reason.description}
										<span class="text-muted-foreground text-base">
											{reason.description}
										</span>
									{/if}
								</span>
								{#if index < SHORTCUT_LIMIT}
									<kbd
										class="text-muted-foreground mt-0.5 rounded border px-1.5 font-sans text-sm"
										aria-hidden="true"
									>
										{index + 1}
									</kbd>
								{/if}
							</Label>
						{/each}
					</RadioGroup.Root>
				</fieldset>
			{/if}

			{#if showNote}
				<div class="flex flex-col gap-2">
					<Label for={noteId} class="text-base font-medium">
						Note for your team <span class="text-muted-foreground font-normal"
							>(optional)</span
						>
					</Label>
					<Textarea
						id={noteId}
						bind:value={note}
						rows={2}
						class="max-h-40 text-base"
						placeholder="For example, which statement it duplicates"
					/>
				</div>
			{:else}
				<Button variant="link" class="h-auto self-start p-0 text-base" onclick={openNote}>
					<Plus class="size-4" />
					Add a note for your team
				</Button>
			{/if}

			<Dialog.Footer class="items-center gap-3 sm:justify-between">
				<p class="text-muted-foreground min-w-0 flex-1 text-base">
					Participants don’t see reasons or notes.
				</p>
				<div class="flex gap-2">
					<Button type="button" variant="ghost" onclick={() => (open = false)}
						>Cancel</Button
					>
					<Button type="submit" variant="destructive" disabled={!canConfirm}
						>{heading}</Button
					>
				</div>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
