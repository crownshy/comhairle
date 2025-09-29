<script lang="ts">
	import {
		DateFormatter,
		getLocalTimeZone,
		parseDate,
		today,
		type DateValue
	} from '@internationalized/date';
	import { defaults, superForm } from 'sveltekit-superforms';
	import { emailsFormSchema, splitEmails } from './schema';
	import { notifications } from '$lib/notifications.svelte';
	import { zodClient, zod } from 'sveltekit-superforms/adapters';
	import * as Form from '$lib/components/ui/form/index.js';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import Input from '$lib/components/ui/input/input.svelte';
	import { apiClient } from '$lib/api/client';
	import * as RadioGroup from '$lib/components/ui/radio-group';
	import CalendarIcon from '@lucide/svelte/icons/calendar';
	import { Calendar } from '$lib/components/ui/calendar';
	import { cn } from '$lib/utils';
	import { buttonVariants } from '../button';

	type Props = {
		onDone: () => void;
		conversation_id: string;
	};

	let { onDone, conversation_id }: Props = $props();

	async function sendEmailInvite() {
		const result = await validateForm({ update: true });
		if (!result.valid) {
			return;
		}

		let expireOption = result.data.expiresOption;
		let expireDate = null;
		if (expireOption == '1 day') {
			expireDate = today(getLocalTimeZone()).add({ days: 1 });
		}
		if (expireOption == '1 week') {
			expireDate = today(getLocalTimeZone()).add({ weeks: 1 });
		}
		if (expireOption == '1 month') {
			expireDate = today(getLocalTimeZone()).add({ months: 1 });
		}
		if (expireOption == 'custom' && result.data.customExpire) {
			expireDate = parseDate(result.data.customExpire);
		}

		try {
			await Promise.all(
				splitEmails(result.data.emails).map(async (email) => {
					return await apiClient.CreateInvite(
						{
							invite_type: { email },
							expires_at: expireDate?.toDate(getLocalTimeZone()).toISOString()
						},
						{ params: { conversation_id: conversation_id } }
					);
				})
			);
			notifications.send({ message: 'Emails sent' });
			onDone();
		} catch (e) {
			notifications.send({ priority: 'ERROR', message: 'Failed to send emails' });
			$message = 'Failed to send emails';
		}
	}

	let emailsForm = superForm(defaults(zod(emailsFormSchema)), {
		validators: zodClient(emailsFormSchema),
		taintedMessage: false,
		validationMethod: 'oninput',
		onSubmit: sendEmailInvite
	});

	//TODO update this based in the internationalization context
	const df = new DateFormatter('en-UK', {
		dateStyle: 'long'
	});

	let { form, enhance, validateForm, message, submitting } = emailsForm;

	let customExpire = $derived($form.customExpire ? parseDate($form.customExpire) : undefined);
</script>

<form method="POST" onsubmit={sendEmailInvite} class="mt-10 flex flex-col gap-y-10" use:enhance>
	<Form.Field form={emailsForm} name="emails" class="">
		<Form.Control>
			{#snippet children({ props })}
				<Form.Label>Emails</Form.Label>
				<Input {...props} bind:value={$form.emails} />
			{/snippet}
		</Form.Control>
		<Form.Description class="text-mutted-foreground"
			>Comma separated list of emails to send invites to.</Form.Description
		>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Fieldset form={emailsForm} name="expiresOption" class="space-y-3">
		<Form.Legend>Invite valid for...</Form.Legend>
		<RadioGroup.Root
			bind:value={$form.expiresOption}
			class="flex flex-row space-x-1"
			name="expiresOption"
		>
			<div class="flex items-center space-y-0 space-x-3">
				<Form.Control>
					{#snippet children({ props })}
						<RadioGroup.Item value="never" {...props} />
						<Form.Label class="font-normal">never</Form.Label>
					{/snippet}
				</Form.Control>
				<Form.Control>
					{#snippet children({ props })}
						<RadioGroup.Item value="1 day" {...props} />
						<Form.Label class="font-normal">1 Day</Form.Label>
					{/snippet}
				</Form.Control>
				<Form.Control>
					{#snippet children({ props })}
						<RadioGroup.Item value="1 week" {...props} />
						<Form.Label class="font-normal">1 Week</Form.Label>
					{/snippet}
				</Form.Control>
				<Form.Control>
					{#snippet children({ props })}
						<RadioGroup.Item value="1 month" {...props} />
						<Form.Label class="font-normal">1 Month</Form.Label>
					{/snippet}
				</Form.Control>
				<Form.Control>
					{#snippet children({ props })}
						<RadioGroup.Item value="custom" {...props} />
						<Form.Label class="font-normal">custom</Form.Label>
					{/snippet}
				</Form.Control>
			</div>
		</RadioGroup.Root>
	</Form.Fieldset>

	{#if $form.expiresOption == 'custom'}
		<Form.Field form={emailsForm} name="customExpire" class="flex flex-col">
			<Form.Control>
				{#snippet children({ props })}
					<Popover.Root>
						<Popover.Trigger
							{...props}
							class={cn(
								buttonVariants({ variant: 'secondary' }),
								'mt-5 w-[280px] justify-start pl-4 text-left font-normal',
								!customExpire && 'text-muted-foreground'
							)}
						>
							{customExpire ? df.format(customExpire.toDate(getLocalTimeZone())) : 'Pick a date'}
							<CalendarIcon class="ml-auto size-4 opacity-50" />
						</Popover.Trigger>
						<Popover.Content class=" w-auto p-0" side="top">
							<Calendar
								type="single"
								value={customExpire as DateValue}
								minValue={today(getLocalTimeZone())}
								calendarLabel="Expire Date"
								onValueChange={(v) => {
									if (v) {
										$form.customExpire = v.toString();
									} else {
										$form.customExpire = '';
									}
								}}
							/>
						</Popover.Content>
					</Popover.Root>
					<Form.Description>Select a date on which to expire the invite</Form.Description>
					<Form.FieldErrors />
					<input hidden value={$form.customExpire} name={'customExpire'} />
				{/snippet}
			</Form.Control>
		</Form.Field>
	{/if}

	<Form.Button class="my-5" disabled={$submitting}>Submit</Form.Button>
	{#if message}
		<p>{$message}</p>
	{/if}
</form>
