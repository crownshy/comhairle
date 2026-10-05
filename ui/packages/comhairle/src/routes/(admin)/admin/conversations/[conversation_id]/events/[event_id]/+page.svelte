<script lang="ts">
	import { page } from '$app/state';
	import * as Form from '$lib/components/ui/form/';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import * as Select from '$lib/components/ui/select';
	import TranslatableField from '$lib/components/Translation/TranslatableField.svelte';
	import { createTextContentSource } from '$lib/components/Translation/translationSource.svelte';
	import Combobox from '$lib/components/ui/combobox/combobox.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import { TimeRangePicker } from '$lib/components/ui/time-picker';
	import { CalendarIcon } from 'lucide-svelte';
	import Calendar from '$lib/components/ui/calendar/calendar.svelte';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import EventSchema from './schema.js';
	import { cn } from '$lib/utils';
	import { buttonVariants } from '$lib/components/ui/button';
	import Button from '$lib/components/ui/button/button.svelte';
	import {
		DateFormatter,
		getLocalTimeZone,
		type DateValue,
		today,
		parseDate,
		parseDateTime,
		toTimeZone,
		toZoned
	} from '@internationalized/date';
	import { notifications } from '$lib/notifications.svelte';
	import { apiClient } from '@crownshy/api-client/client';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';
	import FacilitatorRoleList from './FacilitatorRoleList.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import { utcTimeToLocal } from '$lib/utils/date-time';
	import AgendaEditor from './AgendaEditor.svelte';
	import type { EventAgendaItem } from '@crownshy/api-client/api';
	import type { AgendaItemData } from './agenda-types';
	import { InviteDto } from '@crownshy/api-client/api';
	import CopyButton from '$lib/components/CopyButton.svelte';
	import EmailInvitesList from '$lib/components/ui/email-invites/EmailInvitesList.svelte';
	import EmailInviteForm from '$lib/components/ui/email-invites/EmailInviteForm.svelte';
	import { inviteUrl } from '$lib/utils/invites.js';
	import EventLocationForm from './EventLocationForm.svelte';
	import EventRecordings from './EventRecordings.svelte';
	import EventBreakoutRooms from './EventBreakoutRooms.svelte';
	import { snakeToStartCase } from '$lib/utils/casingUtils.js';
	import type { Locale } from '$lib/paraglide/runtime.js';
	import { permissions } from '$lib/permissions.svelte';

	let url = $derived(page.url);
	let { data } = $props();

	let activeTab = $derived(page.url.searchParams.get('subtab') ?? 'details');

	const event = $derived(data.event);
	const conversation = $derived(data.conversation);
	const canEdit = $derived(
		permissions.can('conversation', 'conversation_update', conversation.id)
	);
	const attendees = $derived(data.attendees);
	const recordings = $derived(data.recordings);

	let emailInvites = $derived(
		data.invites.filter(
			(invite) =>
				typeof invite.inviteType !== 'string' &&
				'email' in invite.inviteType &&
				invite.inviteType.email
		)
	);

	// Shown but not selectable in the facilitators tab: a role needs a registration to attach to.
	let pendingInvites = $derived(
		emailInvites
			.filter((invite) => invite.status === 'pending' || invite.status === 'open')
			.map((invite) => ({
				id: invite.id,
				email:
					typeof invite.inviteType !== 'string' && 'email' in invite.inviteType
						? invite.inviteType.email
						: '',
				status: invite.status
			}))
	);
	let primaryLanguage = $derived<Locale>((data.conversation.primaryLocale as Locale) ?? 'en');
	let supportedLanguages = $derived<Locale[]>(
		(data.conversation.supportedLanguages as Locale[]) ?? ['en']
	);

	const timeZone = getLocalTimeZone();
	const [startDate] = $derived(event.startTime.split('T'));
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
			start_date: startDate,
			start_time: utcTimeToLocal(event.startTime, timeZone),
			end_time: utcTimeToLocal(event.endTime, timeZone),
			signup_mode: event.signupMode,
			format: event.format,
			custom_event_link: event.customEventLink ?? ''
		},
		{
			validators: zodClient(EventSchema),
			taintedMessage: false,
			validationMethod: 'oninput',
			onSubmit: handleUpdateEvent
		}
	);

	let { form, enhance, validateForm, submitting, tainted } = $derived(eventForm);

	// `onEdit` mirrors the primary value into `$form` so superForm's inline validation still runs (ADR-0005).
	const nameSource = createTextContentSource({
		getTranslation: () => event.translations?.name,
		getPrimaryLocale: () => primaryLanguage,
		getSupportedLanguages: () => supportedLanguages,
		getPrimaryFallback: () => $form.name ?? '',
		onEdit: (content) => ($form.name = content),
		refresh: () => invalidate(key('admin/conversation/events'))
	});
	const descriptionSource = createTextContentSource({
		getTranslation: () => event.translations?.description,
		getPrimaryLocale: () => primaryLanguage,
		getSupportedLanguages: () => supportedLanguages,
		getPrimaryFallback: () => $form.description ?? '',
		onEdit: (content) => ($form.description = content),
		refresh: () => invalidate(key('admin/event'))
	});

	function convertTimeToSelectedZone(date: string, time: string, timeZone: string) {
		const localTimeZone = Intl.DateTimeFormat().resolvedOptions().timeZone;
		const dateString = parseDateTime(`${date}T${time}`);
		const toLocalZoned = toZoned(dateString, localTimeZone);

		const zonedDateTime = toTimeZone(toLocalZoned, timeZone);
		const zonedTime = new Intl.DateTimeFormat('en', {
			hour: 'numeric',
			minute: '2-digit',
			hour12: true,
			timeZone: zonedDateTime.timeZone
		}).format(zonedDateTime.toDate());

		return zonedTime;
	}

	let saving = $state(false);

	async function handleUpdateEvent({ cancel }: { cancel: () => void }) {
		// The page route has no server actions, so a SvelteKit POST would 405. Save via the API client.
		cancel();

		if (!canEdit || saving) return;

		const result = await validateForm({ update: true });

		if (!result.valid) return;

		saving = true;

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
				custom_event_link: eventData.custom_event_link?.trim() || '',
				start_time: startTime.toDate(getLocalTimeZone()).toISOString(),
				end_time: endTime.toDate(getLocalTimeZone()).toISOString()
			};

			await apiClient.UpdateEvent(eventParams, {
				params: {
					conversation_id: conversation.id,
					event_id: event.id
				}
			});

			await invalidate(key('admin/event'));
			notifications.send({ message: 'Updated event', priority: 'INFO' });
		} catch (e) {
			console.error(e);
			notifications.send({
				message: 'Something went wrong updating the event',
				priority: 'ERROR'
			});
		} finally {
			saving = false;
		}
	}

	const df = new DateFormatter('en-UK', {
		dateStyle: 'long'
	});

	let eventDate = $derived($form.start_date ? parseDate($form.start_date) : undefined);
	let pageTitle = $derived(`${canEdit ? 'Edit Event' : 'Event'}: ${event.name}`);

	/** Map API agenda items to editor format */
	function apiAgendaToEditor(items: EventAgendaItem[]): AgendaItemData[] {
		return items.map((item) => {
			if ('Basic' in item) {
				return {
					id: crypto.randomUUID(),
					type: 'standard',
					title: item.Basic.title
				};
			} else {
				return {
					id: crypto.randomUUID(),
					type: 'breakout',
					title: '',
					duration: item.BreakoutRoom.estimated_time,
					groupSize: item.BreakoutRoom.max_per_room ?? 4,
					prompts: [
						{
							title: item.BreakoutRoom.prompt,
							instructions: item.BreakoutRoom.instructions
						}
					],
					assignmentMode: 'random',
					balanceBy: []
				};
			}
		});
	}

	/** Map editor format back to API agenda items */
	function editorAgendaToApi(items: AgendaItemData[]): EventAgendaItem[] {
		return items.map((item) => {
			if (item.type === 'standard') {
				return {
					Basic: {
						title: item.title || '',
						description: '',
						estimated_time: 0
					}
				};
			} else {
				const firstPrompt = item.prompts?.[0];
				return {
					BreakoutRoom: {
						prompt: firstPrompt?.title || '',
						instructions: firstPrompt?.instructions || '',
						estimated_time: item.duration ?? 10,
						time_limit: item.duration ? item.duration * 60 : null,
						max_per_room: item.groupSize ?? null
					}
				};
			}
		});
	}

	let agendaItems = $state<AgendaItemData[]>(apiAgendaToEditor(event.agenda ?? []));
	let agendaDirty = $state(false);
	let agendaSaving = $state(false);

	function handleAgendaUpdate(items: AgendaItemData[]) {
		if (!canEdit) return;
		agendaItems = items;
		agendaDirty = true;
	}

	async function handleSaveAgenda() {
		if (!canEdit || agendaSaving) return;
		agendaSaving = true;
		try {
			await apiClient.UpdateEvent(
				{ agenda: editorAgendaToApi(agendaItems) },
				{
					params: {
						conversation_id: conversation.id,
						event_id: event.id
					}
				}
			);
			await invalidate(key('admin/event'));
			agendaDirty = false;
			notifications.send({ message: 'Agenda saved', priority: 'INFO' });
		} catch (e) {
			console.error(e);
			notifications.send({ message: 'Failed to save agenda', priority: 'ERROR' });
		} finally {
			agendaSaving = false;
		}
	}

	async function handleSetAttendeeRole(attendanceId: string, role: string) {
		if (!canEdit) return;
		try {
			await apiClient.UpdateEventAttendance(
				{ role },
				{
					params: {
						conversation_id: conversation.id,
						event_id: event.id,
						attendance_id: attendanceId
					}
				}
			);

			notifications.send({
				priority: 'INFO',
				message: 'Role updated'
			});

			await invalidate(key('admin/event'));
		} catch (e) {
			console.error(e);
			notifications.send({
				priority: 'ERROR',
				message: 'Failed to update role'
			});
		}
	}

	async function emailInvitesSubmitted() {
		await invalidate(key('admin/event'));
	}
</script>

<svelte:head>
	<title>{pageTitle} - Comhairle Admin</title>
</svelte:head>

<div class="mb-6 flex flex-row items-center gap-4">
	<h1 class="text-3xl font-bold">Event: {event?.name}</h1>
	{#if conversation && event}
		<Button href={`/conversations/${conversation.id}/events/${event.id}`}>Event Link</Button>
	{/if}
</div>

{#if activeTab === 'details'}
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
								disabled={!canEdit}
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
								source={descriptionSource}
								disabled={!canEdit}
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
						<Input
							{...props}
							bind:value={$form.capacity}
							type="number"
							disabled={!canEdit}
						/>
						<Form.FieldErrors />
					{/snippet}
				</Form.Control>
			</Form.Field>
		</div>

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
						{#if canEdit}
						<Combobox
							selectedItem={availableTimeZones.find(
								(tz) => tz.value === $form.default_time_zone
							)}
							items={availableTimeZones}
							placeholder="Select a default timezone"
							onSelect={(item) => ($form.default_time_zone = item.value)}
						/>
						{:else}
							<Input
								value={$form.default_time_zone}
								disabled
								aria-label="Default time zone"
							/>
						{/if}
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
									disabled={!canEdit}
									class={cn(
										buttonVariants({ variant: 'outline' }),
										'w-full max-w-xs justify-start pl-4 text-left font-normal',
										!eventDate && 'text-muted-foreground'
									)}
								>
									{eventDate
										? df.format(eventDate.toDate(getLocalTimeZone()))
										: 'Pick a date'}
									<CalendarIcon class="ml-auto size-4 opacity-50" />
								</Popover.Trigger>
								<Popover.Content class="w-auto p-0" side="bottom" align="start">
									<Calendar
										type="single"
										disabled={!canEdit}
										value={eventDate as DateValue}
										minValue={today(getLocalTimeZone())}
										calendarLabel="Event Date"
										onValueChange={(v) => {
											if (!canEdit) return;
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
				{#if canEdit}
					<TimeRangePicker
						startName="start_time"
						endName="end_time"
						bind:startValue={$form.start_time}
						bind:endValue={$form.end_time}
					/>
				{:else}
					<div class="flex gap-2">
						<Input value={$form.start_time} disabled aria-label="Start time" />
						<Input value={$form.end_time} disabled aria-label="End time" />
					</div>
				{/if}
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
						disabled={!canEdit}
						value={$form.format}
						onValueChange={(value: string) => ($form.format = value)}
					>
						<Select.Trigger class="w-45"
							>Format: {snakeToStartCase($form.format)}</Select.Trigger
						>
						<Select.Content>
							<Select.Item value="online">Online</Select.Item>
							<Select.Item value="in_person">In-person</Select.Item>
						</Select.Content>
					</Select.Root>
				</Form.Control>
			</Form.Field>
		</div>

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
								disabled={!canEdit || $form.format !== 'online'}
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
					disabled={!canEdit}
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

		{#if canEdit}
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
		{/if}
	</form>
{:else if activeTab === 'structure'}
	<div class="flex flex-col gap-10 py-6">
		<div class="flex flex-col gap-2">
			<h2 class="text-3xl font-bold">
				Event structure <span class="font-bold">(for facilitator)</span>
			</h2>
			<p class="text-muted-foreground text-base">Plan how your meeting will run</p>
		</div>

		<AgendaEditor bind:items={agendaItems} onUpdate={handleAgendaUpdate} editable={canEdit} />

		{#if canEdit}
			<div class="border-border flex justify-center border-t py-6">
				<Button
					variant="default"
					class="px-12"
					disabled={!canEdit || !agendaDirty || agendaSaving}
					onclick={handleSaveAgenda}
				>
					{agendaSaving ? 'Saving...' : 'Save Agenda'}
				</Button>
			</div>
		{/if}
	</div>
{:else if activeTab === 'facilitators'}
	<div
		class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6"
	>
		<div class="contents">
			<Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">Facilitators</Label>
			<FacilitatorRoleList
				{attendees}
				{pendingInvites}
				onSetRole={handleSetAttendeeRole}
				editable={canEdit}
			/>
		</div>
	</div>
{:else if activeTab === 'location'}
	<div class="flex flex-col gap-10 py-6">
		<div class="flex flex-col gap-2">
			<h2 class="text-3xl font-bold">
				Location <span class="font-bold">(for in-person events)</span>
			</h2>
		</div>

		<EventLocationForm {event} />
	</div>
{:else if activeTab === 'invites'}
	<div class="border-border flex flex-col gap-4 border-t py-6 lg:gap-6">
		<Label class="text-sm font-semibold lg:shrink-0 lg:pt-2">Email invites</Label>
		{#if canEdit}
			<EmailInviteForm
				editable={canEdit}
				conversationId={conversation.id}
				eventId={event.id}
				onDone={emailInvitesSubmitted}
			/>
		{/if}
		<EmailInvitesList {emailInvites} inviteLink={InviteLink} />
	</div>
{:else if activeTab === 'breakout'}
	<EventBreakoutRooms
		conversation_id={conversation.id}
		event_id={event.id}
		{attendees}
		editable={canEdit}
	/>
{:else if activeTab === 'recordings'}
	<EventRecordings
		conversation_id={conversation.id}
		event_id={event.id}
		{recordings}
		editable={canEdit}
	/>
{/if}

{#snippet InviteLink(invite: InviteDto, label: string)}
	<div class="flex flex-row gap-x-2">
		<CopyButton copyText={inviteUrl(url, invite, conversation, event)}>{label}</CopyButton>
	</div>
{/snippet}
