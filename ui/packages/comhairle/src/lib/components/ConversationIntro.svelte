<script lang="ts">
	import type { LocalizedConversationDto } from '@crownshy/api-client/api';
	import type { Snippet } from 'svelte';
	import { resolve } from '$app/paths';
	import ChevronLeft from '@lucide/svelte/icons/chevron-left';
	import MessageSquareText from '@lucide/svelte/icons/message-square-text';
	import ConversationStatusBadge from '$lib/components/ConversationStatusBadge.svelte';
	import { m } from '$lib/paraglide/messages';
	import Conversation from '$lib/interfaces/Conversation';

	type Props = {
		conversation: LocalizedConversationDto;
		/** The join controls: a button, or the email and guest steps. */
		children: Snippet;
		/** The main action again, in a bar along the bottom of phones once the join bar scrolls away. */
		StickyAction?: Snippet;
	};
	let { conversation, children, StickyAction }: Props = $props();

	const statusLabel = $derived(
		conversation.isComplete ? m.conversations_filter_ended() : m.conversations_filter_open()
	);

	let joinBar = $state<HTMLElement>();
	let joinBarVisible = $state(true);

	$effect(() => {
		if (!joinBar) return;
		const observer = new IntersectionObserver(([entry]) => {
			joinBarVisible = entry.isIntersecting;
		});
		observer.observe(joinBar);
		return () => observer.disconnect();
	});
</script>

<!-- Top band on white: back link, status, title and a short description, then a white bar
	with the join controls that sits across the join with the full-width image below.
	Organiser, time needed and closing date belong in the bar too, once the API has them. -->
<div class="bg-background">
	<div class="mx-auto flex w-full max-w-5xl flex-col gap-5 px-4 pt-6 md:px-8 md:pt-10">
		<a
			href={resolve('/')}
			class="text-muted-foreground hover:text-foreground -ml-1 flex w-fit items-center gap-1 text-base font-medium"
		>
			<ChevronLeft class="size-5" />
			{m.conversations_all_heading()}
		</a>
		<div class="flex flex-col items-start gap-4 md:px-6">
			<ConversationStatusBadge
				status={Conversation.getStatus(conversation)}
				label={statusLabel}
			/>
			<h1
				class="text-3xl leading-tight font-bold tracking-tight text-balance sm:text-4xl lg:text-5xl"
			>
				{conversation.title}
			</h1>
			{#if conversation.shortDescription}
				<p class="max-w-3xl text-lg text-pretty whitespace-pre-line">
					{conversation.shortDescription}
				</p>
			{/if}
		</div>

		<!-- Pulled down over the image. -->
		<div
			bind:this={joinBar}
			id="join"
			class="bg-card ring-border/60 relative z-10 -mb-24 flex scroll-mt-28 flex-col gap-4 rounded-3xl p-5 shadow-lg ring-1 sm:-mb-20 md:flex-row md:items-center md:justify-between md:gap-6 md:p-6"
		>
			<div
				class="flex items-baseline justify-between gap-4 md:flex-col md:justify-start md:gap-1 md:pl-2"
			>
				<span class="text-muted-foreground text-base"
					>{m.conversation_status_heading()}</span
				>
				<span class="text-lg font-semibold">{statusLabel}</span>
			</div>
			<!-- The join controls bring their own top margin from the old layout; cancel it here. -->
			<div class="flex flex-col gap-2 *:mt-0 md:items-end">
				{@render children()}
			</div>
		</div>
	</div>
</div>

<div class="bg-primary/10 relative h-72 w-full overflow-hidden sm:h-96 lg:h-128">
	{#if conversation.imageUrl}
		<img src={conversation.imageUrl} alt="" class="absolute inset-0 size-full object-cover" />
	{:else}
		<MessageSquareText class="text-primary/30 absolute inset-0 m-auto size-20" />
	{/if}
</div>

{#if conversation.description}
	<div class="mx-auto w-full max-w-3xl px-4 pt-10 md:px-0 md:pt-14">
		<p class="text-foreground text-lg whitespace-pre-line">{conversation.description}</p>
	</div>
{/if}

{#if StickyAction && !joinBarVisible}
	<div
		class="bg-background/95 border-border fixed inset-x-0 bottom-0 z-40 flex border-t p-3 backdrop-blur sm:hidden"
	>
		{@render StickyAction()}
	</div>
{/if}
