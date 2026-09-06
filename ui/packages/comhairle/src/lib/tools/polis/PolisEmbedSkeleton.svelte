<script lang="ts">
	import PolisVotingSkeleton from './PolisVotingSkeleton.svelte';
</script>

<!--
	The whole step body while we navigate into a Polis step: same card as PolisEmbed,
	so the swap to the real thing is a content change and not a layout change.
-->
<div
	class="bg-muted/50 flex min-h-0 w-full flex-1 flex-col items-center gap-8 overflow-y-auto rounded-2xl"
>
	<PolisVotingSkeleton />
</div>
