<script lang="ts">
	import { onMount } from 'svelte';
	import config from '$lib/cookies/cookieconsent-config';
	import 'vanilla-cookieconsent/dist/cookieconsent.css';
	import * as CookieConsent from 'vanilla-cookieconsent';
	import { browser } from '$app/environment';

	onMount(() => {
		if (browser) {
			CookieConsent.run(config);
		}
	});
</script>
