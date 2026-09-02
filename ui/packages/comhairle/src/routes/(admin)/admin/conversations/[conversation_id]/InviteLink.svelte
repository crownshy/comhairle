<script lang="ts">
	import { page } from '$app/state';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import { inviteUrl } from '$lib/utils/invites';

	interface Props {
		label: string;
		inviteId: string;
		conversationId: string;
		eventId?: string;
	}

	let { label, inviteId, conversationId, eventId }: Props = $props();
</script>

<div class="flex flex-row gap-x-2">
	<CopyButton copyText={inviteUrl(page.url, inviteId, conversationId, eventId)}
		>{label}</CopyButton
	>
</div>
