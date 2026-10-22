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
		question: string;
		options: Option[];
		defaultSelected?: Value;
		placeholder?: Label;
	};

	let { id, name, question, options, defaultSelected, placeholder }: Props = $props();

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
		<Drawer.Content class="mb-8">
			<Drawer.Header class="font-semibold">
				{question}
			</Drawer.Header>
			<section class="mx-5 flex flex-col gap-5 overflow-y-auto">
				{#each options as option (option.value)}
					<label class=" border-primary bg-primary/30 rounded-xl p-5">
						<input type="radio" />
						{option.label}
					</label>
				{/each}
			</section>
		</Drawer.Content>
	</Drawer.Root>
{/if}
