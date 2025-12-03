<script module>
	import ChatBot from '../lib/components/ChatBot.svelte';
	import { defineMeta } from '@storybook/addon-svelte-csf';
	import { fn } from '@storybook/test';
	import '../app.css';

	const { Story } = defineMeta({
		title: 'Components/ChatBot',
		component: ChatBot,
		tags: ['autodocs'],
		argTypes: {
			title: { control: 'text' },
			subtitle: { control: 'text' },
			placeholder: { control: 'text' },
			messages: { control: 'object' },
			initialQuestions: { control: 'object' },
			showInitialQuestions: { control: 'boolean' }
		},
		args: {
			onSendMessage: fn(),
			onQuestionClick: fn(),
			title: 'Deep dive',
			subtitle: 'Try answer some questions from Comhra and explore your views.',
			placeholder: 'Type here',
			showInitialQuestions: true
		}
	});

	const defaultMessages = [
		{
			id: "1",
			content: "Hello! I'm here to help you explore topics and share different perspectives. What would you like to discuss?",
			isBot: true,
			timestamp: new Date()
		}
	];

	const singleMessageExample = [
		{
			id: "1",
			content: "Hello! I'm here to help you explore this topic. What would you like to know?",
			isBot: true,
			timestamp: new Date()
		}
	];

	const emptyMessages = [];

	const conversationExample = [
		{
			id: "1",
			content: "Hello! I'm here to help you explore topics and share different perspectives. What would you like to discuss?",
			isBot: true,
			timestamp: new Date(Date.now() - 300000) // 5 minutes ago
		},
		{
			id: "2",
			content: "I'm interested in learning about climate change policies.",
			isBot: false,
			timestamp: new Date(Date.now() - 240000) // 4 minutes ago
		},
		{
			id: "3",
			content: "That's a really interesting perspective! Can you tell me more about what led you to that view?",
			isBot: true,
			timestamp: new Date(Date.now() - 180000) // 3 minutes ago
		},
		{
			id: "4",
			content: "I've been reading about renewable energy and its impact on the economy.",
			isBot: false,
			timestamp: new Date(Date.now() - 120000) // 2 minutes ago
		},
		{
			id: "5",
			content: "Thanks for that insight. What aspects of this topic do you find most important?",
			isBot: true,
			timestamp: new Date(Date.now() - 60000) // 1 minute ago
		}
	];

	const customQuestions = [
		{ id: "1", text: "What are the main challenges?", variant: "default" },
		{ id: "2", text: "How does this affect me?", variant: "default" },
		{ id: "3", text: "What are possible solutions?", variant: "default" },
		{ id: "4", text: "I have a different question", variant: "primary" }
	];
</script>

<!-- Default ChatBot with initial questions -->
<Story name="Default" args={{ messages: defaultMessages }} />

<!-- ChatBot with custom questions -->
<Story name="Custom Questions" args={{ 
	messages: defaultMessages,
	initialQuestions: customQuestions,
	title: "Policy Discussion",
	subtitle: "Choose a question to start exploring this topic."
}} />

<!-- ChatBot with single message -->
<Story name="Single Message" args={{ 
	messages: singleMessageExample,
	title: "Getting Started",
	subtitle: "Let's begin our conversation about the topic."
}} />

<!-- ChatBot with no messages -->
<Story name="Empty Chat" args={{ 
	messages: emptyMessages,
	title: "New Conversation",
	subtitle: "Start a new conversation with the bot."
}} />

<!-- ChatBot without initial questions -->
<Story name="No Question Buttons" args={{ 
	messages: defaultMessages,
	showInitialQuestions: false,
	title: "Direct Chat",
	subtitle: "Chat directly without suggested questions."
}} />

<!-- ChatBot showing conversation flow (questions hidden after interaction) -->
<Story name="Conversation Example" args={{ 
	messages: conversationExample,
	title: "Climate Policy Chat",
	subtitle: "An example of how conversations develop between users and the bot.",
	placeholder: "Continue the conversation...",
	showInitialQuestions: true,
	initialQuestions: customQuestions
}} />

<!-- ChatBot with custom title and subtitle -->
<Story name="Custom Content" args={{ 
	messages: defaultMessages,
	title: "Policy Discussion",
	subtitle: "Explore different perspectives on government policy and share your thoughts.",
	placeholder: "Share your thoughts..."
}} />

<!-- ChatBot with many messages (scrollable) -->
<Story name="Long Conversation" args={{ 
	messages: [
		{
			id: "1",
			content: "What part of this topic are you most curious to understand right now?",
			isBot: true,
			timestamp: new Date()
		},
		{
			id: "2", 
			content: "What do you already know or think about this topic so far?",
			isBot: true,
			timestamp: new Date()
		},
		{
			id: "3",
			content: "What feels unclear or puzzling to you about this topic at the moment?",
			isBot: true,
			timestamp: new Date()
		},
		{
			id: "4",
			content: "How do you think this topic affects your daily life?",
			isBot: true,
			timestamp: new Date()
		},
		{
			id: "5",
			content: "What sources of information do you typically trust on this subject?",
			isBot: true,
			timestamp: new Date()
		},
		{
			id: "6",
			content: "Are there any aspects of this topic that particularly concern you?",
			isBot: true,
			timestamp: new Date()
		},
		{
			id: "7",
			content: "What changes would you like to see regarding this topic?",
			isBot: true,
			timestamp: new Date()
		}
	],
	title: "Extended Discussion",
	subtitle: "A longer conversation to test scrolling behavior."
}} />
