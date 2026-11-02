<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { Plus } from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import EmailConfigCard from '$lib/components/EmailConfigCard.svelte';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import TabContent from '$lib/components/TabContent.svelte';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { emailTemplateConfigsTour } from './tour';

	useTour(() => emailTemplateConfigsTour);

	let { data } = $props();
	const { emailConfigs } = data;
</script>

<svelte:head>
	<title>Email templates - Comhairle Admin</title>
</svelte:head>

<SectionHeader
	content={{
		eyebrow: 'Library',
		title: 'Email templates',
		description: 'Use this space to manage your custom emails.'
	}}
>
	{#snippet actions()}
		<Button href="/admin/email-template-configs/new">
			<Plus class="h-4 w-4" /> Add custom email
		</Button>
	{/snippet}
</SectionHeader>
<TabContent>
	<div data-tour="email-configs-list" class="flex max-w-175 flex-col gap-6">
		{#each emailConfigs as emailConfig (emailConfig.id)}
			<Card.Root class="overflow-hidden rounded-3xl pb-0 shadow-sm">
				<EmailConfigCard {emailConfig} />
			</Card.Root>
		{/each}
	</div>
</TabContent>
