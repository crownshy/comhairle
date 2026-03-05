<script lang="ts">
	import { Settings } from 'lucide-svelte';
	import type { PageProps } from './$types';
	import UserConversationPreferencesForm from '$lib/components/UserConversationPreferencesForm/UserConversationPreferencesForm.svelte';
	import UserDetailsForm from '$lib/components/UserDetailsForm/UserDetailsForm.svelte';
	import UpgradeAccountModal from '$lib/components/UpgradeAccountModal/UpgradeAccountModal.svelte';
	import type { UserDto } from '@crownshy/api-client/api';

	let { data }: PageProps = $props();
	let participation = $derived(data.participation);
	let user = $state(data.user) as UserDto;

	function handleUpgradeSuccess(upgradedUser: UserDto) {
		user = upgradedUser;
	}
</script>

<svelte:head>
	<title>Settings - Comhairle</title>
</svelte:head>

<div class="my-10 flex flex-col items-start gap-15 md:flex-row">
	<div>
		<div class=" mb-10 flex flex-row items-center gap-4">
			<Settings size={42} />
			<h1 class="text-4xl">Settings</h1>
		</div>
	</div>
	<div class="mt-1 flex flex-col gap-y-10">
		<section id="your_details" class="border-border border-b py-5">
			<h2 class="mb-6 text-3xl">Your Details</h2>
			{#if user.authType === 'annon'}
				<div class="space-y-6">
					<div class="text-center">
						<div class="text-muted-foreground mb-4">
							You are currently signed in as an anonymous account with ID:
						</div>
						<h3 class="my-4 text-center text-2xl font-bold">{user.username}</h3>
					</div>

					<div class=" bg-card p-6">
						<h4 class="mb-2 font-semibold">Upgrade to a Full Account</h4>
						<p class="text-muted-foreground mb-4 text-sm">
							Transform your anonymous account to receive email updates and be
							informed about the results of the conversations you have taken part in.
							Your current participation will be preserved.
						</p>
						<div class="flex w-full flex-row items-end justify-center md:justify-end">
							<UpgradeAccountModal
								onSuccess={handleUpgradeSuccess}
								currentUser={user}
							/>
						</div>
					</div>
				</div>
			{:else}
				<UserDetailsForm {user} />
			{/if}
		</section>

		<section id="notifications" class=" flex flex-col">
			<h2 class="text-3xl">Notifications</h2>
			<p class="my-10">
				Manage how you would like to be contacted about updates on conversations
			</p>
			{#each participation as conversation (conversation.id)}
				<h2 class="text-2xl font-semibold">{conversation.title}</h2>
				<UserConversationPreferencesForm
					conversationId={conversation.id}
					isAnnon={user.authType === 'annon'}
				/>
			{/each}
		</section>
	</div>
</div>
