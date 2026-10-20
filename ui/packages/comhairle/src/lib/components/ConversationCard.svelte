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

	type Props = {
		conversation: ConversationDto | LocalizedConversationDto;
		variant?: 'admin' | 'public';
		organizationName?: string;
		isShared?: boolean;
		layout?: 'card' | 'row';
	};

	let {
		conversation,
		variant = 'admin',
		organizationName,
		isShared = false,
		layout = 'card'
	}: Props = $props();

	let status = $derived(conversationStatus(conversation));
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
			class="bg-primary/10 relative {heightClass} w-full overflow-hidden rounded-xl transition-all duration-300 ease-out group-hover:rounded-2xl group-hover:shadow-lg lg:rounded-3xl lg:group-hover:rounded-[2rem]"
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
			<div class="relative shrink-0 {sizeClass} {BANNER_TONE[status]}">
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
			<Button
				variant="primaryDark"
				class="rounded-full"
				href={`/admin/conversations/${conversation.id}/configure`}
			>
				Edit conversation
				<ArrowRight class="size-4" />
			</Button>
		{/snippet}

		{#if layout === 'row'}
			<article
				class="bg-card border-border flex flex-col-reverse gap-6 overflow-hidden rounded-2xl border p-4 transition-shadow duration-200 hover:shadow-md lg:flex-row lg:items-stretch lg:gap-12 lg:p-6"
			>
				<div class="flex flex-1 flex-col gap-4">
					{@render badges()}
					<h2
						class="text-foreground text-xl leading-7 font-semibold lg:text-2xl lg:leading-8"
					>
						{conversation.title}
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
			<article
				class="bg-card border-border flex h-full flex-col overflow-hidden rounded-xl border transition-shadow duration-200 hover:shadow-md"
			>
				{@render banner('h-22')}
				<div class="flex grow flex-col gap-3 p-5">
					{@render badges()}
					<h2 class="text-foreground text-lg leading-snug font-semibold">
						{conversation.title}
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
	{/if}
{/if}
