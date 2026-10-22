<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import EmailTypeGroup from '../../routes/(admin)/admin/email-template-configs/EmailTypeGroup.svelte';

	const { Story } = defineMeta({
		title: 'Admin/Email templates/EmailTypeGroup',
		component: EmailTypeGroup,
		tags: ['autodocs']
	});

	const inviteWithOverrides = {
		emailType: 'conversation_invite',
		rows: [
			{
				key: 'all',
				source: 'all',
				subject: 'Invitation to take part in a public consultation',
				configId: '00000000-0000-0000-0000-000000000001',
				createdAt: '2026-10-21T10:00:00Z'
			},
			{
				key: 'c1',
				source: 'conversation',
				subject: 'Have your say on the town centre',
				configId: '00000000-0000-0000-0000-000000000002',
				createdAt: '2026-10-21T11:00:00Z',
				conversationId: 'c1',
				conversationTitle: 'Town Centre Regeneration'
			}
		]
	};

	const reminderDefault = {
		emailType: 'event_reminder',
		rows: [
			{ key: 'event_reminder-default', source: 'default', subject: 'Upcoming event reminder' }
		]
	};
</script>

<Story name="Invite with per-conversation versions" args={{ group: inviteWithOverrides }} />
<Story name="Not customised" args={{ group: reminderDefault }} />
<Story
	name="Narrowed to one conversation"
	args={{
		group: { ...inviteWithOverrides, rows: [inviteWithOverrides.rows[1]] },
		showAddForConversation: false
	}}
/>
