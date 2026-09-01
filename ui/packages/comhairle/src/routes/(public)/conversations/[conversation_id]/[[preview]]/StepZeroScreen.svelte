<script lang="ts">
	/**
	 * Step zero: the conversation landing page's first screen, one viewport ending in the call
	 * to action. Everything else lives in {@link ConversationDetail} below the fold.
	 *
	 * Deliberately shaped like a Step rather than like a website page. The chrome above it is
	 * the participant `StepChrome`, so arriving at a Conversation and starting its first Step
	 * look continuous. Not called a Cover: that word belongs to the Step brief's first slide.
	 * See CONTEXT.md, Step zero, and ADR-0021.
	 */
	import type { LocalizedConversationDto } from '@crownshy/api-client/api';
	import * as m from '$lib/paraglide/messages';
	import { ChevronDown } from 'lucide-svelte';
	import { totalMinutes, type StepPreview } from '$lib/components/participant/stepPreview';

	let {
		conversation,
		steps,
		onReadMore
	}: {
		conversation: LocalizedConversationDto;
		steps: StepPreview[];
		/** Scrolls to the detail below. Omitted when there is no detail to scroll to. */
		onReadMore?: () => void;
	} = $props();

	let minutes = $derived(totalMinutes(steps));

	let metaLine = $derived(
		[
			steps.length
				? steps.length === 1
					? m.landing_step_count_one({ count: steps.length })
					: m.landing_step_count({ count: steps.length })
				: null,
			minutes > 0 ? m.landing_approx_minutes({ count: minutes }) : null
		]
			.filter(Boolean)
			.join('  ·  ')
	);
</script>

<section
	class="mx-auto flex w-full max-w-5xl grow flex-col items-center justify-center gap-6 px-6 py-8 text-center"
>
	{#if conversation.imageUrl}
		<img
			class="max-h-[24vh] w-full max-w-md rounded-2xl object-cover"
			src={conversation.imageUrl}
			alt=""
		/>
	{/if}

	<h1 class="max-w-2xl text-4xl leading-tight font-semibold md:text-5xl">
		{conversation.title}
	</h1>

	<p class="text-foreground max-w-xl text-lg md:text-xl">{conversation.shortDescription}</p>

	{#if metaLine}
		<p class="text-muted-foreground text-base">{metaLine}</p>
	{/if}

	{#if onReadMore}
		<button
			type="button"
			class="text-foreground mt-2 inline-flex items-center gap-1.5 text-base underline underline-offset-4"
			onclick={onReadMore}
		>
			{m.landing_what_is_this_about()}
			<ChevronDown class="size-4 shrink-0" aria-hidden="true" />
		</button>
	{/if}
</section>
