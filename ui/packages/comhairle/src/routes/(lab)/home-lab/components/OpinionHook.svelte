<script lang="ts">
	import { Quote as QuoteIcon } from 'lucide-svelte';
	import type { Quote } from '../mockData';

	type Props = { quote: Quote; conversationTitle?: string; href?: string };
	let { quote, conversationTitle, href }: Props = $props();
</script>

<svelte:element
	this={href ? 'a' : 'figure'}
	{href}
	class="bg-card border-border flex h-full flex-col gap-3 rounded-2xl border p-5 transition hover:shadow-md"
>
	<QuoteIcon class="text-primary size-6" />
	<blockquote class="text-lg leading-snug font-semibold">{quote.text}</blockquote>
	<div class="mt-auto flex flex-col gap-2">
		<div class="bg-muted h-2 w-full overflow-hidden rounded-full" aria-hidden="true">
			<div class="bg-primary h-full rounded-full" style="width: {quote.agreePercent}%"></div>
		</div>
		<span class="text-muted-foreground text-base">{quote.agreePercent}% of people agree</span>
		{#if conversationTitle}
			<span class="text-primary text-base font-semibold">{conversationTitle}</span>
		{/if}
	</div>
</svelte:element>
