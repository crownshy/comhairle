<script lang="ts">
	import {
		Send,
		Mic,
		Sparkles
	} from 'lucide-svelte';
	import { ChatClient } from '$lib/api/chatClient.svelte';

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
		chatId?: string;
		knowledgeBaseIds?: string[];
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
		chatId,
		knowledgeBaseIds = [],
		title = "Chat with Bot",
		subtitle = "Try answer some questions from Comhairle and explore your views.",
		botName = "Tutor bot",
		botSubtitle = "Ask questions",
		messages: initialMessages = [
			{
				id: "1",
				content: "I am here to help you explore your understanding to this bot. You can...",
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
	let selectedQuestionId = $state<string | null>(null);
	let chatError = $state<string | null>(null);
	let isInitializing = $state(true);
	
	let actualChatId = $state(chatId);
	let client = $state<ChatClient | null>(null);
	let initialized = false;

	// Initialize chat and session
	$effect(() => {
		if (initialized) return;
		initialized = true;
		
		async function init() {
			try {
				isInitializing = true;
				
				// Create chat if no chatId provided
				if (!actualChatId) {
					const response = await fetch('/api/bot/chats', {
						method: 'POST',
						headers: { 'Content-Type': 'application/json' },
						credentials: 'include',
						body: JSON.stringify({
							name: `Chat ${Date.now()}`,
							knowledge_base_ids: knowledgeBaseIds.length > 0 ? knowledgeBaseIds : undefined
						})
					});

					if (!response.ok) {
						throw new Error('Failed to create chat');
					}

					const chat = await response.json();
					actualChatId = chat.id;
				}

				// Create client and session
				client = new ChatClient(actualChatId);
				await client.createSession(`session-${Date.now()}`);
				
				if (client.error) {
					chatError = client.error;
				}
			} catch (e) {
				chatError = e instanceof Error ? e.message : 'Failed to initialize chat';
				console.error('Chat initialization error:', e);
			} finally {
				isInitializing = false;
			}
		}
		
		init();
	});

	function scrollToBottom() {
		if (chatContainer) {
			setTimeout(() => {
				chatContainer.scrollTop = chatContainer.scrollHeight;
			}, 100);
		}
	}

	async function addBotResponse(userMessage: string) {
		if (!client) return;
		
		isTyping = true;
		scrollToBottom();
		
		await client.send(userMessage);
		
		isTyping = false;
		
		if (client.error) {
			chatError = client.error;
			console.error('Chat error:', client.error);
		} else if (client.currentAnswer) {
			const botResponse: ChatMessage = {
				id: `bot-${Date.now()}`,
				content: client.currentAnswer,
				isBot: true,
				timestamp: new Date()
			};
			chatMessages = [...chatMessages, botResponse];
		}
		
		scrollToBottom();
	}

	function handleQuestionClick(question: InitialQuestion) {
		if (!client || isInitializing) return;
		
		selectedQuestionId = question.id;
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
		if (!client || isInitializing || !inputValue.trim()) return;
		
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

	function handleKeyPress(event: KeyboardEvent) {
		if (event.key === "Enter" && !event.shiftKey) {
			event.preventDefault();
			sendMessage();
		}
	}
</script>

    <!-- DEBUGGING: Remove later - Error Message -->
    {#if chatError}
        <div class="mb-4 p-3 bg-red-50 border border-red-200 rounded-lg">
            <p class="text-sm text-red-600">{chatError}</p>
        </div>
    {/if}

    <!-- DEBUGGING: Remove later - Initializing Message -->
    {#if isInitializing}
        <div class="mb-4 p-3 bg-blue-50 border border-blue-200 rounded-lg">
            <p class="text-sm text-blue-600">Setting up chat...</p>
        </div>
    {/if}

<div class="bg-cs-blue-100 max-w-xxxl rounded-2xl shadow-md border border-cs-grey-200 p-6 mx-auto">
    <!-- Header -->
    <div class="text-center mb-6">
        <p class="text-xs text-cs-grey-500 mb-2">12/23/23</p>
    </div>


    <!-- Chat Messages -->
    <div bind:this={chatContainer} class="space-y-4 mb-6 max-h-96 overflow-y-auto pr-2">
        {#each chatMessages as message, index (message.id)}
            <div class="{message.isBot ? '' : 'flex justify-end'}">
                <!-- Message Content -->
                <div class="{message.isBot ? 'bg-white rounded-br-[16px]' : 'bg-cs-blue-800 rounded-bl-[16px]'} w-fit max-w-xxl rounded-tl-[16px] rounded-tr-[16px] px-3 py-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.15)]">
                    {#if message.isBot}
                        <div>
                            <div class="flex items-start gap-2">
                                {#if index < 1}
                                    <Sparkles class="w-4 h-4 text-cs-blue-600 mt-0.5 flex-shrink-0" />
                                {/if}
                                <p class="text-cs-grey-900 text-sm">{message.content}</p>
                            </div>
							
                            <!-- Quick Reply Buttons -->
                            {#if showInitialQuestions && initialQuestions.length > 0 && index === 0}
                                <div class="self-stretch inline-flex flex-col justify-start items-start gap-3 mt-3">
                                    {#each initialQuestions as question (question.id)}
                                        <button
                                            onclick={() => handleQuestionClick(question)}
                                            disabled={isInitializing}
                                            class="{selectedQuestionId === question.id 
                                                ? 'bg-cs-blue-600 outline-cs-blue-600' 
                                                : 'bg-white outline-cs-blue-300'
                                            } px-2.5 py-1.5 rounded-2xl outline outline-1 outline-offset-[-0.5px] flex flex-col justify-start items-start gap-1 disabled:opacity-50 disabled:cursor-not-allowed"
                                        >
                                            <div class="inline-flex justify-start items-start gap-2.5">
                                                <span class="{selectedQuestionId === question.id 
                                                    ? 'text-white' 
                                                    : 'text-cs-blue-600'
                                                } text-xs font-normal leading-4">{question.text}</span>
                                            </div>
                                        </button>
                                    {/each}
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
	 <div class="flex items-center gap-2">
		<div class="flex-1 flex items-center gap-2 h-12 px-4 py-2 bg-cs-grey-50 rounded-[12px] border border-cs-grey-200">
			<input
				bind:value={inputValue}
				onkeypress={handleKeyPress}
				type="text"
				placeholder={placeholder}
				disabled={isInitializing}
				class="flex-1 px-4 py-2 bg-transparent text-sm text-cs-grey-900 placeholder:text-cs-grey-400 outline-none disabled:opacity-50"
			/>
			<button
				class="p-2.5 text-cs-grey-400 hover:text-cs-grey-600 transition-colors disabled:opacity-50"
				disabled={isInitializing}
				aria-label="Voice input"
			>
				<Mic class="w-5 h-5" />
			</button>
		</div>
		 <button
            onclick={sendMessage}
            class="p-2.5 bg-cs-blue-800 text-white rounded-full hover:bg-cs-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            disabled={!inputValue.trim() || isInitializing}
            aria-label="Send message"
        >
            <Send class="w-5 h-5" />
        </button>
		</div>


</div>