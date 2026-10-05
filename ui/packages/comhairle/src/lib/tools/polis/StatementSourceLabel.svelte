<script lang="ts">
	import { cn } from '$lib/utils';
	import type { StatementSource } from './statementSource';

	type Props = { source: StatementSource };

	let { source }: Props = $props();
</script>

<p class={cn('text-base font-medium', source.labelClass)}>
	{source.label()}
</p>
