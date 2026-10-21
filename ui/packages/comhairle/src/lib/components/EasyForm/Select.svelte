<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import * as Drawer from '$lib/components/ui/drawer';
	import { IsMobile } from '$lib/hooks/is-mobile.svelte';

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

{#if !isMobile}
	<Select.Root {name} type="single" bind:value={selected}>
		<Select.Trigger {id} class="w-full">
			{selected !== undefined
				? options.find((o) => o.value === selected)?.label
				: (defaultSelected ?? placeholder ?? '')}
		</Select.Trigger>
		<Select.Content>
			{#each options as option (option.value)}
				<Select.Item value={option.value}>
					{#snippet children({ selected })}
						{#if selected}
							<span class="border-primary bg-primary/30 rounded-xl">
								{option.label}
							</span>
						{:else}
							{option.label}
						{/if}
					{/snippet}
				</Select.Item>
			{/each}
		</Select.Content>
	</Select.Root>
{:else}
	<Drawer.Root>
		<Drawer.Trigger>
			{selected !== undefined
				? options.find((o) => o.value === selected)?.label
				: (defaultSelected ?? placeholder ?? '')}
		</Drawer.Trigger>
		<Drawer.Content>
			{#each options as option (option.value)}
				<Select.Item value={option.value}>
					{#snippet children({ selected })}
						{#if selected}
							<span class="border-primary bg-primary/30 rounded-xl">
								{option.label}
							</span>
						{:else}
							{option.label}
						{/if}
					{/snippet}
				</Select.Item>
			{/each}
		</Drawer.Content>
	</Drawer.Root>
{/if}
