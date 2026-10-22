<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import * as Drawer from '$lib/components/ui/drawer';
	import { IsMobile } from '$lib/hooks/is-mobile.svelte';
	import { INPUT_STYLE } from '$lib/components/ui/input/styles';
	import { cn } from '$lib/utils';
	import { ChevronDown } from 'lucide-svelte';

	type Value = string;
	type Label = string;

	type Option = { label: Label; value: Value };

	type Props = Pick<HTMLSelectElement, 'id' | 'name'> & {
		options: Option[];
		defaultSelected?: Value;
		placeholder?: Label;
	};

	let { id, name, options, defaultSelected, placeholder }: Props = $props();

	// svelte-ignore state_referenced_locally
	let selected = $state<Value | undefined>(defaultSelected);
	let label = $derived.by(() => {
		if (selected) {
			return options.find((o) => o.value === selected)?.label;
		}
		return defaultSelected ?? placeholder ?? '';
	});
	let isPlaceholder = $derived<boolean>(
		!selected && defaultSelected === undefined && placeholder !== undefined
	);

	const isMobile = new IsMobile();
</script>

{#if !isMobile.current}
	<Select.Root {name} type="single" bind:value={selected}>
		<Select.Trigger {id} class={cn(INPUT_STYLE)} {placeholder}>
			<span
				class="flex w-full flex-row items-center justify-between"
				class:text-muted-foreground={isPlaceholder}
			>
				{label}
			</span>
		</Select.Trigger>
		<Select.Content>
			{#each options as option (option.value)}
				<Select.Item value={option.value}>
					{option.label}
				</Select.Item>
			{/each}
		</Select.Content>
	</Select.Root>
{:else}
	<Drawer.Root>
		<Drawer.Trigger class={cn(INPUT_STYLE)}>
			<span class="my-auto text-sm" class:text-muted-foreground={isPlaceholder}>
				{label}
			</span>
		</Drawer.Trigger>
		<Drawer.Content>
			{#each options as option (option.value)}
				<span class="border-primary bg-primary/30 rounded-xl">
					{option.label}
				</span>
			{/each}
		</Drawer.Content>
	</Drawer.Root>
{/if}
