<script lang="ts">
	import { Search } from 'lucide-svelte';

	type Props = {
		value: string;
		/** Scenario params to keep when the form submits. */
		hiddenFields: [string, string][];
	};
	let { value, hiddenFields }: Props = $props();
</script>

<form method="get" role="search" action="#all">
	{#each hiddenFields as [name, fieldValue] (name)}
		<input type="hidden" {name} value={fieldValue} />
	{/each}
	<label
		class="bg-card border-border focus-within:border-primary flex h-14 w-full items-center gap-3 rounded-full border px-5"
	>
		<Search class="text-muted-foreground size-5 shrink-0" />
		<span class="sr-only">Search conversations</span>
		<input
			type="search"
			name="q"
			{value}
			placeholder="Search by topic or keyword"
			class="placeholder:text-muted-foreground w-full min-w-0 border-none bg-transparent p-0 text-base focus:ring-0 focus:outline-none"
		/>
	</label>
</form>
