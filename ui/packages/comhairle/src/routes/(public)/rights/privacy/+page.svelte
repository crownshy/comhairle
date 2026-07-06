<script lang="ts">
	import ComhairlePrivacyPolicy from '$lib/components/ComhairlePrivacyPolicy.svelte';
</script>

<svelte:head>
	<title>Privacy Policy - Comhairle</title>
</svelte:head>

<ComhairlePrivacyPolicy
	class="text-foreground flex flex-col gap-6 md:gap-8
		[&_h3]:text-xl [&_h3]:font-semibold [&_h3]:md:text-2xl
		[&_p]:text-base [&_p]:leading-7 [&>h2]:text-2xl
		[&>h2]:font-bold [&>h2]:md:text-3xl"
/>
