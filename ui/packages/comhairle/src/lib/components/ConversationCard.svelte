<script lang="ts" module>
	export type ConversationCardLayout = 'card' | 'list';
</script>

<script lang="ts">
	import type { ConversationDto, LocalizedConversationDto } from '@crownshy/api-client/api';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import {
		conversationStatus,
		CONVERSATION_STATUS_BADGE_VARIANTS,
		CONVERSATION_STATUS_LABELS
	} from '$lib/utils/conversationStatus';
	import { ArrowRight, MessageCircle, MessageSquareText } from 'lucide-svelte';
	import { cn } from '$lib/utils';
	import { resolve } from '$app/paths';

	type Props = {
		conversation: ConversationDto | LocalizedConversationDto;
		variant?: 'admin' | 'public';
		organizationName?: string;
		isShared?: boolean;
		layout?: ConversationCardLayout;
	};

	let {
		conversation,
		variant = 'admin',
		organizationName,
		isShared = false,
		layout = 'card'
	}: Props = $props();

	let status = $derived(conversationStatus(conversation));
	let editHref = $derived(
		resolve('/(admin)/admin/conversations/[conversation_id]/configure', {
			conversation_id: conversation.id
		})
	);
	// The title's link stretches over the whole card, so the card opens the conversation.
	const CARD_CLASS =
		'bg-card border-border group hover:border-primary/40 has-[a:focus-visible]:ring-ring relative overflow-hidden border transition duration-200 hover:-translate-y-0.5 hover:shadow-md has-[a:focus-visible]:ring-2';
	const TITLE_LINK_CLASS =
		'group-hover:text-primary transition-colors after:absolute after:inset-0 focus-visible:outline-none';
	const BANNER_TONE: Record<typeof status, string> = {
		live: 'bg-primary/15 text-primary',
		draft: 'bg-muted text-muted-foreground',
		completed: 'bg-foreground/10 text-foreground/60'
	};
</script>

<!-- Image snippet -->
{#snippet image(heightClass: string)}
	<div class="w-full min-w-0 lg:flex-1">
		<div
			class={cn(
				'bg-primary/10 relative w-full overflow-hidden rounded-xl transition-all duration-300 ease-out group-hover:rounded-2xl group-hover:shadow-lg lg:rounded-3xl lg:group-hover:rounded-[2rem]',
				heightClass
			)}
		>
			{#if conversation.imageUrl}
				<img
					class="absolute inset-0 h-full w-full object-cover transition-transform duration-500 ease-out group-hover:scale-105"
					src={conversation.imageUrl}
					alt={conversation.title}
				/>
			{:else}
				<div class="bg-primary/10 absolute inset-0 flex items-center justify-center">
					<MessageSquareText
						class="text-primary/30 group-hover:text-primary/50 h-32 w-32 transition-all duration-300 group-hover:scale-110"
					/>
				</div>
			{/if}
		</div>
	</div>
{/snippet}

{#if conversation}
	<!-- Public variant -->
	{#if variant === 'public'}
		<div
			class="group hover:bg-accent/50 flex w-full flex-col items-stretch gap-6 rounded-2xl p-4 transition-all duration-300 ease-out hover:shadow-md lg:flex-row lg:items-start lg:gap-16 lg:p-6"
		>
			{@render image('h-48 sm:h-64 lg:h-80')}

			<!-- Text content -->
			<div class="flex w-full flex-1 flex-col items-start justify-start gap-6 lg:gap-8">
				<div class="flex flex-col items-start justify-start gap-3 self-stretch lg:gap-4">
					{#if conversation.isLive}
						<Badge variant="default" class="h-7 text-sm">Live</Badge>
					{/if}

					{#if organizationName}
						<p
							class="text-primary self-stretch text-sm leading-5 font-medium transition-colors duration-300"
						>
							{organizationName}
						</p>
					{/if}

					<h2
						class="text-foreground self-stretch text-2xl leading-8 font-semibold transition-colors duration-300 group-hover:text-[var(--color-brand)] lg:text-3xl lg:leading-9"
					>
						{conversation.title}
					</h2>

					<p
						class="text-muted-foreground self-stretch text-base leading-7 font-medium lg:text-lg"
					>
						{conversation.shortDescription}
					</p>
				</div>
			</div>
		</div>
	{:else}
		<!-- Admin variant -->
		{#snippet banner(sizeClass: string)}
			<div class={cn('relative shrink-0', sizeClass, BANNER_TONE[status])}>
				{#if conversation.imageUrl}
					<img
						class="absolute inset-0 h-full w-full object-cover"
						src={conversation.imageUrl}
						alt=""
					/>
				{:else}
					<div class="flex h-full items-center justify-center">
						<MessageCircle class="size-6" />
					</div>
				{/if}
			</div>
		{/snippet}

		{#snippet badges()}
			<div class="flex flex-wrap gap-2">
				<Badge variant={CONVERSATION_STATUS_BADGE_VARIANTS[status]}>
					{CONVERSATION_STATUS_LABELS[status]}
				</Badge>
				{#if isShared}
					<Badge variant="outline">Shared</Badge>
				{/if}
			</div>
		{/snippet}

		{#snippet editButton()}
			<Button variant="primaryDark" class="relative z-10 rounded-full" href={editHref}>
				Edit conversation
				<ArrowRight class="size-4" />
			</Button>
		{/snippet}

		<!-- editHref is built with resolve() above. -->
		<!-- eslint-disable svelte/no-navigation-without-resolve -->
		{#if layout === 'list'}
			<article
				class={cn(
					CARD_CLASS,
					'flex flex-col-reverse gap-6 rounded-2xl p-4 lg:flex-row lg:items-stretch lg:gap-12 lg:p-6'
				)}
			>
				<div class="flex flex-1 flex-col gap-4">
					{@render badges()}
					<h2
						class="text-foreground text-xl leading-7 font-semibold lg:text-2xl lg:leading-8"
					>
						<a href={editHref} class={TITLE_LINK_CLASS}>{conversation.title}</a>
					</h2>
					{#if conversation.shortDescription}
						<p class="text-muted-foreground text-base">
							{conversation.shortDescription}
						</p>
					{/if}
					<div class="mt-auto pt-2">{@render editButton()}</div>
				</div>
				{@render banner(
					'h-40 w-full overflow-hidden rounded-xl sm:h-56 lg:h-72 lg:w-auto lg:flex-1'
				)}
			</article>
		{:else}
			<article class={cn(CARD_CLASS, 'flex h-full flex-col rounded-xl')}>
				{@render banner('h-22')}
				<div class="flex grow flex-col gap-3 p-5">
					{@render badges()}
					<h2 class="text-foreground text-lg leading-snug font-semibold">
						<a href={editHref} class={TITLE_LINK_CLASS}>{conversation.title}</a>
					</h2>
					{#if conversation.shortDescription}
						<p class="text-muted-foreground line-clamp-2 text-base">
							{conversation.shortDescription}
						</p>
					{/if}
					<div class="mt-auto pt-2">{@render editButton()}</div>
				</div>
			</article>
		{/if}
		<!-- eslint-enable svelte/no-navigation-without-resolve -->
	{/if}
{/if}
