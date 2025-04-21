<script lang="ts">
	import * as CookieConsent from 'vanilla-cookieconsent';
	import Button from '$lib/components/ui/button/button.svelte';
</script>

<div class="w-full">
	<h1>Cookies</h1>

	<p>
		For the Comhairle platform to function we will place small data files, known as cookies on your
		computer. To learn more about cookies and how to manage them check out <a
			href="https://allaboutcookies.org.">allaboutcookies.org</a
		>
	</p>

	<p>Comhairle makes use of the following essential cookies</p>

	<div class="relative overflow-x-auto">
		<table class="w-full text-left text-sm text-gray-500 dark:text-gray-400 rtl:text-right">
			<thead class="bg-gray-50 text-xs uppercase text-gray-700 dark:bg-gray-700 dark:text-gray-400">
				<tr>
					<th scope="col" class="px-6 py-3"> Cookie Name </th>
					<th scope="col" class="px-6 py-3"> Purpose </th>
					<th scope="col" class="px-6 py-3"> Expiry </th>
				</tr>
			</thead>
			<tbody>
				<tr class="border-b border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-800">
					<th
						scope="row"
						class="whitespace-nowrap px-6 py-4 font-medium text-gray-900 dark:text-white"
					>
						auth_token
					</th>
					<td class="px-6 py-4">
						Used to login a user and associate their responses with a user account
					</td>
					<td class="px-6 py-4"> 48 hours after last page submission </td>
				</tr>
				<tr class="border-b border-gray-200 bg-white dark:border-gray-700 dark:bg-gray-800">
					<th
						scope="row"
						class="whitespace-nowrap px-6 py-4 font-medium text-gray-900 dark:text-white"
					>
						paraglide_lang
					</th>
					<td class="px-6 py-4"> Stores the user's language preference </td>
					<td class="px-6 py-4"> 400 days after last page submission </td>
				</tr>
			</tbody>
		</table>
	</div>

	Cookie name Purpose Expiry paraglide_lang Stores the user's language preference Check this
	auth_token Used to login a user and associate their responses with a user account 48 hours after
	the last page submission

	<Button variant="outline" onclick={CookieConsent.showPreferences}
		>Show and edit your cookie preferences</Button
	>
</div>

<style>
	div {
		font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
		line-height: 1.7;
		color: #2c2c2c;
		margin: 2rem auto;
		max-width: 800px;
		padding: 1rem;
	}

	h1 {
		font-size: 2.5rem;
		text-align: center;
		color: #1a1a1a;
		margin-bottom: 2rem;
		border-bottom: 2px solid #ddd;
		padding-bottom: 0.5rem;
	}

	h2 {
		font-size: 1.5rem;
		color: #2e4a7d;
		margin-top: 2rem;
		border-left: 4px solid #2e4a7d;
		padding-left: 0.5rem;
	}

	p {
		margin: 1rem 0;
	}

	ul {
		margin: 1rem 0;
		padding-left: 1.5rem;
		list-style: square;
	}

	li {
		margin-bottom: 0.6rem;
	}

	a {
		color: #0077cc;
		text-decoration: underline;
	}

	a:hover {
		text-decoration: none;
		color: #005599;
	}

	code {
		background-color: #f4f4f4;
		padding: 0.2rem 0.4rem;
		border-radius: 4px;
		font-family: 'Courier New', Courier, monospace;
	}

	@media (max-width: 600px) {
		body {
			padding: 1rem;
		}

		h1 {
			font-size: 2rem;
		}

		h2 {
			font-size: 1.3rem;
		}
	}
</style>
