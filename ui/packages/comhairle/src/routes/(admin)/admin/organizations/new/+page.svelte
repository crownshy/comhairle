<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { apiClient } from '@crownshy/api-client/client';
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import type { OrganizationType } from '@crownshy/api-client/api';

	let name = $state('');
	let description = $state('');
	let mission = $state('');
	let orgType: OrganizationType = $state('non_profit');
	let contactEmail = $state('');
	let externalUrl = $state('');
	let allRegions = $state<{ id: string; name: string }[]>([]);
	let selectedRegions = $state<string[]>([]);
	let userEmails = $state('');
	let organizationAdminEmails = $state('');

	onMount(async () => {
		const response = await fetch('/api/regions?limit=500');
		if (!response.ok) {
			return;
		}

		const regionResults = (await response.json()) as {
			records: { id: string; name: string }[];
		};
		allRegions = regionResults.records;
	});

	function toggleRegion(regionId: string, enabled: boolean) {
		if (enabled) {
			if (!selectedRegions.includes(regionId)) {
				selectedRegions = [...selectedRegions, regionId];
			}
			return;
		}

		selectedRegions = selectedRegions.filter((id) => id !== regionId);
	}

	function parseEmailList(value: string): string[] | undefined {
		const emails = value
			.split(/\n|,/)
			.map((entry) => entry.trim())
			.filter(Boolean);

		return emails.length > 0 ? emails : undefined;
	}

	async function handleSubmit(event: Event) {
		event.preventDefault();

		const response = await tryCatchAsync(() =>
			apiClient.CreateOrganization({
				name,
				description,
				mission,
				org_type: orgType,
				contact_email: contactEmail || undefined,
				external_url: externalUrl || undefined,
				regions: selectedRegions,
				user_emails: parseEmailList(userEmails),
				organization_admin_emails: parseEmailList(organizationAdminEmails)
			})
		);

		if (response.err !== null) {
			notifications.send({ priority: 'ERROR', message: 'Failed to create organization' });
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

		goto(`/admin/organizations/${organization.id}`);
	}
</script>

<svelte:head>
	<title>Create organization - Comhairle Admin</title>
</svelte:head>

<div class="flex flex-col gap-6 p-8">
	<h1 class="text-3xl font-semibold">Create organization</h1>

	<form class="flex flex-col gap-4" onsubmit={handleSubmit}>
		<div class="grid gap-4 md:grid-cols-2">
			<div class="flex flex-col gap-2">
				<Label for="name">Name</Label>
				<Input id="name" bind:value={name} required />
			</div>
			<div class="flex flex-col gap-2">
				<Label for="orgType">Type</Label>
				<select
					id="orgType"
					bind:value={orgType}
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
			<Input id="description" bind:value={description} />
		</div>

		<div class="flex flex-col gap-2">
			<Label for="mission">Mission</Label>
			<Input id="mission" bind:value={mission} />
		</div>

		<div class="flex flex-col gap-2">
			<Label for="contactEmail">Contact email</Label>
			<Input id="contactEmail" type="email" bind:value={contactEmail} />
		</div>

		<div class="flex flex-col gap-2">
			<Label for="externalUrl">External URL</Label>
			<Input id="externalUrl" type="url" bind:value={externalUrl} />
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
								checked={selectedRegions.includes(region.id)}
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
				bind:value={userEmails}
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
				bind:value={organizationAdminEmails}
				class="border-input bg-background min-h-24 rounded-md border px-3 py-2 text-sm"
				placeholder="one per line or comma separated"
			></textarea>
		</div>

		<div class="flex justify-end">
			<Button type="submit">Create organization</Button>
		</div>
	</form>
</div>
