<script lang="ts">
	import { cn, type WithElementRef } from '$lib/utils.js';
	import type { HTMLAttributes } from 'svelte/elements';

	type Props = WithElementRef<HTMLAttributes<HTMLTableSectionElement>> & {
		striped?: boolean;
	};

	let {
		ref = $bindable(null),
		class: className,
		striped = true,
		children,
		...restProps
	}: Props = $props();
</script>

<tbody
	bind:this={ref}
	data-slot="table-body"
	class={cn('[&_tr:last-child]:border-0', striped && '[&_tr]:even:bg-muted/30', className)}
	{...restProps}
>
	{@render children?.()}
</tbody>
