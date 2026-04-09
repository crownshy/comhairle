<script lang="ts">
	import {
		DateFormatter,
		getLocalTimeZone,
		parseDate,
		parseDateTime,
		today,
		type DateValue
	} from '@internationalized/date';
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import NewEventSchema from './NewEventSchema';
	import { CalendarIcon } from 'lucide-svelte';
	import Calendar from '$lib/components/ui/calendar/calendar.svelte';
	import { cn } from '$lib/utils';
	import { buttonVariants } from '$lib/components/ui/button';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { goto } from '$app/navigation';
	import { basic_learn_config } from '$lib/workflow_templates';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import { useAdminLayoutSlots } from '../../useAdminLayoutSlots.svelte';

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

		if (!result.valid) return;

		const dateOption = result.data.start_date;
		let startTime = parseDateTime(`${dateOption}T${result.data.start_time}`);
		// TODO: can we always assume end date is the same as the start date?
		let endTime = parseDateTime(`${dateOption}T${result.data.end_time}`);

		try {
			const eventParams = {
				...result.data,
				start_time: startTime.toDate(getLocalTimeZone()).toISOString(),
				end_time: endTime.toDate(getLocalTimeZone()).toISOString()
			};
			let event = await apiClient.CreateEvent(eventParams, {
				params: { conversation_id: conversation.id }
			});

			let workflow = await apiClient.CreateEventWorkflow(
				{
					name: 'Default event workflow',
					description: 'Default event workflow',
					is_active: true,
					is_public: true,
					auto_login: false
				},
				{ params: { conversation_id: conversation.id, event_id: event.id } }
			);

			await apiClient.CreateEventWorkflowStep(
				{
					name: 'Event agenda',
					description: 'The agenda for the event',
					is_offline: false,
					activation_rule: 'manual',
					step_order: 1,
					tool_setup: basic_learn_config,
					required: true
				},
				{
					params: {
						conversation_id: conversation.id,
						event_id: event.id,
						workflow_id: workflow.id
					}
				}
			);

			notifications.send({
				message: 'Event created',
				priority: 'INFO'
			});

			goto(`/admin/conversations/${conversation.id}/events`);
		} catch (e) {
			console.error(e);
			notifications.send({
				message: 'Something went wrong creating the event',
				priority: 'ERROR'
			});
		}
	}

	let startDate = $derived($formData.start_date ? parseDate($formData.start_date) : undefined);

	useAdminLayoutSlots({
		title: titleContentSnippet,
		breadcrumbs: breadcrumbSnippet
	});
</script>

<svelte:head>
	<title>Create New Event - Comhairle Admin</title>
</svelte:head>

{#snippet breadcrumbSnippet()}
	<Breadcrumb.Item>
		<Breadcrumb.Link href="/admin/conversations/{conversation.id}/events"
			>Events</Breadcrumb.Link
		>
	</Breadcrumb.Item>
	<Breadcrumb.Separator />
	<Breadcrumb.Item>New</Breadcrumb.Item>
{/snippet}

{#snippet titleContentSnippet()}
	<h1 class="text-4xl font-bold">New Event</h1>
{/snippet}

<div class="flex flex-col gap-4">
	<h2 class="text-card-foreground text-base font-semibold">Edit information</h2>
</div>

{#if $errorMessage}
	<p class="text-destructive mt-2 text-sm">{$errorMessage}</p>
{/if}

<form method="POST" onsubmit={handleSubmit} class="mt-8 flex flex-col" use:enhance>
	<!-- Title -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field {form} name="name" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
						>Title</Form.Label
					>
					<div class="flex-1">
						<Input {...props} bind:value={$formData.name} placeholder="Title" />
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<!-- Description -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field {form} name="description" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
						>Description</Form.Label
					>
					<div class="flex-1">
						<Input
							{...props}
							bind:value={$formData.description}
							placeholder="Description"
						/>
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<!-- Capacity -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field {form} name="capacity" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
						>Capacity</Form.Label
					>
					<div class="flex-1">
						<Input type="number" {...props} bind:value={$formData.capacity} />
						<Form.FieldErrors />
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<!-- Event date -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field {form} name="start_date" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
						>Event date</Form.Label
					>
					<div class="flex-1">
						<Popover.Root>
							<Popover.Trigger
								{...props}
								class={cn(
									buttonVariants({ variant: 'outline' }),
									'w-full max-w-xs justify-start pl-4 text-left font-normal',
									!startDate && 'text-muted-foreground'
								)}
							>
								{startDate
									? df.format(startDate.toDate(getLocalTimeZone()))
									: 'Pick a date'}
								<CalendarIcon class="ml-auto size-4 opacity-50" />
							</Popover.Trigger>
							<Popover.Content class="w-auto p-0" side="top">
								<Calendar
									type="single"
									value={startDate as DateValue}
									minValue={today(getLocalTimeZone())}
									calendarLabel="Event Date"
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
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<!-- Time (Start & End side by side) -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<p class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">Time</p>
		<div class="flex flex-1 gap-6">
			<Form.Field {form} name="start_time" class="flex-1">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label class="text-sm font-semibold">Start</Form.Label>
						<Input
							type="time"
							{...props}
							bind:value={$formData.start_time}
							class="appearance-none [&::-webkit-calendar-picker-indicator]:hidden [&::-webkit-calendar-picker-indicator]:appearance-none"
						/>
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>

			<Form.Field {form} name="end_time" class="flex-1">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label class="text-sm font-semibold">End</Form.Label>
						<Input
							type="time"
							{...props}
							bind:value={$formData.end_time}
							class="appearance-none [&::-webkit-calendar-picker-indicator]:hidden [&::-webkit-calendar-picker-indicator]:appearance-none"
						/>
					{/snippet}
				</Form.Control>
				<Form.FieldErrors />
			</Form.Field>
		</div>
	</div>

	<!-- Signup mode -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Fieldset {form} name="signup_mode" class="contents">
			<Form.Legend class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
				>Signup mode</Form.Legend
			>
			<div class="flex-1">
				<RadioGroup.Root
					bind:value={$formData.signup_mode}
					class="flex flex-row gap-8"
					name="signup_mode"
				>
					<div class="flex items-center gap-1.5">
						<Form.Control>
							{#snippet children({ props })}
								<RadioGroup.Item value="invite" {...props} />
								<Form.Label class="font-normal">Invite</Form.Label>
							{/snippet}
						</Form.Control>
					</div>
					<div class="flex items-center gap-1.5">
						<Form.Control>
							{#snippet children({ props })}
								<RadioGroup.Item value="open" {...props} />
								<Form.Label class="font-normal">Open</Form.Label>
							{/snippet}
						</Form.Control>
					</div>
				</RadioGroup.Root>
			</div>
		</Form.Fieldset>
	</div>

	<!-- Save Button -->
	<div class="border-border flex justify-center border-t py-6">
		<Form.Button variant="default" class="px-12" disabled={$submitting}>
			Save changes
		</Form.Button>
	</div>
</form>
