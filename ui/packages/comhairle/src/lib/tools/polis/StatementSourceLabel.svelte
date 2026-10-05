<script lang="ts">
	import { cn } from '$lib/utils';
	import type { StatementSource } from './statementSource';

	type Props = { source: StatementSource };

	let { source }: Props = $props();
</script>

<!-- Kept small so the source reads as secondary to the statement; the card colour also marks it.
     A border instead of a tinted fill keeps the small text on the plain card background. -->
<p
	class={cn(
		'inline-flex items-start gap-1 rounded-md border px-1.5 py-px text-xs font-medium',
		source.labelClass
	)}
>
	<source.icon class="mt-0.5 size-3 shrink-0" aria-hidden="true" />
	{source.label()}
</p>
