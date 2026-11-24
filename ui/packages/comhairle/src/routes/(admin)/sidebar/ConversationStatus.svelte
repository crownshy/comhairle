<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
	import { cn } from '$lib/utils';
	import { shareable_conversation_url } from '$lib/urls';
	import { copyToClipboard } from '$lib/utils/copyToClipboard.svelte';
	import ArrowUpRight from '@lucide/svelte/icons/arrow-up-right';
	import Check from '@lucide/svelte/icons/check';
	import Copy from '@lucide/svelte/icons/copy';
	import type { ConversationWithTranslations } from '@crownshy/api-client/api';
	import type { ConversationStatus } from '$lib/interfaces/Conversation';
	import { capitalise } from '$lib/utils/casingUtils';
	import Conversation from '$lib/interfaces/Conversation';

	interface Props {
		conversation: ConversationWithTranslations;
	}

	let { conversation }: Props = $props();

	let status = $derived(Conversation.getStatus(conversation));

	const STATUS_DOT_CLASS: Record<ConversationStatus, string> = {
		draft: 'bg-orange-400',
		live: 'bg-emerald-400',
		completed: 'bg-sidebar-foreground/40'
	};

	const PILL_CLASS = 'bg-primary/20 text-sidebar-foreground hover:bg-primary/30 rounded-full';

	const OPEN_LINK_LABELS: Record<ConversationStatus, string> = {
		draft: 'Preview',
		live: 'View live',
		completed: 'View'
	};

	let openLinkHref = $derived(
		resolve('/(public)/conversations/[conversation_id]/[[preview]]', {
			conversation_id: conversation.id,
			preview: status === 'draft' ? 'preview' : undefined
		})
	);

	const clipboard = copyToClipboard();
	let copyLabel = $derived(clipboard.copied ? 'Copied' : 'Copy conversation link');

	function copyLink() {
		clipboard.copy(shareable_conversation_url(page.url.origin, conversation.id));
	}
</script>

<div class="flex h-8 items-center justify-between gap-3 pr-4 group-data-[collapsible=icon]:hidden">
	<span class="text-sidebar-foreground/80 inline-flex items-center gap-2 text-sm">
		<span class="size-2 shrink-0 rounded-full {STATUS_DOT_CLASS[status]}" aria-hidden="true"
		></span>
		{capitalise(status)}
	</span>
	<div class="flex items-center gap-2">
		<Button
			href={openLinkHref}
			target="_blank"
			variant="secondary"
			class={cn(PILL_CLASS, 'h-8 px-3 text-sm')}
		>
			{OPEN_LINK_LABELS[status]}
			<ArrowUpRight class="size-4" />
		</Button>
		{#if status === 'live'}
			<Button
				variant="secondary"
				size="icon"
				class={cn(PILL_CLASS, 'size-8')}
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
