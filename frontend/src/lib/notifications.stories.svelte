<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { notifications, NotificationsToaster } from './notifications.svelte';

	const { Story } = defineMeta({
		title: '$lib/notifications'
	});
</script>

<Story name="local notification">
	<NotificationsToaster closeButton />
	<button
		onclick={() => {
			notifications.send('I am a toast');
		}}>Show toast</button
	>
</Story>
