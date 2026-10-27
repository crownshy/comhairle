<script lang="ts">
	import SelectItem from './SelectItem.svelte';
	import type { Option } from './types.ts';

	type Props = {
		title: string;
		name: string;
		options: Option[];
		onselect: (value: string[]) => void;
		defaultSelected?: string[];
		multiple?: boolean;
	};

	let { title, name, options, onselect, defaultSelected = [], multiple }: Props = $props();

	// svelte-ignore state_referenced_locally
	let selected = $state.raw<string[]>(defaultSelected);

	function handleSelect(value: string) {
		const index = selected.findIndex((v) => v === value);
		if (index >= 0) {
			selected.splice(index, 1);
		} else {
			selected.push(value);
		}

		onselect(selected);
	}
</script>

<div class="mb-5 text-sm font-semibold">{title}</div>
<section class="flex flex-col gap-y-2">
	{#each options as option (option.value)}
		<SelectItem
			{name}
			onselect={handleSelect}
			multi={multiple}
			{...option}
			defaultSelected={defaultSelected?.includes(option.value) ?? false}
		/>
	{/each}
</section>
