<script lang="ts">
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import Combobox from '$lib/components/ui/combobox/combobox.svelte';
	import type { EmailType } from '@crownshy/api-client/api';

	type Props = {
		open: boolean;
		emailType: EmailType;
		/** Conversations that still send the default, so can get their own version. */
		conversations: { id: string; title: string }[];
	};

	let { open = $bindable(), emailType, conversations }: Props = $props();

	let chosenId = $state<string | null>(null);
	let items = $derived(
		conversations.map((conversation) => ({ value: conversation.id, label: conversation.title }))
	);

	function startEditing() {
		if (!chosenId) return;
		const newPage = resolve('/(admin)/admin/email-template-configs/new');
		// eslint-disable-next-line svelte/no-navigation-without-resolve -- newPage is resolved above
		goto(`${newPage}?type=${emailType}&conversation=${chosenId}`);
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="sm:max-w-lg">
		<Dialog.Header>
			<Dialog.Title>Which conversation is this for?</Dialog.Title>
			<Dialog.Description class="text-base">
				Only this conversation will send the version you write. The others keep the default.
			</Dialog.Description>
		</Dialog.Header>
		{#if items.length > 0}
			<Combobox
				{items}
				placeholder="Choose a conversation"
				emptyMessage="No conversation matches."
				onSelect={(item) => (chosenId = item.value)}
			/>
		{:else}
			<p class="text-muted-foreground text-base">
				Every conversation already has its own version. Edit one from the list instead.
			</p>
		{/if}
		<Dialog.Footer>
			<Button variant="outline" onclick={() => (open = false)}>Cancel</Button>
			<Button disabled={!chosenId} onclick={startEditing}>Continue</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
