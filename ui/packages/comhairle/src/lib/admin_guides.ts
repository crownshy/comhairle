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
 *
 * Screenshots live in src/lib/assets/admin-guide/, named after the topic key
 * (sign-up.webp). Import them below and set `image` on the topic.
 */

import environmentImage from './assets/admin-guide/environment.webp';
import logInImage from './assets/admin-guide/log-in.webp';
import signUpImage from './assets/admin-guide/sign-up.webp';

export type AdminGuideTerm = {
	term: string;
	definition: string;
};

export type AdminGuideImage = {
	src: string;
	alt: string;
};

export type AdminGuideTopic = {
	key: string;
	title: string;
	navLabel: string;
	summary: string;
	image?: AdminGuideImage;
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
				image: {
					src: signUpImage,
					alt: 'The Create an account page, with Username, Email, Password and Confirm Password fields, a Sign Up button and a Sign up as guest button.'
				},
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
				image: {
					src: logInImage,
					alt: 'The Log In page, with Email and Password fields, a Log In button and options to log in with a one-time passcode or a Guest ID.'
				},
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
					'The workspace is where you create and manage [conversations](/admin/info/how-to/getting-started/glossary#term-conversation), launch them to participants, and moderate participant contributions.',
				image: {
					src: environmentImage,
					alt: 'The workspace. The sidebar on the left lists Home, Workspace, Organisations, Emails, Media library and your conversations. On the right, Your conversations shows each conversation with its status, such as Draft or Live, and an Edit conversation button.'
				},
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
							'A conversation is a public consultation on a particular topic that you create and manage in the workspace.'
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
							'The tool providing an activity behind a step, for example Learn step, Participant-led Poll, Thinking space, Survey or Prioritization tool.'
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
				key: 'create-a-conversation',
				title: 'Create a conversation',
				navLabel: 'Create a conversation',
				summary:
					'Create a new conversation from scratch or start with a template that already contains a set of steps.',
				steps: [
					'Click **+ New Conversation**.',
					'Choose **Start from blank** to create a conversation from scratch, or **Choose from template** to start with a ready-made set of steps.',
					'If you choose **Start from blank**, the Conversation is created with a placeholder title like "Untitled 2026-10-05 2:24:30PM".'
				],
				tips: [
					'**Start from blank** gives you an empty conversation that you can build step by step.',
					'**Choose from template** gives you a starting structure that you can adapt to your needs.',
					'Creating a conversation does not make it available to participants. You can configure and review it before launching it.'
				]
			},
			{
				key: 'name-a-conversation',
				title: 'Name a conversation',
				navLabel: 'Name a conversation',
				summary:
					'Give your conversation a clear name that colleagues and participants can easily recognise.',
				steps: [
					'Go to **Configure** → **Details**.',
					'Replace the temporary **Title** with a clear, specific name.',
					'This title will be visible to colleagues and participants.'
				],
				tips: [
					'Use words that participants will recognise rather than internal project names.',
					'Every Conversation needs a unique title. Two conversations cannot have exactly the same title.'
				]
			},
			{
				key: 'add-a-description',
				title: 'Add a description',
				navLabel: 'Add a description',
				summary:
					'Describe what the conversation is about, why you’re asking for input, and what might happen as a result.',
				steps: [
					'Go to **Configure** → **Details**.',
					'In **Description**, explain the topic, why you’re asking for input, and what might happen as a result.'
				],
				tips: [
					'Select **Preview** in the top-right corner to see how the conversation will appear to participants.',
					'To customise other participant-facing text, go to **Configure** → **Content**. Here you can edit the **Call to action**, **FAQs**, **Privacy policy**, and **Thank you message**.',
					'If you leave fields under **Content** blank, Comhairle’s default text will be used.',
					'If you use specialist or unfamiliar terms, explain them under **Configure** → **Glossary**. Their definitions will appear as tooltips when those terms are used in the conversation.'
				]
			},
			{
				key: 'multilingual-conversation',
				title: 'Setup a multilingual conversation',
				navLabel: 'Setup a multilingual conversation',
				summary:
					'Add other languages to your conversation so participants can read and take part in their preferred language.',
				steps: [
					'Go to **Configure** → **Details**.',
					'Under **Language options**, add the languages you want your conversation to support.',
					'Once you add a language, translation fields will become available for the content you can translate.',
					'Add the translated content for each language you support.'
				],
				tips: [
					'Add all the languages you plan to support before you start translating your conversation.',
					'Adding a language does not automatically translate your content. Use the **Translate** button to generate a draft translation, then review and edit it before publishing.',
					'Check that participant-facing content is translated before launching your conversation.'
				]
			},
			{
				key: 'add-banner-image',
				title: 'Add a banner image',
				navLabel: 'Add a banner image',
				summary:
					'Add a banner image to help participants recognise your conversation. It appears alongside the description on the conversation landing and invitation pages.',
				steps: [
					'Go to **Configure** → **Details** and find **Banner image**.',
					'Select **Upload** to add a new image from your computer, or **Media library** to choose an image you’ve already uploaded.',
					'If you choose **Media library**, select an image and click **Select**.'
				],
				tips: [
					'Images you upload are saved to the **Media library**, so you can reuse them in other conversations.',
					'Add **Alt text** to images in the **Media library** to describe their content for people using screen readers.',
					'Choose an image that supports the topic of the conversation and is still easy to understand when displayed at different sizes.'
				]
			},
			{
				key: 'preview',
				title: 'Preview your page',
				navLabel: 'Preview your page',
				summary:
					'See how your conversation will appear to participants without launching it.',
				steps: [
					'Click **Preview** in the top-right corner. The preview opens in a new tab.',
					'Check the **Title**, **Short description**, **Description**, and **Banner image** on the landing page.',
					'Select the join button and go through the conversation steps as a participant would.',
					'If you need to make changes, return to the admin tab and edit your conversation.',
					'Refresh the preview tab to see your changes.'
				],
				tips: [
					'Previewing does not launch your conversation or make it visible to participants, so you can preview it as often as you need.',
					'Go through the full conversation before launching to check that the content, steps, and participant journey work as expected.'
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
