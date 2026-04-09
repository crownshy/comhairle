<script lang="ts">
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Form from '$lib/components/ui/form/';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import { CalendarIcon } from 'lucide-svelte';
	import Calendar from '$lib/components/ui/calendar/calendar.svelte';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import EventSchema from './schema.js';
	import { useAdminLayoutSlots } from '../../useAdminLayoutSlots.svelte.js';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import AdminPrevNextControls from '$lib/components/AdminPrevNextControls.svelte';
	import { cn } from '$lib/utils';
	import { buttonVariants } from '$lib/components/ui/button';
	import {
		DateFormatter,
		getLocalTimeZone,
		type DateValue,
		today,
		parseDate,
		parseDateTime
	} from '@internationalized/date';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { invalidateAll } from '$app/navigation';
	import BadgeInput from '$lib/components/ui/badge-input/badge-input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';

	let { data } = $props();

	const event = $derived(data.event);
	const conversation = $derived(data.conversation);
	let primaryLanguage = $derived(data.conversation.primaryLocale ?? 'en');
	let supportedLanguages = $derived(data.conversation.supportedLanguages ?? ['en']);

	const [startDate, startTimeWithZone] = $derived(event.startTime.split('T'));
	const [, endTimeWithZone] = $derived(event.endTime.split('T'));

	const eventForm = superForm(
		{
			name: event.name,
			description: event.description,
			capacity: event.capacity,
			start_date: startDate,
			start_time: startTimeWithZone.replace('Z', ''),
			end_time: endTimeWithZone.replace('Z', ''),
			signup_mode: event.signupMode
		},
		{
			validators: zodClient(EventSchema),
			taintedMessage: false,
			validationMethod: 'oninput',
			onSubmit: handleUpdateEvent
		}
	);

	let { form, enhance, validateForm, submitting, tainted } = $derived(eventForm);

	async function handleUpdateEvent() {
		const result = await validateForm({ update: true });

		if (!result.valid) return;

		const dateOption = result.data.start_date;
		let startTime = parseDateTime(`${dateOption}T${result.data.start_time}`);
		let endTime = parseDateTime(`${dateOption}T${result.data.end_time}`);

		const {
			name: _name /* eslint-disable-line @typescript-eslint/no-unused-vars */,
			description: _description /* eslint-disable-line @typescript-eslint/no-unused-vars */,
			...eventData
		} = result.data;

		try {
			const eventParams = {
				...eventData,
				start_time: startTime.toDate(getLocalTimeZone()).toISOString(),
				end_time: endTime.toDate(getLocalTimeZone()).toISOString()
			};

			await apiClient.UpdateEvent(eventParams, {
				params: {
					conversation_id: conversation.id,
					event_id: event.id
				}
			});

			await invalidateAll();
			notifications.send({ message: 'Updated event', priority: 'INFO' });
		} catch (e) {
			console.error(e);
			notifications.send({
				message: 'Something went wrong updating the event',
				priority: 'ERROR'
			});
		}
	}

	const df = new DateFormatter('en-UK', {
		dateStyle: 'long'
	});

	useAdminLayoutSlots({
		title: titleContentSnippet,
		breadcrumbs: breadcrumbSnippet
	});

	let eventDate = $derived($form.start_date ? parseDate($form.start_date) : undefined);
	let pageTitle = $derived(`Edit Event: ${event.name}`);

	// TODO: get from facilitators
	let facilitators = $state([]);

	async function handleAddFacilitator(value: string) {
		try {
			await apiClient.CreateFacilitatorEventAttendance(
				{ email: value },
				{
					params: {
						conversation_id: conversation.id,
						event_id: event.id
					}
				}
			);

			notifications.send({
				priority: 'INFO',
				message: 'Facilitator added'
			});

			await invalidateAll();
		} catch (e) {
			console.error(e);
			notifications.send({
				priority: 'ERROR',
				message:
					e.status === 404 ? 'Unable to find user' : 'Failed to add facilitator to event'
			});
		}
	}

	async function handleDeleteFacilitator(id: string) {
		try {
			await apiClient.DeleteEventAttendance({
				params: {
					conversation_id: conversation.id,
					event_id: event.id,
					event_attendance_id: id
				}
			});

			notifications.send({
				priority: 'INFO',
				message: 'Facilitator removed'
			});

			await invalidateAll();
		} catch (e) {
			console.error(e);
			notifications.send({
				priority: 'ERROR',
				message: 'Failed to remove facilitator from event'
			});
		}
	}
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle Admin</title>
</svelte:head>

{#snippet breadcrumbSnippet()}
	<Breadcrumb.Item>{event?.name}</Breadcrumb.Item>
{/snippet}

{#snippet titleContentSnippet()}
	<h1 class="text-4xl font-bold">Event: {event?.name}</h1>
	<!-- TODO: figure out these -->
	<!-- <AdminPrevNextControls -->
	<!-- 	next={{ name: 'design', url: `/admin/conversations/${conversation.id}/design` }} -->
	<!-- /> -->
{/snippet}

<Tabs.Root value="eventDetails" class="flex min-h-0 flex-1 flex-col">
	<div class="bg-sidebar mb-8 flex w-fit shrink-0 flex-row gap-4 rounded-xl p-1">
		<Tabs.Trigger
			value="eventDetails"
			class="text-sidebar-foreground data-[state=active]:text-foreground border-none"
		>
			Details
		</Tabs.Trigger>
		<Tabs.Trigger
			value="facilitators"
			class="text-sidebar-foreground data-[state=active]:text-foreground border-none"
		>
			Facilitators
		</Tabs.Trigger>
	</div>
	<Tabs.Content value="eventDetails">
		<form method="POST" class="flex flex-col" use:enhance>
			<div
				class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
			>
				<Form.Field form={eventForm} name="name" class="contents">
					<Form.Control>
						{#snippet children({ props })}
							<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
								>Name</Form.Label
							>
							<div class="flex-1">
								<TranslatableField
									value={$form.name}
									onValueChange={(v) => ($form.name = v)}
									translation={event.translations?.name}
									primaryLocale={primaryLanguage}
									{supportedLanguages}
									inputProps={props}
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
				<Form.Field form={eventForm} name="description" class="contents">
					<Form.Control>
						{#snippet children({ props })}
							<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
								>Description</Form.Label
							>
							<div class="flex-1">
								<TranslatableField
									value={$form.description}
									onValueChange={(v) => ($form.description = v)}
									translation={event.translations?.description}
									primaryLocale={primaryLanguage}
									{supportedLanguages}
									inputType="textarea"
									inputProps={props}
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
				<Form.Field form={eventForm} name="capacity" class="contents">
					<Form.Control>
						{#snippet children({ props })}
							<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">
								Capacity
							</Form.Label>
							<Input {...props} bind:value={$form.capacity} type="number" />
							<Form.FieldErrors />
						{/snippet}
					</Form.Control>
				</Form.Field>
			</div>

			<div
				class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
			>
				<Form.Field form={eventForm} name="start_date" class="contents">
					<Form.Control>
						{#snippet children({ props })}
							<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">
								Event date
							</Form.Label>
							<Input {...props} bind:value={$form.start_date} />
							<Form.FieldErrors />
						{/snippet}
					</Form.Control>
				</Form.Field>
			</div>

			<div
				class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
			>
				<Form.Field form={eventForm} name="start_time" class="contents">
					<Form.Control>
						{#snippet children({ props })}
							<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">
								Event date
							</Form.Label>
							<Popover.Root>
								<Popover.Trigger
									{...props}
									class={cn(
										buttonVariants({ variant: 'outline' }),
										'w-70 justify-start pl-4 text-left font-normal',
										!startDate && 'text-muted-foreground'
									)}
								>
									{eventDate
										? df.format(eventDate.toDate(getLocalTimeZone()))
										: 'Pick a date'}
									<CalendarIcon class="ml-auto size-4 opacity-50" />
								</Popover.Trigger>
								<Popover.Content class=" w-auto p-0" side="top">
									<Calendar
										type="single"
										value={eventDate as DateValue}
										minValue={today(getLocalTimeZone())}
										calendarLabel="Event Date"
										onValueChange={(v) => {
											if (v) {
												$form.start_date = v.toString();
											} else {
												$form.start_date = '';
											}
										}}
									/>
								</Popover.Content>
							</Popover.Root>
							<Form.FieldErrors />
							<input hidden value={$form.start_date} name="start_date" />
						{/snippet}
					</Form.Control>
				</Form.Field>
			</div>

			<div
				class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
			>
				<Form.Field form={eventForm} name="start_time" class="contents">
					<Form.Control>
						{#snippet children({ props })}
							<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">
								Start time
							</Form.Label>
							<Input
								bind:value={$form.start_time}
								{...props}
								type="time"
								class="appearance-none [&::-webkit-calendar-picker-indicator]:hidden [&::-webkit-calendar-picker-indicator]:appearance-none"
							/>
							<Form.FieldErrors />
						{/snippet}
					</Form.Control>
				</Form.Field>
			</div>

			<div
				class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
			>
				<Form.Field form={eventForm} name="end_time" class="contents">
					<Form.Control>
						{#snippet children({ props })}
							<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">
								End time
							</Form.Label>
							<Input
								bind:value={$form.end_time}
								{...props}
								type="time"
								class="appearance-none [&::-webkit-calendar-picker-indicator]:hidden [&::-webkit-calendar-picker-indicator]:appearance-none"
							/>
							<Form.FieldErrors />
						{/snippet}
					</Form.Control>
				</Form.Field>
			</div>

			<div
				class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
			>
				<Form.Fieldset form={eventForm} name="signup_mode" class="contents">
					<Form.Legend class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
						>Signup mode</Form.Legend
					>
					<RadioGroup.Root
						bind:value={$form.signup_mode}
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
			</div>

			<div class="border-border flex justify-center border-t py-6">
				<Form.Button
					type="submit"
					variant="default"
					class="px-12"
					disabled={$submitting || !$tainted}
				>
					Save Changes
				</Form.Button>
			</div>
		</form>
	</Tabs.Content>
	<Tabs.Content value="facilitators">
		<div
			class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
		>
			<div class="contents">
				<Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">Facilitators</Label
				>
				<BadgeInput
					onAddBadge={handleAddFacilitator}
					onDeleteBadge={handleDeleteFacilitator}
					badges={facilitators}
					placeholder="Enter an email address"
				/>
			</div>
		</div>
	</Tabs.Content>
</Tabs.Root>
