<script lang="ts">
	import Input from '$lib/components/EasyForm/Input.svelte';
	import UpdateSchema from './UpdateSchema';

	interface Props {
		editable: boolean;
		field: keyof typeof UpdateSchema;
		initialValue: string;
		label: string;
	}

	let { editable, field, initialValue, label }: Props = $props();
</script>

<div class="flex flex-col">
	{#if !editable}
		<span class="text-muted-foreground mb-1 text-sm font-semibold">{label}</span>
		<span class="mb-5">{initialValue}</span>
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
