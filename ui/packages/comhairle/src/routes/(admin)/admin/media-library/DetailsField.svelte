<script lang="ts">
	import { Input } from '$lib/components/EasyForm';
	import type { Snippet } from 'svelte';
	import UpdateSchema from './UpdateSchema';

	interface Props {
		editable: boolean;
		field: keyof typeof UpdateSchema;
		initialValue: string;
		label: string;
		readOnlyField: Snippet<[label: string, value: string]>;
	}

	let { editable, field, initialValue, label, readOnlyField }: Props = $props();
</script>

<div class="mb-2 flex flex-col">
	{#if !editable}
		{@render readOnlyField(label, initialValue)}
	{:else}
		<Input
			{...UpdateSchema[field]}
			{label}
			defaultValue={initialValue}
			type="text"
			class="flex flex-row items-center"
		/>
	{/if}
</div>
