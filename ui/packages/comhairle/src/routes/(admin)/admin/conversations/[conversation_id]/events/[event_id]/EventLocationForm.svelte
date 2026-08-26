<script lang="ts">
	import type { LocalizedEventDto } from '@crownshy/api-client/api';
	import { EventLocationSchema } from './schema.js';
	import * as Form from '$lib/components/ui/form/';
	import Input from '$lib/components/ui/input/input.svelte';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { apiClient } from '@crownshy/api-client/client';
	import { invalidate } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte';
	import { key } from '$lib/utils/invalidationKey.js';

	let { event }: { event: LocalizedEventDto } = $props();

	let saving = $state(false);
	const locationForm = superForm(
		{
			location_venue_name: event.location?.venue_name,
			location_address_line_1: event.location?.address_line_1,
			location_address_line_2: event.location?.address_line_2,
			location_address_line_3: event.location?.address_line_3,
			location_city: event.location?.city,
			location_state_province: event.location?.state_province,
			location_postal_code: event.location?.postal_code,
			location_country_code: event.location?.country_code
		},
		{
			validators: zodClient(EventLocationSchema),
			taintedMessage: false,
			validationMethod: 'oninput',
			onSubmit: handleUpdateEventLocation
		}
	);

	let { form, enhance, validateForm, submitting, tainted } = $derived(locationForm);

	async function handleUpdateEventLocation({ cancel }) {
		// We submit via the API client below — prevent SvelteKit from POSTing the form to the
		// page route (which has no server actions and would return 405 Method Not Allowed).
		cancel();

		if (saving) return;

		const result = await validateForm({ update: true });

		if (!result.valid) return;

		saving = true;

		try {
			const {
				location_venue_name,
				location_address_line_1,
				location_address_line_2,
				location_address_line_3,
				location_city,
				location_state_province,
				location_postal_code,
				location_country_code
			} = result.data;

			await apiClient.UpdateEvent(
				{
					location: {
						venue_name: location_venue_name,
						city: location_city,
						state_province: location_state_province,
						postal_code: location_postal_code,
						country_code: location_country_code,
						address_line_1: location_address_line_1,
						...(location_address_line_2 && { address_line_2: location_address_line_2 }),
						...(location_address_line_3 && { address_line_3: location_address_line_3 })
					}
				},
				{
					params: { conversation_id: event.conversationId, event_id: event.id }
				}
			);

			await invalidate(key('conversation/events'));
			notifications.send({ message: 'Updated event location', priority: 'INFO' });
		} catch (e) {
			console.error(e);
			notifications.send({
				message: 'Something went wrong updating the event location',
				priority: 'ERROR'
			});
		} finally {
			saving = false;
		}
	}
</script>

<form method="POST" class="flex flex-col" use:enhance>
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={locationForm} name="location_venue_name" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label
						class="flex flex-col items-start text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
					>
						<span>Venue</span>
					</Form.Label>
					<div class="flex-1">
						<Input
							{...props}
							bind:value={$form.location_venue_name}
							placeholder="Venue"
						/>
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={locationForm} name="location_address_line_1" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label
						class="flex flex-col items-start text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
					>
						<span>Address line 1</span>
					</Form.Label>
					<div class="flex-1">
						<Input
							{...props}
							bind:value={$form.location_address_line_1}
							placeholder="Address line 1"
						/>
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={locationForm} name="location_address_line_2" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label
						class="flex flex-col items-start text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
					>
						<span>Address line 2 (optional)</span>
					</Form.Label>
					<div class="flex-1">
						<Input
							{...props}
							bind:value={$form.location_address_line_2}
							placeholder="Address line 2"
						/>
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={locationForm} name="location_address_line_3" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label
						class="flex flex-col items-start text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
					>
						<span>Address line 3 (optional)</span>
					</Form.Label>
					<div class="flex-1">
						<Input
							{...props}
							bind:value={$form.location_address_line_3}
							placeholder="Address line 3"
						/>
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={locationForm} name="location_city" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label
						class="flex flex-col items-start text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
					>
						<span>City</span>
					</Form.Label>
					<div class="flex-1">
						<Input {...props} bind:value={$form.location_city} placeholder="City" />
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={locationForm} name="location_state_province" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label
						class="flex flex-col items-start text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
					>
						<span>State / province</span>
					</Form.Label>
					<div class="flex-1">
						<Input
							{...props}
							bind:value={$form.location_state_province}
							placeholder="State / province"
						/>
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={locationForm} name="location_postal_code" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label
						class="flex flex-col items-start text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
					>
						<span>Postal code</span>
					</Form.Label>
					<div class="flex-1">
						<Input
							{...props}
							bind:value={$form.location_postal_code}
							placeholder="Postal code"
						/>
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={locationForm} name="location_country_code" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label
						class="flex flex-col items-start text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
					>
						<span>Country code</span>
					</Form.Label>
					<div class="flex-1">
						<Input
							{...props}
							bind:value={$form.location_country_code}
							placeholder="Country code"
						/>
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<div class="border-border flex justify-center border-t py-6">
		<Form.Button
			type="submit"
			variant="default"
			class="px-12"
			disabled={saving || $submitting || !$tainted}
		>
			Save Changes
		</Form.Button>
	</div>
</form>
