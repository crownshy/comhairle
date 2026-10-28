<script lang="ts">
	import UpgradeAccountModal from '$lib/components/UpgradeAccountModal/UpgradeAccountModal.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import UserDetailsForm from '$lib/components/UserDetailsForm/UserDetailsForm.svelte';
	import { Spinner } from '$lib/components/ui/spinner';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';

	let { data } = $props();
	let { user } = $derived(data);
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

		<div class="space-y-6 border-b py-5">
			<div>
				<h2 class="text-lg font-medium">Demographics</h2>
				<p class="text-muted-foreground text-sm">
					Update your demographic information here. This helps us contextualize
					conversation results.
				</p>
			</div>
			{#await data.streamedMyResponses}
				<div class="flex flex-row items-center gap-x-2">
					<Spinner />
					<span>Loading responses</span>
				</div>
			{:then myResponses}
				{#if myResponses.err !== null}
					{console.error(myResponses.err)}
					<ErrorMessage>Could not load responses, please try again</ErrorMessage>
				{:else}
					{#each myResponses.ok as response (response.question)}
						<div class="mb-5 flex flex-col gap-y-1">
							<span class="text-muted-foreground text-sm font-medium">
								{response.question}
							</span>
							{response.answer}
						</div>
					{:else}
						<span class="text-muted-foreground italic">No responses yet</span>
					{/each}
				{/if}
			{/await}
		</div>
	{/if}
</section>
