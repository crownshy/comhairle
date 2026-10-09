<script lang="ts">
	import { cn } from '$lib/utils.js';
	import { LoaderCircle, type IconProps } from 'lucide-svelte';

	let { class: className, ...restProps }: IconProps = $props();
</script>

<LoaderCircle
	role="status"
	aria-label="Loading"
	class={cn('size-4 animate-spin', className)}
	{...restProps}
/>
