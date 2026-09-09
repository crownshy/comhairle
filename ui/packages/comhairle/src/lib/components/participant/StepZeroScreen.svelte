<script lang="ts">
	/**
	 * Step zero: the conversation landing page's first screen, one viewport ending in the call
	 * to action. Everything else lives in {@link BeforeYouStart} below the fold.
	 *
	 * Deliberately shaped like a Step rather than like a website page. The chrome above it is
	 * the participant `StepChrome`, so arriving at a Conversation and starting its first Step
	 * look continuous. Not called a Cover: that word belongs to the Step brief's first slide.
	 * See CONTEXT.md, Step zero, and ADR-0021.
	 */
	import type { LocalizedConversationDto } from '@crownshy/api-client/api';
	import * as m from '$lib/paraglide/messages';
	import { ChevronDown } from 'lucide-svelte';
	import { conversationImageUrl } from '$lib/utils/conversationImage';

	let {
		conversation,
		onReadMore
	}: {
		conversation: LocalizedConversationDto;
		/** Scrolls to the detail below. Omitted when there is no detail to scroll to. */
		onReadMore?: () => void;
	} = $props();

	let imageUrl = $derived(conversationImageUrl(conversation.imageUrl));
</script>

<section
	class="mx-auto flex w-full max-w-5xl grow flex-col items-center justify-center gap-6 px-6 py-8 text-center"
>
	<!-- The box is sized by aspect ratio rather than by the file, so the cover has its shape
		before the image arrives and nothing below it moves when it does. -->
	<img
		class="aspect-video max-h-[24vh] w-full max-w-md rounded-2xl object-cover"
		src={imageUrl}
		alt=""
	/>

	<h1 class="max-w-2xl text-4xl leading-tight font-semibold md:text-5xl">
		{conversation.title}
	</h1>

	<p class="text-foreground max-w-xl text-lg md:text-xl">{conversation.shortDescription}</p>

	<!-- Sticky above the call to action bar: on a short screen the stack above runs past the
		fold, and without this the cue is the part that goes under the bar, so the deck below
		looks like it is not there. Once the stack fits it sits in the flow like anything else.
		The backing is for the overflow case, where it floats over the description. -->
	{#if onReadMore}
		<button
			type="button"
			class="text-foreground bg-background/80 sticky bottom-[var(--cta-clearance,7rem)] mt-2 inline-flex items-center gap-1.5 rounded-full px-4 py-2 text-base underline underline-offset-4 backdrop-blur"
			onclick={onReadMore}
		>
			{m.landing_what_is_this_about()}
			<ChevronDown class="size-4 shrink-0" aria-hidden="true" />
		</button>
	{/if}
</section>
