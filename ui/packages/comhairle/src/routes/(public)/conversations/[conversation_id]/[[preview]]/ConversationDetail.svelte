<script lang="ts" module>
	import type { LocalizedConversationDto as Conversation } from '@crownshy/api-client/api';
	import type { StepPreview as Step } from '$lib/components/participant/stepPreview';
	import * as messages from '$lib/paraglide/messages';

	/**
	 * The sections this component would render, in order. Only sections with content are
	 * included, so the jump strip never promises an empty one and the cover knows whether
	 * there is anything below to scroll to.
	 *
	 * `label` is the jump pill, which is shorter than the section's own heading: the strip
	 * has to stay one line on a narrow screen, the heading can be explicit.
	 */
	export function landingSections(conversation: Conversation, steps: Step[]) {
		return [
			{
				id: 'about',
				label: messages.landing_nav_about(),
				show: !!conversation.description
			},
			{
				id: 'steps',
				label: messages.landing_nav_steps(),
				show: steps.length > 0
			},
			{
				id: 'questions',
				label: messages.landing_questions_heading(),
				show: !!conversation.faqs
			},
			{
				id: 'your-data',
				label: messages.landing_your_data_heading(),
				show: !!conversation.privacyPolicy
			}
		].filter((s) => s.show);
	}
</script>

<script lang="ts">
	/**
	 * Everything about a conversation that is not the cover: the long description, the steps,
	 * the FAQs and the privacy policy, as sections under a sticky strip of jump links.
	 *
	 * Sections rather than a sheet so the content is linkable, scrollable and free to grow.
	 * A modal would have to hold rich text, a step list and a privacy policy inside a
	 * viewport-height scroll area, on top of the page it came from. See ADR-0021.
	 */
	import type { ComhairleDocument, LocalizedConversationDto } from '@crownshy/api-client/api';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import * as m from '$lib/paraglide/messages';
	import type { StepPreview } from '$lib/components/participant/stepPreview';

	let {
		conversation,
		steps,
		availableDocuments = []
	}: {
		conversation: LocalizedConversationDto;
		steps: StepPreview[];
		availableDocuments?: ComhairleDocument[];
	} = $props();

	let sections = $derived(landingSections(conversation, steps));

	function scrollTo(id: string) {
		document.getElementById(id)?.scrollIntoView({ behavior: 'smooth', block: 'start' });
	}

	// Which pill is lit. Without it the strip is four identical buttons and reads as a menu
	// rather than as a position in the page.
	let activeSection = $state('');

	$effect(() => {
		const nodes = sections
			.map((s) => document.getElementById(s.id))
			.filter((n): n is HTMLElement => !!n);
		if (!nodes.length) return;

		const observer = new IntersectionObserver(
			(entries) => {
				const visible = entries.filter((e) => e.isIntersecting);
				if (visible.length) activeSection = visible[0].target.id;
			},
			// A band just under the sticky strip, so exactly one section is active at a time.
			{ rootMargin: '-72px 0px -70% 0px' }
		);
		nodes.forEach((n) => observer.observe(n));
		return () => observer.disconnect();
	});
</script>

{#if sections.length}
	<div id="conversation-detail" class="bg-background/95 sticky top-0 z-20 border-y backdrop-blur">
		<nav class="mx-auto flex w-full max-w-5xl gap-2 overflow-x-auto px-5 py-3 md:px-6">
			{#each sections as section (section.id)}
				<button
					type="button"
					class="shrink-0 rounded-full px-4 py-1.5 text-sm font-medium transition-colors {activeSection ===
					section.id
						? 'bg-foreground text-background'
						: 'bg-accent text-accent-foreground'}"
					aria-current={activeSection === section.id ? 'true' : undefined}
					onclick={() => scrollTo(section.id)}
				>
					{section.label}
				</button>
			{/each}
		</nav>
	</div>

	<div class="mx-auto flex w-full max-w-5xl flex-col gap-10 px-5 pt-10 pb-28 md:px-6">
		{#if conversation.description}
			<section id="about" class="scroll-mt-20">
				<h2 class="mb-4 text-2xl font-semibold">{m.landing_about_heading()}</h2>
				<p class="text-foreground text-base leading-relaxed md:text-lg">
					{conversation.description}
				</p>
			</section>
		{/if}

		{#if steps.length}
			<section id="steps" class="scroll-mt-20">
				<h2 class="mb-4 text-2xl font-semibold">{m.landing_steps_heading()}</h2>
				<ol class="flex flex-col gap-4">
					{#each steps as step, index (step.id)}
						{@const StepIcon = step.icon}
						<li class="flex items-center gap-4">
							<span
								class="bg-accent text-accent-foreground flex size-10 shrink-0 items-center justify-center rounded-full"
							>
								{#if StepIcon}
									<StepIcon class="size-5" aria-hidden="true" />
								{:else}
									<span class="text-sm font-medium">{index + 1}</span>
								{/if}
							</span>
							<span class="min-w-0 flex-1 text-base">{step.name}</span>
							<span class="text-muted-foreground shrink-0 text-sm">
								{#if step.minutes}{m.landing_step_minutes({
										count: step.minutes
									})}{/if}
								{#if step.optional}{step.minutes
										? ' · '
										: ''}{m.landing_step_optional()}{/if}
							</span>
						</li>
					{/each}
				</ol>
			</section>
		{/if}

		{#if conversation.faqs}
			<section id="questions" class="scroll-mt-20">
				<h2 class="mb-4 text-2xl font-semibold">{m.landing_questions_heading()}</h2>
				<ContentRenderer
					content={conversation.faqs}
					{availableDocuments}
					conversationId={conversation.id}
				/>
			</section>
		{/if}

		{#if conversation.privacyPolicy}
			<section id="your-data" class="scroll-mt-20">
				<h2 class="mb-4 text-2xl font-semibold">{m.landing_your_data_heading()}</h2>
				<ContentRenderer
					content={conversation.privacyPolicy}
					{availableDocuments}
					conversationId={conversation.id}
				/>
			</section>
		{/if}
	</div>
{/if}
