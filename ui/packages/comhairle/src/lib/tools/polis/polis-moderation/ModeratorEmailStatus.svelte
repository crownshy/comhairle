<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { notifications } from '$lib/notifications.svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import Bell from '@lucide/svelte/icons/bell';
	import BellOff from '@lucide/svelte/icons/bell-off';
	import { savePolisSetting, type PolisStepTarget } from '../savePolisSetting';

	type Props = {
		step: PolisStepTarget;
		/** Shows the Turn on / Turn off control. Everyone else just reads the sentence. */
		canEdit: boolean;
	};

	let { step, canEdit }: Props = $props();

	let notifyModerators = $derived(step.toolConfig.notify_moderators ?? true);
	let saving = $state(false);

	async function toggle() {
		if (saving) return;
		const next = !notifyModerators;
		saving = true;
		notifyModerators = next;
		const res = await tryCatchAsync(() => savePolisSetting(step, 'notify_moderators', next));
		if (res.err !== null) {
			notifyModerators = !next;
			notifications.send({ priority: 'ERROR', message: 'Failed to update moderator emails' });
		}
		saving = false;
	}
</script>

<p class="text-muted-foreground flex flex-wrap items-center gap-x-2 gap-y-1 text-base">
	{#if notifyModerators}
		<Bell class="size-4 shrink-0" />
		Moderators get an email at midday when statements are waiting.
	{:else}
		<BellOff class="size-4 shrink-0" />
		Moderators aren’t emailed about waiting statements.
	{/if}
	{#if canEdit}
		<Button variant="link" disabled={saving} onclick={toggle} class="h-auto p-0 text-base">
			{notifyModerators ? 'Turn off' : 'Turn on'}
		</Button>
	{/if}
</p>
