<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { Popover } from 'bits-ui';
	import ArrowRight from '@lucide/svelte/icons/arrow-right';
	import { HANDBOOK_GLOSSARY } from '$lib/admin_guide_glossary';
	import { learnMore, resolveLearnMore } from '$lib/admin_guide_learn_more.svelte';
	import GuideRichText from './GuideRichText.svelte';

	// Hover, tap or Enter shows the definition. The learnMore link opens in
	// GuideLearnMoreDialog, and is hidden on the page it points to.
	let { text, term }: { text: string; term: string } = $props();

	let open = $state(false);
	const entry = $derived(HANDBOOK_GLOSSARY.entries.get(term));
	const link = $derived(
		entry?.learnMore && resolve(entry.learnMore.href as '/') !== page.url.pathname
			? entry.learnMore
			: undefined
	);

	function openLink(e: MouseEvent) {
		if (!link) return;
		// Cmd/Ctrl/Shift-click still opens the full page in a new tab or window.
		if (e.metaKey || e.ctrlKey || e.shiftKey) return;
		const target = resolveLearnMore(link.href);
		if (!target) return;
		e.preventDefault();
		open = false;
		learnMore.show(target);
	}
</script>

<Popover.Root bind:open
	><Popover.Trigger
		openOnHover
		openDelay={150}
		closeDelay={150}
		aria-label={`${text}: show glossary definition`}
		class="decoration-primary/60 hover:decoration-primary hover:bg-primary/10 focus-visible:outline-primary data-[state=open]:bg-primary/10 cursor-help rounded-sm underline decoration-dotted underline-offset-4 focus-visible:outline-2 focus-visible:outline-offset-1"
		>{text}</Popover.Trigger
	><Popover.Portal>
		<Popover.Content
			side="top"
			sideOffset={6}
			collisionPadding={12}
			trapFocus={false}
			onOpenAutoFocus={(e) => e.preventDefault()}
			class="bg-popover text-popover-foreground border-border data-[state=open]:animate-in data-[state=open]:fade-in-0 data-[state=open]:zoom-in-95 z-50 w-80 max-w-[90vw] rounded-lg border p-4 text-base leading-6 shadow-md"
		>
			<p class="text-foreground font-semibold">{term}</p>
			<p class="text-muted-foreground mt-1">
				<GuideRichText text={entry?.definition ?? ''} />
			</p>
			{#if link}
				<div class="border-border mt-3 border-t pt-3">
					<a
						href={resolve(link.href as '/')}
						onclick={openLink}
						class="text-primary inline-flex items-center gap-1 font-semibold hover:underline"
					>
						{link.label}
						<ArrowRight class="size-3.5" aria-hidden="true" />
					</a>
				</div>
			{/if}
		</Popover.Content>
	</Popover.Portal></Popover.Root
>
