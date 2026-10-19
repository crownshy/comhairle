<script lang="ts">
	import { cn } from '$lib/utils.js';
	import LoaderCircle from '@lucide/svelte/icons/loader-circle';
	import type { ComponentProps } from 'svelte';

	type Props = ComponentProps<typeof LoaderCircle>;

	let { class: className, ...restProps }: Props = $props();
</script>

<LoaderCircle
	role="status"
	aria-label="Loading"
	class={cn('size-4 animate-spin', className)}
	{...restProps}
/>
