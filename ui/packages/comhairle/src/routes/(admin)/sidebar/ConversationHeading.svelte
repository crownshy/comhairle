<script lang="ts">
	import { resolve } from '$app/paths';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { Check, ChevronDown, CircleX, ExternalLink, Rocket } from 'lucide-svelte';
	import type { ConversationWithTranslations } from '@crownshy/api-client/api';
	import { getTextInLocale } from '$lib/components/Translation/translationUtils';
	import { permissions } from '$lib/permissions.svelte';
	import {
		CONVERSATION_STATUS_LABELS,
		conversationStatus,
		type ConversationStatus
	} from '$lib/utils/conversationStatus';

	type Props = {
		conversation: ConversationWithTranslations;
		onLaunch: () => void;
		onToggleComplete: () => void;
	};

	let { conversation, onLaunch, onToggleComplete }: Props = $props();

	let canLaunch = $derived(
		permissions.can('conversation', 'conversation_launch', conversation.id)
	);
	let status = $derived(conversationStatus(conversation));
	let title = $derived(
		getTextInLocale(
			conversation.translations?.title,
			conversation.primaryLocale ?? 'en',
			conversation.title
		) || conversation.title
	);

	const statusDotClass: Record<ConversationStatus, string> = {
		draft: 'bg-sidebar-foreground/50',
		live: 'bg-sidebar-primary',
		completed: 'bg-sidebar-foreground/30'
	};
</script>

<div class="flex flex-col items-start gap-2 pr-4 group-data-[collapsible=icon]:hidden">
	<h1 class="text-sidebar-foreground line-clamp-2 text-lg leading-snug font-semibold" {title}>
		{title}
	</h1>

	<DropdownMenu.Root>
		<DropdownMenu.Trigger
			class="border-sidebar-foreground/20 text-sidebar-foreground hover:bg-sidebar-foreground/10 inline-flex h-8 items-center gap-2 rounded-full border px-3 text-sm"
			aria-label="Conversation status: {CONVERSATION_STATUS_LABELS[status]}"
		>
			<span class="size-2 rounded-full {statusDotClass[status]}" aria-hidden="true"></span>
			{CONVERSATION_STATUS_LABELS[status]}
			<ChevronDown class="size-3.5" />
		</DropdownMenu.Trigger>
		<DropdownMenu.Content align="start" class="w-56">
			{#if status === 'draft'}
				<DropdownMenu.Item disabled={!canLaunch} onclick={onLaunch}>
					<Rocket class="size-4" />
					Launch Conversation
				</DropdownMenu.Item>
			{:else}
				<DropdownMenu.Item>
					{#snippet child({ props })}
						<a
							{...props}
							href={resolve('/(public)/conversations/[conversation_id]/[[preview]]', {
								conversation_id: conversation.id
							})}
							target="_blank"
						>
							<ExternalLink class="size-4" />
							Live Conversation Link
						</a>
					{/snippet}
				</DropdownMenu.Item>
				<DropdownMenu.Separator />
				{#if status === 'live'}
					<DropdownMenu.Item
						class="text-destructive focus:text-destructive focus:bg-destructive/10"
						disabled={!canLaunch}
						onclick={onToggleComplete}
					>
						<CircleX class="text-destructive size-4" />
						End Conversation
					</DropdownMenu.Item>
				{:else}
					<DropdownMenu.Item disabled={!canLaunch} onclick={onToggleComplete}>
						<Check class="size-4" />
						Re-open Conversation
					</DropdownMenu.Item>
				{/if}
			{/if}
		</DropdownMenu.Content>
	</DropdownMenu.Root>
</div>
