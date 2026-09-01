<script lang="ts">
	import CookiesSettingsContent from '$lib/components/rights/CookiesSettingsContent.svelte';
	import * as m from '$lib/paraglide/messages';
</script>

<svelte:head>
	<title>{m.cookies_title()} - Comhairle</title>
</svelte:head>

<CookiesSettingsContent />
