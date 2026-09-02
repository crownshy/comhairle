<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { Label } from '$lib/components/ui/label';
	import { apiClient } from '@crownshy/api-client/client';
	import { key } from '$lib/utils/invalidationKey';
	import { notifications } from '$lib/notifications.svelte';
	import FacilitatorRoleList from './FacilitatorRoleList.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';

	const { data, params } = $props();
	const { attendees, pendingInvites } = $derived(data);

	async function handleSetAttendeeRole(attendanceId: string, role: string) {
		const result = await tryCatchAsync(() =>
			apiClient.UpdateEventAttendance(
				{ role },
				{
					params: {
						conversation_id: params.conversation_id,
						event_id: params.event_id,
						attendance_id: attendanceId
					}
				}
			)
		);

		if (result.err !== null) {
			console.error(result.err);
			notifications.send({
				priority: 'ERROR',
				message: 'Failed to update role'
			});
		}

		notifications.send({
			priority: 'INFO',
			message: 'Role updated'
		});

		await invalidate(key('event/facilitators'));
	}
</script>

<div class="border-border flex flex-col gap-4 border-t py-6 lg:flex-row lg:items-start lg:gap-6">
	<div class="contents">
		<Label class="text-sm font-semibold lg:w-50 lg:shrink-0 lg:pt-2">Facilitators</Label>
		<FacilitatorRoleList {attendees} {pendingInvites} onSetRole={handleSetAttendeeRole} />
	</div>
</div>
