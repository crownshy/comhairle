<script lang="ts">
	import { onMount } from 'svelte';
	import buildCookieConsentConfig from '$lib/cookies/cookieconsent-config';
	import 'vanilla-cookieconsent/dist/cookieconsent.css';
	import * as CookieConsent from 'vanilla-cookieconsent';
	import { browser } from '$app/environment';

	onMount(() => {
		if (browser) {
			CookieConsent.run(buildCookieConsentConfig());
		}
	});
</script>
