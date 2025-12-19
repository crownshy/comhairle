<script lang="ts">
	import { tick } from 'svelte';
	import {
		SendHorizontal,
		Mic,
		Sparkles
	} from 'lucide-svelte';
	import { ChatClient } from '$lib/api/chatClient.svelte';
	import MessageWithReferences from '$lib/components/Chatbot/MessageWithReferences.svelte';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import type { ChatMessage, InitialQuestion, ChatBotProps, ChatReference } from './types';

	let {
		chatId,
		conversationId,
		userId,
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
			},		
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
	let scrollAreaRef: HTMLElement | null = $state(null);
	let textareaRef: HTMLTextAreaElement | null = $state(null);
	let chatMessages = $state([...initialMessages]);
	let hasStartedConversation = $state(false);
	let selectedQuestionId = $state<string | null>(null);
	let chatError = $state<string | null>(null);
	let isInitializing = $state(true);
	let needsScroll = $state(false);
	
	let client = $state<ChatClient | null>(null);
	let initialized = false;

	$effect(() => {
		if (initialized) return;
		initialized = true;
		
		async function init() {
			try {
				isInitializing = true;
				client = new ChatClient(chatId, userId, conversationId);
				
				const sessionId = await client.getOrCreateUserSession();
				if (!sessionId) {
					return;
				}
				
				const session = await client.getSession(sessionId);
				if (!session) {
					chatError = client.error || 'Failed to load session';
					return;
				}
				
				if (session.messages?.length) {
					const historicalMessages: ChatMessage[] = session.messages.map((msg, idx) => ({
						id: msg.id ? `${msg.id}-${msg.role}` : `msg-${idx}`,
						content: msg.content,
						isBot: msg.role === 'assistant',
						timestamp: new Date(),
						reference: msg.reference?.length ? {
							total: msg.reference.length,
							chunks: msg.reference.map(ref => ({
								id: ref.id,
								content: ref.content,
								document_id: ref.document_id,
								document_name: ref.document_name,
								dataset_id: ref.dataset_id
							}))
						} : null
					}));
					chatMessages = [...initialMessages, ...historicalMessages];
					hasStartedConversation = true;
				}
			} catch (e) {
				chatError = e instanceof Error ? e.message : 'Failed to initialize chat';
				console.error('Chat init error:', e);
			} finally {
				isInitializing = false;
				needsScroll = true;
			}
		}
		
		init();
	});

	$effect(() => {
		if (needsScroll && scrollAreaRef) {
			const viewport = scrollAreaRef.querySelector('[data-slot="scroll-area-viewport"]');
			if (viewport) {
				viewport.scrollTop = viewport.scrollHeight;
				needsScroll = false;
			}
		}
	});

	function scrollToBottom() {
		if (scrollAreaRef) {
			const viewport = scrollAreaRef.querySelector('[data-slot="scroll-area-viewport"]');
			if (viewport) {
				viewport.scrollTop = viewport.scrollHeight;
			}
		}
	}

	// Auto-scroll when streaming starts or content updates
	$effect(() => {
		if (client?.isStreaming) {
			scrollToBottom();
		}
	});

	// Auto-scroll on each chunk update
	$effect(() => {
		if (client?.currentAnswer) {
			scrollToBottom();
		}
	});

	// Auto-resize textarea
	function resizeTextarea() {
		if (!textareaRef) return;
		textareaRef.style.height = '24px'; 
		const lineHeight = 20;
		const maxHeight = lineHeight * 10; // 10 rows max
		const newHeight = Math.min(textareaRef.scrollHeight, maxHeight);
		textareaRef.style.height = `${newHeight}px`;
	}

	$effect(() => {
		inputValue;
		resizeTextarea();
	});

	async function addBotResponse(userMessage: string) {
		if (!client) return;
		
		await tick();
		scrollToBottom();
		
		await client.send(userMessage);
		
		if (client.error) {
			chatError = client.error;
			console.error('Chat error:', client.error);
		} else if (client.currentAnswer) {
			const botResponse: ChatMessage = {
				id: `bot-${Date.now()}`,
				content: client.currentAnswer,
				isBot: true,
				timestamp: new Date(),
				reference: client.currentReference
			};
			chatMessages = [...chatMessages, botResponse];
		}
		
		await tick();
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

	async function sendMessage() {
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
		
		await tick();
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


{#if isInitializing}
	<div class="bg-chat-primary-lighter max-w-xxxl p-6 mx-auto h-full min-h-[60vh] flex flex-col items-center justify-center">
		<div class="flex flex-col items-center gap-3">
			<div class="flex items-center gap-2">
				<span class="w-2 h-2 bg-chat-primary rounded-full animate-bounce" style="animation-delay: 0ms"></span>
				<span class="w-2 h-2 bg-chat-primary rounded-full animate-bounce" style="animation-delay: 150ms"></span>
				<span class="w-2 h-2 bg-chat-primary rounded-full animate-bounce" style="animation-delay: 300ms"></span>
			</div>
			<p class="text-sm text-chat-text-muted">Loading chat...</p>
		</div>
	</div>
{:else}
	<div class="bg-chat-primary-lighter max-w-xxxl pt-3 p-6 mx-auto h-full flex flex-col">
		<ScrollArea.Root bind:ref={scrollAreaRef} class="flex-1 min-h-0">
			<div class="text-center mt-2 mb-4 flex-shrink-0">
				<p class="text-xs text-chat-text-muted">{new Date().toISOString().slice(0, 10).replace(/-/g, '.')}</p>
			</div>

			{#if chatError}
				<div class="p-3 mb-2 bg-red-50 border border-red-200 rounded-lg">
					<p class="text-sm text-red-600">{chatError}</p>
				</div>
			{:else}
				<div bind:this={chatContainer} class="space-y-4 pr-4">
				{#each chatMessages as message, index (message.id)}
				<div class="{message.isBot ? '' : 'flex justify-end'}">
					<!-- Message Content -->
					<div class="{message.isBot ? 'bg-white rounded-br-[16px]' : 'bg-chat-primary-dark rounded-bl-[16px]'} w-fit max-w-xxl rounded-tl-[16px] rounded-tr-[16px] px-3 py-2.5 ">
						{#if message.isBot}
							<div>
								<div class="flex items-start gap-2">
									{#if index < 1}
										<Sparkles class="w-4 h-4 text-chat-primary mt-0.5 flex-shrink-0" />
									{/if}
									<span class="text-chat-text text-sm">
											<MessageWithReferences content={message.content} reference={message.reference} />
										</span>
								</div>
								
								<!-- Quick Reply Buttons -->
								{#if showInitialQuestions && initialQuestions.length > 0 && index === 0}
									<div class="self-stretch inline-flex flex-col justify-start items-start gap-3 mt-3">
										{#each initialQuestions as question (question.id)}
											<button
												onclick={() => handleQuestionClick(question)}
												disabled={isInitializing}
												class="{selectedQuestionId === question.id 
													? 'bg-chat-primary outline-chat-primary' 
													: 'bg-white outline-chat-primary-light'
												} px-2.5 py-1.5 rounded-2xl outline outline-1 outline-offset-[-0.5px] flex flex-col justify-start items-start gap-1 disabled:opacity-50 disabled:cursor-not-allowed"
											>
												<div class="inline-flex justify-start items-start gap-2.5">
													<span class="{selectedQuestionId === question.id 
														? 'text-white' 
														: 'text-chat-primary'
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
			
			<!-- Streaming Response -->
			{#if client?.isStreaming}
				<div>
					<div class="bg-white rounded-br-[16px] w-fit max-w-xxl rounded-tl-[16px] rounded-tr-[16px] px-3 py-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.15)]">
						<div class="flex items-start gap-2">
							<span class="text-chat-text text-sm">
								{#if client.currentAnswer}
									<MessageWithReferences content={client.currentAnswer} reference={client.currentReference} />
								{:else}
									<span class="flex items-center gap-1">
										<span class="w-2 h-2 bg-chat-primary-light rounded-full animate-bounce"></span>
										<span class="w-2 h-2 bg-chat-primary-light rounded-full animate-bounce" style="animation-delay: 0.1s"></span>
										<span class="w-2 h-2 bg-chat-primary-light rounded-full animate-bounce" style="animation-delay: 0.2s"></span>
									</span>
								{/if}
							</span>
							{#if client.currentAnswer}
								<span class="inline-block w-1.5 h-4 bg-chat-primary animate-pulse ml-0.5"></span>
							{/if}
						</div>
					</div>
				</div>
			{/if}
				</div>
			{/if}
		</ScrollArea.Root>

		<!-- Input Area -->
		<div class="flex items-end gap-2 flex-shrink-0 pt-4">
			<div class="flex-1 flex items-end gap-2 bg-white rounded-[12px] border shadow-md border-chat-border">
				<textarea
					bind:this={textareaRef}
					bind:value={inputValue}
					onkeydown={(e) => {
						if (e.key === 'Enter' && !e.shiftKey) {
							e.preventDefault();
							sendMessage();
						}
					}}
					placeholder={placeholder}
					disabled={isInitializing}
					rows={1}
					class="self-center flex-1 px-4 py-3 bg-transparent text-sm text-chat-text placeholder:text-chat-text-muted outline-none disabled:opacity-50 resize-none overflow-y-auto leading-5 min-h-6"
					style="max-height: 200px;"
				></textarea>
				<button
					class="p-2.5 text-chat-text-muted hover:text-chat-neutral transition-colors disabled:opacity-50"
					disabled={isInitializing}
					aria-label="Voice input"
				>
					<Mic class="w-5 h-5" />
				</button>
			</div>
			<button
				onclick={sendMessage}
				class="p-3 bg-chat-primary-dark text-white rounded-full hover:bg-chat-primary transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
				disabled={!inputValue.trim() || isInitializing || client?.isStreaming}
				aria-label="Send message"
			>
				<SendHorizontal class="w-5 h-5" />
			</button>
		</div>
	</div>
{/if}
