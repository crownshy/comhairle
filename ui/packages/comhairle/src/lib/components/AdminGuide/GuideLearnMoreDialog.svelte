<script lang="ts">
	// Shows a glossary tooltip's "learn more" page in a dialog, so the reader doesn't lose
	// their place. The content is the same component the full page uses. Mounted once in
	// the handbook layout; opened with learnMore.show().
	import { afterNavigate } from '$app/navigation';
	import { resolve } from '$app/paths';
	import ExternalLink from '@lucide/svelte/icons/square-arrow-out-up-right';
	import * as Dialog from '$lib/components/ui/dialog';
	import { learnMore } from '$lib/admin_guide_learn_more.svelte';
	import GuideTopicContent from './GuideTopicContent.svelte';
	import ToolGuideContent from './ToolGuideContent.svelte';

	const current = $derived(learnMore.current);
	const title = $derived(
		current?.kind === 'topic' ? current.topic.title : (current?.guide.title ?? '')
	);

	// A link inside the content (including "Open full page") leaves this page: close.
	afterNavigate(({ from, to }) => {
		if (from?.url.pathname !== to?.url.pathname) learnMore.open = false;
	});
</script>

<Dialog.Root bind:open={learnMore.open}>
	<Dialog.Content class="block max-h-[85vh] overflow-y-auto p-6 sm:max-w-3xl md:p-8">
		{#if current}
			<Dialog.Title class="sr-only">{title}</Dialog.Title>
			<a
				href={resolve(current.href as '/')}
				class="text-primary inline-flex items-center gap-1.5 text-sm font-medium hover:underline"
			>
				<ExternalLink class="size-3.5" aria-hidden="true" />
				Open full page
			</a>
			<div class="mt-4">
				{#if current.kind === 'topic'}
					<GuideTopicContent
						guideTitle={current.guide.title}
						topic={current.topic}
						embedded
					/>
				{:else}
					<ToolGuideContent guide={current.guide} embedded />
				{/if}
			</div>
		{/if}
	</Dialog.Content>
</Dialog.Root>
