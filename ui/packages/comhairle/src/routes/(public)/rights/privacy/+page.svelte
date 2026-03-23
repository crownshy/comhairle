<script lang="ts">
	import ComhairlePrivacyPolicy from '$lib/components/ComhairlePrivacyPolicy.svelte';
</script>

<svelte:head>
	<title>Privacy Policy - Comhairle</title>
</svelte:head>

<ComhairlePrivacyPolicy />

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
