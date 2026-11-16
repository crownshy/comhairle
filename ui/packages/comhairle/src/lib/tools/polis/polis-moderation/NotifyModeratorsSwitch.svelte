<script lang="ts">
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';
	import Label from '$lib/components/ui/label/label.svelte';
	import Switch from '$lib/components/ui/switch/switch.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { permissions } from '$lib/permissions.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { savePolisSetting, type PolisStepTarget } from '../savePolisSetting';

	let { target }: { target: PolisStepTarget } = $props();

	const canEdit = $derived(
		permissions.can('conversation', 'conversation_update', target.conversationId)
	);

	let notifyModerators = $derived(target.toolConfig.notify_moderators ?? true);
	let saving = $state(false);

	async function save(checked: boolean) {
		if (!canEdit || saving) return;
		saving = true;
		notifyModerators = checked;
		const res = await tryCatchAsync(() =>
			savePolisSetting(target, 'notify_moderators', checked)
		);
		if (res.err !== null) {
			notifyModerators = !checked;
			notifications.send({ priority: 'ERROR', message: 'Failed to update moderator emails' });
		} else {
			await invalidate(key('admin/conversation/workflow'));
		}
		saving = false;
	}
</script>

<div class="flex items-center gap-3">
	<Switch
		id="notifyModerators"
		disabled={!canEdit || saving}
		checked={notifyModerators}
		onCheckedChange={save}
	/>
	<Label for="notifyModerators" class="text-base font-normal">
		Email moderators daily when statements are waiting
	</Label>
</div>
