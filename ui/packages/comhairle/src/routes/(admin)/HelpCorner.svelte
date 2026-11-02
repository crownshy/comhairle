<script lang="ts">
	import { ArrowUpRight, BookOpen, CircleHelp, Compass, MessageCircle, X } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { helpStore } from '$lib/help/helpStore.svelte';
	import { tourStore } from '$lib/tours/tourStore.svelte';
	import HelpChat from './HelpChat.svelte';

	type Props = { chatSrc: string };
	let { chatSrc }: Props = $props();

	const ITEM_CLASS = 'group flex cursor-pointer items-center gap-2.5';
	const LABEL_CLASS =
		'bg-card border-border text-foreground rounded-md border px-3 py-1.5 text-sm font-medium whitespace-nowrap shadow-md';
	const ICON_CLASS =
		'border-primary bg-card text-primary group-hover:bg-accent flex size-12 items-center justify-center rounded-full border shadow-md';

	let expanded = $state(false);
	let chatOpen = $state(false);
	let chatMounted = $state(false);

	function toggle() {
		if (chatOpen) {
			chatOpen = false;
			return;
		}
		expanded = !expanded;
	}

	function openChat() {
		expanded = false;
		chatOpen = true;
		chatMounted = true;
	}

	function startTour() {
		expanded = false;
		tourStore.replay();
	}
</script>

<svelte:window onkeydown={(event) => event.key === 'Escape' && (expanded = false)} />

<HelpChat src={chatSrc} open={chatOpen} mounted={chatMounted} />

<!-- flex-col-reverse: options sit above the button but come after it in tab order. -->
<div class="fixed end-6 bottom-6 z-40 flex flex-col-reverse items-end gap-3">
	<Button
		onclick={toggle}
		aria-expanded={expanded}
		aria-controls="help-options"
		class="h-14 gap-2.5 rounded-full px-5 text-base font-semibold shadow-lg"
	>
		{#if expanded || chatOpen}
			<X class="size-5" aria-hidden="true" />
		{:else}
			<CircleHelp class="size-5" aria-hidden="true" />
		{/if}
		{chatOpen ? 'Close help' : 'Help'}
	</Button>

	{#if expanded}
		<div
			id="help-options"
			class="motion-safe:animate-in motion-safe:fade-in motion-safe:slide-in-from-bottom-2 flex flex-col-reverse items-end gap-3 pe-1"
		>
			<button type="button" class={ITEM_CLASS} onclick={openChat}>
				<span class={LABEL_CLASS}>Ask the assistant</span>
				<span
					class="bg-primary text-primary-foreground flex size-12 items-center justify-center rounded-full shadow-md"
				>
					<MessageCircle class="size-5" aria-hidden="true" />
				</span>
			</button>
			{#if tourStore.active}
				<button type="button" class={ITEM_CLASS} onclick={startTour}>
					<span class={LABEL_CLASS}>Show me around</span>
					<span class={ICON_CLASS}><Compass class="size-5" aria-hidden="true" /></span>
				</button>
			{/if}
			{#if helpStore.guide}
				<a
					href={helpStore.guide.href}
					target="_blank"
					rel="noopener noreferrer"
					class={ITEM_CLASS}
				>
					<span class={LABEL_CLASS}>
						Guide: {helpStore.guide.label}
						<ArrowUpRight class="inline size-3.5" aria-hidden="true" />
						<span class="sr-only">(opens in a new tab)</span>
					</span>
					<span class={ICON_CLASS}><BookOpen class="size-5" aria-hidden="true" /></span>
				</a>
			{/if}
		</div>
	{/if}
</div>
