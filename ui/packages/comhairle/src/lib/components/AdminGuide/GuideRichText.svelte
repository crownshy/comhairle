<script lang="ts">
	import { isExternalHref, parseGuideText } from '$lib/admin_guide_text';
	import GuideUiIcon from './GuideUiIcon.svelte';

	// Guide text can mark **bold** UI names, [links](/path) and :icon: buttons;
	// parseGuideText splits it into pieces so nothing is injected as raw HTML.
	let { text }: { text: string } = $props();
</script>

{#each parseGuideText(text) as part, i (i)}
	{#if part.kind === 'bold'}<strong class="font-semibold">{part.text}</strong>
	{:else if part.kind === 'icon'}<GuideUiIcon name={part.name} />
	{:else if part.kind === 'link'}<a
			href={part.href}
			class="text-primary font-medium underline underline-offset-2 hover:no-underline"
			target={isExternalHref(part.href) && !part.href.startsWith('mailto:')
				? '_blank'
				: undefined}
			rel={isExternalHref(part.href) ? 'noopener noreferrer' : undefined}>{part.text}</a
		>
	{:else}{part.text}{/if}
{/each}
