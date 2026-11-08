<script lang="ts">
	import type { HTMLTableAttributes } from 'svelte/elements';
	import { cn, type WithElementRef } from '$lib/utils.js';

	let {
		ref = $bindable(null),
		class: className,
		children,
		...restProps
	}: WithElementRef<HTMLTableAttributes> = $props();
</script>

<div
	data-slot="table-container"
	class="border-border relative w-full overflow-x-auto rounded-xl border"
>
	<table
		bind:this={ref}
		data-slot="table"
		class={cn('w-full table-auto caption-bottom text-sm', className)}
		{...restProps}
	>
		{@render children?.()}
	</table>
</div>
