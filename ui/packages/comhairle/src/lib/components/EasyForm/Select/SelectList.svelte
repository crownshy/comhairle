<script lang="ts">
	import SelectItem from './SelectItem.svelte';
	import type { Option } from './types.ts';

	type Props = {
		title: string;
		name: string;
		options: Option[];
		onselect: (value: string) => void;
	};

	let { title, name, options, onselect }: Props = $props();
</script>

<div class="mb-5 text-xl font-semibold">{title}</div>
<section class="flex flex-col gap-y-2">
	{#each options as option (option.value)}
		<SelectItem {name} {onselect} {...option} />
	{/each}
</section>
