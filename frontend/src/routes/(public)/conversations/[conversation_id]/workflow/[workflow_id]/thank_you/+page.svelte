<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import { workflow_step_url } from '$lib/urls';
	import type { PageProps } from './$types';
	import FeedbackModal from '$lib/components/FeedbackModal.svelte';
	import UserConversationPreferencesForm from '$lib/components/UserConversationPreferencesForm/UserConversationPreferencesForm.svelte';
	import UpgradeAccountModal from '$lib/components/UpgradeAccountModal/UpgradeAccountModal.svelte';
	import EmailRegistrationForm from '$lib/components/EmailRegistrationForm/EmailRegistrationForm.svelte';

	let { data }: PageProps = $props();
	let user = $derived(data.user);
	let conversation = $derived(data.conversation);
	let workflow = $derived(data.workflow);
</script>

<div class="prose mx-auto mt-20">
	<h1>Thank You For Participating</h1>

	Thank you for taking part in the conversation:<strong>{conversation.title}</strong> <br />
	We value your time and effort.

	{#if user.auth_type === 'annon'}
		<p>You are currently signed in as an anonymous user. Use your anonymous id</p>
		<h2 class="text-center">{user.username}</h2>
		<p>to log back in at a later time to see the results of the conversations.</p>
	{:else}
		<p>
			The conversation is ongoing. When it's done we will email you with a link to the results and
			will follow up with notifications on how those results are used.
		</p>
	{/if}
	<h2>Next steps</h2>
	You can continue to contribute, let us know what you thought of the process or sign up for updates
	on this project and others which you might be interested in.
	<div class="mx-auto mt-10 flex flex-col justify-center gap-2 text-center md:flex-row">
		<Button
			class="no-underline"
			variant="secondary"
			href={workflow_step_url(conversation.id, workflow.id, 1)}>Contribute some more</Button
		>
		<FeedbackModal conversationId={conversation.id} />
	</div>

	<h2>Keep informed</h2>

	<p>
		Sometimes we invite people who’ve taken part in Scottish Government research to take part in
		future research on a related topic. This future research may be carried out by the Scottish
		Government, other government bodies, or a reputable research organisation working with them. You
		may never be contacted again, but even if you are, you’ll still be free to decide whether you
		want to take part in the research or not. Are you willing to be contacted again for future
		research purposes relating to this topic?
	</p>

	{#if user.auth_type === 'annon'}
		<p>
			To receive updates on this conversation and future conversations, either update to a full
			account
		</p>

		<div class="mt-5 mb-10 w-full">
			<UpgradeAccountModal currentUser={user} />
		</div>
		<p>or submit your email</p>
		<div class="mb-10">
			<EmailRegistrationForm conversation_id={conversation.id} />
		</div>
	{:else}
		<p>
			Use the following switches to opt into future communications. You can also update this on your
			settings page.
		</p>
		<UserConversationPreferencesForm
			conversationId={conversation.id}
			isAnnon={user.auth_type === 'annon'}
		/>
	{/if}
</div>
