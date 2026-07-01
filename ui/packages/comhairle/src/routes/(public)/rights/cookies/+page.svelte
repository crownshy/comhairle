<script lang="ts">
	import * as CookieConsent from 'vanilla-cookieconsent';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Table from '$lib/components/ui/table';

	const sectionTitle = 'text-2xl font-bold text-foreground md:text-3xl';
	const body = 'text-base leading-7 text-foreground';

	const cookies = [
		{
			name: 'auth_token',
			purpose: 'Used to login a user and associate their responses with a user account',
			expiry: '48 hours after last page submission'
		},
		{
			name: 'paraglide_lang',
			purpose: "Stores the user's language preference",
			expiry: '400 days after last page submission'
		}
	];
</script>

<svelte:head>
	<title>Cookie Policy - Comhairle</title>
</svelte:head>

<section class="flex flex-col gap-8 md:gap-12">
	<h2 class={sectionTitle}>Cookie Policy</h2>

	<div class="flex flex-col gap-5">
		<p class={body}>
			For the Comhairle platform to function we will place small data files, known as cookies
			on your computer. To learn more about cookies and how to manage them check out
			<a href="https://allaboutcookies.org" class="text-primary underline"
				>allaboutcookies.org</a
			>
		</p>

		<Button variant="outline" size="sm" onclick={CookieConsent.showPreferences}>
			Show and edit your cookie preferences
		</Button>

		<p class={body}>Comhairle makes use of the following essential cookies:</p>

		<div class="overflow-x-auto">
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>Cookie Name</Table.Head>
						<Table.Head>Purpose</Table.Head>
						<Table.Head>Expiry</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each cookies as cookie (cookie.name)}
						<Table.Row>
							<Table.Cell class="text-foreground font-medium"
								>{cookie.name}</Table.Cell
							>
							<Table.Cell class="text-muted-foreground">{cookie.purpose}</Table.Cell>
							<Table.Cell class="text-muted-foreground">{cookie.expiry}</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
		</div>
	</div>
</section>
