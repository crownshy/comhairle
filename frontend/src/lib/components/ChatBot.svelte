<script lang="ts">
	import {
		Send,
		Mic,
		Sparkles
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
		botName?: string;
		botSubtitle?: string;
		messages?: ChatMessage[];
		placeholder?: string;
		initialQuestions?: InitialQuestion[];
		showInitialQuestions?: boolean;
		onSendMessage?: (message: string) => void;
		onQuestionClick?: (question: string) => void;
	}

	let {
		title = "Chat with Bot",
		subtitle = "Try answer some questions from Comhairle and explore your views.",
		botName = "Tutor bot",
		botSubtitle = "Ask questions",
		messages: initialMessages = [
			{
				id: "1",
				content: "Hi, you can ask me anything.",
				isBot: true,
				timestamp: new Date()
			},
			{
				id: "2",
				content: "I suggest you some names you can ask me..",
				isBot: true,
				timestamp: new Date()
			}
		],
		placeholder = "Ask questions...",
		initialQuestions = [
			{ id: "1", text: "Explain this to me", variant: "default" },
			{ id: "2", text: "How did NPF come about?", variant: "default" },
			{ id: "3", text: "What decisions will this influence", variant: "primary" },
			{ id: "4", text: "Ask something else", variant: "default" }
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


<div class="bg-cs-blue-100 rounded-2xl shadow-md border border-cs-grey-200 p-6 max-w-lg mx-auto">
    <!-- Header -->
    <div class=" text-center mb-6">
        <p class="text-xs text-cs-grey-500 mb-2">Deep dive</p>
        <h2 class="text-2xl font-semibold text-cs-grey-900">{title}</h2>
    </div>

    <!-- Bot Info Card -->
    <div class="bg red-pink-500 flex items-center gap-3 mb-6 p-4 bg-cs-grey-50 rounded-xl">
        <div class="w-10 h-10 bg-cs-blue-800 rounded-full flex items-center justify-center flex-shrink-0">
            <Sparkles class="w-5 h-5 text-white" />
        </div>
        <div>
            <p class="font-semibold text-cs-grey-900">{botName}</p>
            <p class="text-sm text-cs-blue-600">{botSubtitle}</p>
        </div>
    </div>

    <!-- Chat Messages -->
    <div bind:this={chatContainer} class="space-y-4 mb-6 max-h-96 overflow-y-auto pr-2">
        {#each chatMessages as message, index (message.id)}
            <div class="{message.isBot ? '' : 'flex justify-end'}">
                <!-- Message Content -->
                <div class="inline-block {message.isBot ? 'bg-white' : 'bg-cs-blue-800'} rounded-[16px] px-4 py-3 max-w-sm">
                    {#if message.isBot}
                        <div class="flex items-start gap-2 ">
                            {#if index < 2}
                                <Sparkles class="w-4 h-4 text-cs-blue-600 mt-0.5 flex-shrink-0" />
                            {/if}
                            <p class="text-cs-grey-900 text-sm">{message.content}</p>
							
							<!-- Quick Reply Buttons - Show after second bot message -->
							  {#if message.isBot && showInitialQuestions && initialQuestions.length > 0 && !hasStartedConversation && index === chatMessages.length - 1}
								<div class="mt-3">
									<div class="flex flex-wrap gap-2 max-w-sm">
										{#each initialQuestions as question (question.id)}
											<button
												onclick={() => handleQuestionClick(question)}
												class="{question.variant === 'primary' 
													? 'bg-cs-blue-800 text-white hover:bg-cs-blue-700 border-cs-blue-800' 
													: 'bg-white text-cs-grey-900 hover:bg-cs-grey-50 border-cs-grey-300'
												} px-4 py-2 rounded-full text-xs font-medium transition-colors border"
											>
												{question.text}
											</button>
										{/each}
									</div>
								</div>
							{/if}


                        </div>
                    {:else}
                        <p class="text-white text-sm">{message.content}</p>
                    {/if}
                </div>
                
              
            </div>
        {/each}
        
        <!-- Typing Indicator -->
        {#if isTyping}
            <div>
                <div class="bg-cs-grey-100 rounded-[16px] px-4 py-3 inline-block">
                    <div class="flex items-center gap-1">
                        <div class="w-2 h-2 bg-cs-blue-400 rounded-full animate-bounce"></div>
                        <div class="w-2 h-2 bg-cs-blue-400 rounded-full animate-bounce" style="animation-delay: 0.1s"></div>
                        <div class="w-2 h-2 bg-cs-blue-400 rounded-full animate-bounce" style="animation-delay: 0.2s"></div>
                    </div>
                </div>
            </div>
        {/if}
    </div>

    <!-- Input Area -->
    <div class="flex items-center gap-2 p-2 bg-cs-grey-50 rounded-full border border-cs-grey-200">
        <input
            bind:value={inputValue}
            onkeypress={handleKeyPress}
            type="text"
            placeholder={placeholder}
            class="flex-1 px-4 py-2 bg-transparent text-sm text-cs-grey-900 placeholder:text-cs-grey-400 outline-none"
        />
        <button
            class="p-2.5 text-cs-grey-400 hover:text-cs-grey-600 transition-colors"
            aria-label="Voice input"
        >
            <Mic class="w-5 h-5" />
        </button>
        <button
            onclick={sendMessage}
            class="p-2.5 bg-cs-blue-800 text-white rounded-full hover:bg-cs-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            disabled={!inputValue.trim()}
            aria-label="Send message"
        >
            <Send class="w-5 h-5" />
        </button>
    </div>
</div>

