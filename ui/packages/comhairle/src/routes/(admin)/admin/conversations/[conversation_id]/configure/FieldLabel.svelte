<script lang="ts">
	import type { ComponentProps } from 'svelte';
	import InfoHover from './InfoHover.svelte';
	import { FormLabel } from '$lib/components/ui/form';

	interface Props extends ComponentProps<typeof InfoHover> {
		type?: 'form' | 'plain';
		label: string;
	}

	let { type = 'form', label, ...props }: Props = $props();
</script>

<div class="flex items-center gap-2 lg:w-50">
	{#if type === 'form'}
		<FormLabel class="text-sm font-semibold">{label}</FormLabel>
	{:else}
		<p class="text-sm font-semibold">{label}</p>
	{/if}
	{#if props.info}
		<InfoHover {...props} />
	{/if}
</div>
