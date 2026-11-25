<script lang="ts">
	import type { ComponentProps } from 'svelte';
	import Search from '@lucide/svelte/icons/search';
	import { Input } from '$lib/components/ui/input';
	import { cn } from '$lib/utils';

	type Props = Omit<ComponentProps<typeof Input>, 'type' | 'files' | 'ref' | 'value'> & {
		value?: string;
		/** Sizes the magnifier to match a taller field. */
		iconClass?: string;
	};

	let { value = $bindable(''), class: className, iconClass, ...restProps }: Props = $props();
</script>

<label class="pile min-w-0">
	<Input {...restProps} type="search" bind:value class={cn('pl-9 md:text-base', className)} />
	<Search
		class={cn(
			'text-muted-foreground pointer-events-none ml-3 size-4 justify-self-start',
			iconClass
		)}
		aria-hidden="true"
	/>
</label>
