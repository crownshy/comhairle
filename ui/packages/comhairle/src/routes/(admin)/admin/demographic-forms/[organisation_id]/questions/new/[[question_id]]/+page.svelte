<script lang="ts">
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabContent from '$lib/components/TabContent.svelte';
	import { Button } from '$lib/components/ui/button';
</script>

<SectionHeader
	content={{
		title: 'New question'
	}}
>
	{#snippet actions()}
		<Button>Create</Button>
	{/snippet}
</SectionHeader>

<TabContent>
	<h2>New question</h2>
</TabContent>
