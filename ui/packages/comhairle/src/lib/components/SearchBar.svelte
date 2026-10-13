<script lang="ts">
	import type { ComponentProps } from 'svelte';
	import { Search } from 'lucide-svelte';
	import { Input } from '$lib/components/ui/input';
	import { cn } from '$lib/utils';

	type Props = Omit<ComponentProps<typeof Input>, 'type' | 'files' | 'ref' | 'value'> & {
		value?: string;
	};

	let { value = $bindable(''), class: className, ...restProps }: Props = $props();
</script>

<label class="pile min-w-0">
	<Input {...restProps} type="search" bind:value class={cn('pl-9 md:text-base', className)} />
	<Search
		class="text-muted-foreground pointer-events-none ml-3 size-4 justify-self-start"
		aria-hidden="true"
	/>
</label>
