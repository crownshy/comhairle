<script lang="ts">
	import { cn } from '$lib/utils';
	import type { StatementSource } from './statementSource';

	type Props = { source: StatementSource };

	let { source }: Props = $props();
</script>

<!-- Kept small so the source reads as secondary to the statement; the card colour also marks it,
     which is why phones can drop the icon to keep the label on one line. -->
<p class={cn('inline-flex items-start gap-1.5 text-xs font-medium sm:text-sm', source.labelClass)}>
	<source.icon class="mt-0.5 size-4 shrink-0 max-sm:hidden" aria-hidden="true" />
	{source.label()}
</p>
