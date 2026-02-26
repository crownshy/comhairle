<script lang="ts">
	import {
		DateFormatter,
		getLocalTimeZone,
		parseDate,
		today,
		type DateValue
	} from '@internationalized/date';
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import { superForm } from "sveltekit-superforms";
	import { zodClient } from "sveltekit-superforms/adapters";
	import NewEventSchema from "./NewEventSchema";
	import { CalendarIcon } from 'lucide-svelte';
	import Calendar from '$lib/components/ui/calendar/calendar.svelte';
	import { cn } from '$lib/utils';
	import { buttonVariants } from '$lib/components/ui/button';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crown-shy/api-client/client';

	let { data } = $props();
	let { form: formDefaults, conversation } = data;

	const form = superForm(formDefaults, {
		validators: zodClient(NewEventSchema)
	});

	const { form: formData, enhance, message: errorMessage, validateForm, submitting } = form;

	const df = new DateFormatter('en-UK', {
		dateStyle: 'long'
	});
	
	async function handleSubmit(e: Event) {
		const result = await validateForm();

		if (result.valid) {
			try {
				const eventParams = {
					...result.data,
				}
				await apiClient.CreateEvent(eventParams, { params: { conversation_id: conversation.id }});

			} catch (e) {
				console.error(e);
				notifications.send({ message: "Something went wrong creating the event", priority: "ERROR" });
			}
		}
	}

	let startDate = $derived($formData.start_time ? parseDate($formData.start_date) : undefined);
</script>

<form onsubmit={handleSubmit} class="space-y-4" method="POST" use:enhance>
	<h2 class="text-xl font-bold">Create a new event</h2>

	{#if $errorMessage}
		<p class="text-destructive text-sm">{$errorMessage}</p>
	{/if}

	<!-- TODO: translations for hardcoded text -->
	<Form.Field {form} name="name">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Enter a name for your event</Form.Label>
				<Input {...props} bind:value={$formData.name} placeholder="Name" />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="description">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Enter a description of your event</Form.Label>
				<Input {...props} bind:value={$formData.description} placeholder="Description" />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="capacity">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Enter the max capacity of attendees for your event</Form.Label>
				<Input type="number" {...props} bind:value={$formData.capacity} class="w-fit" />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>


	<Form.Field {form} name="start_date">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Select a date for your event</Form.Label>
				<Popover.Root>
					<Popover.Trigger
						{...props}
						class={cn(
							buttonVariants({ variant: 'outline' }),
							'w-70 justify-start pl-4 text-left font-normal',
							!startDate && 'text-muted-foreground'
						)}
					>
						{startDate
							? df.format(startDate.toDate(getLocalTimeZone()))
							: 'Pick a date'}
						<CalendarIcon class="ml-auto size-4 opacity-50" />
					</Popover.Trigger>
					<Popover.Content class=" w-auto p-0" side="top">
						<Calendar
							type="single"
							value={startDate as DateValue}
							minValue={today(getLocalTimeZone())}
							calendarLabel="Expire Date"
							onValueChange={(v) => {
								if (v) {
									$formData.start_date = v.toString();
								} else {
									$formData.start_date = '';
								}
							}}
						/>
					</Popover.Content>
				</Popover.Root>
				<Form.FieldErrors />
				<input hidden value={$formData.start_date} name="start_date" />
			{/snippet}
		</Form.Control>
	</Form.Field>

	<Form.Field {form} name="start_time">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Enter a start time for your event</Form.Label>
				<Input type="time" {...props} bind:value={$formData.start_time} />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Field {form} name="end_time">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Enter an end time for your event</Form.Label>
				<Input type="time" {...props} bind:value={$formData.end_time} />
			{/snippet}
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>

	<Form.Fieldset form={form} name="signup_mode" class="space-y-3">
		<Form.Legend>Signup mode</Form.Legend>
		<RadioGroup.Root
			bind:value={$formData.signup_mode}
			class="flex flex-row space-x-1"
			name="signup_mode"
		>
			<div class="flex items-center space-y-0 space-x-3">
				<Form.Control>
					{#snippet children({ props })}
						<RadioGroup.Item value="invite" {...props} />
						<Form.Label class="font-normal">Invite</Form.Label>
					{/snippet}
				</Form.Control>
				<Form.Control>
					{#snippet children({ props })}
						<RadioGroup.Item value="open" {...props} />
						<Form.Label class="font-normal">Open</Form.Label>
					{/snippet}
				</Form.Control>
			</div>
		</RadioGroup.Root>
	</Form.Fieldset>

	<Form.Button class="my-5" disabled={$submitting}>Submit</Form.Button>
</form>
