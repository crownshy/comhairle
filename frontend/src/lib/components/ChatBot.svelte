<script lang="ts">
	import {
		ArrowRight
	} from 'lucide-svelte';

	interface ChatMessage {
		id: string;
		content: string;
		isBot: boolean;
		timestamp: Date;
	}

	interface InitialQuestion {
		id: string;
		text: string;
		variant?: 'default' | 'primary';
	}

	interface ChatBotProps {
		title?: string;
		subtitle?: string;
		messages?: ChatMessage[];
		placeholder?: string;
		initialQuestions?: InitialQuestion[];
		showInitialQuestions?: boolean;
		onSendMessage?: (message: string) => void;
		onQuestionClick?: (question: string) => void;
	}

	let {
		title = "Deep dive",
		subtitle = "Try answer some questions from the chatbot and explore your views.",
		messages: initialMessages = [
			{
				id: "1",
				content: "Hello! I'm here to help you explore topics. What would you like to discuss?",
				isBot: true,
				timestamp: new Date()
			}
		],
		placeholder = "Type here",
		initialQuestions = [
			{ id: "1", text: "Explain this to me", variant: "default" },
			{ id: "2", text: "Give me some context", variant: "default" },
			{ id: "3", text: "What decisions will this influence?", variant: "default" },
			{ id: "4", text: "Ask something else", variant: "primary" }
		],
		showInitialQuestions = true,
		onSendMessage = (message: string) => console.log("Message sent:", message),
		onQuestionClick = (question: string) => console.log("Question clicked:", question)
	}: ChatBotProps = $props();

	let inputValue = $state("");
	let chatContainer: HTMLDivElement;
	let chatMessages = $state([...initialMessages]);
	let isTyping = $state(false);
	let hasStartedConversation = $state(false);

	const mockResponses = [
		"That's a really interesting perspective! Can you tell me more about what led you to that view?",
		"I appreciate you sharing that. How do you think others might see this differently?",
		"That's a thoughtful point. What experiences have shaped your thinking on this?",
		"Thanks for that insight. What aspects of this topic do you find most important?",
		"I can see why you'd think that way. What questions do you still have about this?",
		"That's a valuable contribution to our discussion. How might this affect different communities?",
		"Your perspective adds depth to this conversation. What would you like to explore further?",
		"I find that fascinating. What do you think would happen if we approached this differently?",
		"Thank you for being so open. What hopes do you have regarding this topic?",
		"That's worth considering. How do you think we could address the challenges you've mentioned?"
	];

	function getRandomResponse(): string {
		return mockResponses[Math.floor(Math.random() * mockResponses.length)];
	}

	function scrollToBottom() {
		if (chatContainer) {
			setTimeout(() => {
				chatContainer.scrollTop = chatContainer.scrollHeight;
			}, 100);
		}
	}

	function addBotResponse(userMessage: string) {
		isTyping = true;
		scrollToBottom();
		
		// Simulate bot typing delay; will be removed
		setTimeout(() => {
			const botResponse: ChatMessage = {
				id: `bot-${Date.now()}`,
				content: getRandomResponse(),
				isBot: true,
				timestamp: new Date()
			};
			
			chatMessages = [...chatMessages, botResponse];
			isTyping = false;
			scrollToBottom();
		}, 1000 + Math.random() * 2000); // Random delay between 1-3 seconds
	}

	function handleQuestionClick(question: InitialQuestion) {
		hasStartedConversation = true;
		
		const userMessage: ChatMessage = {
			id: `user-${Date.now()}`,
			content: question.text,
			isBot: false,
			timestamp: new Date()
		};
		
		chatMessages = [...chatMessages, userMessage];
		
		onQuestionClick(question.text);
		
		scrollToBottom();
		addBotResponse(question.text);
	}

	function sendMessage() {
		if (inputValue.trim()) {
			hasStartedConversation = true;
			
			const userMessage: ChatMessage = {
				id: `user-${Date.now()}`,
				content: inputValue.trim(),
				isBot: false,
				timestamp: new Date()
			};
			
			chatMessages = [...chatMessages, userMessage];
			
			onSendMessage(inputValue.trim());
			
			const messageToRespond = inputValue.trim();
			inputValue = "";
			
			scrollToBottom();
			addBotResponse(messageToRespond);
		}
	}

	function handleKeyPress(event: KeyboardEvent) {
		if (event.key === "Enter" && !event.shiftKey) {
			event.preventDefault();
			sendMessage();
		}
	}
</script>


<div class="bg-white rounded-[14px] shadow-sm border p-10">
    <div class="text-center mb-6">
        <div class="flex items-center justify-center gap-2 mb-3">
            <div class="bg-pink-500 text-white text-xs font-medium px-2.5 py-1 rounded-full shadow-sm">
                AI
            </div>
            <h2 class="text-xl font-semibold text-gray-900">Ask "Bot"</h2>
        </div>
        <p class="text-sm text-gray-600">{subtitle}</p>
    </div>

    <div bind:this={chatContainer} class="space-y-4 mb-6 h-96 overflow-y-auto">
        {#each chatMessages as message (message.id)}
            <div class="flex items-start space-x-3 {message.isBot ? '' : 'flex-row-reverse'}">
                <div class="flex-shrink-0">
                    {#if message.isBot}
                        <div class="w-8 h-8 bg-pink-500 rounded-full flex items-center justify-center">
                            <span class="text-white text-sm font-medium">AI</span>
                        </div>
                    {:else}
                        <div class="w-8 h-8 bg-blue-500 rounded-full flex items-center justify-center">
                            <span class="text-white text-sm font-medium">U</span>
                        </div>
                    {/if}
                </div>
                
                <div class="flex-1">
                    <div class=" {message.isBot ? 'bg-gray-50' : 'bg-blue-500 text-white'} rounded-[8px] px-6 py-3 max-w-md {message.isBot ? '' : 'ml-auto mr-2'}">
                        <p class="{message.isBot ? 'text-gray-900' : 'text-white'} text-sm">{message.content}</p>
                    </div>
                    
                    {#if message.isBot && showInitialQuestions && initialQuestions.length > 0 && !hasStartedConversation && chatMessages.indexOf(message) === 0}
                        <div class="mt-4 ml-0">
                            <div class="flex flex-wrap gap-2 max-w-md">
                                {#each initialQuestions as question (question.id)}
                                    <button
                                        onclick={() => handleQuestionClick(question)}
                                        class="{question.variant === 'primary' 
                                            ? 'bg-blue-600 text-white hover:bg-blue-700' 
                                            : 'bg-blue-50 text-gray-900 hover:bg-blue-100'
                                        } px-3 py-2 rounded-full text-xs font-medium transition-colors shadow-sm"
                                    >
                                        {question.text}
                                    </button>
                                {/each}
                            </div>
                        </div>
                    {/if}
                </div>
            </div>
        {/each}
        
        {#if isTyping}
            <div class="flex items-start space-x-3">
                <div class="flex-shrink-0">
                    <div class="w-8 h-8 bg-pink-500 rounded-full flex items-center justify-center">
                        <span class="text-white text-sm font-medium">AI</span>
                    </div>
                </div>
                <div class="flex-1">
                    <div class="bg-gray-50 rounded-[8px] p-5 max-w-md">
                        <div class="flex space-x-1">
                            <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce"></div>
                            <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style="animation-delay: 0.1s"></div>
                            <div class="w-2 h-2 bg-gray-400 rounded-full animate-bounce" style="animation-delay: 0.2s"></div>
                        </div>
                    </div>
                </div>
            </div>
        {/if}
    </div>

    <div class="border-t pt-4">
        <div class="flex items-center space-x-3">
            <div class="flex-1 relative">
                <input
                    bind:value={inputValue}
                    onkeypress={handleKeyPress}
                    type="text"
                    placeholder={placeholder}
                    class="w-full px-4 py-3 border border-gray-300 rounded-[8px] focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none"
                />
            </div>
            <button
                onclick={sendMessage}
                class="bg-black text-white p-3 rounded-full hover:opacity-80 transition-colors disabled:cursor-not-allowed"
                disabled={!inputValue.trim()}
                aria-label="Send message"
            >
             
				<ArrowRight />
            </button>
        </div>
    </div>
</div>

