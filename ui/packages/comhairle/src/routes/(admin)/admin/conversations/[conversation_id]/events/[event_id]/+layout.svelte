<script lang="ts">
	import TabContent from '../../TabContent.svelte';

	const { children } = $props();
</script>

<TabContent>
	{@render children()}
</TabContent>
