<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { notifications, NotificationsToaster } from './notifications.svelte';

	const { Story } = defineMeta({
		title: '$lib/notifications'
	});
</script>

<Story name="default prority notification">
	<NotificationsToaster closeButton />
	<button
		onclick={() => {
			notifications.send({ message: 'I am a toast' });
		}}>Send notification</button
	>
</Story>

<Story name="info prority notification">
	<NotificationsToaster closeButton />
	<button
		onclick={() => {
			notifications.send({ priority: 'INFO', message: 'The sky is blue' });
		}}>Send notification</button
	>
</Story>

<Story name="warning prority notification">
	<NotificationsToaster closeButton />
	<button
		onclick={() => {
			notifications.send({ priority: 'WARNING', message: 'The sky is turning grey' });
		}}>Send notification</button
	>
</Story>

<Story name="error prority notification">
	<NotificationsToaster closeButton />
	<button
		onclick={() => {
			notifications.send({ priority: 'ERROR', message: 'It is raining' });
		}}>Send notification</button
	>
</Story>

<Story name="success prority notification">
	<NotificationsToaster closeButton />
	<button
		onclick={() => {
			notifications.send({ priority: 'SUCCESS', message: 'Your umbrella is up' });
		}}>Send notification</button
	>
</Story>

<Story name="notification with action">
	<NotificationsToaster closeButton />
	<button
		onclick={() => {
			notifications.send({
				message: 'more notify',
				action: {
					label: 'click me',
					onClick: () => {
						notifications.send({ message: 'you can notify from a notification' });
					}
				}
			});
		}}>Send notification</button
	>
</Story>
