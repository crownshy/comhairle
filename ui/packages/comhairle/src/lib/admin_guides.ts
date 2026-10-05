export type AdminGuideTopic = {
	key: string;
	title: string;
	navLabel: string;
	content: string;
};

export type AdminGuide = {
	key: string;
	title: string;
	navLabel: string;
	topics: AdminGuideTopic[];
};

export const ADMIN_GUIDES: Record<string, AdminGuide> = {
	'getting-started': {
		key: 'getting-started',
		title: 'Getting started',
		navLabel: 'Getting started',
		topics: [
			{
				key: 'environment',
				title: 'Get familiar with the environment',
				navLabel: 'Get familiar with the environment',
				content:
					'Comhairle helps you build and run Conversations. Use the admin area to prepare your content, design the participant journey, and review contributions.'
			},
			{
				key: 'roles-and-permissions',
				title: 'Your role and permissions',
				navLabel: 'Your role and permissions',
				content:
					'Your permissions determine which Conversations you can access and edit. Contact your organisation administrator if you need access.'
			},
			{
				key: 'key-terms',
				title: 'Key terms',
				navLabel: 'Key terms',
				content:
					'A Conversation is a consultation. Its Workflow is an ordered sequence of Steps, each backed by an engagement tool.'
			}
		]
	},
	'create-a-conversation': {
		key: 'create-a-conversation',
		title: 'Create a conversation',
		navLabel: 'Create a conversation',
		topics: [
			{
				key: 'name-your-conversation',
				title: 'Name your conversation',
				navLabel: 'Name your conversation',
				content:
					'Create a Conversation in the admin area and give it a clear name that participants will recognise.'
			},
			{
				key: 'add-a-description',
				title: 'Add a description',
				navLabel: 'Add a description',
				content:
					'In Configure, explain the purpose of your Conversation and what you are asking participants to contribute.'
			},
			{
				key: 'add-images-and-files',
				title: 'Add images and files',
				navLabel: 'Add images and files',
				content: 'Use the media library to upload supporting images and files.'
			},
			{
				key: 'preview',
				title: 'Preview your page',
				navLabel: 'Preview your page',
				content:
					'Preview the participant page to check the title, description, and supporting content before launch.'
			}
		]
	},
	'design-the-engagement-process': {
		key: 'design-the-engagement-process',
		title: 'Design the engagement process',
		navLabel: 'Design the engagement process',
		topics: [
			{
				key: 'design-steps',
				title: 'Decide the engagement steps',
				navLabel: 'Decide the engagement steps',
				content:
					'Plan what participants need to learn and contribute, then build that journey as Steps in the Design board.'
			},
			{
				key: 'choose-tools',
				title: 'Choose tools',
				navLabel: 'Choose tools',
				content:
					'Use the Tool palette to add engagement tools. The Tools Guide explains how each tool works and when to use it.'
			},
			{
				key: 'arrange-step-order',
				title: 'Arrange step order',
				navLabel: 'Arrange step order',
				content: 'Arrange the Steps in the order participants should encounter them.'
			},
			{
				key: 'configure-a-step',
				title: 'Configure a step',
				navLabel: 'Configure a step',
				content:
					'Open each Step to set up its content and tool settings. Preview the Step to check the participant experience.'
			}
		]
	},
	'manage-a-conversation': {
		key: 'manage-a-conversation',
		title: 'Manage a conversation',
		navLabel: 'Manage a conversation',
		topics: [
			{
				key: 'preview-and-launch',
				title: 'Preview and launch',
				navLabel: 'Preview and launch',
				content:
					'Go through the participant journey in preview and check each Step. Launch your Conversation when its content and Steps are ready.'
			},
			{
				key: 'invite-participants',
				title: 'Invite participants',
				navLabel: 'Invite participants',
				content: 'Share the participant link with the people you want to take part.'
			},
			{
				key: 'moderation',
				title: 'Moderate contributions',
				navLabel: 'Moderate contributions',
				content:
					'Use Moderation in a Wiki Poll Step to review participant statements against your moderation policy.'
			},
			{
				key: 'review-insight',
				title: 'Review insights and close',
				navLabel: 'Review insights and close',
				content:
					'Open each Step’s Insights to explore its responses and results. Close the Conversation when the engagement period ends.'
			}
		]
	}
};

export const ADMIN_GUIDE_NAV = Object.values(ADMIN_GUIDES);
