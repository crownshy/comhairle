<script lang="ts">
	import { cn } from '$lib/utils';
	import type { StatementSource } from './statementSource';

	type Props = { source: StatementSource };

	let { source }: Props = $props();
</script>

<!-- Kept small so the source reads as secondary to the statement. The tint carries the source
     colour because the highlight colours are too light for small text. -->
<p
	class={cn(
		'text-foreground/80 inline-flex items-start gap-1.5 rounded-md px-2 py-0.5 text-xs font-medium sm:text-sm',
		source.tagClass
	)}
>
	<source.icon class="mt-px size-3.5 shrink-0 sm:mt-0.5 sm:size-4" aria-hidden="true" />
	{source.label()}
</p>
