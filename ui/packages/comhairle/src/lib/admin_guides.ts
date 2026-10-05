export type AdminGuide = {
	key: string;
	title: string;
	navLabel: string;
	sections: { heading: string; content: string }[];
};

export const ADMIN_GUIDES: Record<string, AdminGuide> = {
	'getting-started': {
		key: 'getting-started',
		title: 'Getting started',
		navLabel: 'Getting started',
		sections: [
			{
				heading: 'Get familiar with the environment',
				content:
					'Comhairle helps you build and run Conversations. Use the admin area to prepare your content, design the participant journey, and review contributions.'
			},
			{
				heading: 'Your role and permissions',
				content:
					'Your permissions determine which Conversations you can access and edit. Contact your organisation administrator if you need access.'
			},
			{
				heading: 'Key terms',
				content:
					'A Conversation is a consultation. Its Workflow is an ordered sequence of Steps, each backed by an engagement tool. Insights show results for a Step; a Report brings findings together across a Conversation.'
			}
		]
	},
	'create-a-conversation': {
		key: 'create-a-conversation',
		title: 'Create a conversation',
		navLabel: 'Create a conversation',
		sections: [
			{
				heading: 'Name your conversation',
				content:
					'Create a Conversation in the admin area and give it a clear name that participants will recognise.'
			},
			{
				heading: 'Add a description',
				content:
					'In Configure, explain the purpose of your Conversation and what you are asking participants to contribute.'
			},
			{
				heading: 'Add images and files',
				content: 'Use the media library to upload supporting images and files.'
			},
			{
				heading: 'Preview your page',
				content:
					'Preview the participant page to check the title, description, and supporting content before launch.'
			}
		]
	},
	'design-the-engagement-process': {
		key: 'design-the-engagement-process',
		title: 'Design the engagement process',
		navLabel: 'Design the engagement process',
		sections: [
			{
				heading: 'Decide the engagement steps',
				content:
					'Plan what participants need to learn and contribute, then build that journey as Steps in the Design board.'
			},
			{
				heading: 'Choose tools',
				content:
					'Use the Tool palette to add engagement tools. The Tools Guide explains how each tool works and when to use it.'
			},
			{
				heading: 'Arrange step order',
				content: 'Arrange the Steps in the order participants should encounter them.'
			},
			{
				heading: 'Configure a step',
				content:
					'Open each Step to set up its content and tool settings. Preview the Step to check the participant experience.'
			}
		]
	},
	'manage-a-conversation': {
		key: 'manage-a-conversation',
		title: 'Manage a conversation',
		navLabel: 'Manage a conversation',
		sections: [
			{
				heading: 'Preview and launch',
				content:
					'Go through the participant journey in preview and check each Step. Launch your Conversation when its content and Steps are ready.'
			},
			{
				heading: 'Invite participants',
				content: 'Share the participant link with the people you want to take part.'
			},
			{
				heading: 'Moderate contributions',
				content:
					'Use Moderation in a Wiki Poll Step to review participant statements against your moderation policy.'
			},
			{
				heading: 'Review insights and close',
				content:
					'Open each Step’s Insights to explore its responses and results. Close the Conversation when the engagement period ends.'
			}
		]
	}
};

export const ADMIN_GUIDE_NAV = Object.values(ADMIN_GUIDES);
