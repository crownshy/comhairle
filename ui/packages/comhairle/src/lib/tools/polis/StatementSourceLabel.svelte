<script lang="ts">
	import { cn } from '$lib/utils';
	import type { StatementSource } from './statementSource';

	type Props = { source: StatementSource };

	let { source }: Props = $props();
</script>

<p
	class={cn(
		'inline-flex items-start gap-1.5 rounded-lg px-2 py-0.5 text-base font-medium',
		source.labelClass
	)}
>
	<source.icon class="mt-1 size-4 shrink-0" aria-hidden="true" />
	{source.label()}
</p>
