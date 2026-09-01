<script lang="ts">
	import { utcTimeToLocal } from '$lib/utils/date-time';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import * as Form from '$lib/components/ui/form/';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import * as Select from '$lib/components/ui/select';
	import Combobox from '$lib/components/ui/combobox/combobox.svelte';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import { TimeRangePicker } from '$lib/components/ui/time-picker';
	import EventDetailsSchema from './schema';
	import { createTextContentSource } from '$lib/components/Translation/translationSource.svelte';
	import type { Locale } from '$lib/paraglide/runtime';
	import { DEFAULT_LOCALE } from '$lib/utils/constants';
	import { snakeToSentenceCase } from '$lib/utils/casingUtils';
	import { guardUnsavedChanges } from '$lib/utils/unsavedChangesGuard.svelte';
	import { cn } from '$lib/utils';
	import { buttonVariants } from '$lib/components/ui/button';
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
	import { hasUnsavedChanges } from '$lib/components/Translation/translationUtils';
	import { Calendar as CalendarIcon } from '@lucide/svelte';
	import { Calendar } from '$lib/components/ui/calendar';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { key } from '$lib/utils/invalidationKey';
	import { invalidate } from '$app/navigation';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { omit } from '$lib/utils/objects';
	import { EventFormat } from '@crownshy/api-client/api';

	let { data } = $props();
	const { event, conversation } = $derived(data);

	const timeZone = getLocalTimeZone();
	const availableTimeZones = Intl.supportedValuesOf('timeZone').map((tz) => ({
		value: tz,
		label: tz
	}));

	const eventForm = superForm(
		{
			name: event.name,
			description: event.description,
			capacity: event.capacity,
			default_time_zone: event.defaultTimeZone,
			start_date: event.startTime.split('T')[0],
			start_time: utcTimeToLocal(event.startTime, timeZone),
			end_time: utcTimeToLocal(event.endTime, timeZone),
			format: event.format,
			custom_event_link: event.customEventLink ?? '',
			signup_mode: event.signupMode
		},
		{
			validators: zodClient(EventDetailsSchema),
			taintedMessage: false,
			validationMethod: 'oninput',
			onSubmit: handleUpdateEvent
		}
	);

	let { form, enhance, validateForm, submitting, tainted } = $derived(eventForm);

	let primaryLocale = $derived<Locale>(
		(data.conversation.primaryLocale as Locale) ?? DEFAULT_LOCALE
	);
	let supportedLocales = $derived<Locale[]>(
		(data.conversation.supportedLanguages as Locale[]) ?? [DEFAULT_LOCALE]
	);

	let eventDate = $derived($form.start_date ? parseDate($form.start_date) : undefined);

	const dateFormatter = new DateFormatter('en-UK', {
		dateStyle: 'long'
	});

	let saving = $state(false);

	const nameSource = createTextContentSource({
		getTranslation: () => event.translations?.name,
		getPrimaryLocale: () => primaryLocale,
		getSupportedLanguages: () => supportedLocales,
		getPrimaryFallback: () => $form.name ?? '',
		onEdit: (content) => ($form.name = content)
	});
	const descriptionSource = createTextContentSource({
		getTranslation: () => event.translations?.description,
		getPrimaryLocale: () => primaryLocale,
		getSupportedLanguages: () => supportedLocales,
		getPrimaryFallback: () => $form.description ?? '',
		onEdit: (content) => ($form.description = content)
	});

	// Warn on refresh / navigate-away while a field is still autosaving.
	guardUnsavedChanges(() => [nameSource, descriptionSource].some(hasUnsavedChanges));

	async function handleUpdateEvent({ cancel }: { cancel: () => void }) {
		// We submit via the API client below — prevent SvelteKit from POSTing the form to the
		// page route (which has no server actions and would return 405 Method Not Allowed).
		cancel();

		if (saving) return;

		const result = await validateForm({ update: true });

		if (!result.valid) return;

		saving = true;

		const dateOption = result.data.start_date;
		let startTime = parseDateTime(`${dateOption}T${result.data.start_time}`);
		let endTime = parseDateTime(`${dateOption}T${result.data.end_time}`);

		const eventData = omit(result.data, 'name', 'description');

		const eventParams = {
			...eventData,
			custom_event_link: eventData.custom_event_link?.trim() || '',
			start_time: startTime.toDate(getLocalTimeZone()).toISOString(),
			end_time: endTime.toDate(getLocalTimeZone()).toISOString()
		};

		const updateEvent = await tryCatchAsync(() =>
			apiClient.UpdateEvent(eventParams, {
				params: {
					conversation_id: conversation.id,
					event_id: event.id
				}
			})
		);

		saving = false;

		if (updateEvent.err !== null) {
			console.error(updateEvent.err);
			notifications.send({
				message: 'Something went wrong updating the event',
				priority: 'ERROR'
			});
			return;
		}

		await invalidate(key('conversation/event'));
		notifications.send({ message: 'Updated event', priority: 'INFO' });
	}

	function convertTimeToSelectedZone(date: string, time: string, timeZone: string) {
		const localTimeZone = Intl.DateTimeFormat().resolvedOptions().timeZone;
		const dateString = parseDateTime(`${date}T${time}`);
		const toLocalZoned = toZoned(dateString, localTimeZone);

		const zonedDateTime = toTimeZone(toLocalZoned, timeZone);
		const zonedTime = new Intl.DateTimeFormat(DEFAULT_LOCALE, {
			hour: 'numeric',
			minute: '2-digit',
			hour12: true,
			timeZone: zonedDateTime.timeZone
		}).format(zonedDateTime.toDate());

		return zonedTime;
	}
</script>

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
							source={nameSource}
							{primaryLocale}
							supportedLanguages={supportedLocales}
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
							source={descriptionSource}
							{primaryLocale}
							supportedLanguages={supportedLocales}
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

	<!-- Default time zone -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={eventForm} name="default_time_zone" class="contents">
			<Form.Control>
				<Form.Label
					class="flex flex-col items-start text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
				>
					<span>Default time zone</span>
					<span class="font-normal">Time zone event is taking place in</span>
				</Form.Label>
				<div class="flex-1">
					<Combobox
						selectedItem={availableTimeZones.find(
							(tz) => tz.value === $form.default_time_zone
						)}
						items={availableTimeZones}
						placeholder="Select a default timezone"
						onSelect={(item) => ($form.default_time_zone = item.value)}
					/>
				</div>
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
					<div class="flex-1">
						<Popover.Root>
							<Popover.Trigger
								{...props}
								class={cn(
									buttonVariants({ variant: 'outline' }),
									'w-full max-w-xs justify-start pl-4 text-left font-normal',
									!eventDate && 'text-muted-foreground'
								)}
							>
								{eventDate
									? dateFormatter.format(eventDate.toDate(getLocalTimeZone()))
									: 'Pick a date'}
								<CalendarIcon class="ml-auto size-4 opacity-50" />
							</Popover.Trigger>
							<Popover.Content class="w-auto p-0" side="bottom" align="start">
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
					</div>
				{/snippet}
			</Form.Control>
		</Form.Field>
	</div>

	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<p class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">Time</p>
		<div class="flex flex-1 flex-col gap-2 2xl:flex-row 2xl:items-center">
			<TimeRangePicker
				startName="start_time"
				endName="end_time"
				bind:startValue={$form.start_time}
				bind:endValue={$form.end_time}
			/>
			<Form.Field form={eventForm} name="start_time" class="contents">
				<Form.FieldErrors class="text-destructive text-sm" />
			</Form.Field>
			<Form.Field form={eventForm} name="end_time" class="contents">
				<Form.FieldErrors class="text-destructive text-sm" />
			</Form.Field>
			{#if $form.default_time_zone !== 'Europe/London'}
				<div class="text-muted-foreground flex gap-2">
					<span>{$form.default_time_zone}</span>
					<span>-</span>
					<span
						>{convertTimeToSelectedZone(
							$form.start_date,
							$form.start_time,
							$form.default_time_zone
						)}</span
					>
					<span>-</span>
					<span
						>{convertTimeToSelectedZone(
							$form.start_date,
							$form.end_time,
							$form.default_time_zone
						)}</span
					>
				</div>
			{/if}
		</div>
	</div>

	<!-- Format -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={eventForm} name="format" class="contents">
			<Form.Control>
				<Form.Label
					class="flex flex-col items-start text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
				>
					<span>Format</span>
				</Form.Label>
				<Select.Root
					type="single"
					value={$form.format}
					onValueChange={(value) => ($form.format = value as EventFormat)}
				>
					<Select.Trigger class="w-45"
						>Format: {snakeToSentenceCase($form.format)}</Select.Trigger
					>
					<Select.Content>
						<Select.Item value="online">Online</Select.Item>
						<Select.Item value="in_person">In-person</Select.Item>
					</Select.Content>
				</Select.Root>
			</Form.Control>
		</Form.Field>
	</div>

	<!-- Custom Event Link -->
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<Form.Field form={eventForm} name="custom_event_link" class="contents">
			<Form.Control>
				{#snippet children({ props })}
					<Form.Label
						class="flex flex-col items-start text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2"
					>
						<span>Custom event link</span>
						<span class="font-normal">Override default Jitsi meeting link</span>
					</Form.Label>
					<div class="flex-1">
						<Input
							{...props}
							bind:value={$form.custom_event_link}
							placeholder={`/conversations/${conversation.id}/events/${event.id}/live`}
							disabled={$form.format !== 'online'}
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
			disabled={saving || $submitting || !$tainted}
		>
			Save Changes
		</Form.Button>
	</div>
</form>
