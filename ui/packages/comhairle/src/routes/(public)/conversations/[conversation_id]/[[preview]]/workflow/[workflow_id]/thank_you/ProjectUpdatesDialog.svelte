<script lang="ts">
	import type { UserDto } from '@crownshy/api-client/api';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import EmailRegistrationForm from '$lib/components/EmailRegistrationForm/EmailRegistrationForm.svelte';
	import UserConversationPreferencesForm from '$lib/components/UserConversationPreferencesForm/UserConversationPreferencesForm.svelte';
	import { m } from '$lib/paraglide/messages';

	type Props = {
		user: UserDto;
		conversationId: string;
		class?: string;
	};
	let { user, conversationId, class: className }: Props = $props();

	let open = $state(false);
	let isGuest = $derived(user.authType === 'guest');
</script>

<Dialog.Root bind:open>
	<Dialog.Trigger>
		{#snippet child({ props })}
			<Button {...props} size="xl" class={className}>
				{m.thank_you_get_updates()}
			</Button>
		{/snippet}
	</Dialog.Trigger>
	<Dialog.Content class="max-h-[calc(100dvh-2rem)] gap-6 overflow-y-auto p-8 sm:max-w-2xl">
		<Dialog.Header class="gap-3">
			<Dialog.Title class="text-2xl">{m.thank_you_get_updates()}</Dialog.Title>
			<Dialog.Description class="text-lg">
				{isGuest
					? m.thank_you_get_updates_guest_body()
					: m.thank_you_get_updates_account_body()}
			</Dialog.Description>
		</Dialog.Header>
		{#if isGuest}
			<EmailRegistrationForm conversation_id={conversationId} onDone={() => (open = false)} />
		{:else}
			<UserConversationPreferencesForm {conversationId} isGuest={false} />
		{/if}
	</Dialog.Content>
</Dialog.Root>
