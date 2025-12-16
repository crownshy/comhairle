<script lang="ts">
	import * as m from '$lib/paraglide/messages';
	import Button from '$lib/components/ui/button/button.svelte';
	import ChatBot from '$lib/components/Chatbot/ChatBot.svelte'

	// do you think it's a good idea to have this wrapper?
	
	function handleSendMessage(message: string) {
		console.log('Message sent from:', message);
	}

	function handleQuestionClick(question: string) {
		console.log('Question clicked from:', question);
	}

	const wrapperQuestions = [
		{ id: "1", text: "What is Comhairle about?", variant: "default" as const },
		{ id: "2", text: "How can I participate?", variant: "default" as const },
		{ id: "3", text: "What topics are being discussed?", variant: "default" as const },
		{ id: "4", text: "Tell me something interesting", variant: "primary" as const }
	];
</script>

<ChatBot 
	title="Demo Chat"
	subtitle="Ask questions and explore different perspectives on topics that matter to you."
	placeholder="Type your message here..."
	initialQuestions={wrapperQuestions}
	showInitialQuestions={true}
	onSendMessage={handleSendMessage}
	onQuestionClick={handleQuestionClick}
/>
