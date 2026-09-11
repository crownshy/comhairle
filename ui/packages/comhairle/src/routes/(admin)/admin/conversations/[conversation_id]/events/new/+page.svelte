<script lang="ts">
	import {
		DateFormatter,
		getLocalTimeZone,
		parseDate,
		parseDateTime,
		today,
		toTimeZone,
		toZoned,
		type DateValue
	} from '@internationalized/date';
	import * as Form from '$lib/components/ui/form';
	import { Input } from '$lib/components/ui/input';
	import { TimeRangePicker } from '$lib/components/ui/time-picker';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import Combobox from '$lib/components/ui/combobox/combobox.svelte';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import NewEventSchema from './NewEventSchema';
	import { CalendarIcon, AlertCircle } from 'lucide-svelte';
	import Calendar from '$lib/components/ui/calendar/calendar.svelte';
	import { cn } from '$lib/utils';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { goto } from '$app/navigation';
	import { basic_learn_config } from '$lib/workflow_templates';
	import BadgeInput from '$lib/components/ui/badge-input/badge-input.svelte';
	import * as Alert from '$lib/components/ui/alert';
	import { key } from '$lib/utils/invalidationKey';
	import TabContent from '../../TabContent.svelte';

	let { data } = $props();
	let { form: formDefaults, conversation, user } = data;

	if (user?.email && !(formDefaults.data.facilitators ?? []).includes(user.email)) {
		formDefaults.data.facilitators = [user.email, ...(formDefaults.data.facilitators ?? [])];
	}
	if (!formDefaults.data.default_time_zone) {
		formDefaults.data.default_time_zone = 'Europe/London';
	}

	const availableTimeZones = Intl.supportedValuesOf('timeZone').map((tz) => ({
		value: tz,
		label: tz
	}));

	const form = superForm(formDefaults, {
		validators: zodClient(NewEventSchema),
		onSubmit: handleSubmit
	});

	function convertTimeToSelectedZone(date: string, time: string, timeZone: string) {
		if (!date || !time || !timeZone) return '';
		const localTimeZone = Intl.DateTimeFormat().resolvedOptions().timeZone;
		const dateString = parseDateTime(`${date}T${time}`);
		const toLocalZoned = toZoned(dateString, localTimeZone);
		const zonedDateTime = toTimeZone(toLocalZoned, timeZone);
		return new Intl.DateTimeFormat('en', {
			hour: 'numeric',
			minute: '2-digit',
			hour12: true,
			timeZone: zonedDateTime.timeZone
		}).format(zonedDateTime.toDate());
	}

	const { form: formData, enhance, message: errorMessage, validateForm, submitting } = form;

	let saving = $state(false);
	let submitError = $state<string | null>(null);

	const df = new DateFormatter('en-UK', {
		dateStyle: 'long'
	});

	async function handleSubmit({ cancel }: { cancel: () => void }) {
		// Submission is fully client-side (apiClient + goto). Cancel the SvelteKit POST so it
		// doesn't hit the page route, which has no server action (would 405).
		cancel();

		if (saving) return;

		const result = await validateForm({ update: true });

		if (!result.valid) {
			submitError = 'Please fix the errors below before saving.';
			return;
		}

		submitError = null;
		saving = true;

		const dateOption = result.data.start_date;
		let startTime = parseDateTime(`${dateOption}T${result.data.start_time}`);
		// TODO: can we always assume end date is the same as the start date?
		let endTime = parseDateTime(`${dateOption}T${result.data.end_time}`);
		try {
			const { facilitators: formFacilitators, ...eventData } = result.data;
			const facilitators =
				user?.email && !formFacilitators.includes(user.email)
					? [user.email, ...formFacilitators]
					: formFacilitators;
			const eventParams = {
				...eventData,
				start_time: startTime.toDate(getLocalTimeZone()).toISOString(),
				end_time: endTime.toDate(getLocalTimeZone()).toISOString()
			};
			let event = await apiClient.CreateEvent(eventParams, {
				params: { conversation_id: conversation.id }
			});

			const facilitatorResults = await Promise.allSettled(
				facilitators.map((email) =>
					apiClient.CreateFacilitatorEventAttendance(
						{ email },
						{ params: { conversation_id: conversation.id, event_id: event.id } }
					)
				)
			);
			const failedFacilitators = facilitatorResults
				.map((r, i) => ({ r, email: facilitators[i] }))
				.filter(({ r }) => r.status === 'rejected')
				.map(({ email }) => email);
			if (failedFacilitators.length === facilitators.length) {
				const msg = `Could not add facilatators (${failedFacilitators.join(', ')}). They may not be registered users. Event was not created.`;
				submitError = msg;
				notifications.send({ priority: 'ERROR', message: msg });
				return;
			}
			if (failedFacilitators.length > 0) {
				notifications.send({
					priority: 'WARNING',
					message: `Could not add facilitator(s): ${failedFacilitators.join(', ')}`
				});
			}

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

			goto(`/admin/conversations/${conversation.id}/events`, {
				invalidate: [key('conversation')]
			});
		} catch (e) {
			console.error(e);
			submitError = 'Something went wrong creating the event.';
			notifications.send({
				message: 'Something went wrong creating the event',
				priority: 'ERROR'
			});
		} finally {
			saving = false;
		}
	}

	let startDate = $derived($formData.start_date ? parseDate($formData.start_date) : undefined);
</script>

<svelte:head>
	<title>Create New Event - Comhairle Admin</title>
</svelte:head>

<TabContent>
	<h1 class="mb-4 text-3xl font-bold">New Event</h1>

	<div class="flex flex-col gap-4">
		<h2 class="text-card-foreground text-base font-semibold">Edit information</h2>
	</div>

	{#if submitError || $errorMessage}
		<Alert.Root variant="destructive" class="mt-4">
			<AlertCircle class="size-4" />
			<Alert.Title>Could not create event</Alert.Title>
			<Alert.Description>{submitError ?? $errorMessage}</Alert.Description>
		</Alert.Root>
	{/if}

	<form method="POST" class="mt-8 flex flex-col" use:enhance>
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
							<Input
								type="number"
								{...props}
								placeholder="Capacity"
								value={$formData.capacity ? $formData.capacity : ''}
								oninput={(e) => {
									const n = (e.currentTarget as HTMLInputElement).valueAsNumber;
									$formData.capacity = Number.isNaN(n) ? 0 : n;
								}}
							/>
							<Form.FieldErrors />
						</div>
					{/snippet}
				</Form.Control>
			</Form.Field>
		</div>

		<!-- Default time zone -->
		<div
			class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
		>
			<Form.Field {form} name="default_time_zone" class="contents">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label
							class="flex flex-col items-start text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
						>
							<span>Default time zone</span>
							<span class="font-normal">Time zone event is taking place in</span>
						</Form.Label>
						<div class="flex-1">
							<Combobox
								selectedItem={availableTimeZones.find(
									(tz) => tz.value === $formData.default_time_zone
								)}
								items={availableTimeZones}
								placeholder="Select a default timezone"
								onSelect={(item) => ($formData.default_time_zone = item.value)}
							/>
							<input
								hidden
								{...props}
								value={$formData.default_time_zone}
								name="default_time_zone"
							/>
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
										'bg-background border-input selection:bg-primary dark:bg-input/30 selection:text-primary-background ring-offset-background placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive flex h-9 w-full min-w-0 items-center rounded-lg border px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 md:text-sm',
										'max-w-xs justify-start pl-4 text-left font-normal',
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

		<!-- Time (Start to End range) -->
		<div
			class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
		>
			<p class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">Time</p>
			<div class="flex flex-1 flex-col gap-2 2xl:flex-row 2xl:items-center">
				<TimeRangePicker
					startName="start_time"
					endName="end_time"
					class={cn(
						'dark:bg-input/30',
						!$formData.start_time && !$formData.end_time && 'text-muted-foreground'
					)}
					bind:startValue={$formData.start_time}
					bind:endValue={$formData.end_time}
				/>
				<Form.Field {form} name="start_time" class="contents">
					<Form.FieldErrors class="text-destructive text-sm" />
				</Form.Field>
				<Form.Field {form} name="end_time" class="contents">
					<Form.FieldErrors class="text-destructive text-sm" />
				</Form.Field>
				{#if $formData.default_time_zone && $formData.default_time_zone !== 'Europe/London' && $formData.start_date && $formData.start_time && $formData.end_time}
					<div class="text-muted-foreground flex gap-2">
						<span>{$formData.default_time_zone}</span>
						<span>-</span>
						<span
							>{convertTimeToSelectedZone(
								$formData.start_date,
								$formData.start_time,
								$formData.default_time_zone
							)}</span
						>
						<span>-</span>
						<span
							>{convertTimeToSelectedZone(
								$formData.start_date,
								$formData.end_time,
								$formData.default_time_zone
							)}</span
						>
					</div>
				{/if}
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

		<!-- Facilitators -->
		<div
			class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
		>
			<Form.Field {form} name="facilitators" class="contents">
				<Form.Control>
					{#snippet children({ props })}
						<Form.Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
							>Facilitators</Form.Label
						>
						<div class="flex-1">
							<BadgeInput
								{...props}
								type="email"
								placeholder="facilitator@example.com"
								class="gap-3!"
								badges={($formData.facilitators ?? []).map((email, i) => ({
									id: String(i),
									value: email
								}))}
								onAddBadge={(value) => {
									const v = value.trim();
									if (!v) return;
									const list = $formData.facilitators ?? [];
									if (list.includes(v)) return;
									$formData.facilitators = [...list, v];
								}}
								onDeleteBadge={(id) => {
									const list = $formData.facilitators ?? [];
									$formData.facilitators = list.filter(
										(_, i) => String(i) !== id
									);
								}}
							/>
							<Form.FieldErrors />
						</div>
					{/snippet}
				</Form.Control>
			</Form.Field>
		</div>

		<!-- Save Button -->
		<div class="border-border flex flex-col items-center gap-3 border-t py-6">
			{#if submitError}
				<p class="text-destructive text-sm" role="alert">{submitError}</p>
			{/if}
			<Form.Button variant="default" class="px-12" disabled={saving || $submitting}>
				Save changes
			</Form.Button>
		</div>
	</form>
</TabContent>
