<script lang="ts">
	import { tick } from 'svelte';
	import { SendHorizontal, Mic, Sparkles } from 'lucide-svelte';
	import { getChatSession } from '$lib/api/chatSession.svelte';
	import MessageWithReferences from '$lib/components/Chatbot/MessageWithReferences.svelte';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import type { ChatMessage, InitialQuestion, ChatBotProps } from './types';

	let {
		chatId,
		conversationId,
		userId,
		knowledgeBaseIds = [],
		title = 'Chat with Bot',
		subtitle = 'Try answer some questions from Comhairle and explore your views.',
		botName = 'Learning assistant',
		botSubtitle = 'Ask questions',
		messages: initialMessages = [
			// TODO: add back with better wording once we know where how to configure default questions
			// {
			// 	id: '1',
			// 	content: 'I am here to help you explore your understanding to this bot. You can...',
			// 	isBot: true,
			// 	timestamp: new Date()
			// }
		],
		placeholder = 'Ask questions...',
		initialQuestions = [
			{ id: '1', text: 'Explain this to me', variant: 'default' },
			{
				id: '2',
				text: "What's Scotland's strategy for the space sector?",
				variant: 'default'
			},
			{ id: '3', text: 'What decisions will this influence', variant: 'primary' },
			{ id: '4', text: 'Ask something else', variant: 'default' }
		],
		showInitialQuestions = false, // TODO: change back to true once we have a way of configuring
		active = true,
		onSendMessage = (message: string) => console.log('Message sent:', message),
		onQuestionClick = (question: string) => console.log('Question clicked:', question)
	}: ChatBotProps = $props();

	let inputValue = $state('');
	let chatContainer: HTMLDivElement;
	let scrollAreaRef: HTMLElement | null = $state(null);
	let textareaRef: HTMLTextAreaElement | null = $state(null);
	let selectedQuestionId = $state<string | null>(null);
	let needsScroll = $state(false);

	const session = $derived(conversationId ? getChatSession(conversationId) : null);
	let initStartedFor = $state<string | null>(null);

	$effect(() => {
		if (!session) return;
		if (initStartedFor === session.conversationId) return;
		initStartedFor = session.conversationId;
		session.init().then(() => {
			needsScroll = true;
		});
	});

	let isInitializing = $derived(
		!!conversationId && (session?.initializing || !session?.initialized)
	);
	let chatError = $derived(
		conversationId ? (session?.error ?? null) : 'Unable to start chat: missing conversation ID.'
	);
	let sessionMessages = $derived(session?.messages ?? []);
	let hasStartedConversation = $derived(sessionMessages.length > 0);
	let chatMessages = $derived<ChatMessage[]>([
		...initialMessages,
		...sessionMessages.map((m) => ({
			id: m.id,
			content: m.content,
			isBot: m.role === 'assistant',
			timestamp: m.timestamp,
			reference: m.reference
		}))
	]);

	$effect(() => {
		if (needsScroll && scrollAreaRef) {
			tick().then(() => {
				if (!scrollAreaRef) return;
				const viewport = scrollAreaRef.querySelector('[data-slot="scroll-area-viewport"]');
				if (viewport) {
					viewport.scrollTop = viewport.scrollHeight;
					needsScroll = false;
				}
			});
		}
	});

	$effect(() => {
		if (active && chatMessages.length > 0) {
			tick().then(scrollToBottom);
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

	// Auto-scroll on stream activity / content updates from the shared session.
	$effect(() => {
		if (!session) return;
		// Track streaming state and the tail message content so this re-runs on chunks.
		const _streaming = session.isStreaming;
		const tail = session.messages[session.messages.length - 1];
		const _content = tail?.content;
		void _streaming;
		void _content;
		scrollToBottom();
	});

	// Auto-resize textarea
	function resizeTextarea() {
		if (!textareaRef) return;
		textareaRef.style.height = '24px';
		const lineHeight = 20;
		const maxHeight = lineHeight * 10; // 10 rows max
		const newHeight = Math.min(textareaRef.scrollHeight || 44, maxHeight);
		textareaRef.style.height = `${newHeight}px`;
	}

	$effect(() => {
		inputValue;
		resizeTextarea();
	});

	async function ask(question: string) {
		if (!session || isInitializing) return;
		await tick();
		scrollToBottom();
		await session.send(question);
		await tick();
		scrollToBottom();
	}

	function handleQuestionClick(question: InitialQuestion) {
		if (!session || isInitializing) return;
		selectedQuestionId = question.id;
		onQuestionClick(question.text);
		void ask(question.text);
	}

	async function sendMessage() {
		if (!session || isInitializing || !inputValue.trim()) return;
		const messageToRespond = inputValue.trim();
		inputValue = '';
		onSendMessage(messageToRespond);
		await ask(messageToRespond);
	}

	function handleKeyPress(event: KeyboardEvent) {
		if (event.key === 'Enter' && !event.shiftKey) {
			event.preventDefault();
			sendMessage();
		}
	}
</script>

{#if isInitializing}
	<div
		class="bg-chat-primary-lighter mx-auto flex h-full flex-col items-center justify-center p-6"
	>
		<div class="flex flex-col items-center gap-3">
			<div class="flex items-center gap-2">
				<span
					class="bg-chat-primary h-2 w-2 animate-bounce rounded-full"
					style="animation-delay: 0ms"
				></span>
				<span
					class="bg-chat-primary h-2 w-2 animate-bounce rounded-full"
					style="animation-delay: 150ms"
				></span>
				<span
					class="bg-chat-primary h-2 w-2 animate-bounce rounded-full"
					style="animation-delay: 300ms"
				></span>
			</div>
			<p class="text-chat-text-muted text-sm">Loading chat...</p>
		</div>
	</div>
{:else}
	<div
		class="bg-chat-primary-lighter max-w-xxxl mx-auto flex h-full min-h-0 flex-col overflow-hidden rounded-lg p-6 pt-3"
	>
		<ScrollArea.Root bind:ref={scrollAreaRef} class="min-h-0 flex-1 overflow-hidden">
			<div class="mt-2 mb-4 shrink-0 text-center">
				<p class="text-chat-text-muted text-xs">
					{new Date().toISOString().slice(0, 10).replace(/-/g, '.')}
				</p>
			</div>

			{#if chatError}
				<div class="mb-2 rounded-lg border border-red-200 bg-red-50 p-3">
					<p class="text-sm text-red-600">{chatError}</p>
				</div>
			{:else}
				<div bind:this={chatContainer} class="space-y-4 pr-4">
					{#each chatMessages as message, index (message.id)}
						<div class={message.isBot ? '' : 'flex justify-end'}>
							<!-- Message Content -->
							<div
								class="{message.isBot
									? 'bg-chat-bubble rounded-br-2xl'
									: 'bg-chat-primary-dark rounded-bl-2xl'} max-w-xxl w-fit rounded-tl-2xl rounded-tr-2xl px-3 py-2.5"
							>
								{#if message.isBot}
									<div>
										<div class="flex items-start gap-2">
											{#if index < 1}
												<Sparkles
													class="text-chat-primary mt-0.5 h-4 w-4 shrink-0"
												/>
											{/if}
											<span class="text-chat-text text-sm">
												<MessageWithReferences
													content={message.content}
													reference={message.reference}
												/>
											</span>
										</div>

										<!-- Quick Reply Buttons -->
										{#if showInitialQuestions && initialQuestions.length > 0 && index === 0}
											<div
												class="mt-3 inline-flex flex-col items-start justify-start gap-3 self-stretch"
											>
												{#each initialQuestions as question (question.id)}
													<button
														onclick={() =>
															handleQuestionClick(question)}
														disabled={isInitializing}
														class="{selectedQuestionId === question.id
															? 'bg-chat-primary outline-chat-primary'
															: 'outline-chat-primary-light bg-chat-bubble'} flex flex-col items-start justify-start gap-1 rounded-2xl px-2.5 py-1.5 outline outline-1 outline-offset-[-0.5px] disabled:cursor-not-allowed disabled:opacity-50"
													>
														<div
															class="inline-flex items-start justify-start gap-2.5"
														>
															<span
																class="{selectedQuestionId ===
																question.id
																	? 'text-white'
																	: 'text-chat-primary'} text-xs leading-4 font-normal"
																>{question.text}</span
															>
														</div>
													</button>
												{/each}
											</div>
										{/if}
									</div>
								{:else}
									<p class="text-primary-foreground text-sm">{message.content}</p>
								{/if}
							</div>
						</div>
					{/each}

					<!-- Streaming placeholder bubble (empty assistant message while waiting for first chunk) -->
					{#if session?.isStreaming && (!chatMessages.length || chatMessages[chatMessages.length - 1]?.isBot === false || (chatMessages[chatMessages.length - 1]?.isBot === true && !chatMessages[chatMessages.length - 1]?.content?.trim()))}
						<div>
							<div
								class="max-w-xxl bg-chat-bubble w-fit rounded-tl-2xl rounded-tr-2xl rounded-br-2xl px-3 py-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.15)]"
							>
								<span class="flex items-center gap-1">
									<span
										class="bg-chat-primary-light h-2 w-2 animate-bounce rounded-full"
									></span>
									<span
										class="bg-chat-primary-light h-2 w-2 animate-bounce rounded-full"
										style="animation-delay: 0.1s"
									></span>
									<span
										class="bg-chat-primary-light h-2 w-2 animate-bounce rounded-full"
										style="animation-delay: 0.2s"
									></span>
								</span>
							</div>
						</div>
					{/if}
				</div>
			{/if}
		</ScrollArea.Root>

		<!-- Input Area -->
		<div class="flex shrink-0 items-end gap-2 pt-4">
			<div
				class="border-chat-border bg-chat-bubble flex flex-1 items-end gap-2 rounded-xl border shadow-md"
			>
				<textarea
					bind:this={textareaRef}
					bind:value={inputValue}
					onkeydown={(e) => {
						if (e.key === 'Enter' && !e.shiftKey) {
							e.preventDefault();
							sendMessage();
						}
					}}
					{placeholder}
					disabled={isInitializing}
					rows={1}
					class="text-chat-text placeholder:text-chat-text-muted min-h-6 flex-1 resize-none self-center overflow-y-auto bg-transparent px-4 py-3 text-sm leading-5 outline-none disabled:opacity-50"
					style="max-height: 200px;"
				></textarea>
				<!-- TODO: add back in once functionality provided
				<button
					class="text-chat-text-muted hover:text-chat-neutral p-2.5 transition-colors disabled:opacity-50"
					disabled={isInitializing}
					aria-label="Voice input"
				>
					<Mic class="h-5 w-5" />
				</button>
				-->
			</div>
			<button
				onclick={sendMessage}
				class="bg-chat-primary-dark hover:bg-chat-primary rounded-full p-3 text-white transition-colors disabled:cursor-not-allowed disabled:opacity-50"
				disabled={!inputValue.trim() || isInitializing || session?.isStreaming}
				aria-label="Send message"
			>
				<SendHorizontal class="h-5 w-5" />
			</button>
		</div>
	</div>
{/if}
