<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import * as Drawer from '$lib/components/ui/drawer';
	import { IsMobile } from '$lib/hooks/is-mobile.svelte';
	import { INPUT_STYLE } from '$lib/components/ui/input/styles';
	import { cn } from '$lib/utils';

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

	const isMobile = new IsMobile();
</script>

{#if !isMobile.current}
	<Select.Root {name} type="single" bind:value={selected}>
		<Select.Trigger {id} class={cn(INPUT_STYLE)} {placeholder}>
			<span
				class="w-full justify-start"
				class:text-muted-foreground={selected === undefined &&
					defaultSelected === undefined &&
					placeholder !== undefined}
			>
				{selected !== undefined
					? options.find((o) => o.value === selected)?.label
					: (defaultSelected ?? placeholder ?? '')}
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
			{selected !== undefined
				? options.find((o) => o.value === selected)?.label
				: (defaultSelected ?? placeholder ?? '')}
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
