<script lang="ts" module>
	export type ConversationCardLayout = 'card' | 'list';
</script>

<script lang="ts">
	import type { LocalizedConversationDto } from '@crownshy/api-client/api';
	import type { Snippet } from 'svelte';
	import ConversationStatusBadge from '$lib/components/ConversationStatusBadge.svelte';
	import { conversationStatus } from '$lib/utils/conversationStatus';
	import { MessageCircle, MessageSquareText } from '@lucide/svelte';
	import { cn } from '$lib/utils';
	import { conversationLandingUrl } from '$lib/utils/conversationLanding';
	import { resolve } from '$app/paths';

	type ActionsSnippetArgs = {
		href: string;
	};

	type Props = {
		conversation: LocalizedConversationDto;
		for: 'admin' | 'public';
		layout?: ConversationCardLayout;
		// Small facts under the description, e.g. "Shared", or later "Closes in 5 days".
		Meta?: Snippet;
		Actions?: Snippet<[ActionsSnippetArgs]>;
	};

	let { conversation, for: _for, layout = 'card', Meta, Actions }: Props = $props();

	let status = $derived(conversationStatus(conversation));
	let href = $derived(
		_for === 'admin'
			? conversationLandingUrl(conversation)
			: resolve('/(public)/conversations/[conversation_id]', {
					conversation_id: conversation.slug || conversation.id
				})
	);

	// The title's link stretches over the whole card, so the card opens the conversation.
	const CARD_CLASS =
		'bg-card border-border group hover:border-primary/40 has-[a:focus-visible]:ring-ring relative border transition duration-200 hover:-translate-y-0.5 hover:shadow-md has-[a:focus-visible]:ring-2';
	const TITLE_LINK_CLASS =
		'group-hover:text-primary transition-colors after:absolute after:inset-0 focus-visible:outline-none';

	function BANNER_STYLE(s: typeof status): string {
		switch (s) {
			case 'live':
				return 'bg-primary/15 text-primary';
			case 'draft':
				return 'bg-muted text-muted-foreground';
			case 'completed':
				return 'bg-foreground/10 text-foreground/60';
		}
	}
</script>

{#if conversation}
	<!-- Pages pass `href` already built with resolve(), which this rule can't see. -->
	<!-- eslint-disable svelte/no-navigation-without-resolve -->
	{#if layout === 'list'}
		<article
			class={cn(
				CARD_CLASS,
				'flex flex-col-reverse gap-6 rounded-2xl p-4 lg:flex-row lg:items-stretch lg:gap-12 lg:p-6'
			)}
		>
			<div class="flex flex-col gap-4">
				<ConversationStatusBadge {status} />
				{@render Meta?.()}
				<svelte:element
					this={_for === 'admin' ? 'h2' : 'h3'}
					class="text-foreground text-xl leading-7 font-semibold lg:text-2xl lg:leading-8"
				>
					<a {href} class={TITLE_LINK_CLASS}>{conversation.title}</a>
				</svelte:element>
				{#if conversation.shortDescription}
					<p class="text-muted-foreground text-base">
						{conversation.shortDescription}
					</p>
				{/if}
				<div class="mt-auto pt-2">{@render Actions?.({ href })}</div>
			</div>
			<div
				class={cn(
					'h-40 w-full shrink-0 rounded-xl sm:h-56 lg:h-72 lg:w-auto lg:flex-1',
					BANNER_STYLE(status)
				)}
			>
				{#if conversation.imageUrl}
					<img
						class="h-full w-full rounded-xl object-cover"
						src={conversation.imageUrl}
						alt="Image for {conversation.title}"
					/>
				{:else}
					<div class="flex items-center justify-center rounded-xl">
						<MessageCircle class="size-6" />
					</div>
				{/if}
			</div>
		</article>
	{:else}
		<article
			class={cn(
				CARD_CLASS,
				'grid grid-rows-[auto_auto_auto_auto_auto] gap-4 rounded-2xl p-3'
			)}
		>
			<div class="bg-primary/10 relative aspect-video w-full rounded-xl">
				{#if conversation.imageUrl}
					<img
						src={conversation.imageUrl}
						alt="Image for {conversation.title}"
						class="inset-0 h-full w-full rounded-xl object-cover transition-transform duration-500 ease-out group-hover:scale-102"
					/>
				{:else}
					<MessageSquareText
						class="text-primary/30 absolute inset-0 m-auto size-16 rounded-xl"
					/>
				{/if}
				<ConversationStatusBadge
					{status}
					class="absolute top-3 left-3 border-transparent shadow-md backdrop-blur"
				/>
			</div>

			<svelte:element
				this={_for === 'admin' ? 'h2' : 'h3'}
				class="text-foreground text-xl leading-7 font-semibold"
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
				<div class="z-2 mt-auto flex flex-wrap items-center gap-3 pt-2">
					{@render Actions({ href })}
				</div>
			{/if}
		</article>
	{/if}
{/if}
