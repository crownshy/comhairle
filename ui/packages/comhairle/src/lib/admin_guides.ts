/**
 * Content for the Comhairle Admin Guide (/admin/info/how-to/<guide>/<topic>).
 *
 * Every topic follows the same template so readers know where to look:
 *   summary  – one or two sentences: what this is and why it matters
 *   steps    – numbered actions, in order
 *   terms    – definitions, for glossary-style topics instead of steps
 *   tips     – "Good to know": gotchas and advice
 * The "Next" link is worked out from the order of topics below.
 *
 * Inline formatting in any of these strings:
 *   **Launch**            bold – use for button, tab and field names as they appear on screen
 *   [Sign up](/auth/signup)  link – site-relative paths for pages in Comhairle, full URLs otherwise
 */

export type AdminGuideTerm = {
	term: string;
	definition: string;
};

export type AdminGuideTopic = {
	key: string;
	title: string;
	navLabel: string;
	summary: string;
	steps?: string[];
	terms?: AdminGuideTerm[];
	tips?: string[];
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
				key: 'sign-up',
				title: 'Sign up for an admin account',
				navLabel: 'Sign up for an admin account',
				summary:
					'Create an account and request the admin role to gain access to the admin workspace.',
				steps: [
					'Go to [Sign up](/auth/signup) and create an account.',
					'Email [team@crown-shy.com](mailto:team@crown-shy.com) from the email address you used to sign up and ask for the admin role to be added to your account.',
					'Your request will usually be processed within one working day.'
				],
				tips: [
					'When requesting access, let us know who referred you and which organisation you’re joining on behalf of. This helps us verify your request and grant the appropriate access.',
					'If possible, CC the person who referred you, someone from your organisation who already has access, or a member of the CrownShy team who knows you.'
				]
			},
			{
				key: 'log-in',
				title: 'Log in as an admin user',
				navLabel: 'Log in as an admin user',
				summary: 'Log in with your admin account to access the admin workspace.',
				steps: [
					'Go to [Login](/auth/login)',
					'Enter your email address and password.',
					'If your account has the admin role, you will be taken to the admin workspace.'
				],
				tips: [
					'If you’re taken to the homepage instead, select **Workspace** in the top navigation bar to enter the workspace.'
				]
			},
			{
				key: 'environment',
				title: 'Get familiar with the environment',
				navLabel: 'Get familiar with the environment',
				summary:
					'The workspace is where you create and manage conversations, launch them to participants, and moderate participant contributions.',
				steps: [
					'Use the sidebar to navigate between different areas of the workspace. The content on the right updates based on your selection.',
					'Select the **collapse icon** to collapse the sidebar. Select it again to reopen it.'
				],
				tips: [
					'Under **Your conversations**: **Owned Conversations** lists conversations you’ve created. **Permitted Conversations** lists conversations that have been shared with you.',
					'**Media library** is where you can upload and manage media, such as images and videos, to use in your conversations.',
					'Images and files you upload are kept in the **Media library**, so you can reuse them across Conversations.',
					'**Emails** is where you can create and manage email templates, including emails used to invite participants to a conversation.',
					'**Organisation** shows the organisation your account belongs to.',
					'**Home** takes you out of the admin workspace and back to the main site.',
					'**Workspace** takes you back to the landing page of admin workspace.',
					'**Settings** is where you can manage your account settings, including changing your password.',
					"Most fields save automatically as you type, so there's no Save button to look for."
				]
			},
			{
				key: 'roles-and-permissions',
				title: 'Your role and permissions',
				navLabel: 'Your role and permissions',
				summary:
					'What you can see and change depends on whether you created a Conversation or were added to it by someone else.',
				steps: [
					'Conversations you create appear under **Owned Conversations**. As the owner, you can edit everything and decide who else has access.',
					'To bring in a colleague, open the Conversation and go to **Configure** → **Team**. Find them by email address and grant them content editor permission.',
					'Conversations that others have shared with you appear under **Permitted Conversations**.',
					'To give another organisation read access, go to **Configure** → **Access** and add it under **Co-hosting organizations**.'
				],
				tips: [
					'Only the owner of a Conversation sees the **Team** tab.',
					"If a colleague can't find a Conversation, check they've been added in **Team**.",
					"If you can't access something you expect to, contact your organisation administrator."
				]
			},
			{
				key: 'glossary',
				title: 'Glossary',
				navLabel: 'Glossary',
				summary: 'These words come up throughout the admin area and this guide.',
				terms: [
					{
						term: 'Conversation',
						definition:
							'A consultation on one topic. It has a title, a description and a series of steps that participants go through.'
					},
					{
						term: 'Step',
						definition:
							'One stage of the participant journey, such as reading background material or answering a poll. Each step uses one engagement tool.'
					},
					{
						term: 'Process design',
						definition:
							'Where you build the ordered list of steps (sometimes called the workflow) for a Conversation.'
					},
					{
						term: 'Engagement tool',
						definition:
							'The activity behind a step, for example Learn step, Participant-led Poll, Thinking space, Survey or Prioritization tool.'
					},
					{
						term: 'Preview',
						definition:
							'A view of the Conversation exactly as participants will see it, without making it live.'
					},
					{
						term: 'Launch',
						definition:
							'Making the Conversation live so participants can take part. After launch, the Conversation can no longer be edited.'
					},
					{
						term: 'Moderation',
						definition:
							'Reviewing statements that participants write in a Participant-led Poll, and accepting or rejecting them.'
					},
					{
						term: 'Insights',
						definition: 'The responses and results collected in a step.'
					}
				]
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
				summary:
					'The title is the first thing participants see: on the landing page, in invitations and on listing cards.',
				steps: [
					'On **Your conversations**, click **Create New Conversation** and choose **Start from blank**. To begin with a ready-made set of steps, choose **Choose from templates** instead.',
					'The Conversation is created with a placeholder title like "Untitled 2026-10-05 2:24:30PM", and opens on **Configure** → **Details**.',
					'Replace the placeholder in **Title** with a clear, specific name.',
					'Add a **Short description**: one line shown under the title and on listing cards.'
				],
				tips: [
					'Write the title in words participants use, not internal project names.',
					"Every Conversation needs a unique title. Two Conversations can't share exactly the same one."
				]
			},
			{
				key: 'add-a-description',
				title: 'Add a description',
				navLabel: 'Add a description',
				summary:
					"The description tells participants what's being discussed and what might happen as a result of their input.",
				steps: [
					'Go to **Configure** → **Details**.',
					'In **Description**, introduce the topic and outline the actions that might be taken as a result of the Conversation.',
					'Under **Language options**, add any other languages you support. You can then translate each field into them.',
					'For the rest of the participant-facing text, open **Configure** → **Content**. There you can set the **Call to action** button label, **FAQs**, **Privacy policy** and **Thank you message**.'
				],
				tips: [
					"Fields in **Content** fall back to Comhairle's defaults if you leave them blank.",
					'Use **Configure** → **Glossary** to explain specialist terms. Their explanations appear as tooltips wherever the term is used in the Conversation.'
				]
			},
			{
				key: 'add-images-and-files',
				title: 'Add images and files',
				navLabel: 'Add images and files',
				summary:
					'A banner image appears beside the description on the landing and invitation pages.',
				steps: [
					'Go to **Configure** → **Details** and find **Banner image**.',
					"Click **Upload** to add a new image from your computer, or **Media library** to reuse one you've already uploaded.",
					'If you chose the media library, pick the image and click **Select**.'
				],
				tips: [
					'Everything you upload is stored in the **Media library**, so you can reuse it in other Conversations.',
					'Add **Alt** text to images in the media library so people using screen readers know what they show.'
				]
			},
			{
				key: 'preview',
				title: 'Preview your page',
				navLabel: 'Preview your page',
				summary:
					'Preview shows the Conversation exactly as participants will see it, without launching it.',
				steps: [
					'Click **Preview** in the top-right corner of any Conversation page. It opens in a new tab.',
					'Check the title, short description, description and banner image on the landing page.',
					'Click through the join button and the steps as a participant would.',
					'Fix anything in the admin tab, then refresh the preview tab to see the change.'
				],
				tips: [
					"Preview doesn't launch the Conversation or make it visible to participants, so use it as often as you like."
				]
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
				summary:
					'Steps are the stages participants go through, in order. Plan them on paper before you build them.',
				steps: [
					'Start with the outcome: decide what you need from participants by the end.',
					'Work backwards: what do participants need to learn first, and what will they contribute?',
					'Open **Process design**. Choose **Start from blank**, or **Choose from templates** to apply a ready-made sequence.',
					'Click **Add step** for each stage you planned.'
				],
				tips: [
					'Applying a template replaces any steps the Conversation already has.',
					'Starting with a Learn step gives everyone the same background before they contribute.'
				]
			},
			{
				key: 'choose-tools',
				title: 'Choose tools',
				navLabel: 'Choose tools',
				summary:
					'Each step uses one engagement tool. Pick the tool that fits what you want participants to do.',
				steps: [
					'In **Process design**, click **Add step**.',
					"Browse the tools. Each one lists what it's best for, its features and what you'd get from it.",
					'Choose a tool to add it as a new step at the end of the list.'
				],
				tips: [
					'Not sure which tool to use? The **Engagement tools** section of this guide explains each one in detail.'
				]
			},
			{
				key: 'arrange-step-order',
				title: 'Arrange step order',
				navLabel: 'Arrange step order',
				summary: 'Participants go through the steps from top to bottom.',
				steps: [
					'Open **Process design**.',
					'Drag a step card up or down to move it.',
					"If dragging is awkward, open the step's actions menu and choose **Move up** or **Move down**.",
					'Use the same menu to **Rename** or **Delete** a step.'
				],
				tips: [
					"Deleting a step permanently removes it and its configuration, and the remaining steps are renumbered. This can't be undone."
				]
			},
			{
				key: 'configure-a-step',
				title: 'Configure a step',
				navLabel: 'Configure a step',
				summary: 'Each step has its own content and rules. Set them up before you launch.',
				steps: [
					'In **Process design**, open the step.',
					'On the **Setup** tab, add the content for the step, such as Learn pages or poll questions. What you see here depends on the tool.',
					'On the **Configure** tab, edit the step **Name** and **Description**.',
					"Turn on **Required step** if participants mustn't skip it, and **Revisitable step** if they can go back to it later.",
					'Choose a **Data protocol** to control whether participants are asked to share their responses, and with whom.'
				],
				tips: [
					"Use **Preview** after each step you set up to check it from a participant's view."
				]
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
				summary:
					'Launching makes the Conversation live for participants. Check everything first: once launched, it can no longer be edited.',
				steps: [
					'Click **Preview** and go through the whole journey as a participant, step by step.',
					'Go to **Configure** → **Access**. Decide whether to turn on **Only allow participation by invite**, and whether to **Show conversation publicly**.',
					"When you're ready, click **Launch Conversation** and confirm with **Launch**.",
					'A **Launched** badge appears at the top, and **Recruit**, **Monitor**, **Notify** and **Report** become available.'
				],
				tips: [
					"You can't edit a Conversation after launch, so finish your final preview first.",
					'With **Only allow participation by invite** off, anyone with the link can take part.'
				]
			},
			{
				key: 'invite-participants',
				title: 'Invite participants',
				navLabel: 'Invite participants',
				summary:
					'Once the Conversation is launched, use **Recruit** to share it by link, QR code or email.',
				steps: [
					'Open **Recruit**.',
					'Click **New Invite Link** to create a link you can share. Give it a **Label**, such as "Newsletter" or "Town hall", so you can tell your links apart.',
					'Copy the link, or use its **QR code** on posters and at in-person events.',
					'To invite specific people, enter their addresses under **Emails**, separated by commas. Choose how long the invite is valid for, then click **Submit**.'
				],
				tips: [
					'Each link shows how many people have **Accepted**, so you can see which channel works best.',
					'You can find the public address any time under **Live Conversation Link** in the actions menu at the top right.'
				]
			},
			{
				key: 'moderation',
				title: 'Moderate contributions',
				navLabel: 'Moderate contributions',
				summary:
					'In a Participant-led Poll, participants write their own statements. Moderation is where you accept or reject them.',
				steps: [
					'Set your reasons for rejecting statements, such as "Off-topic", in **Configure** → **Moderation policy**. Moderators choose from this list.',
					'In **Process design**, open the poll step and go to its **Moderation** tab.',
					'Filter statements by **Pending**, **Accepted**, **Rejected** or **All**, or search for a word.',
					'Accept or reject each statement. When you reject one, pick a reason.'
				],
				tips: [
					"Check **Pending** regularly while the Conversation is live, so new statements don't wait long for review."
				]
			},
			{
				key: 'review-insight',
				title: 'Review insights and close',
				navLabel: 'Review insights and close',
				summary:
					'Insights show what participants said in each step. End the Conversation when the engagement period is over.',
				steps: [
					'In **Process design**, open a step and go to its **Insights** tab. Insights are available for Participant-led Poll, Thinking space, Prioritization tool and Survey steps.',
					'Use **Monitor** and **Report** to look at the Conversation as a whole.',
					'To close the Conversation, open the **More actions** menu at the top right, choose **End Conversation**, then confirm with **End**.'
				],
				tips: [
					'Ending is reversible. Choose **Re-open Conversation** from the same menu to let participants take part again.'
				]
			}
		]
	}
};

export const ADMIN_GUIDE_NAV = Object.values(ADMIN_GUIDES);
