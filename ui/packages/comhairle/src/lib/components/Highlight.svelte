<script lang="ts">
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

<!--
	The "highlight" callout from the Your Rights designs: a full-height primary
	bar on the left with the content beside it. Used for account types, compliance
	notes, contact details, etc.
-->
<div class="flex items-stretch gap-3">
	<div class="bg-primary w-1 self-stretch rounded"></div>
	<div class="text-foreground flex-1 text-base leading-7">
		{@render children()}
	</div>
</div>
