<script lang="ts">
	import PageHeader from '$lib/components/PageHeader.svelte';
	import TabContent from '../TabContent.svelte';

	const { children } = $props();
</script>

<svelte:head>
	<title>Manage Invites - Comhairle Admin</title>
</svelte:head>

<TabContent>
	<PageHeader title="Recruit" />
	{@render children()}
</TabContent>
