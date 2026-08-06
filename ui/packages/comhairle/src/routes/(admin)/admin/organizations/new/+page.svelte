<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import type { LocalizedRegionDto, OrganizationType } from '@crownshy/api-client/api';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { z } from 'zod';

	const organizationFormSchema = z.object({
		name: z.string().trim().min(1, 'Name is required'),
		description: z.string().optional(),
		mission: z.string().optional(),
		org_type: z.enum(['non_profit', 'governmental', 'other'] as const),
		contact_email: z.string().optional(),
		external_url: z.string().optional(),
		regions: z.array(z.string().uuid()).default([]),
		user_emails: z.string().optional(),
		organization_admin_emails: z.string().optional()
	});

	const organizationForm = superForm(
		{
			name: '',
			description: '',
			mission: '',
			org_type: 'non_profit' as OrganizationType,
			contact_email: '',
			external_url: '',
			regions: [] as string[],
			user_emails: '',
			organization_admin_emails: ''
		},
		{
			validators: zodClient(organizationFormSchema),
			taintedMessage: false,
			onSubmit: async ({ cancel }) => {
				cancel();
				submitting = true;

				const validation = await validateForm({ update: true });
				if (!validation.valid) {
					submitting = false;
					return;
				}

				const response = await tryCatchAsync(() =>
					apiClient.CreateOrganization({
						name: $form.name,
						description: $form.description || undefined,
						mission: $form.mission || undefined,
						org_type: $form.org_type,
						contact_email: $form.contact_email || undefined,
						external_url: $form.external_url || undefined,
						regions: $form.regions,
						user_emails: parseEmailList($form.user_emails ?? ''),
						organization_admin_emails: parseEmailList(
							$form.organization_admin_emails ?? ''
						)
					})
				);

				if (response.err !== null) {
					notifications.send({
						priority: 'ERROR',
						message: 'Failed to create organization'
					});
					submitting = false;
					return;
				}

				const organization = response.ok;
				const summary = organization.adminBootstrapSummary;
				const failedCount = summary.failures.length;

				notifications.send({
					priority: failedCount > 0 ? 'WARNING' : 'INFO',
					message:
						failedCount > 0
							? `Organization created. ${summary.assigned} of ${summary.attempted} administrators assigned; ${failedCount} need follow-up.`
							: 'Organization created'
				});

				submitting = false;
				goto(`/admin/organizations/${organization.id}`);
			}
		}
	);

	const { form, enhance, validateForm } = organizationForm;

	let allRegions = $state<LocalizedRegionDto[]>([]);
	let submitting = $state(false);

	onMount(async () => {
		const response = await apiClient.ListRegions({ queries: { limit: 500 } });
		allRegions = response.records;
	});

	function toggleRegion(regionId: string, enabled: boolean) {
		const selectedRegionIds = new Set($form.regions);

		if (enabled) {
			if (!selectedRegionIds.has(regionId)) {
				$form.regions = [...selectedRegionIds, regionId];
			}
			return;
		}

		$form.regions = [...selectedRegionIds].filter((id) => id !== regionId);
	}

	function parseEmailList(value: string): string[] | undefined {
		const emails = value
			.split(/\n|,/)
			.map((entry) => entry.trim())
			.filter(Boolean);

		return emails.length > 0 ? emails : undefined;
	}
</script>

<svelte:head>
	<title>Create organization - Comhairle Admin</title>
</svelte:head>

<div class="flex flex-col gap-6 p-8">
	<h1 class="text-3xl font-semibold">Create organization</h1>

	<form class="flex flex-col gap-4" method="POST" use:enhance>
		<div class="grid gap-4 md:grid-cols-2">
			<div class="flex flex-col gap-2">
				<Label for="name">Name</Label>
				<Input id="name" bind:value={$form.name} required />
			</div>
			<div class="flex flex-col gap-2">
				<Label for="orgType">Type</Label>
				<select
					id="orgType"
					bind:value={$form.org_type}
					class="border-input bg-background h-10 rounded-md border px-3 py-2 text-sm"
				>
					<option value="non_profit">Non-profit</option>
					<option value="governmental">Governmental</option>
					<option value="other">Other</option>
				</select>
			</div>
		</div>

		<div class="flex flex-col gap-2">
			<Label for="description">Description</Label>
			<Input id="description" bind:value={$form.description} />
		</div>

		<div class="flex flex-col gap-2">
			<Label for="mission">Mission</Label>
			<Input id="mission" bind:value={$form.mission} />
		</div>

		<div class="flex flex-col gap-2">
			<Label for="contactEmail">Contact email</Label>
			<Input id="contactEmail" type="email" bind:value={$form.contact_email} />
		</div>

		<div class="flex flex-col gap-2">
			<Label for="externalUrl">External URL</Label>
			<Input id="externalUrl" type="url" bind:value={$form.external_url} />
		</div>

		<div class="flex flex-col gap-2">
			<Label>Regions</Label>
			<div class="grid gap-2 rounded-md border p-3 md:grid-cols-2">
				{#if allRegions.length === 0}
					<p class="text-muted-foreground text-sm">No regions available.</p>
				{:else}
					{#each allRegions as region}
						<label class="flex items-center gap-2 text-sm">
							<input
								type="checkbox"
								checked={$form.regions.includes(region.id)}
								onchange={(event) =>
									toggleRegion(
										region.id,
										(event.currentTarget as HTMLInputElement).checked
									)}
							/>
							{region.name}
						</label>
					{/each}
				{/if}
			</div>
		</div>

		<div class="flex flex-col gap-2">
			<Label for="userEmails">User emails to add</Label>
			<textarea
				id="userEmails"
				bind:value={$form.user_emails}
				class="border-input bg-background min-h-24 rounded-md border px-3 py-2 text-sm"
				placeholder="one per line or comma separated"
			></textarea>
		</div>

		<div class="flex flex-col gap-2">
			<Label for="organizationAdminEmails">Initial organization administrators</Label>
			<p class="text-muted-foreground text-sm">
				These users can update or delete the organization and add or remove members.
			</p>
			<textarea
				id="organizationAdminEmails"
				bind:value={$form.organization_admin_emails}
				class="border-input bg-background min-h-24 rounded-md border px-3 py-2 text-sm"
				placeholder="one per line or comma separated"
			></textarea>
		</div>

		<div class="flex justify-end">
			<Button type="submit" disabled={submitting}>Create organization</Button>
		</div>
	</form>
</div>
