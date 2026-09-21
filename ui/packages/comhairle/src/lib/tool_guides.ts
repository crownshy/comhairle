const guideImages = import.meta.glob(
	'./assets/tools-{polis,thinkingspace,learnstep,video,prioritisation,survey}.{png,jpg,jpeg,webp,avif}',
	{
		eager: true,
		query: '?url',
		import: 'default'
	}
) as Record<string, string>;

const polisGuideImage = guideImages['./assets/tools-polis.png'];
const thinkingSpaceGuideImage = guideImages['./assets/tools-thinkingspace.png'];
const learnStepGuideImage = guideImages['./assets/tools-learnstep.png'];
const videoGuideImage = guideImages['./assets/tools-video.png'];
const prioritisationGuideImage = guideImages['./assets/tools-prioritisation.png'];
const surveyGuideImage = guideImages['./assets/tools-survey.png'];

/**
 * Editorial content for the Comhairle Tools Guide (/admin/info/tools/<key>).
 *
 * Copy for Wiki Poll, Thinking space, Learn and Video call is transcribed from the
 * Figma mockups (obvious typos fixed). Tools without provided copy carry the section
 * scaffold with a "coming soon" stub. Refine copy here — the pages just render it.
 */

const polisTitle = 'Participant-led Poll';

export type GuideSection = {
	heading?: string;
	/** Trusted static HTML (rendered with {@html}). */
	html?: string;
	/** Show a placeholder media block if image doesn't exist, otherwise displace the assigned image. */
	image?: {
		src?: string;
		alt?: string;
	};
};

export type ToolGuide = {
	key: string;
	navLabel: string;
	title: string;
	atAGlance?: {
		bestFor: string;
		participantTime: string;
		setupTime: string;
	};
	sections: GuideSection[];
};

// we probably don't need these anymore
// const stub = (title: string, navLabel: string, key: string): ToolGuide => ({
// 	key,
// 	navLabel,
// 	title,
// 	sections: [
// 		{
// 			heading: 'What you need to know',
// 			html: '<p>Detailed guidance for this tool is coming soon.</p>'
// 		},
// 		{ heading: 'How it works', html: '<p>Coming soon.</p>' },
// 		{ heading: 'Mostly used in…', html: '<p>Coming soon.</p>' },
// 		{ heading: 'Data collection and analysis', html: '<p>Coming soon.</p>' },
// 		{ heading: 'A typical participant experience', html: '<p>Coming soon.</p>' },
// 		{ heading: 'How to set this up', html: '<p>Coming soon.</p>' }
// 	]
// });

export const TOOL_GUIDES: Record<string, ToolGuide> = {
	polis: {
		key: 'polis',
		navLabel: polisTitle,
		title: polisTitle,
		atAGlance: {
			bestFor: 'Complex stakeholder views',
			participantTime: '10 to 15 minutes',
			setupTime: '15 to 30 minutes'
		},
		sections: [
			{
				heading: 'What you need to know',
				html: `<p>${polisTitle} is a crowd survey tool that lets participants input their views and vote agree/pass/disagree on others' contributions. This enables understanding what opinion groups there are for a given topic, what representative views these groups hold, and importantly, revealing shared common ground across opinion groups.</p>
<p>It is mostly used when the organiser seeks to discover the starting point of reaching common ground of a controversial topic with complex stakeholder groups. (See case study)</p>
<p>Its built-in feature of opinion groups discovery was also referred to as very useful for early stage consultations, especially its ability to reveal what views people might have given a topic.</p>`
			},
			{
				image: {
					src: polisGuideImage,
					alt: 'Participant-led Poll interface'
				}
			},
			{
				heading: 'How it works',
				html: `<p>${polisTitle} is statement based; it lets participants vote agree/pass/disagree on others' statement contributions. The statements are in text form and limited to no more than 140 words. Participants are presented with statements others made and asked to vote 'Agree' if they agree, 'Disagree' if not fully agreed, and 'Pass/Skip' if neither.</p>
<p>Participants are able to input their views to this wiki-styled poll (where the statements under polling are crowdsourced). They can do this anytime while interacting with ${polisTitle}, including while casting their votes on others' statements.</p>
<p>The data of participant votes on each statement enable discovery of opinion groups (forming participant clusters of who voted similarly), and their respective representative opinion. Importantly, this collective data also reveals what their shared understandings might be across opinion groups (identifying bridging opinions capturing the same votes across participants from different opinion groups).</p>`
			},
			{
				heading: 'Mostly used in…',
				html: `<p>${polisTitle} is mostly used in topics which contain complex stakeholder groups and are anticipated to have controversial opinions. Being able to find common ground helps identify a starting point for further collaborative and constructive discussion. Therefore, ${polisTitle} is often seen used before an in-person discussion, which helps ease the tension between formerly opposing opinion groups.</p>`
			},
			{
				heading: 'A typical participant experience',
				html: `<p>Participants typically interact with ${polisTitle} for about 10 to 15 minutes and in this time they go through about 20 statements and perhaps add one or two of their own statements to the poll. Comhairle provides an option for organisers to configure a minimum number of statements each participant should go through, before they can move on to the next step of the end-to-end engagement process.</p>`
			},
			{
				heading: 'How to set this up',
				html: `<p>Setting up a ${polisTitle} is extremely easy. Setting up a ${polisTitle} typically takes organisers about 15 to 30 minutes adding content and configuring settings when contents are ready.</p>
<p>Organisers will need to prepare the following:</p>
<ul>
<li>A short overview description of the topic (about 50 words)</li>
<li>A short instruction guiding how participants could contribute their views (about 50 words)</li>
<li>A set of seed statements (about 10 to 15 of them)</li>
</ul>
<p>Organisers will also need to decide:</p>
<ul>
<li>Whether this poll is open to the public or shared amongst invited participants only.</li>
<li>Whether a minimum number of statements each participant should go through, and how many of them.</li>
</ul>`
			},
			{
				heading: 'The open source tool we use: Pol.is',
				html: `<p>${polisTitle} is powered by an open source civic tech tool named Pol.is. Polis is created and stewarded by the Computational Democracy Project, and is a groundbreaking open-source platform for collective intelligence. It allows groups to contribute statements, vote agree/pass/disagree on others' contributions, and visualise where consensus and differences lie.</p>`
			}
		]
	},

	thinking_space: {
		key: 'thinking_space',
		navLabel: 'Thinking space',
		title: 'Thinking space',
		atAGlance: {
			bestFor: 'Private or shared reflection',
			participantTime: '10 to 15 minutes',
			setupTime: '10 to 20 minutes'
		},
		sections: [
			{
				heading: 'What you need to know',
				html: `<p>Thinking space is a conversational tool that asks participants questions that expand and strengthen views they might have on a given topic. This helps participants find their stance or build their articulation in a natural, dialogue-based way. This tool uses an LLM that generates good coaching, non-guiding and context-aware questions which help participants elicit their own thinking and identify blind spots if any.</p>
<p>It's useful when a topic touches ethical, value or principle level questions, such as "Lower voting age to 16." It is also helpful when a topic is too distant to some participants who might find it difficult to come up with their own views while navigating a topic they hardly thought of before, such as "Space sector policy".</p>
<p>It's reported by organisers that it's helpful to be used right after a learning step which onboards participants about a complex topic, or right before an in-person workshop so the participants at least think about the topic to a certain degree. Some organisers prefer to consider this thinking space as a private reflection space for participants; some prefer to keep it as a collective conversation space where a view from one participant could be viewed by others, which sparks discussion.</p>`
			},
			{
				image: {
					src: thinkingSpaceGuideImage,
					alt: 'Thinking space interface'
				}
			},
			{
				heading: 'How it works',
				html: `<p>Thinking space is questions-and-responses based. An LLM is prompted to ask coaching questions based on a topic and some questions the organiser sets up; the LLM generates follow-up questions according to the user's response and the intention behind why the organiser set up those questions.</p>
<p>A few types of follow-up questions include (but are not limited to):</p>
<ul>
<li><strong>View-expanding questions:</strong> ask questions that inspire participants to think from various perspectives.</li>
<li><strong>Decision-relevant questions:</strong> similar to an interview, ask questions that link to the heart of what the organisers running the conversation really want to know.</li>
<li><strong>Critical questions:</strong> ask questions that challenge ideas and examine consequences or assumptions.</li>
</ul>
<p>At the end of the experience, the Thinking Space gives a summary of what the participant's view might be based on the back and forth interactions, which the participant can approve or edit to their liking.</p>`
			},
			{
				heading: 'Mostly used in…',
				html: `<p>Thinking space is mostly used in topics which have multiple sub-level aspects to explore. Being able to explore and learn about a topic through a conversation interface gives participants opportunities to try out initial views they might have picked up from learning about the topic. The coaching questions help them think deeper and consider various views other people might have, and ponder their personal preferences or disposition.</p>`
			},
			{
				heading: 'Data collection and analysis',
				html: `<p>Thinking space can be used as a private mode for individual participants to keep their own views to themselves, or could be configured as a collaborative mode where their responses would be integrated as part of the collective intelligence.</p>
<p>Either way, the organiser needs to be clear about how data will be used, and participants should be offered the ability to take part with a normal account (where their email is stored) or an anonymous account (where their personal information is not stored).</p>`
			},
			{
				heading: 'A typical participant experience',
				html: `<p>Participants typically interact with Thinking Space for about 10 to 15 minutes and in this time go through about 2 to 6 questions, highly dependent on how much time they need to answer each question. Comhairle provides an option for organisers to configure a minimum number of follow-up questions each participant should go through, before they can move on to the next interview question prompted by the organiser.</p>`
			},
			{
				heading: 'How to set this up',
				html: `<p>Setting up a Thinking Space is fairly simple. Setting up a Thinking Space typically takes organisers about 10 to 20 minutes adding content and configuring settings when contents are ready.</p>
<p>Organisers will need to prepare the following:</p>
<ul>
<li>A set of interview questions they want participants to go through (about 10–20 words each; we recommend at least 2 and no more than 6 interview questions).</li>
<li>For each interview question, an explanation of why they are asked or what is hoped to get out of asking them (about 10 to 20 words for each explanation).</li>
<li>A handful of documents that inform the Knowledge space for question generation.</li>
</ul>
<p>Organisers will also need to decide:</p>
<ul>
<li>Whether a minimum number of follow-up questions each participant should go through, and how many of them.</li>
</ul>`
			},
			{
				heading: 'The open source tool we use: Ragflow',
				html: `<p>Thinking Space is powered by an open source tool named RagFlow, a leading Retrieval-Augmented Generation (RAG) engine and agent orchestration engine designed for deep document understanding. Behind each Thinking Space there is a Knowledge base that informs the generation of the follow-up questions to be relevant to the topic.</p>`
			}
		]
	},

	learn: {
		key: 'learn',
		navLabel: 'Learn step',
		title: 'Learning step (Rich content page)',
		atAGlance: {
			bestFor: 'Topic onboarding',
			participantTime: '5 to 45 minutes',
			setupTime: '15 to 30 minutes once content is ready'
		},
		sections: [
			{
				heading: 'What you need to know',
				html: `<p>Learning step is a step for organisers to provide content for participants to review. Content here can include paragraphs and rich media (video, audio, images). Organisers are provided with a rich media editor to curate and arrange content.</p>
<p>It's commonly the very first step of a digital engagement, onboarding participants with topic-related content and context that they should know about.</p>
<p>Sometimes, a digital engagement could use a Learn step to provide interim guidance as participants contribute. So the Learning step can be used versatilely, at any moment for organisers to provide content or guidance, and not limited to topic onboarding at the beginning.</p>
<p>The Media library that comes with the Learning step is a place where organisers can upload files to support their content. Be cautious that these files are shared across conversations within the organisation. Therefore, when uploading media files, be aware that they can be viewed by other organisers in your organisation.</p>
<p>The Learn step also comes with an optional feature, Learning Assistant, that the organiser can choose to switch on.</p>`
			},
			{
				image: {
					src: learnStepGuideImage,
					alt: 'learn step interface'
				}
			},
			{
				heading: 'How it works',
				html: `<p>How the Learn step works is quite straightforward; it works similarly to a blog content builder where editors can create articles with optional visual or other rich media content. Our Learn step's rich media editor is very powerful. It allows editors not only to upload rich media but also to attach supporting documents in between articles if desired.</p>
<p>Participants going through content provided in a Learn step are able to view the content as if they were reading a blog post. If the Learning Assistant is switched on (by the organiser), participants can use it to ask questions and learn interactively.</p>`
			},
			{
				heading: 'Mostly used in…',
				html: `<p>Learn step is mostly used in topic onboarding at the beginning of a digital online engagement. Its rich functions also enable flexibility to be used in between other steps to guide participants slightly more intentionally, which sometimes is used to provide extra context before participants dive into other steps such as a Survey or a Wiki Poll (Pol.is).</p>`
			},
			{
				heading: 'Data collection and analysis',
				html: `<p>No personal data is traced in the Learn step. If the Learning Assistant is switched on, the questions participants ask are captured. Organisers should then decide and inform the data sharing protocol.</p>
<ul>
<li><strong>Private —</strong> only participants can access.</li>
<li><strong>Limited —</strong> only participants and organisers can access.</li>
<li><strong>Collaborative —</strong> participants themselves, organisers and other participants can access (not identifiable).</li>
<li><strong>Open —</strong> everyone can access (not identifiable).</li>
</ul>`
			},
			{
				heading: 'A typical participant experience',
				html: `<p>Depending on reading speed and habit, as well as the amount of reading content organisers provide, participants typically interact with the Learn step for about 5 to 45 minutes, on average 3 to 6 minutes per page. We recommend that organisers consider providing learning material moderately and not overwhelm participants with too much material before they can contribute.</p>`
			},
			{
				heading: 'How to set this up',
				html: `<p>Setting up a Learn step is straightforward. Organisers often assign an Editor to set up a Learn step. Because preparing a Learn step is relatively collaborative and requires the organisation team's iterative discussion, it typically takes a couple of days to really finalise the content. But when the content is ready, it typically takes about 15 to 30 minutes to check the content and bring it into content pages.</p>
<p>Organisers (or Editors) will often prepare the following:</p>
<ul>
<li>Onboarding materials (about 500 words)</li>
<li>(optional) A few images</li>
<li>(optional) A video / audio file</li>
</ul>
<p>Organisers will also need to decide:</p>
<ul>
<li>Whether to switch on the Learning Assistant, and if so, configure the data sharing protocol for the Learning Assistant.</li>
</ul>`
			},
			{
				heading: 'The open source tool we use: Tiptap',
				html: `<p>Learn step is powered by <u>Tiptap</u>, an open-source, headless rich-text editor framework favoured by web developers. It provides no default user interface, giving developers complete control to build custom WYSIWYG editing experiences (like Notion or Google Docs) that perfectly match their own design systems and tech stacks.</p>`
			}
		]
	},

	online_group_conversation: {
		key: 'online_group_conversation',
		navLabel: 'Video call',
		title: 'Video call',
		atAGlance: {
			bestFor: 'Live deliberation',
			participantTime: '45 to 120 minutes',
			setupTime: 'Depends on the agenda'
		},
		sections: [
			{
				heading: 'What you need to know',
				html: `<p>Video Call is a structured synchronous discussion tool that lets participants meet live to deliberate on a topic, rather than contributing asynchronously as with text-based tools. Conversation hosts configure and run the call, facilitators guide participants through an agenda, and participants join to take part in real time.</p>
<p>It is mostly used when the organiser wants participants to talk to each other directly, work through disagreement in the moment, or build on the common ground surfaced by earlier tools (such as Wiki Poll) through live discussion.</p>
<p>Its built-in support for breakout rooms, real-time transcription, and upcoming features such as embedded interactive tools (e.g. polling mid-call) also makes it useful for running an entire structured deliberation event within a single session.</p>`
			},
			{
				image: {
					src: videoGuideImage,
					alt: 'video interface'
				}
			},
			{
				heading: 'How it works',
				html: `<p>Video Call is session based. A conversation host sets up a call with a start date/time, end time, and a name and description of what the call is about. Hosts can also assign a facilitator, estimate expected capacity, decide whether to use breakout rooms, and choose whether an agenda is shown to participants during the call.</p>
<p>Participants sign up ahead of time, either by direct invitation or while taking part in the wider Comhairle conversation. They can cancel their signup or switch to a different call availability if the host has offered more than one time slot.</p>
<p>Once the call starts, the facilitator can move the group through the agenda, open breakout rooms, and moderate the session (including removing participants who misbehave).</p>
<p>Real-time transcription captures what is said and helps make participants' points more legible as the conversation unfolds. On our roadmap, we aim to provide hosts with the ability to embed other interactive tools directly into the call — for example setting aside 10 minutes for participants to prioritise a set of proposals, or to use a Wiki Poll, as part of the live session. After the call, a summary is generated, including a transcript with separate audio channels per speaker.</p>`
			},
			{
				heading: 'Mostly used in…',
				html: `<p>Video Call is mostly used for topics that benefit from live, synchronous discussion — for example where participants need to hear each other directly, ask questions in real time, or work through nuance that is hard to capture in a written statement. It is often used after an asynchronous tool like Wiki Poll has already surfaced opinion groups and common ground, so the live call can focus discussion on bridging remaining differences.</p>`
			},
			{
				heading: 'Data collection and analysis',
				html: `<p>When running a Video Call, participants should be informed that the call will be transcribed (main room, and breakout rooms where enabled) and that the resulting transcript, audio channels, and any votes or inputs made through embedded tools during the call will be captured and used for analysis.</p>
<p>The default data sharing protocol is set to open for transcriptions. We so far do not track the user id for each transcription snippet, so it's technically not possible to trace or identify what was said by whom. However, if a participant's name was mentioned, it might be guessable what views they gave.</p>`
			},
			{
				heading: 'A typical participant experience',
				html: `<p>Participants sign up for a call in advance, either through an email invitation or while taking part in the related Comhairle conversation. On the day, they join the live call and take part in the agenda set by the facilitator (which may include breakout room discussions and short interactive activities like a mid-call poll). A typical call runs from 45 minutes to 120 minutes, depending on the agenda. The size of a video call could range from around 6 to 8 people for small group discussion, to 20–40 people for topic-based deliberation, which usually includes breakout rooms. Each participant can access the call summary afterwards.</p>`
			},
			{
				heading: 'How to set this up',
				html: `<p>Conversation hosts will need to prepare the following:</p>
<ul>
<li>Name and description of the call (required)</li>
<li>Start date/time, including any additional call availabilities (required)</li>
<li>End time (required)</li>
<li>Assign a facilitator (required)</li>
</ul>
<p>Hosts will also need to decide:</p>
<ul>
<li>Whether to use breakout rooms (and, if so, whether each room should be transcribed separately)</li>
<li>Whether to display an agenda to participants during the call</li>
<li>Whether the call is open to anyone who can find it on the platform, or invite-only</li>
<li>Who to invite, and how</li>
</ul>`
			},
			{
				heading: 'The open source tool we use: Jitsi',
				html: `<p><u>Jitsi</u> is a secure, 100% open-source video conferencing platform. It lets you host high-quality online meetings, webinars, and audio calls directly from a web browser or mobile app with no accounts required. Organizations use it to retain full control over their communication data. Our Video call tool is designed with deliberation in mind: it is made convenient for facilitators to support participants in different breakout rooms quickly (as if going around tables in a world café workshop), and to capture themes discussed in a meeting for planning next steps.</p>`
			}
		]
	},
	survey: {
		key: 'survey',
		navLabel: 'Survey',
		title: 'Survey',
		atAGlance: {
			bestFor: 'Collecting structured feedback',
			participantTime: '5 to 15 minutes',
			setupTime: '15 to 30 minutes once content is ready'
		},
		sections: [
			{
				heading: 'What you need to know',
				html: "<p>Survey is a structured data collection tool that lets organisers ask participants a series of questions and collect their responses in a consistent format. Unlike discussion-based tools, where participants respond to and build on each other\’s contributions, Survey is primarily designed to gather individual views, experiences, preferences, or demographic information. It is mostly used when the organiser wants to collect input from a larger number of participants, understand patterns across a group, or gather information before or after another stage of a Comhairle engagement. Surveys can include a mixture of question types, such as multiple choice, single choice, scales, rankings, and free-text responses. This makes them useful both for collecting structured quantitative data and for giving participants opportunities to explain their views in their own words. Survey can also be used alongside other Comhairle tools. For example, an organiser might use a survey at the beginning of an engagement to understand participants\’ starting views, use a Participant-led Poll or discussion to explore those views in more depth, and then use a second survey to understand whether participants' views have changed.</p>"
			},
			{
				image: {
					src: surveyGuideImage,
					alt: 'survey interface'
				}
			},
			{
				heading: 'How it works',
				html: '<p>Survey is form based. A conversation host creates a survey by giving it a name and description, then adds the questions they want participants to answer. For each question, hosts can choose the appropriate response format. Depending on the question, this might include selecting one answer, selecting multiple answers, choosing a position on a scale, ranking options, or entering a written response. Hosts can decide which questions are required and which are optional. They can also organise questions into sections where a longer survey needs to be broken into different themes or stages. Once the survey is published, participants can access it through the relevant Comhairle conversation or through an invitation to the entire conversation. They work through the questions at their own pace and submit their responses when they have completed the survey. The survey responses are stored as structured data, allowing organisers to review individual responses as well as analyse patterns across the participant group. Depending on the question type, responses can be summarised through counts, distributions, averages, or other appropriate analysis. Where free-text questions are included, written responses can also be reviewed and analysed for common themes, ideas, or areas of disagreement.</p>'
			},
			{
				heading: 'Mostly used in…',
				html: "<p>Survey is mostly used when the organiser needs to collect structured information from participants at scale, particularly where responses need to be compared across people or groups. </p> Common uses include: <ul><li> Understanding participants' existing views before a deliberative process </li> <li>Collecting demographic or background information</li> <li>Asking participants to prioritise or choose between options</li><li>Measuring changes in views before and after an engagement</li><li>Collecting ideas or suggestions through open-ended questions</li> </ul> Survey can be particularly useful as part of a wider deliberative process. For example, a survey could establish participants' initial views, followed by a Participant-led Poll or discussion to explore the issue, and then a final survey to capture participants' views after deliberation.</p>"
			},

			{
				heading: 'A typical participant experience',
				html: '<p>Participants typically interact with a Survey for around 5 to 15 minutes, depending on the number and complexity of questions. A short survey might contain around 5 to 10 questions and take only a few minutes to complete. A longer survey may contain 20 or more questions, particularly where demographic information and several different topics are being covered. We suggest the survey to be less than 10 questions. Comhairle provides an option for organisers to configure required questions, meaning participants may need to answer certain questions before they can submit their response. Participants normally access the survey through the relevant Comhairle conversation and commonly a topic learn page would be provided before participants entering a survey step. They work through the questions in sequence, selecting or entering their responses, and submit the survey at the end. Once submitted, participants won’t be able to go back and change their answers. The experience is generally asynchronous, meaning participants do not need to be online at the same time as other participants. This makes Survey suitable for reaching participants who have different schedules or who need more time to consider their answers.</p>'
			},
			{
				heading: 'How to set this up',
				html: '<p>Conversation hosts will need to prepare the following: <ul><li>Name and description of the survey (required)</li><li>Questions and response options (required)</li><li>Question types, such as multiple choice, scales, rankings, or free text</li><li>Which questions are required and which are optional</li><li>Any sections or grouping of questions</li><li>The introduction or instructions participants should see before starting</li><li>The survey question logic for questions dependent on answer of a previous question </li></ul>Organisers will also need to decide: <ul><li>How the responses will be analysed and reported</li></ul></p>'
			},
			{
				heading: 'The open source tool we use: HeyForm',
				html: '<p><a target="_new" href="https://heyform.net/">HeyForm (open a new tab)</a> is an open-source form and survey platform that provides the underlying functionality for Comhairle\'s Survey tool. It supports a range of question types and form configurations, allowing organisers to create structured surveys and collect responses through a web-based interface. Using HeyForm as the underlying technology allows Comhairle to provide flexible survey functionality while integrating it into the wider deliberation journey. Rather than treating the survey as a standalone questionnaire, Comhairle can use it alongside other engagement tools to collect information at different stages of a participant\'s journey.The Survey tool is therefore designed not just for collecting responses, but for making those responses useful within a broader process of participation and deliberation.</p>'
			}
		]
	},
	prioritization: {
		key: 'prioritization',
		navLabel: 'Prioritization tool',
		title: 'Prioritization tool',
		atAGlance: {
			bestFor: 'Prioritising a set of proposals',
			participantTime: '10 to 20 minutes',
			setupTime: '30 to 60 minutes once content is ready'
		},
		sections: [
			{
				heading: 'What you need to know',
				html: '<p>Prioritisation is a structured tool that lets organisers present participants with a set of proposals and ask them to give their views on each one. Rather than asking participants to develop proposals themselves, the tool is designed to help participants review, assess, and prioritise proposals that have already been developed. Organisers can configure the questions participants are asked about each proposal. For example, they might ask participants how strongly they agree or disagree with a proposal, how important the underlying issue is for them, and whether they have any comments or suggested changes. The tool is particularly useful as a second stage of engagement. An organiser might first run a discussion, consultation, or deliberative process, then use what was heard to produce a more concise set of proposals. Participants can then return to review those proposals and indicate which ones they support or consider most important. It can also be used when proposals have come from a wider public discussion and the organiser wants a specific group — such as stakeholders, practitioners, community representatives, or subject experts — to review them and provide feedback.</p>'
			},
			{
				image: {
					src: prioritisationGuideImage,
					alt: 'survey interface'
				}
			},
			{
				heading: 'How it works',
				html: '<p>The organiser creates a set of proposals and defines the questions participants will answer about them. Each proposal can include a title, description, and supporting information to help participants understand what is being proposed. The organiser can then configure the response format for each question. This might include an agreement scale, a rating, a choice between options, or a free-text comment. Participants work through the proposals and provide their views. Depending on the configuration, they may be able to comment on individual proposals as well as provide a structured response. Responses are collected across all proposals, allowing organisers to compare how participants responded to different proposals and identify areas of support, disagreement, or uncertainty.</p>'
			},
			{
				heading: 'Mostly used in…',
				html: '<p>Prioritisation is mostly used after an initial stage of engagement, when an organiser has enough information to turn discussion and feedback into a defined set of proposals. Typical uses include: <br><ul><li>Reviewing proposals developed through a deliberative process</li><li>Asking participants to respond to recommendations that have been developed from earlier discussions</li><li>Prioritising ideas generated through a public consultation</li><li>Testing whether participants agree with a proposed set of actions</li><li>Asking stakeholders or experts to review proposals</li><li>Giving participants an opportunity to comment on how earlier feedback has been translated into proposals</li><li>Comparing levels of support across a defined set of options</li></ul>The tool can therefore help create a clear link between what participants said earlier and what happens next. Organisers can explain how earlier contributions have informed the proposals, and then give participants an opportunity to respond to the resulting set.</p>'
			},

			{
				heading: 'A typical participant experience',
				html: '<p>Participants typically spend around 10–20 minutes working through a set of proposals.</p> <br><p>A typical activity might contain 5-7 proposals, with participants asked one or two questions about each. For example, they might indicate how strongly they agree with each proposal and then optionally provide a comment explaining their response. </p><br> <p> Participants can work through the proposals at their own pace and submit their responses once they have completed the activity. </p><br> <p> For a second-round engagement, organisers may provide participants with a summary of the earlier discussion or an explanation of how the proposals were developed before asking them to provide their views.</p>'
			},
			{
				heading: 'How to set this up',
				html: '<p>Conversation hosts will need to prepare: <ul><li>Name and description of the activity (required)</li><li>The set of proposals participants will review (required)</li><li>Questions for each proposal (required)</li><li>Response formats, including agreement scales, choices, or free text</li><li>Supporting information or context participants need to understand each proposal</li></ul>Hosts will also need to decide:<ul><li>Whether the proposals are displayed in order or at random for participants</li><li>Which questions are required and which are optional</li><li>How the results will be analysed and presented alongside earlier engagement</li></ul> Before launching the activity, organisers should make sure that proposals are clearly worded, distinct from one another, and sufficiently developed for participants to give a meaningful response.</p>'
			}
		]
	}
};

/** Order of tools in the guide's left navigation. */
export const GUIDE_NAV_ORDER = [
	'polis',
	'thinking_space',
	'learn',
	'survey',
	'prioritization',

	'online_group_conversation'
];

export const GUIDE_NAV = GUIDE_NAV_ORDER.map((k) => TOOL_GUIDES[k]).filter(Boolean);
