<script lang="ts">
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
	import { shareable_conversation_url } from '$lib/urls';
	import { copyToClipboard } from '$lib/utils/copyToClipboard.svelte';
	import type { ConversationDto } from '@crownshy/api-client/api';
	import { ArrowUpRight, Check, Copy } from 'lucide-svelte';

	let { conversation }: { conversation: ConversationDto } = $props();

	let link = $derived(shareable_conversation_url(page.url.origin, conversation.id));
	const clipboard = copyToClipboard();
</script>

<section class="bg-background border-border flex flex-col gap-3 rounded-xl border px-5 py-5">
	<div class="flex flex-col gap-1">
		<h2 class="text-lg font-semibold">Conversation link</h2>
		<p class="text-muted-foreground">
			{conversation.isInviteOnly
				? 'This conversation is invite only, so people need an email invite or an invite link to take part. Share this link with people you have already invited.'
				: 'Share this link so people can find the conversation and take part.'}
		</p>
	</div>
	<p class="bg-muted rounded-md px-3 py-2 font-mono text-base break-all">{link}</p>
	<div class="flex flex-wrap gap-2">
		<Button class="rounded-full" onclick={() => clipboard.copy(link)}>
			{#if clipboard.copied}
				<Check class="size-4" />
				Copied
			{:else}
				<Copy class="size-4" />
				Copy link
			{/if}
		</Button>
		<Button class="rounded-full" variant="outline" href={link} target="_blank">
			<ArrowUpRight class="size-4" />
			Open
		</Button>
	</div>
</section>
