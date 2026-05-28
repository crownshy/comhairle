<script lang="ts">
	import CheckIcon from '@lucide/svelte/icons/check';
	import ChevronsUpDownIcon from '@lucide/svelte/icons/chevrons-up-down';
	import { tick } from 'svelte';
	import * as Command from '$lib/components/ui/command/index.js';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { cn } from '$lib/utils.js';

	type Props = {
		items: { value: string; label: string }[];
		selectedItem?: { value: string; label: string };
		placeholder?: string;
		emptyMessage?: string;
		onSelect?: (item: { value: string; label: string }) => void;
	};

	let { items, selectedItem, placeholder, emptyMessage, onSelect }: Props = $props();

	const inputPlaceholder = $derived(selectedItem?.label || placeholder || 'Select an option...');

	let open = $state(false);
	let triggerRef = $state<HTMLButtonElement | null>(null);

	function handleSelect(item: { value: string; label: string }) {
		selectedItem = { value: item.value, label: item.label };
		onSelect?.(item);
		closeAndFocusTrigger();
	}

	// We want to refocus the trigger button when the user selects
	// an item from the list so users can continue navigating the
	// rest of the form with the keyboard.
	function closeAndFocusTrigger() {
		open = false;
		tick().then(() => {
			triggerRef?.focus();
		});
	}
</script>

<Popover.Root bind:open>
	<Popover.Trigger bind:ref={triggerRef}>
		{#snippet child({ props })}
			<Button
				variant="outline"
				class="w-50 justify-between"
				{...props}
				role="combobox"
				aria-expanded={open}
			>
				{inputPlaceholder}
				<ChevronsUpDownIcon class="ms-2 size-4 shrink-0 opacity-50" />
			</Button>
		{/snippet}
	</Popover.Trigger>
	<Popover.Content class="w-50 p-0">
		<Command.Root>
			<Command.Input placeholder={inputPlaceholder} />
			<Command.List>
				<Command.Empty>{emptyMessage || 'No option found.'}</Command.Empty>
				<Command.Group>
					{#each items as item (item.value)}
						<Command.Item value={item.value} onSelect={() => handleSelect(item)}>
							<CheckIcon
								class={cn(
									'me-2 size-4',
									selectedItem?.value !== item.value && 'text-transparent'
								)}
							/>
							{item.label}
						</Command.Item>
					{/each}
				</Command.Group>
			</Command.List>
		</Command.Root>
	</Popover.Content>
</Popover.Root>
