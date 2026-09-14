<script lang="ts">
	import { tick, type Snippet } from 'svelte';
	import * as Command from '$lib/components/ui/command';
	import * as Popover from '$lib/components/ui/popover';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import { Label } from '$lib/components/ui/label';
	import { Textarea } from '$lib/components/ui/textarea';
	import { cn } from '$lib/utils';
	import { composeReason, type RejectReason } from '$lib/moderation/moderationPolicy';
	import { Check, ChevronsUpDown } from '@lucide/svelte';

	type Props = {
		/** The reject control that opens the popover (rendered inside the trigger). */
		trigger: Snippet;
		/** Confirm the reject, passing the composed reason (undefined when blank). */
		onConfirm: (reason: string | undefined) => void;
		/** The conversation's moderation policy reasons. Empty leaves only the note. */
		reasons: RejectReason[];
		/** Heading, e.g. "Reject statement" or "Reject 3 statements". */
		heading?: string;
		/** When true the trigger is inert and the popover cannot open. */
		disabled?: boolean;
	};

	let {
		trigger,
		onConfirm,
		reasons,
		heading = 'Reject statement',
		disabled = false
	}: Props = $props();

	// Every row renders one of these popovers, so ids need a per-instance prefix.
	const uid = $props.id();

	let open = $state(false);
	// A reason is optional: the moderator can confirm with neither. '' is "no reason picked".
	let selected = $state('');
	let note = $state('');

	// Picked once per open: whichever side of the reject button has more room. The body is
	// capped to that room, so a long note makes it scroll instead of flipping the popover to
	// the other side mid-edit.
	let side = $state<'top' | 'bottom'>('bottom');
	let triggerEl = $state<HTMLElement | null>(null);

	function chooseSide() {
		const rect = triggerEl?.getBoundingClientRect();
		if (!rect) return;
		side = window.innerHeight - rect.bottom >= rect.top ? 'bottom' : 'top';
	}

	// The reason picker is a combobox: a list with a search box, opened from the field.
	let pickerOpen = $state(false);
	let search = $state('');
	let pickerTrigger = $state<HTMLButtonElement | null>(null);

	function reset() {
		selected = '';
		note = '';
		search = '';
	}

	function confirm() {
		onConfirm(composeReason(selected || null, note));
		open = false;
	}

	function pick(label: string) {
		selected = label;
		pickerOpen = false;
		// Back to the field so Tab carries on to the note.
		tick().then(() => pickerTrigger?.focus());
	}

	// Typing on the closed field opens the list with that character already in the search.
	function openOnType(e: KeyboardEvent) {
		if (pickerOpen || e.key.length !== 1 || e.metaKey || e.ctrlKey || e.altKey) return;
		if (e.key === ' ') return; // Space keeps its usual job of opening the list.
		e.preventDefault();
		search = e.key;
		pickerOpen = true;
	}
</script>

<Popover.Root
	bind:open
	onOpenChange={(o) => {
		if (o) chooseSide();
	}}
	onOpenChangeComplete={(o) => {
		// After the close animation, whatever closed it (Cancel, Reject, outside click), so
		// every open starts empty and a previous statement's reason can't carry over.
		if (!o) reset();
	}}
>
	<Popover.Trigger bind:ref={triggerEl} {disabled}>
		{@render trigger()}
	</Popover.Trigger>
	<Popover.Content
		class="w-[28rem] max-w-[calc(100vw-2rem)]"
		{side}
		align="end"
		collisionPadding={8}
	>
		<!-- 2rem is the content's own padding. -mx-1/px-1 keeps focus rings from being clipped. -->
		<div
			class="-mx-1 flex max-h-[calc(var(--bits-floating-available-height)-2rem)] flex-col gap-4 overflow-y-auto px-1"
		>
			<span class="text-base font-semibold">{heading}</span>

			{#if reasons.length > 0}
				<div class="flex flex-col gap-2">
					<div class="flex items-center justify-between">
						<Label for="{uid}-reason" class="text-sm font-medium">
							Reason <span class="text-muted-foreground font-normal">(optional)</span>
						</Label>
						{#if selected}
							<Button
								variant="link"
								size="sm"
								class="h-auto p-0"
								onclick={() => (selected = '')}
							>
								Clear
							</Button>
						{/if}
					</div>
					<Popover.Root
						bind:open={pickerOpen}
						onOpenChange={(o) => {
							if (!o) search = '';
						}}
					>
						<Popover.Trigger
							bind:ref={pickerTrigger}
							id="{uid}-reason"
							role="combobox"
							onkeydown={openOnType}
							class={cn(
								buttonVariants({ variant: 'outline' }),
								'h-10 w-full justify-between rounded-lg px-3 text-base font-normal'
							)}
						>
							{#if selected}
								<span class="truncate">{selected}</span>
							{:else}
								<span class="text-muted-foreground">Choose or search a reason</span>
							{/if}
							<ChevronsUpDown class="size-4 shrink-0 opacity-50" />
						</Popover.Trigger>
						<Popover.Content class="w-(--bits-popover-anchor-width) p-0" align="start">
							<Command.Root>
								<Command.Input
									bind:value={search}
									placeholder="Search reasons"
									class="text-base"
								/>
								<!-- Short enough to stay clear of the screen edge; longer lists scroll. -->
								<Command.List class="max-h-72">
									<Command.Empty>No matching reason.</Command.Empty>
									<Command.Group>
										{#each reasons as reason (reason.label)}
											<Command.Item
												value={reason.label}
												keywords={reason.description
													? [reason.description]
													: []}
												onSelect={() => pick(reason.label)}
												class="items-start py-2"
											>
												<Check
													class={cn(
														'mt-1 size-4',
														selected !== reason.label &&
															'text-transparent'
													)}
												/>
												<div class="flex flex-col gap-0.5">
													<span class="text-base font-medium"
														>{reason.label}</span
													>
													{#if reason.description}
														<span class="text-muted-foreground text-sm">
															{reason.description}
														</span>
													{/if}
												</div>
											</Command.Item>
										{/each}
									</Command.Group>
								</Command.List>
							</Command.Root>
						</Popover.Content>
					</Popover.Root>
				</div>
			{/if}

			<div class="flex flex-col gap-2">
				<Label for="{uid}-note" class="text-sm font-medium">
					Note <span class="text-muted-foreground font-normal">(optional)</span>
				</Label>
				<Textarea
					id="{uid}-note"
					bind:value={note}
					rows={2}
					class="max-h-40"
					placeholder="Explain why, for example which statement it duplicates"
				/>
			</div>

			<div class="flex justify-end gap-2">
				<Button variant="ghost" size="sm" onclick={() => (open = false)}>Cancel</Button>
				<Button variant="destructive" size="sm" onclick={confirm}>Reject</Button>
			</div>
		</div>
	</Popover.Content>
</Popover.Root>
