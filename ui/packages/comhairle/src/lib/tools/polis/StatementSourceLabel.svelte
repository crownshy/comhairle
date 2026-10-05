<script lang="ts">
	import { cn } from '$lib/utils';
	import type { StatementSource } from './statementSource';

	type Props = { source: StatementSource };

	let { source }: Props = $props();
</script>

<p
	class={cn(
		'inline-flex items-start gap-2 rounded-2xl border px-2.5 py-1 text-base font-medium sm:items-center sm:rounded-full sm:px-3',
		source.labelClass
	)}
>
	<source.icon class="mt-0.5 size-5 shrink-0 sm:mt-0" aria-hidden="true" />
	{source.label()}
</p>
