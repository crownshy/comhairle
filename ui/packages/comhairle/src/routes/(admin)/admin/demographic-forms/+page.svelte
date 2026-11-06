<script lang="ts">
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabContent from '$lib/components/TabContent.svelte';

	const { data } = $props();
</script>

<SectionHeader
	content={{
		eyebrow: 'Forms',
		title: 'Demographic forms',
		description:
			'Build questions, put them together into forms, then add a form to any conversation as a demographic step.'
	}}
/>

<TabContent>
	<h2 class="text-muted-foreground text-xl font-semibold">Select an organisation</h2>
</TabContent>
