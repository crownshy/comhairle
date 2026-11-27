<script lang="ts">
	import { goto } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import { useTour } from '$lib/tours/useTour.svelte';
	import { newOrganizationTour } from './tour';
	import { Form, Input, Select, Submit, type OnSubmit } from '$lib/components/EasyForm';
	import ErrorMessage from '$lib/components/ErrorMessage.svelte';
	import { resolve } from '$app/paths';
	import TabContent from '$lib/components/TabContent.svelte';
	import SectionHeader from '$lib/components/SectionHeader.svelte';
	import { key } from '$lib/utils/invalidationKey';

	const { data } = $props();

	useTour(() => newOrganizationTour);

	type OrganisationType = 'non_profit' | 'governmental' | 'other';

	const createOrganisation: OnSubmit = async (formData) => {
		const name = formData.get('name');
		const org_type = formData.get<OrganisationType>('org-type');
		const regions = formData.getAll('regions');

		if (!name || !org_type) {
			return;
		}

		const createOrganisation = await tryCatchAsync(() =>
			apiClient.CreateOrganization({
				name,
				org_type,
				description: formData.get('description') ?? '',
				mission: formData.get('mission') ?? '',
				contact_email: formData.get('contact-email'),
				external_url: formData.get('external-url'),
				regions
			})
		);

		if (createOrganisation.err !== null) {
			notifications.send({
				message: 'Failed to create organization',
				priority: 'ERROR'
			});
			return;
		}

		await goto(resolve('/(admin)/admin/organizations'), {
			invalidate: [key('admin/organizations')]
		});
	};
</script>

<svelte:head>
	<title>Create organisation - Comhairle Admin</title>
</svelte:head>

<Form onsubmit={createOrganisation}>
	{#snippet children({ isSubmitting })}
		<SectionHeader
			content={{
				eyebrow: 'Administration',
				title: 'Create organisation'
			}}
		>
			{#snippet actions()}
				<Submit text="Create organisation" {isSubmitting} />
			{/snippet}
		</SectionHeader>

		<TabContent>
			<section class="flex max-w-4xl flex-col gap-y-3">
				<div class="grid grid-cols-2 items-end gap-x-5">
					<Input name="name" label="Name" required />
					<div class="shrink-0 grow pt-5">
						<Select
							id="org-type"
							name="org-type"
							required
							question="Type"
							placeholder="Organisation type"
							preventDeselection
							defaultSelected="non_profit"
							options={[
								['non_profit', 'Non-profit'],
								['governmental', 'Governmental'],
								['other', 'Other']
							]}
						/>
					</div>
				</div>
				<Input name="description" label="Description" />
				<Input name="mission" label="Mission" />
				<Input name="contact-email" label="Contact email" type="email" />
				<Input name="external-url" label="External URL" type="url" />
				<div class="flex flex-col gap-2 pt-2">
					Regions
					<div class="grid gap-2 rounded-md border p-3 md:grid-cols-2">
						{#if data.regions.err !== null}
							<ErrorMessage>Could not load regions please try again</ErrorMessage>
						{:else}
							{#each data.regions.ok as region (region.id)}
								<label class="flex items-center gap-2 text-sm">
									<input type="checkbox" name="regions" value={region.id} />
									{region.name}
								</label>
							{:else}
								<p class="text-muted-foreground text-sm">No regions available.</p>
							{/each}
						{/if}
					</div>
				</div>
			</section>
		</TabContent>
	{/snippet}
</Form>
