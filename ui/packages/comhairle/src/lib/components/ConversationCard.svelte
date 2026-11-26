<script lang="ts" module>
	export type ConversationCardLayout = 'card' | 'list';
</script>

<script lang="ts">
	import type { LocalizedConversationDto } from '@crownshy/api-client/api';
	import type { Snippet } from 'svelte';
	import ConversationStatusBadge from '$lib/components/ConversationStatusBadge.svelte';
	import ChevronRight from '@lucide/svelte/icons/chevron-right';
	import MessageSquareText from '@lucide/svelte/icons/message-square-text';
	import { cn } from '$lib/utils';
	import { conversationLandingUrl } from '$lib/utils/conversationLanding';
	import { resolve } from '$app/paths';
	import Conversation from '$lib/interfaces/Conversation';

	type Props = {
		conversation: LocalizedConversationDto;
		for: 'admin' | 'public';
		layout?: ConversationCardLayout;
		/** Overrides the status badge text, e.g. "Live now" on the public home page. */
		statusLabel?: string;
		// Small facts under the description, e.g. "Shared", or later "Closes in 5 days".
		Meta?: Snippet;
		Actions?: Snippet;
	};

	let { conversation, for: _for, layout = 'card', statusLabel, Meta, Actions }: Props = $props();

	let status = $derived(Conversation.getStatus(conversation));
	let href = $derived.by(() => {
		if (_for === 'admin') {
			return conversationLandingUrl(conversation);
		}
		if (conversation.isComplete) {
			return resolve('/(public)/conversations/[conversation_id]/[[preview]]/report', {
				conversation_id: conversation.slug || conversation.id
			});
		}
		return resolve('/(public)/conversations/[conversation_id]', {
			conversation_id: conversation.slug || conversation.id
		});
	});

	// The title's link stretches over the whole card, so the card opens the conversation.
	const CARD_CLASS =
		'bg-card border-border group hover:border-primary/40 has-[a:focus-visible]:ring-ring relative border transition duration-200 hover:-translate-y-0.5 hover:shadow-md has-[a:focus-visible]:ring-2';
	const TITLE_LINK_CLASS =
		'group-hover:text-primary transition-colors after:absolute after:inset-0 focus-visible:outline-none';
</script>

{#if conversation}
	<!-- Pages pass `href` already built with resolve(), which this rule can't see. -->
	<!-- eslint-disable svelte/no-navigation-without-resolve -->
	{#if layout === 'list'}
		<article class={cn(CARD_CLASS, 'flex items-center gap-4 rounded-2xl p-3')}>
			<div
				class="bg-primary/10 relative size-20 shrink-0 overflow-hidden rounded-xl sm:size-36"
			>
				{#if conversation.imageUrl}
					<img
						src={conversation.imageUrl}
						alt="Image for {conversation.title}"
						class="absolute inset-0 size-full object-cover"
					/>
				{:else}
					<MessageSquareText class="text-primary/40 absolute inset-0 m-auto size-8" />
				{/if}
			</div>

			<div class="flex min-w-0 grow flex-col items-start gap-1.5">
				<ConversationStatusBadge {status} label={statusLabel} />
				<svelte:element
					this={_for === 'admin' ? 'h2' : 'h3'}
					class="text-foreground line-clamp-2 text-base leading-snug font-semibold sm:text-lg"
				>
					<a {href} class={TITLE_LINK_CLASS}>{conversation.title}</a>
				</svelte:element>
				{#if conversation.shortDescription}
					<p class="text-muted-foreground line-clamp-1 text-sm sm:text-base">
						{conversation.shortDescription}
					</p>
				{/if}
				{#if Meta}
					<div class="text-muted-foreground flex flex-wrap gap-x-4 gap-y-1 text-sm">
						{@render Meta()}
					</div>
				{/if}
			</div>

			{#if Actions}
				<div class="hidden shrink-0 sm:flex">{@render Actions()}</div>
			{/if}
			<ChevronRight
				class={cn('text-muted-foreground size-5 shrink-0', Actions && 'sm:hidden')}
				aria-hidden="true"
			/>
		</article>
	{:else}
		<article
			class={cn(
				CARD_CLASS,
				'relative grid grid-rows-[1fr_auto_auto_auto_auto] gap-4 rounded-2xl p-3'
			)}
		>
			<div class="bg-primary/10 w-full rounded-xl">
				{#if conversation.imageUrl}
					<img
						src={conversation.imageUrl}
						alt="Image for {conversation.title}"
						class="aspect-video h-full w-full rounded-xl object-cover transition-transform duration-500 ease-out group-hover:scale-102"
					/>
				{:else}
					<MessageSquareText class="text-primary/30 inset-0 m-auto size-16 rounded-xl" />
				{/if}
				<ConversationStatusBadge
					{status}
					label={statusLabel}
					class="absolute top-4 left-4 border-transparent px-3.5 py-1.5 text-base shadow-md backdrop-blur"
				/>
			</div>

			<svelte:element
				this={_for === 'admin' ? 'h2' : 'h3'}
				class="text-foreground line-clamp-3 text-xl leading-7 font-semibold"
			>
				<a {href} class={TITLE_LINK_CLASS}>{conversation.title}</a>
			</svelte:element>
			{#if conversation.shortDescription}
				<p class="text-muted-foreground line-clamp-2 text-base">
					{conversation.shortDescription}
				</p>
			{/if}
			{#if Meta}
				<div class="text-muted-foreground flex flex-wrap gap-x-5 gap-y-2 text-base">
					{@render Meta()}
				</div>
			{/if}
			{#if Actions}
				<div class="flex flex-wrap items-center gap-3 pt-2">
					{@render Actions()}
				</div>
			{/if}
		</article>
	{/if}
{/if}
