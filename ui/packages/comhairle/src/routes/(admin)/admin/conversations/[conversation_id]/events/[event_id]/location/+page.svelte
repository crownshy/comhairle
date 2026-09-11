<script lang="ts">
	import EventLocationSchema from './schema';
	import * as Form from '$lib/components/ui/form/';
	import Input from '$lib/components/ui/input/input.svelte';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { apiClient } from '@crownshy/api-client/client';
	import { invalidate } from '$app/navigation';
	import { notifications } from '$lib/notifications.svelte';
	import { key } from '$lib/utils/invalidationKey.js';
	import { tryCatchAsync } from '$lib/utils/errorHandling';

	const { data } = $props();
	const { event } = $derived(data);

	let saving = $state(false);
	const locationForm = superForm(
		{
			venue_name: event.location?.venue_name,
			address_line_1: event.location?.address_line_1,
			address_line_2: event.location?.address_line_2,
			address_line_3: event.location?.address_line_3,
			city: event.location?.city,
			state_province: event.location?.state_province,
			postal_code: event.location?.postal_code,
			country_code: event.location?.country_code
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

		const updateEvent = await tryCatchAsync(() =>
			apiClient.UpdateEvent(
				{ location: result.data },
				{
					params: { conversation_id: event.conversationId, event_id: event.id }
				}
			)
		);

		saving = false;

		if (updateEvent.err !== null) {
			console.error(updateEvent.err);
			notifications.send({
				message: 'Something went wrong updating the event location',
				priority: 'ERROR'
			});
			return;
		}

		await invalidate(key('conversation/events'));
		notifications.send({ message: 'Updated event location', priority: 'INFO' });
	}
</script>

<div class="flex flex-col gap-10 py-6">
	<div class="flex flex-col gap-2">
		<h2 class="text-3xl font-bold">
			Location <span class="font-bold">(for in-person events)</span>
		</h2>
	</div>

	<form method="POST" class="flex flex-col" use:enhance>
		<div
			class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
		>
			<Form.Field form={locationForm} name="venue_name" class="contents">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label
							class="flex flex-col items-start text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
						>
							<span>Venue</span>
						</Form.Label>
						<div class="flex-1">
							<Input {...props} bind:value={$form.venue_name} placeholder="Venue" />
							<Form.FieldErrors />
						</div>
					{/snippet}
				</Form.Control>
			</Form.Field>
		</div>

		<div
			class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
		>
			<Form.Field form={locationForm} name="address_line_1" class="contents">
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
								bind:value={$form.address_line_1}
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
			<Form.Field form={locationForm} name="address_line_2" class="contents">
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
								bind:value={$form.address_line_2}
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
			<Form.Field form={locationForm} name="address_line_3" class="contents">
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
								bind:value={$form.address_line_3}
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
			<Form.Field form={locationForm} name="city" class="contents">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label
							class="flex flex-col items-start text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
						>
							<span>City</span>
						</Form.Label>
						<div class="flex-1">
							<Input {...props} bind:value={$form.city} placeholder="City" />
							<Form.FieldErrors />
						</div>
					{/snippet}
				</Form.Control>
			</Form.Field>
		</div>

		<div
			class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
		>
			<Form.Field form={locationForm} name="state_province" class="contents">
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
								bind:value={$form.state_province}
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
			<Form.Field form={locationForm} name="postal_code" class="contents">
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
								bind:value={$form.postal_code}
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
			<Form.Field form={locationForm} name="country_code" class="contents">
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
								bind:value={$form.country_code}
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
</div>
