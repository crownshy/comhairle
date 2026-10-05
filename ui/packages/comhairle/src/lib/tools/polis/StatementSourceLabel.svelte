<script lang="ts">
	import { cn } from '$lib/utils';
	import type { StatementSource } from './statementSource';

	type Props = { source: StatementSource };

	let { source }: Props = $props();
</script>

<!-- Kept small so the source reads as secondary to the statement; the card colour also marks it. -->
<p
	class={cn(
		'inline-flex items-start gap-1 rounded-md px-1.5 py-0.5 text-xs font-medium',
		source.labelClass
	)}
>
	<source.icon class="mt-0.5 size-3 shrink-0" aria-hidden="true" />
	{source.label()}
</p>
