<script lang="ts">
	import type { Snippet } from 'svelte';
	import * as Popover from '$lib/components/ui/popover';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { REJECT_REASONS, composeReason } from './rejectReasons';

	type Props = {
		/** The reject control that opens the popover (rendered inside the trigger). */
		trigger: Snippet;
		/** Confirm the reject, passing the composed reason (undefined when blank). */
		onConfirm: (reason: string | undefined) => void;
		/** Heading, e.g. "Reject statement" or "Reject 3 statements". */
		heading?: string;
		/** When true the trigger is inert and the popover cannot open. */
		disabled?: boolean;
	};

	let { trigger, onConfirm, heading = 'Reject statement', disabled = false }: Props = $props();

	let open = $state(false);
	// A reason is optional: the moderator can confirm with neither chip nor note.
	let selected = $state<string | null>(null);
	let note = $state('');

	function reset() {
		selected = null;
		note = '';
	}

	function confirm() {
		onConfirm(composeReason(selected, note));
		open = false;
	}
</script>

<Popover.Root
	bind:open
	onOpenChange={(o) => {
		if (!o) reset();
	}}
>
	<Popover.Trigger {disabled}>
		{@render trigger()}
	</Popover.Trigger>
	<Popover.Content class="w-80" align="end">
		<div class="flex flex-col gap-3">
			<div class="flex flex-col gap-1">
				<span class="text-base font-semibold">{heading}</span>
				<span class="text-muted-foreground text-sm">
					Add an optional reason. Only moderators see this.
				</span>
			</div>

			<div class="flex flex-wrap gap-2">
				{#each REJECT_REASONS as reason (reason)}
					<button
						type="button"
						aria-pressed={selected === reason}
						onclick={() => (selected = selected === reason ? null : reason)}
						class={`inline-flex cursor-pointer items-center rounded-full border px-3 py-1.5 text-sm transition-colors ${
							selected === reason
								? 'border-primary bg-primary text-primary-foreground'
								: 'border-border bg-secondary text-secondary-foreground hover:bg-secondary/80'
						}`}
					>
						{reason}
					</button>
				{/each}
			</div>

			<Textarea bind:value={note} rows={2} placeholder="Add a note (optional)…" />

			<div class="flex justify-end gap-2">
				<Button variant="ghost" size="sm" onclick={() => (open = false)}>Cancel</Button>
				<Button variant="destructive" size="sm" onclick={confirm}>Reject</Button>
			</div>
		</div>
	</Popover.Content>
</Popover.Root>
