<script module>
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import EmailTypeTab from '../../routes/(admin)/admin/email-template-configs/EmailTypeTab.svelte';

	const { Story } = defineMeta({
		title: 'Admin/Email templates/EmailTypeTab',
		component: EmailTypeTab,
		tags: ['autodocs']
	});

	const schema = {
		email_type: 'conversation_invite',
		default_subject: 'Invitation to take part in a public consultation',
		template: 'conversation_invite.html',
		variables: ['conversation_title'],
		slots: []
	};

	const base = {
		key: 'all',
		source: 'all',
		subject: 'Invitation to take part in a public consultation',
		configId: '00000000-0000-0000-0000-000000000001',
		createdAt: '2026-10-21T10:00:00Z'
	};

	const longTitle =
		'Renewable energy in rural areas: community views on wind, solar and local ownership across the Highlands and Islands';

	const withVersions = {
		emailType: 'conversation_invite',
		base,
		conversationVersions: [
			{
				key: 'c1',
				source: 'conversation',
				subject: 'You’re invited to share your views on renewable energy',
				configId: '00000000-0000-0000-0000-000000000002',
				createdAt: '2026-10-22T10:00:00Z',
				conversationId: 'c1',
				conversationTitle: longTitle
			}
		],
		usingBase: [
			{ id: 'c2', title: 'Town centre regeneration' },
			{ id: 'c3', title: 'Air quality' }
		]
	};

	const eventReminder = {
		emailType: 'event_reminder',
		base: {
			key: 'event_reminder-default',
			source: 'default',
			subject: 'Upcoming event reminder'
		},
		conversationVersions: [],
		usingBase: []
	};
</script>

<Story
	name="Invite with a conversation version"
	args={{ view: withVersions, schema, conversations: [] }}
/>
<Story
	name="Event email, not customised"
	args={{
		view: eventReminder,
		schema: { ...schema, email_type: 'event_reminder' },
		conversations: []
	}}
/>
