<script lang="ts" module>
	export type ConversationCardLayout = 'card' | 'list';
</script>

<script lang="ts">
	import type { ConversationDto, LocalizedConversationDto } from '@crownshy/api-client/api';
	import type { Snippet } from 'svelte';
	import ConversationStatusBadge from '$lib/components/ConversationStatusBadge.svelte';
	import { conversationStatus } from '$lib/utils/conversationStatus';
	import { ChevronRight, MessageSquareText } from 'lucide-svelte';
	import { cn } from '$lib/utils';

	type Props = {
		conversation: ConversationDto | LocalizedConversationDto;
		/** Where the card goes. Each page decides: admin URL or public URL. */
		href: string;
		layout?: ConversationCardLayout;
		/** 2 under a page title, 3 under a section heading (like "Conversations" on home). */
		headingLevel?: 2 | 3;
		/** A translated status label for public pages; admin uses the built-in English. */
		statusLabel?: string;
		/** Small facts under the description, e.g. "Shared", or later "Closes in 5 days". */
		meta?: Snippet;
		/** Buttons at the bottom, e.g. "Open conversation" or "Take part". */
		actions?: Snippet;
	};

	let {
		conversation,
		href,
		layout = 'card',
		headingLevel = 2,
		statusLabel,
		meta,
		actions
	}: Props = $props();

	let status = $derived(conversationStatus(conversation));
	// The title's link stretches over the whole card, so the card opens the conversation.
	const CARD_CLASS =
		'bg-card border-border group hover:border-primary/40 has-[a:focus-visible]:ring-ring relative overflow-hidden border transition duration-200 hover:-translate-y-0.5 hover:shadow-md has-[a:focus-visible]:ring-2';
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
						alt=""
						class="absolute inset-0 size-full object-cover"
					/>
				{:else}
					<MessageSquareText class="text-primary/40 absolute inset-0 m-auto size-8" />
				{/if}
			</div>

			<div class="flex min-w-0 grow flex-col items-start gap-1.5">
				<ConversationStatusBadge {status} label={statusLabel} />
				<svelte:element
					this={`h${headingLevel}`}
					class="text-foreground line-clamp-2 text-base leading-snug font-semibold sm:text-lg"
				>
					<a {href} class={TITLE_LINK_CLASS}>{conversation.title}</a>
				</svelte:element>
				{#if conversation.shortDescription}
					<p class="text-muted-foreground line-clamp-1 text-sm sm:text-base">
						{conversation.shortDescription}
					</p>
				{/if}
				{#if meta}
					<div class="text-muted-foreground flex flex-wrap gap-x-4 gap-y-1 text-sm">
						{@render meta()}
					</div>
				{/if}
			</div>

			{#if actions}
				<div class="hidden shrink-0 sm:flex">{@render actions()}</div>
			{/if}
			<ChevronRight
				class={cn('text-muted-foreground size-5 shrink-0', actions && 'sm:hidden')}
				aria-hidden="true"
			/>
		</article>
	{:else}
		<article class={cn(CARD_CLASS, 'flex h-full flex-col gap-4 rounded-2xl p-3')}>
			<div class="bg-primary/10 relative aspect-video w-full overflow-hidden rounded-xl">
				{#if conversation.imageUrl}
					<img
						src={conversation.imageUrl}
						alt=""
						class="absolute inset-0 h-full w-full object-cover transition-transform duration-500 ease-out group-hover:scale-105"
					/>
				{:else}
					<MessageSquareText class="text-primary/30 absolute inset-0 m-auto size-16" />
				{/if}
				<ConversationStatusBadge
					{status}
					label={statusLabel}
					class="absolute top-3 left-3 border-transparent shadow-md backdrop-blur"
				/>
			</div>

			<div class="flex grow flex-col items-start gap-3 px-2 pb-2">
				<svelte:element
					this={`h${headingLevel}`}
					class="text-foreground text-xl leading-7 font-semibold"
				>
					<a {href} class={TITLE_LINK_CLASS}>{conversation.title}</a>
				</svelte:element>
				{#if conversation.shortDescription}
					<p class="text-muted-foreground line-clamp-2 text-base">
						{conversation.shortDescription}
					</p>
				{/if}
				{#if meta}
					<div class="text-muted-foreground flex flex-wrap gap-x-5 gap-y-2 text-base">
						{@render meta()}
					</div>
				{/if}
				{#if actions}
					<div class="mt-auto flex flex-wrap items-center gap-3 pt-2">
						{@render actions()}
					</div>
				{/if}
			</div>
		</article>
	{/if}
	<!-- eslint-enable svelte/no-navigation-without-resolve -->
{/if}
