<script lang="ts">
	import { isExternalHref, parseGuideText } from '$lib/admin_guide_text';
	import { markGlossaryTerms } from '$lib/admin_guide_glossary';
	import GuideGlossaryTerm from './GuideGlossaryTerm.svelte';
	import GuideUiIcon from './GuideUiIcon.svelte';

	// Guide text can mark **bold** UI names, [links](/path) and :icon: buttons;
	// parseGuideText splits it into pieces so nothing is injected as raw HTML.
	// `terms`: glossary words to show as tooltips in this text (from planGlossaryTerms,
	// so each word is only marked at its first mention on the page).
	let { text, terms }: { text: string; terms?: Set<string> } = $props();

	let parts = $derived(
		terms?.size ? markGlossaryTerms(parseGuideText(text), new Set(terms)) : parseGuideText(text)
	);
</script>

{#each parts as part, i (i)}
	{#if part.kind === 'bold'}<strong class="font-semibold">{part.text}</strong>
	{:else if part.kind === 'icon'}<GuideUiIcon name={part.name} />
	{:else if part.kind === 'term'}<GuideGlossaryTerm text={part.text} term={part.term} />
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
