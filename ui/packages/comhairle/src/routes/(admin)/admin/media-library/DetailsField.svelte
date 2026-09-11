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

<div class="flex flex-col">
	{#if !editable}
		{@render readOnlyField(label, initialValue)}
	{:else}
		<Input
			{...UpdateSchema[field]}
			{label}
			defaultValue={initialValue}
			type="text"
			class="mb-5 flex flex-row items-center"
		/>
	{/if}
</div>
