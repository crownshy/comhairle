<script lang="ts">
	import { invalidate } from '$app/navigation';
	import type { PageProps } from './$types';
	import UserDetailsForm from '$lib/components/UserDetailsForm/UserDetailsForm.svelte';
	import UserDemographicsForm from './UserDemographicsForm.svelte';
	import UpgradeAccountModal from '$lib/components/UpgradeAccountModal/UpgradeAccountModal.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import { key } from '$lib/utils/invalidationKey';

	let { data }: PageProps = $props();
	let { user, demographicQuestions, demographicResponses } = $derived(data);
</script>

<PageHeader title="Your profile" description="Update account details and demographic data here" />

<section>
	{#if user.authType === 'guest'}
		<div class="space-y-6">
			<div class="text-center">
				<div class="text-muted-foreground mb-4">
					You are currently signed in as a guest account with ID:
				</div>
				<h3 class="my-4 text-center text-2xl font-bold">{user.guestCode}</h3>
			</div>

			<div class=" bg-card p-6">
				<h4 class="mb-2 font-semibold">Upgrade to a Full Account</h4>
				<p class="text-muted-foreground mb-4 text-sm">
					Transform your anonymous account to receive email updates and be informed about
					the results of the conversations you have taken part in. Your current
					participation will be preserved.
				</p>
				<div class="flex w-full flex-row items-end justify-center md:justify-end">
					<UpgradeAccountModal
						onSuccess={(upgradedUser) => void (user = upgradedUser)}
						currentUser={user}
					/>
				</div>
			</div>
		</div>
	{:else}
		<UserDetailsForm {user} />
		{#key demographicResponses}
			<UserDemographicsForm
				questions={demographicQuestions}
				responses={demographicResponses}
				userId={user.id}
				onSaved={() => invalidate(key('public/settings/profile'))}
			/>
		{/key}
	{/if}
</section>
