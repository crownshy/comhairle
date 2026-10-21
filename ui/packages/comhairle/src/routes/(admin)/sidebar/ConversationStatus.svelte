<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import { shareable_conversation_url } from '$lib/urls';
	import { copyToClipboard } from '$lib/utils/copyToClipboard.svelte';
	import { ArrowUpRight, Check, Copy } from 'lucide-svelte';
	import type { ConversationWithTranslations } from '@crownshy/api-client/api';
	import {
		CONVERSATION_STATUS_LABELS,
		conversationStatus,
		type ConversationStatus
	} from '$lib/utils/conversationStatus';

	interface Props {
		conversation: ConversationWithTranslations;
	}

	let { conversation }: Props = $props();

	let status = $derived(conversationStatus(conversation));

	const STATUS_DOT_CLASS: Record<ConversationStatus, string> = {
		draft: 'bg-orange-400',
		live: 'bg-emerald-400',
		completed: 'bg-sidebar-foreground/40'
	};

	const PILL_CLASS = 'bg-primary/20 text-sidebar-foreground hover:bg-primary/30 rounded-full';

	const clipboard = copyToClipboard();
	let copyLabel = $derived(clipboard.copied ? 'Copied' : 'Copy conversation link');

	function copyLink() {
		clipboard.copy(shareable_conversation_url(page.url.origin, conversation.id));
	}
</script>

<div class="flex h-9 items-center justify-between gap-3 pr-4 group-data-[collapsible=icon]:hidden">
	<span class="text-sidebar-foreground/80 inline-flex items-center gap-2 text-base">
		<span class="size-2 shrink-0 rounded-full {STATUS_DOT_CLASS[status]}" aria-hidden="true"
		></span>
		{CONVERSATION_STATUS_LABELS[status]}
	</span>
	<div class="flex items-center gap-2">
		<Button
			href={resolve('/(public)/conversations/[conversation_id]/[[preview]]', {
				conversation_id: conversation.id,
				preview: 'preview'
			})}
			target="_blank"
			variant="secondary"
			class={cn(PILL_CLASS, 'h-9 px-4 text-base')}
		>
			Preview
			<ArrowUpRight class="size-4" />
		</Button>
		{#if status === 'live'}
			<Button
				variant="secondary"
				size="icon"
				class={cn(PILL_CLASS, 'size-9')}
				title={copyLabel}
				onclick={copyLink}
			>
				{#if clipboard.copied}
					<Check class="size-4" />
				{:else}
					<Copy class="size-4" />
				{/if}
				<span class="sr-only">{copyLabel}</span>
			</Button>
		{/if}
	</div>
</div>
