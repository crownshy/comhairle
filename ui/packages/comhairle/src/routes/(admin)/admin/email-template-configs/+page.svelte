<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { Plus } from 'lucide-svelte';
	import * as Card from '$lib/components/ui/card';
	import EmailConfigCard from '$lib/components/EmailConfigCard.svelte';

	let { data } = $props();
	const { emailConfigs } = data;
</script>

<svelte:head>
	<title>Custom Emails - Comhairle Admin</title>
</svelte:head>

<h1 class="text-4xl font-bold">Custom emails</h1>

<p class="text-muted-foreground mb-10 text-base font-medium">
	Use this space to manage your custom emails.
</p>

<div class="mx-auto flex max-w-175 flex-col gap-6">
	{#each emailConfigs as emailConfig (emailConfig.id)}
		<Card.Root class="overflow-hidden rounded-3xl pb-0 shadow-sm">
			<EmailConfigCard {emailConfig} />
		</Card.Root>
	{/each}

	<div class="flex justify-center pt-6">
		<Button href="/admin/email-template-configs/new">
			<Plus class="h-4 w-4" /> Add custom email
		</Button>
	</div>
</div>
