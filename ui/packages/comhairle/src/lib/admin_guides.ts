/**
 * Content for the Comhairle Handbook (/admin/info/how-to/<guide>/<topic>).
 *
 * Every topic follows the same template so readers know where to look:
 *   summary  – one or two sentences: what this is and why it matters
 *   steps    – numbered actions, in order
 *   tips     – "Good to know": gotchas and advice
 * The "Next" link is worked out from the order of topics below.
 *
 * The glossary (/admin/info/glossary) is separate: ADMIN_GLOSSARY at the bottom of this file.
 * Link to a term with [text](/admin/info/glossary#term-<term-in-kebab-case>).
 *
 * Inline formatting in any of these strings:
 *   **Launch**            bold – use for button, tab and field names as they appear on screen
 *   [Sign up](/auth/signup)  link – site-relative paths for pages in Comhairle, full URLs otherwise
 *
 * Screenshots live in src/lib/assets/admin-guide/, named after the topic key
 * (sign-up.webp). Import them below and set `image` on the topic.
 */

import environmentImage from './assets/admin-guide/environment.webp';
import rolesAndPermissionsImage from './assets/admin-guide/roles-and-permissions.webp';
import addPeopleImage from './assets/admin-guide/add-people.webp';
import createAConversationImage from './assets/admin-guide/create-a-conversation.webp';
import nameAConversationImage from './assets/admin-guide/name-a-conversation.webp';
import addADescriptionImage from './assets/admin-guide/add-a-description.webp';
import multilingualConversationImage from './assets/admin-guide/multilingual-conversation.webp';
import addBannerImageImage from './assets/admin-guide/add-banner-image.webp';
import selectBannerImageImage from './assets/admin-guide/select-banner-image.webp';
import previewImage from './assets/admin-guide/preview.webp';
import chooseATemplateImage from './assets/admin-guide/choose-a-template.webp';
import designStepsImage from './assets/admin-guide/design-steps.webp';
import arrangeStepOrderImage from './assets/admin-guide/arrange-step-order.webp';
import configureAStepImage from './assets/admin-guide/configure-a-step.webp';
import configureAccessImage from './assets/admin-guide/configure-access.webp';
import launchAConversationImage from './assets/admin-guide/launch-a-conversation.webp';
import launchedImage from './assets/admin-guide/launched.webp';
import recruitImage from './assets/admin-guide/recruit.webp';
import monitorImage from './assets/admin-guide/monitor.webp';
import moderateImage from './assets/admin-guide/moderate.webp';
import stepNavigationImage from './assets/admin-guide/step-navigation.webp';
import moderationInDepthImage from './assets/admin-guide/moderation-in-depth.webp';
import closeAConversationImage from './assets/admin-guide/close-a-conversation.webp';
import toggleRevisitAfterFinishImage from './assets/admin-guide/toggle-revisit-after-finish.webp';
import rejectStatementImage from './assets/admin-guide/reject-statement.webp';
import splitStatementImage from './assets/admin-guide/split-statement.webp';
import findPreviewButtonImage from './assets/admin-guide/find-preview-button.webp';
import findWorkspaceImage from './assets/admin-guide/find-workspace.webp';
import reportImage from './assets/admin-guide/report.webp';
import notifyImage from './assets/admin-guide/notify.webp';
import logInImage from './assets/admin-guide/log-in.webp';
import signUpImage from './assets/admin-guide/sign-up.webp';

export type AdminGuideTerm = {
	term: string;
	definition: string;
};

export type AdminGuideImage = {
	src: string;
	alt: string;
	/** Short visible caption shown under the image. */
	caption?: string;
	/** In a carousel: the step (1, 2, …) this screenshot illustrates. Hovering or
	 * clicking that step shows this image, and the step is highlighted while it's shown. */
	step?: number;
	/** In a carousel: the Good to know tip (1, 2, …) this screenshot illustrates. Works like `step`. */
	tip?: number;
};

/** One part of the screen, explained in the "environment" section of an Advanced topic. */
export type AdminGuideScreenPart = {
	/** Exactly as it appears on screen. */
	name: string;
	description: string;
};

/**
 * A decision an admin makes, used by the Advanced category. Each one says when to
 * choose it, how to carry it out, and what happens as a result.
 */
export type AdminGuideDecision = {
	title: string;
	when: string;
	how: string[];
	/** What happens next, including knock-on effects people tend to miss. */
	result: string;
	/** Optional screenshot showing where to make the change. */
	image?: AdminGuideImage;
};

/** A titled reference list, e.g. an example moderation policy with codes. */
export type AdminGuideReference = {
	title: string;
	intro?: string;
	items: { code: string; name: string; description: string }[];
};

/** A pointer to another topic, shown as a sticky card beside the content. */
export type AdminGuideRelated = {
	title: string;
	text: string;
	/** Link text, e.g. the topic name. */
	label: string;
	href: string;
};

export type AdminGuideTopic = {
	key: string;
	title: string;
	navLabel: string;
	summary: string;
	image?: AdminGuideImage;
	/** Several screenshots shown as a carousel. Use instead of `image`. */
	images?: AdminGuideImage[];
	/** Advanced topics: introduce the screen before the decisions. */
	environment?: { title: string; intro?: string; parts: AdminGuideScreenPart[] };
	steps?: string[];
	/** Advanced topics: the decisions to make and how to make them. */
	decisions?: { title: string; intro?: string; options: AdminGuideDecision[] };
	reference?: AdminGuideReference;
	tips?: string[];
	/** A card pinned to the right of the page pointing to a related topic. */
	related?: AdminGuideRelated;
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
					'Go to [Sign up](/auth/signup), fill in your details and select :signup: to create an account.',
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
				images: [
					{
						src: logInImage,
						alt: 'The Log In page, with Email and Password fields, a Log In button and options to log in with a one-time passcode or a Guest ID.',
						caption: 'The Log In page.',
						step: 2
					},
					{
						src: findWorkspaceImage,
						alt: 'The Comhairle homepage while logged in as an admin. The header shows Home, About, Participate, Your Rights, a language menu, a Workspace button and the account menu.',
						caption:
							'Admins see **Workspace** in the top navigation bar on the homepage.',
						tip: 1
					}
				],
				steps: [
					'Go to [Log in](/auth/login).',
					'Enter your email address and password, then select :login:.',
					'If your account has the admin role, you will be taken to the admin workspace.'
				],
				tips: [
					'If you’re taken to the homepage instead, select :workspace: in the top navigation bar to enter the workspace.'
				]
			},
			{
				key: 'environment',
				title: 'Get familiar with the environment',
				navLabel: 'Get familiar with the environment',
				summary:
					'The workspace is where you create and manage [conversations](/admin/info/glossary#term-conversation), [launch](/admin/info/glossary#term-launch) them to participants, and moderate participant contributions.',
				image: {
					src: environmentImage,
					alt: 'The workspace. The sidebar on the left lists Conversations, then Media library and Email templates under Library, and Organisations, Regions and System roles under Administration, with the Handbook and your account at the bottom. On the right, Your conversations has status filters, a search box and a card for each conversation showing its status and an Edit conversation button.'
				},
				steps: [
					'Use the sidebar to navigate between different areas of the workspace. The content on the right updates based on your selection.',
					'Select the **collapse icon** :collapse: to collapse the sidebar. To reopen it, select the **expand icon** :expand:.'
				],
				tips: [
					'**Conversations** lists every conversation you’ve created or that has been shared with you. Open one and the sidebar switches to that conversation; select **← Conversations** at the top to switch back.',
					'**Media library** is where you can upload and manage media, such as images and videos, to use in your conversations.',
					'Images and files you upload are kept in the **Media library**, so you can reuse them across Conversations.',
					'**Email templates** is where you can create and manage email templates, including emails used to invite participants to a conversation.',
					'**Organisations** shows the organisations your account belongs to.',
					'**Handbook**, at the bottom of the sidebar, opens these guides.',
					'Your profile menu at the bottom of the sidebar has **Back to home**, which takes you out of the admin workspace and back to the main site.',
					'**Settings**, in your profile menu, is where you can manage your account settings, including changing your password.',
					"Most fields save automatically as you type, so there's no Save button to look for."
				]
			},
			{
				key: 'roles-and-permissions',
				title: 'Your role and permissions',
				navLabel: 'Your role and permissions',
				summary:
					'What you can see and change depends on whether you created a Conversation or were added to it by someone else.',
				images: [
					{
						src: rolesAndPermissionsImage,
						alt: 'The Team tab under Configure, listing the people who work on the conversation with their roles, such as a colleague with the Observer role. Role filters (Admin, Observer, Content Editor, Moderator, Translator and Data Access) and an Add people button sit above the list.',
						caption: 'The Team tab lists everyone with a role on the conversation.',
						step: 3
					},
					{
						src: addPeopleImage,
						alt: "The Add people window. Person is selected, a colleague's email address is entered and the Observer role is ticked, with Cancel and Save changes buttons at the bottom. Organization is the other option at the top.",
						caption: 'Choose a person or an organisation, then the roles they get.',
						step: 4
					}
				],
				steps: [
					'Conversations you create appear in **Your conversations**. As the owner, you can edit everything and decide who else has access.',
					'Conversations that others have shared with you appear there too. Select the **Shared** filter to see only those.',
					'To see who works on a conversation, open it and go to **Configure** → **Team**. Each person is listed with their roles.',
					'To bring in a colleague, click **Add people**, choose **Person** and enter their email address. Tick the roles they need, such as **Content Editor** or **Moderator**, then click **Save changes**.',
					'To give everyone in another organisation access, choose **Organization** instead of **Person** and select the organisation.'
				],
				tips: [
					'Only the owner of a Conversation sees the **Team** tab.',
					"If a colleague can't find a Conversation, check they've been added in **Team**.",
					"If you can't access something you expect to, contact your organisation administrator."
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
					'Create a new [conversation](/admin/info/glossary#term-conversation) from scratch or start with a template that already contains a set of [steps](/admin/info/glossary#term-step).',
				image: {
					src: createAConversationImage,
					alt: 'Your conversations, with the Create new conversation button highlighted in yellow at the top right.'
				},
				steps: [
					'Click **Create new conversation**.',
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
				image: {
					src: nameAConversationImage,
					alt: 'The Details tab under Configure, with the Title field highlighted in yellow.'
				},
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
				image: {
					src: addADescriptionImage,
					alt: 'The Details tab under Configure, with the Short description field highlighted in yellow, between the Title and Description fields.'
				},
				steps: [
					'Go to **Configure** → **Details**.',
					'In **Short description**, sum up the conversation in a sentence. It appears under the title on the landing page and on the conversation’s card.',
					'In **Description**, explain the topic, why you’re asking for input, and what might happen as a result.'
				],
				tips: [
					'Select :preview: at the top of the sidebar, next to the conversation’s status, to see how it will appear to participants.',
					'To customise other participant-facing text, go to **Configure** → **Content**. Here you can edit the **Call to action**, **FAQs**, **Privacy policy**, and **Thank you message**.',
					'If you leave fields under **Content** blank, Comhairle’s default text will be used.',
					'If you use specialist or unfamiliar terms, explain them under **Glossary** in **Configure** → **Content**. Their definitions will appear as tooltips when those terms are used in the conversation.'
				]
			},
			{
				key: 'multilingual-conversation',
				title: 'Setup a multilingual conversation',
				navLabel: 'Setup a multilingual conversation',
				summary:
					'Add other languages to your conversation so participants can read and take part in their preferred language.',
				image: {
					src: multilingualConversationImage,
					alt: 'The Details tab under Configure with a list of languages open, showing English selected and other languages such as Gaelic, Welsh, Chinese, Spanish, French and Arabic.'
				},
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
				images: [
					{
						src: addBannerImageImage,
						alt: 'The Banner image setting on the Details tab, with Media library and Upload buttons above the current banner, a photo looking up at tall trees.',
						caption:
							'Choose Upload for a new image, or Media library for one you’ve already uploaded.',
						step: 2
					},
					{
						src: selectBannerImageImage,
						alt: 'The Media library window, with an image named CrownShy banner selected and a Select button at the top right.',
						caption: 'In the Media library, select an image and click Select.',
						step: 3
					}
				],
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
				images: [
					{
						src: findPreviewButtonImage,
						alt: 'A conversation’s Configure page, with the Preview button highlighted in yellow at the top of the sidebar, next to the Draft status.',
						caption:
							'Preview is at the top of the sidebar, next to the conversation’s status.',
						step: 1
					},
					{
						src: previewImage,
						alt: 'A preview of the conversation landing page, marked This is a preview of the conversation, showing the title, short description, a Join the conversation button and the banner image.',
						caption: 'The preview shows your landing page as participants will see it.',
						step: 2
					}
				],
				steps: [
					'Click :preview: at the top of the sidebar, next to the conversation’s status. The preview opens in a new tab.',
					'Check the **Title**, **Short description**, **Description**, and **Banner image** on the landing page.',
					'Select the join button and go through the conversation steps as a participant would.',
					'If you need to make changes, return to the admin tab and edit your conversation.',
					'Refresh the preview tab to see your changes.'
				],
				tips: [
					'Previewing does not [launch](/admin/info/glossary#term-launch) your conversation or make it visible to participants, so you can preview it as often as you need.',
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
				key: 'choose-a-template',
				title: 'Choose a template',
				navLabel: 'Choose a template',
				summary:
					'Start with a ready-made conversation flow, then adapt the [steps](/admin/info/glossary#term-step) to suit your topic and what you want participants to do.',
				image: {
					src: chooseATemplateImage,
					alt: "The Choose a template window. Templates such as Informed-participants survey, Understand opinion groups and Citizen workshop are listed on the left, with the selected template's workflow steps on the right and a Get started button."
				},
				steps: [
					'Before choosing a template, consider what you want to learn from the conversation, what you want participants to contribute, how they will contribute, and what they need to know first.',
					'Go to **Process design** and click **Choose from templates**.',
					'Choose a template that best matches the conversation you want to run, then click **Get started**.',
					'Adapt the template by adding, removing, or rearranging steps.'
				],
				tips: [
					'Templates are a starting point. You can change the steps and their order to suit your conversation.',
					'If none of the templates suit your needs, you can [design the engagement steps](/admin/info/how-to/design-the-engagement-process/design-steps) yourself.'
				]
			},
			{
				key: 'design-steps',
				title: 'Design the engagement steps',
				navLabel: 'Design the engagement steps',
				summary:
					'Build your own conversation flow by deciding what participants need to learn, consider, and contribute at each step.',
				image: {
					src: designStepsImage,
					alt: "The Add a step window. Step types such as Topic onboarding, Participant-led poll, Survey and Individual view exploration are listed on the left. The selected type's description, a link to its Introduction & setup guide, examples and a Best for list appear on the right, with an Add this step button."
				},
				steps: [
					'Before designing your steps, consider what you want to learn from the conversation, what you want participants to contribute, how they will contribute, and what they need to know first.',
					'Go to **Process design** and choose **Start from blank**.',
					'Click **Add step**. You’ll be asked to choose an [engagement tool](/admin/info/glossary#term-engagement-tool) for the step, based on what you want participants to do.',
					'Choose a tool, then click **Add this step** to add it to your conversation.',
					'Repeat this for each stage of your conversation, then arrange the steps in the order you want participants to go through them.'
				],
				tips: [
					'Each step uses an **engagement tool**. Different tools support different ways of participating, such as learning about a topic, contributing a response, or interacting with other participants’ contributions.',
					'If you’re not sure where to start, browse the premade **templates** or explore **case studies** to see how others have designed their engagement steps.',
					'Starting with a **Learn step** gives everyone the same background before they contribute.',
					'You can add, remove, or rearrange steps as your conversation develops.'
				]
			},
			{
				key: 'arrange-step-order',
				title: 'Arrange step order',
				navLabel: 'Arrange step order',
				summary:
					'Set the order of your [steps](/admin/info/glossary#term-step). Participants will go through them from top to bottom.',
				image: {
					src: arrangeStepOrderImage,
					alt: "Process design with two step cards, New Survey Step and New Polis Step. The second step's actions menu is open, showing Move up, Move down, Rename, Learn more and Delete."
				},
				steps: [
					'Go to **Process design**.',
					'Drag a step card up or down to change its position.',
					'Alternatively, open the step’s actions menu :actions: and select **Move up** or **Move down**.',
					'You can also use this menu to **Rename** or **Delete** a step, or select **Learn more** to read about its engagement tool.'
				],
				tips: [
					'Think about the participant journey when ordering your steps: what do they need to know or consider before moving on to the next activity?',
					'Deleting a step permanently removes the step and its configuration. The remaining steps will be renumbered, and this cannot be undone.'
				]
			},
			{
				key: 'configure-a-step',
				title: 'Configure a step',
				navLabel: 'Configure a step',
				summary:
					'Set up the content and behaviour of each step before participants go through your conversation.',
				image: {
					src: configureAStepImage,
					alt: 'The Configure tab of a step, with the Name and Description fields, and the Revisitable step and Required step switches.'
				},
				steps: [
					'In **Process design**, select the step you want to configure.',
					'Open the **Setup** tab to add the content participants will interact with, such as learning materials or poll questions. The options available depend on the [engagement tool](/admin/info/glossary#term-engagement-tool) you chose.',
					'Open the **Configure** tab to edit the step’s **Name** and **Description**.',
					'Turn on **Required step** if participants must complete the step before moving on.',
					'Turn on **Revisitable step** if participants should be able to return to the step later.'
				],
				tips: [
					'The settings available under **Setup** depend on the **engagement tool** used for that step.',
					'Use **Preview** as you configure your steps to check how they will appear and work for participants.'
				]
			},
			{
				key: 'set-up-the-learning-assistant',
				title: 'Set up the Learning Assistant',
				navLabel: 'Set up the Learning Assistant',
				summary:
					'The Learning Assistant answers participants’ questions about your topic, using your Learn steps and the documents you upload.',
				steps: [
					'Open **Learning Assistant** in the sidebar.',
					'Turn on **Show Learning Assistant**.',
					'Under **Learn content**, click **Sync learn content** so the assistant can answer questions about your Learn steps. Wait until it says **Learn content synced and ready**.',
					'Under **Documents**, upload any other files the assistant should answer from. You can upload PDF, text, image and MP4 files up to 50 MB each.',
					'Under **Cross-language Search**, choose the languages participants might ask in, so they get answers from documents written in another language.',
					'Under **Target reading age**, enter an age between 5 and 18. The assistant writes its answers for that reading level.'
				],
				tips: [
					'The Learning Assistant only appears to participants once at least one document has finished processing.',
					'Changes to Learn steps are not picked up automatically. Click **Sync learn content** again after you finish editing them.',
					'Re-syncing replaces the learning material document, so sources cited in earlier answers will no longer open for participants.'
				]
			},
			{
				key: 'run-events',
				title: 'Run events',
				navLabel: 'Run events',
				summary:
					'Use **Events** to plan online or in-person meetings for your conversation, invite people, and manage breakout rooms and recordings.',
				steps: [
					'Open **Events** in the sidebar and click **Add event**.',
					'Enter a **Title**, **Description** and **Capacity**, choose a **Default time zone**, **Event date** and **Time**, and choose a **Signup mode**: **Invite** or **Open**. Click **Save changes**.',
					'To change an event later, click **Edit event** on its card.',
					'On the **Details** tab, choose a **Format**: **Online** or **In-person**. Online events use a Jitsi meeting link unless you enter a **Custom event link**.',
					'On the **Event Structure** tab, click **+ Add agenda item** or **+ Add breakout session** to plan how the meeting will run, then click **Save Agenda**.',
					'Use the **Facilitators**, **Location**, **Invites**, **Breakout Rooms** and **Recordings** tabs to set up the rest of the event.'
				],
				tips: [
					'On **Breakout Rooms**, click **Auto-assign** to sort attendees into rooms, or **Edit rooms** to arrange them yourself.',
					'Click **Event Link** to open the event page participants see.'
				]
			}
		]
	},
	'launch-a-conversation': {
		key: 'launch-a-conversation',
		title: 'Launch a conversation',
		navLabel: 'Launch a conversation',
		topics: [
			{
				key: 'preview-before-launch',
				title: 'Preview before launch',
				navLabel: 'Preview before launch',
				summary:
					'Go through the full conversation as a participant and check that everything is ready before you launch.',
				images: [
					{
						src: findPreviewButtonImage,
						alt: 'A conversation’s Configure page, with the Preview button highlighted in yellow at the top of the sidebar, next to the Draft status.',
						caption:
							'Preview is at the top of the sidebar, next to the conversation’s status.',
						step: 1
					},
					{
						src: previewImage,
						alt: 'A preview of the conversation landing page, marked This is a preview of the conversation, showing the title, short description, a Join the conversation button and the banner image.',
						caption:
							'The preview opens on the landing page. Select Join the conversation to go through the steps.',
						step: 2
					}
				],
				steps: [
					'Click :preview: at the top of the sidebar, next to the conversation’s status.',
					'Go through the conversation from beginning to end as a participant would.',
					'Check that the content, steps, [engagement tools](/admin/info/glossary#term-engagement-tool), and participant journey work as expected.',
					'Return to the admin workspace to make any final changes.'
				],
				tips: [
					'Previewing does not launch your conversation or make it available to participants.',
					'Once you launch a conversation, you can no longer edit it, so use this opportunity to make any final changes.'
				]
			},
			{
				key: 'configure-access',
				title: 'Configure access',
				navLabel: 'Configure access',
				summary:
					'Choose who can take part in your conversation and what participants can do before and after completing it.',
				image: {
					src: configureAccessImage,
					alt: 'The Access section of Configure Details, with on and off switches grouped under Visibility (Show conversation publicly), Who can take part (Only allow participation by invite, Automatically log in with an anonymous account) and After finishing (Allow revisit after finishing).'
				},
				steps: [
					'Go to **Configure** → **Details** and scroll to **Access**.',
					'Review the access and participation settings and turn on the options that suit your conversation.',
					'Use **Show conversation publicly** to allow anyone to view the conversation’s data once it has been launched.',
					'Use **Only allow participation by invite** to restrict participation to people invited and managed by admins.',
					'Use **Automatically log in with an anonymous account** to create a temporary account for participants who are not signed in.',
					'Use **Allow revisit after finishing** to let participants return to the conversation steps after they have finished.',
					'Use **Enable signup prompts** to show participants an option to sign up after taking part.',
					'Use **Show thank you page anonymous instructions** to show a thank you page to anonymous participants after they finish.',
					'Use **Show thank you page feedback button** to show a feedback button on the thank you page.'
				],
				tips: [
					'Review these settings before launching, as they affect how participants access and move through your conversation.',
					'**Show conversation publicly** controls whether people can view the conversation’s data; it is different from allowing people to participate.',
					'If **Only allow participation by invite** is turned on, admins will need to invite and manage the people who can take part.',
					'Consider the experience of participants who are not signed in when choosing the anonymous account, sign-up, and thank you page settings.'
				]
			},
			{
				key: 'launch-a-conversation',
				title: 'Launch a conversation',
				navLabel: 'Launch a conversation',
				summary:
					'[Launch](/admin/info/glossary#term-launch) your conversation when it is ready for participants to take part.',
				images: [
					{
						src: launchAConversationImage,
						alt: 'The Launch page. Before you launch lists Preview it as a participant, Process design, Details and access, and Team. Below it, a warning box explains what happens when you launch, above the Launch conversation button, highlighted in yellow.',
						caption:
							'The Launch page, with the Launch conversation button below Before you launch.',
						step: 3
					},
					{
						src: launchedImage,
						alt: 'The Launched page, confirming Your conversation is launched, with a Conversation link card (Copy link and Open buttons) and Next steps such as Invite participants, Keep participants informed and Follow participation.',
						caption:
							'After launching, the page shows your conversation link and what to do next.',
						step: 4
					}
				],
				steps: [
					'Open **Launch** in the sidebar.',
					'Work through **Before you launch**, including a final **Preview**, to make sure everything is ready.',
					'Click **Launch conversation**, then confirm by clicking **Launch**.',
					'Once launched, the sidebar shows **Launched**, and **Engage** (**Recruit**, **Notify**, **Insights**, **Monitor**, and **Moderate** if you have a Participant-led poll), **Close** and **Report** become available.'
				],
				tips: [
					'Previewing does not launch your conversation or make it available to participants.',
					'Once you launch a conversation, you can no longer edit it, so use this opportunity to make any final changes.'
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
				key: 'recruit',
				title: 'Recruit participants',
				navLabel: 'Recruit participants',
				summary:
					'Once your conversation is launched, use **Recruit** to invite people to take part by link, QR code or email.',
				image: {
					src: recruitImage,
					alt: 'The Recruit page on its Email tab. A Conversation link card with Copy link and Open buttons sits above an Emails field, Invite valid for options and a Submit button. Open Links and Physical tabs are at the top.'
				},
				steps: [
					'Open **Recruit**. It becomes available after you launch the conversation.',
					'Under **Conversation link**, click **Copy link** to share the conversation’s main link, or **Open** to check it.',
					'Go to **Open Links**.',
					'Click **New Invite Link** to create a link you can share. Give it a **Label**, such as "Newsletter" or "Town hall", so you can tell your links apart.',
					'Copy the **Link**, or use its **QR code** on posters and at in-person events.',
					'To invite specific people by email, enter their addresses under **Emails**, separated by commas. Choose how long the invite is valid for, then click **Submit**.'
				],
				tips: [
					'Create a separate link for each channel you use. The **Accepted** column shows how many people joined through each link, so you can see which channels work best.',
					'If **Only allow participation by invite** is turned on under **Access** in **Configure** → **Details**, only people you invite can take part.',
					'You can find the conversation’s public address at any time under **Live Conversation Link** in the more actions menu at the top right.'
				]
			},
			{
				key: 'monitor',
				title: 'Monitor participation',
				navLabel: 'Monitor participation',
				summary:
					'Use **Monitor** to see how many people are taking part, how far they get through your steps, and how long it takes them.',
				image: {
					src: monitorImage,
					alt: 'The Monitor page under Engage. Overview shows Total Users, Active Now, Time Spent and Completed, above a Daily signups chart with Signups and Active counts.'
				},
				steps: [
					'Open **Monitor**. It becomes available after you launch the conversation.',
					'Check **Overview** for total users, how many are **Active now**, time spent and completions, and the daily signups chart.',
					'Under **Progress**, see how many participants have **Started** and **Completed** each step.',
					'Check **Time to complete** to see the median time participants take to finish.',
					'Under **Follow up**, click **Download Contacts** for a list of participants who agreed to be contacted, or **Download Demographics** for demographic data from participants’ profiles.'
				],
				tips: [
					'If many people start a step but few complete it, the step may be too long or unclear. Use **Preview** to go through it again as a participant.',
					'Downloaded files contain personal data. Store and share them in line with your organisation’s data protection policy.'
				]
			},
			{
				key: 'see-insights',
				title: 'See insights for a step',
				navLabel: 'See insights for a step',
				summary:
					'Each step has its own [Insights](/admin/info/glossary#term-insights) page showing the responses and results collected in that step. For results across the whole conversation, use **Report**.',
				steps: [
					'Under **Engage** in the sidebar, open **Insights**. It becomes available after you launch the conversation.',
					'Click the step you want to look at.',
					'In a Participant-led poll, see the totals for **Participants**, **Statements** and **Vote cast**, then the **Consensus continuum**, **Area of consensus**, **Area of disagreement** and **Opinion groups**.',
					'Click **Generate summary** to add an AI summary of the poll. Click **Regenerate summary** to update it after more votes come in.',
					'Click **Download CSV** to export the results.'
				],
				tips: [
					'**Insights** lists Participant-led poll, Thinking space, Prioritization and Survey steps. Learn steps have no Insights.',
					'A Participant-led poll shows **No insights yet** until participants start voting.',
					'To show a live Participant-led poll on a screen in the room, click **Open room display**. It opens in a new window.',
					'Downloaded files may contain personal data. Store and share them in line with your organisation’s data protection policy.'
				]
			},
			{
				key: 'notify',
				title: 'Notify participants',
				navLabel: 'Notify participants',
				summary:
					'Send a message to everyone who has taken part in your conversation, for example to share an update or invite them back.',
				image: {
					src: notifyImage,
					alt: 'The Notify page under Engage, with Delivery method set to In-app notification, Notification title and Message content fields and a Send notification to all participants button. A Recipients preview on the right shows who will receive it.'
				},
				steps: [
					'Open **Notify**. It becomes available after you launch the conversation.',
					'Under **Delivery method**, choose **In-app notification** or **Email**.',
					'Enter a **Notification title** and **Message content**. For an email, enter an **Email subject** and **Email body** instead.',
					'Check **Recipients preview** to see who will receive the message.',
					'Click **Send notification to all participants**, or **Send email to all participants**.'
				],
				tips: [
					'Messages go to everyone who has taken part in this conversation. You can’t choose individual recipients here.',
					'Before sending an email, use **Send test email** to send it to a single address and check how it looks.',
					'Check your message carefully before sending, because it goes to all participants at once.'
				]
			},
			{
				key: 'moderate',
				title: 'Moderate contributions',
				navLabel: 'Moderate contributions',
				summary:
					'In a Participant-led poll, participants can write their own statements. Moderate them so that only appropriate statements are shown to others.',
				image: {
					src: moderateImage,
					alt: 'The Moderation tab of a Participant-led poll step in Process design. Statements moderation has Download CSV, Sync from Polis and Add seed statements buttons, a search box, the All, Seeded, Accepted, Pending and Rejected filters, and a list of statements. The Reject statement popover is open, with an optional Reason and Note, and Cancel and Reject buttons.'
				},
				steps: [
					'Before you launch, set your reasons for rejecting statements, such as "Off-topic", under **Configure** → **Moderation policy**.',
					'In **Process design**, open the Participant-led poll step and go to its **Moderation** tab.',
					'Filter statements by **Pending**, **Accepted**, **Rejected**, **Seeded** or **All**, or search for a word.',
					'Accept or reject each statement. When you reject a statement, choose a reason from your moderation policy.'
				],
				tips: [
					'Check **Pending** regularly while the conversation is live, so new statements don’t wait long for review.',
					'Moderation is only available for Participant-led poll steps.',
					'For when to accept, reject or split a statement, and an example moderation policy, see [Moderation in depth](/admin/info/how-to/advanced/moderation-in-depth).'
				],
				related: {
					title: 'Not sure how to moderate?',
					text: 'Learn when to accept, reject or split a statement, with an example moderation policy.',
					label: 'Moderation in depth',
					href: '/admin/info/how-to/advanced/moderation-in-depth'
				}
			},
			{
				key: 'report',
				title: 'Create a report',
				navLabel: 'Create a report',
				summary:
					'Use **Report** to summarise what came out of your conversation, record its impact, and share the results.',
				image: {
					src: reportImage,
					alt: 'The Report page under Results, with View Report and a Publish Report switch at the top, then a Summary editor and a Report body editor, each with formatting tools and an Embed report component button.'
				},
				steps: [
					'Open **Report**. It becomes available after you launch the conversation.',
					'Under **Summary**, write a short introduction. It appears at the top of the public report.',
					'Under **Report body**, write the full report. Click **Embed report component** to add findings from the conversation, such as charts, into the text.',
					'Under **Facilitator Notes**, click **Add feedback** to record notes gathered by facilitators.',
					'Under **Impacts**, click **Add an impact** to record what the conversation has led to, and choose an **Impact Type**: **Policy**, **Debate** or **Followup Conversation**.',
					'Turn on **Publish Report** to make the report public, then click **View Report** to see it as others will.'
				],
				tips: [
					'To look at the responses to a single step, open [Insights](/admin/info/glossary#term-insights) under **Engage** and click the step.',
					'When the engagement period is over, [close the conversation](/admin/info/how-to/manage-a-conversation/close-a-conversation). You can still write and publish the report after closing.'
				]
			},
			{
				key: 'close-a-conversation',
				title: 'Close a conversation',
				navLabel: 'Close a conversation',
				summary:
					'Close your conversation when the engagement period is over. Participants can no longer take part, and everything collected so far stays available.',
				image: {
					src: closeAConversationImage,
					alt: 'The Close page, with a Before you close checklist: Let participants know, Check final participation and Start your report. Below it, a What happens when you close box explains that participants can no longer take part, everything collected stays available in Monitor and Report, and the conversation can be re-opened. A Close conversation button sits at the bottom.'
				},
				steps: [
					'Open **Close** in the sidebar. It becomes available after you launch the conversation.',
					'Work through **Before you close**: let participants know, check the final numbers in **Monitor**, and start your **Report**.',
					'Read **What happens when you close**.',
					'Click **Close conversation**, then confirm by clicking **End Conversation**.',
					'Once closed, the sidebar shows **Closed** and the page says **This conversation is closed.**'
				],
				tips: [
					'Closing is reversible. On the **Close** page, click **Re-open Conversation** to let people take part again.',
					'Results in **Monitor**, **Insights** and **Report** stay available after you close.',
					'You need permission to launch the conversation before you can close or re-open it.'
				]
			}
		]
	},
	advanced: {
		key: 'advanced',
		title: 'Advanced',
		navLabel: 'Advanced',
		topics: [
			{
				key: 'moderation-in-depth',
				title: 'Moderation in depth',
				navLabel: 'Moderation in depth',
				summary:
					'Get to know the [moderation](/admin/info/glossary#term-moderation) screen for a Participant-led poll, then work through the decision each new statement needs and how to carry it out.',
				image: {
					src: moderationInDepthImage,
					alt: 'The Moderation tab of a Participant-led poll step, filtered to Accepted. Two statements have an Edited badge and Edited from: the original statement about park benches and bins. Each row has translate, pencil, tick and cross icons, and Download CSV, Sync from Polis and Add seed statements buttons sit above the search box and filters.',
					caption:
						'The moderation screen for a Participant-led poll step, showing two statements split from one.'
				},
				environment: {
					title: 'The moderation screen',
					intro: 'Open your poll step in **Process design** and go to **Moderation**. Statements are grouped into tabs by where they came from and what has been decided about them.',
					parts: [
						{
							name: 'Download CSV',
							description:
								'Downloads the moderation log: every statement, its status, and the reason and note recorded for each decision.'
						},
						{
							name: 'Sync from Polis',
							description:
								'Pulls in new participant statements. The list does not update on its own, so click this before you start moderating.'
						},
						{
							name: 'Add seed statements',
							description:
								'Adds your own statements to start the conversation. They go straight into **Seeded** and **Accepted**.'
						},
						{
							name: 'All',
							description:
								'Every statement: seed statements, participant statements, edited statements, and those pending, accepted or rejected.'
						},
						{ name: 'Seeded', description: 'The conversation starters you added.' },
						{
							name: 'Accepted',
							description:
								'Seed statements and participant statements accepted for participants to see and vote on.'
						},
						{
							name: 'Pending',
							description: 'Participant statements waiting for a moderation decision.'
						},
						{ name: 'Rejected', description: 'Statements a moderator has rejected.' },
						{
							name: 'Row icons',
							description:
								'Each statement has a pencil :split: to split or reword it, a tick :accept: to accept it, and a cross :reject: to reject it.'
						}
					]
				},
				decisions: {
					title: 'Decisions and how to make them',
					intro: 'Each statement in **Pending** needs a decision and an action. Use your moderation policy to decide. Every action you take is recorded in the moderation log automatically.',
					options: [
						{
							title: 'Publish as written',
							when: 'The statement is clear, on topic, makes a single point and meets your moderation policy.',
							how: ['Click the tick :accept: on the statement’s row.'],
							result: 'The statement moves to **Accepted**. Participants can see and vote on it straight away.'
						},
						{
							title: 'Reject and record a reason',
							when: 'The statement breaks your moderation policy, for example it is harmful, off topic or a duplicate.',
							how: [
								'Click the cross :reject: on the statement’s row to open **Reject statement**.',
								'Choose a **Reason** from your moderation policy (optional).',
								'Add a **Note** (optional), for example the statement it duplicates, or your initials for the log.',
								'Confirm to reject.'
							],
							result: 'The statement moves to **Rejected** and is hidden from participants. The reason and note are kept in the moderation log for your team; participants are not told why.',
							image: {
								src: rejectStatementImage,
								alt: 'The Reject statement popover open on a statement’s row, with the Reason "Off-topic or unclear" chosen, a Note reading "This statement isn’t about trees", and Cancel and Reject buttons.',
								caption:
									'Rejecting an off-topic statement, with a note for the moderation log.'
							}
						},
						{
							title: 'Split or reword',
							when: 'The statement is unclear, or makes more than one point so participants can’t agree or disagree with it in one vote.',
							how: [
								'Click the pencil :split: on the statement’s row to open **Split or reword statement**.',
								'Write the replacement statement. To split it, click **+ Add another statement** for each extra point.',
								'Click **Split statement**.'
							],
							result: 'The new statements are posted and appear in **Accepted** with an **Edited** badge and **Edited from:** the original. The original is rejected. Votes do not carry over to the new statements.',
							image: {
								src: splitStatementImage,
								alt: 'The Split or reword statement dialog. The original statement, "The park needs more benches near the playground and the bins should be emptied more often.", is split into two replacement statements, one about benches and one about bins, with Add another statement, Cancel and Split statement buttons.',
								caption:
									'Splitting a statement that makes two proposals into two separate statements.'
							}
						},
						{
							title: 'Leave in Pending for discussion',
							when: 'You are unsure, and the statement is borderline enough to need a second opinion.',
							how: [
								'Leave the statement in **Pending**.',
								'Let your colleagues know which statement needs discussing, then come back and make the decision together.'
							],
							result: 'Participants don’t see the statement until someone accepts it, as long as the step requires approval (see **Good to know**).'
						}
					]
				},
				reference: {
					title: 'Example moderation policy',
					intro: 'Your reject reasons come from the moderation policy, set under **Configure** → **Moderation policy**. Here is an example you can adapt.',
					items: [
						{
							code: 'A',
							name: 'Harmful, abusive or discriminatory',
							description:
								'Abuse, threats, hate speech, or content that discriminates against a person or group.'
						},
						{
							code: 'B',
							name: 'Privacy and personal information',
							description:
								'Names, contact details or other information that could identify a person.'
						},
						{
							code: 'C',
							name: 'Advertising, campaigning or lobbying',
							description: 'Promotes a product, service, campaign or organisation.'
						},
						{
							code: 'D',
							name: 'Illegal content',
							description: 'Content that breaks the law.'
						},
						{
							code: 'E',
							name: 'Off-topic',
							description: 'Not related to the conversation’s question.'
						},
						{
							code: 'F',
							name: 'Duplicate',
							description:
								'Says the same as a statement already accepted. Note which one in the **Note** field.'
						},
						{
							code: 'G',
							name: 'Unclear',
							description:
								'Hard to understand. Reject it, or reword it if the meaning is clear enough.'
						},
						{
							code: 'H',
							name: 'Multi-theme',
							description:
								'Makes more than one point. Reject it, or split it so each statement can be voted on with a single agree or disagree.'
						}
					]
				},
				tips: [
					'Before you launch, decide whether statements need your approval. In the poll step’s **Setup**, **No comments shown without moderator approval** holds every participant statement in **Pending** until you accept it. Without it, statements are shown straight away and you can only remove them after people may have seen and voted on them.',
					'If statements need approval, check **Pending** often while the conversation is live, or participants will have little to vote on.',
					'A statement only appears in [Insights](/admin/info/glossary#term-insights) and its CSV once it is accepted and has at least one vote.',
					'Preview and the live conversation use separate polls. Statements and votes from preview do not carry over when you launch.'
				]
			},
			{
				key: 'access-settings',
				title: 'Access and participation settings',
				navLabel: 'Access and participation settings',
				summary:
					'Get to know the **Access** settings, then decide who can see your conversation, who can take part, and what people see when they finish. Make these decisions before you launch.',
				image: {
					src: configureAccessImage,
					alt: 'The Access section of Configure Details, with toggles for who can see and take part in the conversation.',
					caption: 'The Access section of Configure Details.'
				},
				environment: {
					title: 'The Access settings',
					intro: 'Go to **Configure** → **Details** and scroll to **Access**. Each setting is a toggle.',
					parts: [
						{
							name: 'Show conversation publicly',
							description:
								'Off by default. Decides who can view the conversation’s documents and data.'
						},
						{
							name: 'Only allow participation by invite',
							description: 'Off by default. Decides who can take part.'
						},
						{
							name: 'Automatically log in with an anonymous account',
							description: 'Off by default. Decides whether people need an account.'
						},
						{
							name: 'Allow revisit after finishing',
							description:
								'On by default. See [Step navigation](/admin/info/how-to/advanced/step-navigation).'
						},
						{
							name: 'Thank-you page settings',
							description:
								'**Enable signup prompts**, **Show thank you page anonymous instructions** and **Show thank you page feedback button**. All on by default.'
						}
					]
				},
				decisions: {
					title: 'Decisions and how to make them',
					options: [
						{
							title: 'Make the conversation’s data public',
							when: 'You want anyone to be able to read the conversation’s documents and results, for example for transparency.',
							how: ['Turn on **Show conversation publicly**.'],
							result: 'Once launched, anyone, even without an account, can open the conversation’s documents and data. This does not change who can take part; for that, see the next decision.'
						},
						{
							title: 'Limit who can take part',
							when: 'Only a specific group should take part, such as a panel or a recruited sample.',
							how: [
								'Turn on **Only allow participation by invite**.',
								'After launch, create invites in **Recruit**.'
							],
							result: 'Only people you invite can take part. If you don’t create invites, no one will be able to join.'
						},
						{
							title: 'Let people take part without an account',
							when: 'You want as few barriers as possible, for example on a public link or a QR code at an event.',
							how: [
								'Turn on **Automatically log in with an anonymous account**.',
								'Keep **Show thank you page anonymous instructions** on.'
							],
							result: 'Visitors get a temporary anonymous account and can upgrade it later. They see their temporary ID on the thank-you page; without it they may not be able to come back to their contributions.'
						},
						{
							title: 'Choose what people see when they finish',
							when: 'You want to adjust the thank-you page.',
							how: [
								'Turn **Enable signup prompts** off if you don’t want to encourage people to create an account.',
								'Turn **Show thank you page feedback button** off if you don’t want feedback on the process.'
							],
							result: 'The thank-you page shows only what you leave on.'
						}
					]
				}
			},
			{
				key: 'step-navigation',
				title: 'Step navigation',
				navLabel: 'Step navigation',
				summary:
					'Three settings decide whether participants can skip steps, go back to them, and return after they have finished. Their effects overlap, so make these decisions together.',
				image: {
					src: stepNavigationImage,
					alt: 'A step’s Configure tab in Process design, with Name, Description and Data protocol fields. Below them, highlighted, are the Revisitable step toggle (Can users revisit this step?), turned off, and the Required step toggle (Can users skip this step?), turned on.',
					caption:
						'A step’s Configure tab, where Required step and Revisitable step are set.'
				},
				environment: {
					title: 'Where the settings are',
					parts: [
						{
							name: 'Required step',
							description:
								'In **Process design** → your step → **Configure**. On by default.'
						},
						{
							name: 'Revisitable step',
							description:
								'In **Process design** → your step → **Configure**. Off by default. Applies while a participant is still working through the conversation.'
						},
						{
							name: 'Allow revisit after finishing',
							description:
								'In **Configure** → **Details**, under **Access**. On by default. Applies to the whole conversation after a participant finishes every step.'
						}
					]
				},
				decisions: {
					title: 'Decisions and how to make them',
					options: [
						{
							title: 'Let participants skip a step',
							when: 'The step is optional, such as extra background reading.',
							how: ['Turn off **Required step** for that step.'],
							result: 'Participants can move on without completing it. Keep steps that later steps depend on required.'
						},
						{
							title: 'Let participants go back to a step',
							when: 'Participants may want to check or change what they did earlier, such as re-reading background before voting.',
							how: ['Turn on **Revisitable step** for that step.'],
							result: 'Participants can return to the step while they are still working through the conversation. After they finish, **Allow revisit after finishing** decides instead.'
						},
						{
							title: 'Close the conversation to people who have finished',
							when: 'Contributions should be final once someone finishes, for example so results can’t shift after the fact.',
							how: [
								'Turn off **Allow revisit after finishing** under **Access** in **Configure** → **Details**.'
							],
							result: 'Participants who finish can’t reach any step, the revisit links disappear, and further contributions are rejected. Adding a step to a live conversation re-opens it for everyone who had already finished.',
							image: {
								src: toggleRevisitAfterFinishImage,
								alt: 'The Access section of Configure Details, with the Allow revisit after finishing toggle turned off, below the thank-you page settings.',
								caption:
									'Allow revisit after finishing, turned off, in the Access section.'
							}
						}
					]
				}
			}
		]
	}
};

export const ADMIN_GUIDE_NAV = Object.values(ADMIN_GUIDES);

/** The anchor id for a glossary term, e.g. 'Seed statement' → 'term-seed-statement'. */
export function glossaryTermId(term: string) {
	return `term-${term.toLowerCase().replace(/[^a-z0-9]+/g, '-')}`;
}

/** Reference page at /admin/info/glossary. Terms are shown A–Z whatever order they're written in. */
export const ADMIN_GLOSSARY = {
	title: 'Glossary',
	summary: 'Words that come up throughout the admin area and this guide.',
	terms: (
		[
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
				term: 'Participant-led poll',
				definition:
					'An engagement tool where participants agree, disagree or pass on short statements, and can write statements of their own for others to vote on.'
			},
			{
				term: 'Statement',
				definition:
					'A short point in a Participant-led poll that participants vote on. A statement should make one point, so people can agree or disagree with it in a single vote.'
			},
			{
				term: 'Seed statement',
				definition:
					'A statement you add yourself to start a Participant-led poll, so participants have something to vote on from the beginning. Seed statements are accepted automatically.'
			},
			{
				term: 'Moderation',
				definition:
					'Reviewing statements that participants write in a Participant-led Poll, and accepting or rejecting them.'
			},
			{
				term: 'Moderation policy',
				definition:
					'The list of reasons, such as Off-topic or Duplicate, that moderators choose from when rejecting a statement. Set it under **Configure** → **Moderation policy**.'
			},
			{
				term: 'Insights',
				definition: 'The responses and results collected in a step.'
			}
		] satisfies AdminGuideTerm[]
	).sort((a, b) => a.term.localeCompare(b.term))
};
