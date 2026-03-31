<script lang="ts">
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import ComhairlePrivacyPolicy from '$lib/components/ComhairlePrivacyPolicy.svelte';

	let { data } = $props();
	let { conversation } = data;
</script>

{#if conversation.privacyPolicy}
	<ContentRenderer content={conversation.privacyPolicy} />
{:else}
	<ComhairlePrivacyPolicy
		class="[&_h1]:text-primary [&_h2]:text-primary flex flex-col gap-4 [&_h1,&_h2,&_h3,&_h4,&_h5,&_h6]:font-bold [&_ul]:list-inside [&_ul]:list-[square]!"
	/>
{/if}
