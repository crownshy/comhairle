<script lang="ts">
	import * as CookieConsent from 'vanilla-cookieconsent';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Table from '$lib/components/ui/table';
	import * as m from '$lib/paraglide/messages';

	const sectionTitle = 'text-2xl font-bold text-foreground md:text-3xl';
	const body = 'text-base leading-7 text-foreground';

	const cookies = [
		{ name: 'auth_token', purpose: m.cookies_auth_purpose(), expiry: m.cookies_auth_expiry() },
		{
			name: 'paraglide_lang',
			purpose: m.cookies_lang_purpose(),
			expiry: m.cookies_lang_expiry()
		}
	];
</script>

<section class="flex flex-col gap-8 md:gap-12">
	<h2 class={sectionTitle}>{m.cookies_title()}</h2>

	<div class="flex flex-col gap-5">
		<p class={body}>
			{m.cookies_intro_pre()}<a
				href="https://allaboutcookies.org"
				class="text-primary underline">allaboutcookies.org</a
			>
		</p>

		<Button variant="outline" size="sm" onclick={CookieConsent.showPreferences}>
			{m.cookies_prefs_button()}
		</Button>

		<p class={body}>{m.cookies_essential_intro()}</p>

		<div class="overflow-x-auto">
			<Table.Root>
				<Table.Header>
					<Table.Row>
						<Table.Head>{m.cookies_col_name()}</Table.Head>
						<Table.Head>{m.cookies_col_purpose()}</Table.Head>
						<Table.Head>{m.cookies_col_expiry()}</Table.Head>
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
