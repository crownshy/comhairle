<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import * as Drawer from '$lib/components/ui/drawer';
	import { IsMobile } from '$lib/hooks/is-mobile.svelte';
	import { INPUT_STYLE } from '$lib/components/ui/input/styles';
	import { cn } from '$lib/utils';
	import SelectItem from './SelectItem.svelte';

	type Value = string;
	type Label = string;

	type Option = [value: Value, label: Label];

	type Props = Pick<HTMLSelectElement, 'id' | 'name'> & {
		question: string;
		options: Option[];
		required?: boolean;
		defaultSelected?: Value;
		placeholder?: Label;
		preventDeselection?: boolean;
		onValueChange?: (value: Value) => void;
	};

	let {
		id,
		name,
		question,
		options,
		required,
		defaultSelected,
		placeholder,
		preventDeselection,
		onValueChange
	}: Props = $props();

	let selected = $derived<Value | undefined>(defaultSelected);
	let label = $derived(options.find(([value]) => value === selected)?.[1] ?? placeholder ?? '');
	let drawerOpen = $state(false);
	let isPlaceholder = $derived<boolean>(
		!selected && defaultSelected === undefined && placeholder !== undefined
	);

	const isMobile = new IsMobile();
	const titleId = $props.id();
</script>

{#if !isMobile.current}
	<Select.Root
		{name}
		{required}
		type="single"
		bind:value={selected}
		{onValueChange}
		allowDeselect={!preventDeselection}
	>
		<Select.Trigger {id} class={cn(INPUT_STYLE)} {placeholder}>
			<span
				class="flex w-full flex-row items-center justify-between"
				class:text-muted-foreground={isPlaceholder}
			>
				{label}
			</span>
		</Select.Trigger>
		<Select.Content>
			{#each options as [value, label] (value)}
				<Select.Item {value}>
					{label}
				</Select.Item>
			{/each}
		</Select.Content>
	</Select.Root>
{:else}
	<Drawer.Root bind:open={drawerOpen}>
		<Drawer.Trigger {id} class={cn(INPUT_STYLE)}>
			<span class="my-auto text-base" class:text-muted-foreground={isPlaceholder}>
				{label}
			</span>
		</Drawer.Trigger>
		<Drawer.Content class="mb-8">
			<Drawer.Header>
				<Drawer.Title id={titleId}>{question}</Drawer.Title>
			</Drawer.Header>
			<fieldset aria-labelledby={titleId} class="mx-5 flex flex-col gap-5 overflow-y-auto">
				{#each options as [value, label] (value)}
					<SelectItem
						{name}
						{label}
						{value}
						defaultSelected={value === selected}
						onselect={(value) => {
							selected = value;
							drawerOpen = false;
							onValueChange?.(value);
						}}
					/>
				{/each}
			</fieldset>
		</Drawer.Content>
	</Drawer.Root>
{/if}
