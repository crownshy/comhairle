<script lang="ts">
	import { ArrowRight } from 'lucide-svelte';

	let { label = 'See all' }: { label?: string } = $props();
</script>

<!-- Sits inside a `group` link so the arrow moves when the whole tile is hovered. -->
<span
	class="text-primary flex items-center gap-1 text-sm font-medium opacity-80 transition-opacity group-hover:opacity-100"
>
	{label}
	<ArrowRight class="size-3.5 transition-transform group-hover:translate-x-0.5" />
</span>
