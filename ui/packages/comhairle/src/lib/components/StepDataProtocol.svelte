<script lang="ts">
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import {
		defaultDataProtocolParagraphs,
		hasDataProtocolText,
		toolUsesAI
	} from '$lib/dataProtocol';
	import { m } from '$lib/paraglide/messages';
	import type { ComhairleDocument } from '@crownshy/api-client/api';

	type Props = {
		/** The admin's text for this step, or blank to show the tool default. */
		text: string | null | undefined;
		toolType: string | undefined;
		onOpenFaq: () => void;
		availableDocuments?: ComhairleDocument[];
		conversationId?: string;
	};

	let { text, toolType, onOpenFaq, availableDocuments = [], conversationId }: Props = $props();
</script>

<section class="border-border mb-6 flex flex-col gap-3 border-b pb-6">
	<h2 class="text-primary text-xl font-bold">{m.data_protocol_step_heading()}</h2>
	{#if hasDataProtocolText(text)}
		<ContentRenderer content={text ?? ''} {availableDocuments} {conversationId} />
	{:else}
		{#each defaultDataProtocolParagraphs(toolType) as paragraph, index (index)}
			<p class="text-base">{paragraph}</p>
		{/each}
	{/if}
	{#if toolUsesAI(toolType)}
		<button
			type="button"
			class="text-primary self-start text-base underline underline-offset-4"
			onclick={onOpenFaq}
		>
			{m.data_protocol_ai_faq()}
		</button>
	{/if}
</section>
