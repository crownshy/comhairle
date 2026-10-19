<script lang="ts">
	import type { ComponentProps } from 'svelte';
	import InfoHover from './InfoHover.svelte';
	import { FormLabel } from '$lib/components/ui/form';
	import { cn } from '$lib/utils';

	interface Props extends ComponentProps<typeof InfoHover> {
		type?: 'form' | 'plain';
		label: string;
		class?: string;
	}

	let { type = 'form', label, class: className, ...props }: Props = $props();
</script>

<div class={cn('flex items-center gap-2 lg:w-50', className)}>
	{#if type === 'form'}
		<FormLabel class="text-sm font-semibold">{label}</FormLabel>
	{:else}
		<p class="text-sm font-semibold">{label}</p>
	{/if}
	{#if props.info}
		<InfoHover {...props} />
	{/if}
</div>
