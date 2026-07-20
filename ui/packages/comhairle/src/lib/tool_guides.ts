/**
 * Editorial content for the Comhairle Tools Guide (/admin/info/tools/<key>).
 *
 * Copy for Wiki Poll, Thinking space, Learn and Video call is transcribed from the
 * Figma mockups (obvious typos fixed). Tools without provided copy carry the section
 * scaffold with a "coming soon" stub. Refine copy here — the pages just render it.
 */

export type GuideSection = {
	heading?: string;
	/** Trusted static HTML (rendered with {@html}). */
	html?: string;
	/** Show a placeholder media block. */
	image?: boolean;
};

export type ToolGuide = {
	key: string;
	navLabel: string;
	title: string;
	sections: GuideSection[];
};

const stub = (title: string, navLabel: string, key: string): ToolGuide => ({
	key,
	navLabel,
	title,
	sections: [
		{
			heading: 'What you need to know',
			html: '<p>Detailed guidance for this tool is coming soon.</p>'
		},
		{ heading: 'How it works', html: '<p>Coming soon.</p>' },
		{ heading: 'Mostly used in…', html: '<p>Coming soon.</p>' },
		{ heading: 'Data collection and analysis', html: '<p>Coming soon.</p>' },
		{ heading: 'A typical participant experience', html: '<p>Coming soon.</p>' },
		{ heading: 'How to set this up', html: '<p>Coming soon.</p>' }
	]
});

export const TOOL_GUIDES: Record<string, ToolGuide> = {
	polis: {
		key: 'polis',
		navLabel: 'Wiki Poll (Pol.is)',
		title: 'Wiki Poll (Pol.is)',
		sections: [
			{
				heading: 'What you need to know',
				html: `<p>Wiki Poll (<u>Pol.is</u>) is a crowd survey tool that lets participants input their views and vote agree/pass/disagree on others' contributions. This enables understanding what opinion groups there are for a given topic, what representative views these groups hold, and importantly, revealing shared common ground across opinion groups.</p>
<p>It is mostly used when the organiser seeks to discover the starting point of reaching common ground of a controversial topic with complex stakeholder groups. (See case study)</p>
<p>Its built-in feature of opinion groups discovery was also referred to as very useful for early stage consultations, especially its ability to reveal what views people might have given a topic.</p>`
			},
			{ image: true },
			{
				heading: 'How it works',
				html: `<p>Wiki Poll (<u>Pol.is</u>) is statement based; it lets participants vote agree/pass/disagree on others' statement contributions. The statements are in text form and limited to no more than 140 words. Participants are presented with statements others made and asked to vote 'Agree' if they agree, 'Disagree' if not fully agreed, and 'Pass/Skip' if neither.</p>
<p>Participants are able to input their views to this wiki-styled poll (where the statements under polling are crowdsourced). They can do this anytime while interacting with Wiki Poll (<u>Pol.is</u>), including while casting their votes on others' statements.</p>
<p>The data of participant votes on each statement enable discovery of opinion groups (forming participant clusters of who voted similarly), and their respective representative opinion. Importantly, this collective data also reveals what their shared understandings might be across opinion groups (identifying bridging opinions capturing the same votes across participants from different opinion groups).</p>`
			},
			{
				heading: 'Mostly used in…',
				html: `<p>Wiki Poll is mostly used in topics which contain complex stakeholder groups and are anticipated to have controversial opinions. Being able to find common ground helps identify a starting point for further collaborative and constructive discussion. Therefore, Wiki Poll is often seen used before an in-person discussion, which helps ease the tension between formerly opposing opinion groups.</p>`
			},
			{
				heading: 'Data collection and analysis',
				html: `<p>When running a Wiki Poll (<u>Pol.is</u>), participants should be informed that their statement and votes data will be captured and used for analysis.</p>`
			},
			{
				heading: 'A typical participant experience',
				html: `<p>Participants typically interact with Wiki Poll for about 10 to 15 minutes and in this time they go through about 20 statements and perhaps add one or two of their own statements to the poll. Comhairle provides an option for organisers to configure a minimum number of statements each participant should go through, before they can move on to the next step of the end-to-end engagement process.</p>`
			},
			{
				heading: 'How to set this up',
				html: `<p>Setting up a Wiki Poll is extremely easy. Setting up a Wiki Poll typically takes organisers about 15 to 30 minutes adding content and configuring settings when contents are ready.</p>
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
				html: `<p>Wiki Poll (<u>Pol.is</u>) is powered by an open source civic tech tool named Pol.is. Polis is created and stewarded by the Computational Democracy Project, and is a groundbreaking open-source platform for collective intelligence. It allows groups to contribute statements, vote agree/pass/disagree on others' contributions, and visualise where consensus and differences lie.</p>`
			}
		]
	},

	thinking_space: {
		key: 'thinking_space',
		navLabel: 'Thinking space',
		title: 'Thinking space',
		sections: [
			{
				heading: 'What you need to know',
				html: `<p>Thinking space is a conversational tool that asks participants questions that expand and strengthen views they might have on a given topic. This helps participants find their stance or build their articulation in a natural, dialogue-based way. This tool uses an LLM that generates good coaching, non-guiding and context-aware questions which help participants elicit their own thinking and identify blind spots if any.</p>
<p>It's useful when a topic touches ethical, value or principle level questions, such as "Lower voting age to 16." It is also helpful when a topic is too distant to some participants who might find it difficult to come up with their own views while navigating a topic they hardly thought of before, such as "Space sector policy".</p>
<p>It's reported by organisers that it's helpful to be used right after a learning step which onboards participants about a complex topic, or right before an in-person workshop so the participants at least think about the topic to a certain degree. Some organisers prefer to consider this thinking space as a private reflection space for participants; some prefer to keep it as a collective conversation space where a view from one participant could be viewed by others, which sparks discussion.</p>`
			},
			{ image: true },
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
		sections: [
			{
				heading: 'What you need to know',
				html: `<p>Learning step is a step for organisers to provide content for participants to review. Content here can include paragraphs and rich media (video, audio, images). Organisers are provided with a rich media editor to curate and arrange content.</p>
<p>It's commonly the very first step of a digital engagement, onboarding participants with topic-related content and context that they should know about.</p>
<p>Sometimes, a digital engagement could use a Learn step to provide interim guidance as participants contribute. So the Learning step can be used versatilely, at any moment for organisers to provide content or guidance, and not limited to topic onboarding at the beginning.</p>
<p>The Media library that comes with the Learning step is a place where organisers can upload files to support their content. Be cautious that these files are shared across conversations within the organisation. Therefore, when uploading media files, be aware that they can be viewed by other organisers in your organisation.</p>
<p>The Learn step also comes with an optional feature, Learning Assistant, that the organiser can choose to switch on.</p>`
			},
			{ image: true },
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
		sections: [
			{
				heading: 'What you need to know',
				html: `<p>Video Call is a structured synchronous discussion tool that lets participants meet live to deliberate on a topic, rather than contributing asynchronously as with text-based tools. Conversation hosts configure and run the call, facilitators guide participants through an agenda, and participants join to take part in real time.</p>
<p>It is mostly used when the organiser wants participants to talk to each other directly, work through disagreement in the moment, or build on the common ground surfaced by earlier tools (such as Wiki Poll) through live discussion.</p>
<p>Its built-in support for breakout rooms, real-time transcription, and upcoming features such as embedded interactive tools (e.g. polling mid-call) also makes it useful for running an entire structured deliberation event within a single session.</p>`
			},
			{ image: true },
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

	heyform: stub('Survey', 'Survey', 'heyform'),
	prioritization: stub('Prioritisation tool', 'Prioritisation tool', 'prioritization'),
	elicitation_bot: stub('Elicitation Bot', 'Elicitation Bot', 'elicitation_bot'),
	lived_experience: stub('Lived Experience', 'Lived Experience', 'lived_experience')
};

/** Order of tools in the guide's left navigation. */
export const GUIDE_NAV_ORDER = [
	'polis',
	'thinking_space',
	'learn',
	'heyform',
	'prioritization',
	'elicitation_bot',
	'lived_experience',
	'online_group_conversation'
];

export const GUIDE_NAV = GUIDE_NAV_ORDER.map((k) => TOOL_GUIDES[k]).filter(Boolean);
