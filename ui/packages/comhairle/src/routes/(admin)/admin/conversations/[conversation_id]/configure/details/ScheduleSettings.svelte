<script lang="ts">
	import { onDestroy } from 'svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import { invalidate } from '$app/navigation';
	import { apiClient } from '@crownshy/api-client/client';
	import type { ConversationWithTranslations } from '@crownshy/api-client/api';
	import DateTimePicker from '$lib/components/DateTimePicker.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { permissions } from '$lib/permissions.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { key } from '$lib/utils/invalidationKey';
	import { guardUnsavedChanges } from '$lib/utils/unsavedChangesGuard.svelte';
	import FieldLabel from '../FieldLabel.svelte';

	type Props = {
		conversation: ConversationWithTranslations;
	};

	let { conversation }: Props = $props();

	const canEdit = $derived(
		permissions.can('conversation', 'conversation_update', conversation.id)
	);

	let launchDate = $derived(conversation.launchDate ?? null);
	let closeDate = $derived(conversation.closeDate ?? null);

	type ScheduleField = 'launch_date' | 'close_date';

	const FIELD_LABELS: Record<ScheduleField, string> = {
		launch_date: 'Launch date',
		close_date: 'Close date'
	};

	// Saves send data without reloading; the conversation reloads once when the page is left.
	const savesInFlight = new SvelteSet<Promise<void>>();
	let hasSaved = false;
	guardUnsavedChanges(() => savesInFlight.size > 0);
	onDestroy(() => {
		Promise.allSettled(savesInFlight).then(() => {
			if (hasSaved) invalidate(key('admin/conversation'));
		});
	});

	function saveDate(field: ScheduleField, value: string | null, revert: () => void) {
		const save = sendDate(field, value, revert);
		savesInFlight.add(save);
		save.finally(() => savesInFlight.delete(save));
	}

	async function sendDate(field: ScheduleField, value: string | null, revert: () => void) {
		if (!canEdit) return;
		const result = await tryCatchAsync(() =>
			apiClient.UpdateConversation(
				{ [field]: value },
				{ params: { conversation_id: conversation.id } }
			)
		);
		if (result.err !== null) {
			revert();
			notifications.send({
				message: `Failed to update ${FIELD_LABELS[field].toLowerCase()}`,
				priority: 'ERROR'
			});
			return;
		}
		hasSaved = true;
		notifications.send({
			message: `${FIELD_LABELS[field]} ${value ? 'saved' : 'removed'}`,
			priority: 'INFO'
		});
	}

	function setLaunchDate(value: string | null) {
		const previous = launchDate;
		launchDate = value;
		saveDate('launch_date', value, () => (launchDate = previous));
	}

	function setCloseDate(value: string | null) {
		const previous = closeDate;
		closeDate = value;
		saveDate('close_date', value, () => (closeDate = previous));
	}
</script>

<section class="border-border flex flex-col gap-1 border-t pt-6" aria-labelledby="schedule-heading">
	<h3 id="schedule-heading" class="text-lg font-semibold">Schedule</h3>
	<p class="text-muted-foreground text-base">
		When you plan to launch and close the conversation. Setting these will launch / close the
		conversation automatically.
	</p>

	<div class="flex flex-col">
		<div class="flex flex-col gap-4 py-6 lg:flex-row lg:items-center lg:gap-6">
			<FieldLabel
				type="plain"
				label="Launch date"
				info="The date and time you plan to open the conversation to participants."
			/>
			<DateTimePicker
				class="flex-1"
				label="Launch date"
				value={launchDate}
				onValueChange={setLaunchDate}
				bounds={{ before: closeDate }}
				disabled={!canEdit}
			/>
		</div>
		<div class="flex flex-col gap-4 pb-6 lg:flex-row lg:items-center lg:gap-6">
			<FieldLabel
				type="plain"
				label="Close date"
				info="The date and time you plan to stop taking contributions. Must be after the launch date."
			/>
			<DateTimePicker
				class="flex-1"
				label="Close date"
				value={closeDate}
				onValueChange={setCloseDate}
				bounds={{ after: launchDate }}
				disabled={!canEdit}
			/>
		</div>
	</div>
</section>
