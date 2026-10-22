<script lang="ts">
	import { invalidate } from '$app/navigation';
	import type { PageProps } from './$types';
	import UpgradeAccountModal from '$lib/components/UpgradeAccountModal/UpgradeAccountModal.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import { key } from '$lib/utils/invalidationKey';
	import { Label } from '$lib/components/ui/label';
	import { notifications } from '$lib/notifications.svelte';
	import { Form, Input, SelectList, Submit, validate } from '$lib/components/EasyForm';
	import Demographics from '$lib/interfaces/Demographics';
	import { Second } from '$lib/utils/units';
	import { Button } from '$lib/components/ui/button';
	import { apiClient } from '@crownshy/api-client/client';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import UserDetailsSchema from './UserDetailsSchema';
	import UpdatePasswordDialog from './UpdatePasswordDialog.svelte';

	let { data }: PageProps = $props();
	let { user, demographicQuestions, demographicResponses } = $derived(data);

	async function handleUserDetails(formData: FormData) {
		const result = validate(formData, UserDetailsSchema);
		if (result.err !== null) {
			console.error(result.err);
			notifications.send({
				message: `Validation error: ${result.err}`,
				priority: 'ERROR'
			});
			return;
		}

		const username = formData.get('username') as string;
		const email = formData.get('email') as string;

		const updatedValues: { username?: string; email?: string } = {};
		let hasChanges = false;

		if (username !== user.username) {
			updatedValues.username = username;
			hasChanges = true;
		}

		if (email !== user.email) {
			updatedValues.email = email;
			hasChanges = true;
		}

		if (!hasChanges) {
			return;
		}

		const updatedUser = await tryCatchAsync(() => apiClient.UpdateUserDetails(updatedValues));
		if (updatedUser.err !== null) {
			console.error(updatedUser.err);
			notifications.send({
				message: 'Failed to update user details. Please try again.',
				priority: 'ERROR'
			});
			return;
		}
	}

	function getInitialValue(questionSlug: string): string {
		return (
			demographicResponses.find((r) => r.questionSlug === questionSlug)?.value?.toString() ??
			''
		);
	}

	async function handleDemographics(formData: FormData) {
		const updateQuestions = [];
		let failed = false;

		for (const question of demographicQuestions) {
			const update = Demographics.updateResponse({
				questionSlug: question.slug,
				userId: user.id,
				newValue: (formData.get(question.slug) as string) ?? undefined,
				currentResponse: demographicResponses.find((r) => r.questionSlug === question.slug)
			}).then((result) => {
				if (result !== undefined && result.err !== null) {
					failed = true;
					notifications.send({
						message: `Failed to update: ${question.slug}`,
						priority: 'ERROR',
						duration: 5 * Second
					});
				}
			});

			if (update === undefined) {
				continue;
			}

			updateQuestions.push(update);
		}

		await Promise.all(updateQuestions);

		if (failed) {
			return;
		}

		notifications.send({
			message: 'Demographics updated successfully',
			priority: 'INFO',
			duration: 5 * Second
		});

		await invalidate(key('public/settings/profile'));
	}
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
		<div class="space-y-6 border-b py-5">
			<div>
				<h2 class="text-lg font-medium">Account Details</h2>
				<span class="text-muted-foreground text-sm"
					>Update your username and password here.</span
				>
			</div>

			<Form class="flex flex-col gap-4" handleSubmission={handleUserDetails}>
				{#snippet children(isSubmitting)}
					<Input
						{...UserDetailsSchema.username}
						label="Username"
						defaultValue={user.username}
						placeholder="Enter your username"
					/>
					<Input
						{...UserDetailsSchema.email}
						label="Email"
						type="email"
						defaultValue={user.email}
						placeholder="Enter your email"
					/>
					<Submit class="self-end" text="Save Changes" {isSubmitting} />
				{/snippet}
			</Form>

			<span class="block text-sm font-medium">Password</span>
			<UpdatePasswordDialog>
				<Button>Change password</Button>
			</UpdatePasswordDialog>
		</div>

		<div class="space-y-6 border-b py-5">
			<div>
				<h2 class="text-lg font-medium">Demographics</h2>
				<p class="text-muted-foreground text-sm">
					Update your demographic information here. This helps us contextualize
					conversation results.
				</p>
			</div>

			{#if demographicQuestions.length === 0}
				<SelectList
					title="Ethnicity"
					name="hello"
					options={[
						{ label: 'hello', value: 'world' },
						{ label: 'a', value: 'b' },
						{ label: 'c', value: 'd' }
					]}
					onselect={console.log}
				/>
				<span class="text-muted-foreground mt-4 text-sm italic">
					No demographic questions are currently available.
				</span>
			{:else}
				<Form class="grid gap-4 md:grid-cols-2" handleSubmission={handleDemographics}>
					{#snippet children(isSubmitting)}
						{#each demographicQuestions as question (question.slug)}
							<div class="space-y-2">
								<!-- Use question.label or question.text if your backend provides it, otherwise format the slug -->
								<Label for={question.slug}>{question.displayName}</Label>
								{#if question.bucketConfig?.type === 'string' && question.bucketConfig?.options?.length > 0}
									<SelectList
										title="Ethnicity"
										name={question.slug}
										options={[
											{ label: 'hello', value: 'world' },
											{ label: 'a', value: 'b' },
											{ label: 'c', value: 'd' }
										]}
										onselect={console.log}
									/>
								{:else}
									<Input
										id={question.slug}
										name={question.slug}
										defaultValue={getInitialValue(question.slug)}
									/>
								{/if}
							</div>
						{/each}

						<Submit
							text="Make changes"
							class="self-end justify-self-end"
							{isSubmitting}
						/>
					{/snippet}
				</Form>
			{/if}
		</div>
	{/if}
</section>
