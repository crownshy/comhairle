<script lang="ts">
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import { createTextContentSource } from '$lib/components/Translation/translationSource.svelte';
	import type { ProposalSection } from '../types';

	type Props = {
		section: ProposalSection;
		primaryLocale: string;
		supportedLocales: string[];
	};

	let { section, primaryLocale, supportedLocales }: Props = $props();

	// Each section owns its own source; the component boundary is the stable init site for the runes,
	// so a section added/removed in the list mounts/disposes its source for free (see ADR-0005). The
	// getter reads the live `section` prop, so a list refresh (new bodyTranslations) is picked up.
	const source = createTextContentSource({
		getTranslation: () => section.bodyTranslations,
		getPrimaryLocale: () => primaryLocale,
		getSupportedLanguages: () => supportedLocales
	});
</script>

<TranslatableField
	{source}
	{primaryLocale}
	supportedLanguages={supportedLocales}
	editorType="rich"
	placeholder="Describe this section"
	minHeight="160px"
	dialogTitle="Translate section"
/>
