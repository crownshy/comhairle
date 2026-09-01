<script lang="ts">
	import TermsOfServiceContent from '$lib/components/rights/TermsOfServiceContent.svelte';
	import * as m from '$lib/paraglide/messages';
</script>

<svelte:head>
	<title>{m.tos_title()} - Comhairle</title>
</svelte:head>

<TermsOfServiceContent />
