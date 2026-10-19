<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
	import { shareable_conversation_url } from '$lib/urls';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { ArrowUpRight, Check, Copy } from 'lucide-svelte';
	import type { ConversationWithTranslations } from '@crownshy/api-client/api';
	import {
		CONVERSATION_STATUS_LABELS,
		conversationStatus,
		type ConversationStatus
	} from '$lib/utils/conversationStatus';

	let { conversation }: { conversation: ConversationWithTranslations } = $props();

	let status = $derived(conversationStatus(conversation));

	const statusDotClass: Record<ConversationStatus, string> = {
		draft: 'bg-orange-400',
		live: 'bg-emerald-400',
		completed: 'bg-sidebar-foreground/40'
	};

	const pillClass =
		'bg-primary/20 text-sidebar-foreground hover:bg-primary/30 h-9 rounded-full px-4 text-base';

	let copied = $state(false);

	async function copyLink() {
		const link = shareable_conversation_url(page.url.origin, conversation.id);
		const result = await tryCatchAsync(() => navigator.clipboard.writeText(link));
		if (result.err !== null) return;
		copied = true;
		setTimeout(() => (copied = false), 2000);
	}
</script>

<div class="flex h-9 items-center justify-between gap-3 pr-4 group-data-[collapsible=icon]:hidden">
	<span class="text-sidebar-foreground/80 inline-flex items-center gap-2 text-base">
		<span class="size-2 shrink-0 rounded-full {statusDotClass[status]}" aria-hidden="true"
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
			class={pillClass}
		>
			Preview
			<ArrowUpRight class="size-4" />
		</Button>
		{#if status === 'live'}
			<Button
				variant="secondary"
				size="icon"
				class="bg-primary/20 text-sidebar-foreground hover:bg-primary/30 size-9 rounded-full"
				title={copied ? 'Copied' : 'Copy conversation link'}
				onclick={copyLink}
			>
				{#if copied}
					<Check class="size-4" />
				{:else}
					<Copy class="size-4" />
				{/if}
				<span class="sr-only">{copied ? 'Copied' : 'Copy conversation link'}</span>
			</Button>
		{/if}
	</div>
</div>
