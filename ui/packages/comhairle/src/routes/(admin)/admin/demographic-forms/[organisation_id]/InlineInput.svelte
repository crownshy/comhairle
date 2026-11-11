<script lang="ts">
	import { Input } from '$lib/components/EasyForm';
	import { cn } from '$lib/utils';
	import type { ComponentProps } from 'svelte';

	let { class: className, ...props }: ComponentProps<typeof Input> = $props();
</script>

<Input
	{...props}
	class={cn('border-primary rounded-none border-0 border-b-3 bg-transparent! pl-1', className)}
/>
