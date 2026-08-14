<script lang="ts">
	import type { Snippet } from 'svelte';
	import * as Popover from '$lib/components/ui/popover';
	import { Button } from '$lib/components/ui/button';
	import { Toggle } from '$lib/components/ui/toggle';
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
					<Toggle
						size="sm"
						pressed={selected === reason}
						onPressedChange={(on) => (selected = on ? reason : null)}
						class="border-border bg-secondary text-secondary-foreground data-[state=on]:border-primary data-[state=on]:bg-primary data-[state=on]:text-primary-foreground rounded-full border px-3"
					>
						{reason}
					</Toggle>
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
