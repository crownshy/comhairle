<script lang="ts">
	import { tick } from 'svelte';
	import { SendHorizontal, Mic, Sparkles, MessageCircle } from 'lucide-svelte';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import ExtractedClaims from './ExtractedClaims.svelte';
	import type { ElicitationMessage, ExtractedClaim, ElicitationBotProps } from './types';

	let {
		chatId,
		conversationId,
		userId,
		botName = 'Elicitation bot',
		botSubtitle = 'Ask questions',
		messages: initialMessages = [
			{
				id: '1',
				content: 'I am here to help you explore your understanding to this bot. You can...',
				isBot: true,
				timestamp: new Date()
			}
		],
		claims: initialClaims = [
			{ id: '1', content: 'What decisions will this influence', status: 'pending' as const },
			{ id: '2', content: 'What decisions will this influence', status: 'editing' as const },
			{ id: '3', content: 'What decisions will this influence', status: 'editing' as const }
		],
		placeholder = 'Ask questions....',
		onSendMessage = (message: string) => console.log('Message sent:', message),
		onClaimApprove = (claimId: string) => console.log('Claim approved:', claimId),
		onClaimEdit = (claimId: string, newContent: string) => console.log('Claim edited:', claimId, newContent),
		onClaimRemove = (claimId: string) => console.log('Claim removed:', claimId),
		onAddClaim = () => console.log('Add claim clicked')
	}: ElicitationBotProps = $props();

	let inputValue = $state('');
	let scrollAreaRef: HTMLElement | null = $state(null);
	let textareaRef: HTMLTextAreaElement | null = $state(null);
	let chatMessages = $state<ElicitationMessage[]>([...initialMessages]);
	let isMobile = $state(false);
	let activeTab = $state('chat');
	let hasUnseenClaims = $state(false);
	let previousClaimsCount = $state(initialClaims.length);

	$effect(() => {
		if (initialMessages.length > 0) {
			chatMessages = [...initialMessages];
		}
	});

	$effect(() => {
		if (initialClaims.length > previousClaimsCount && activeTab === 'chat' && isMobile) {
			hasUnseenClaims = true;
		}
		previousClaimsCount = initialClaims.length;
	});

	const initialQuestions = [
		{ id: '1', text: 'What does this bot do?', variant: 'default' as const },
	];

	let selectedQuestionId = $state<string | null>(null);

	$effect(() => {
		const mediaQuery = window.matchMedia('(max-width: 1024px)');
		isMobile = mediaQuery.matches;

		const handler = (e: MediaQueryListEvent) => {
			isMobile = e.matches;
		};

		mediaQuery.addEventListener('change', handler);
		return () => mediaQuery.removeEventListener('change', handler);
	});

	function scrollToBottom() {
		if (scrollAreaRef) {
			const viewport = scrollAreaRef.querySelector('[data-slot="scroll-area-viewport"]');
			if (viewport) {
				viewport.scrollTop = viewport.scrollHeight;
			}
		}
	}

	function resizeTextarea() {
		if (!textareaRef) return;
		textareaRef.style.height = '24px';
		const lineHeight = 20;
		const maxHeight = lineHeight * 10;
		const newHeight = Math.min(textareaRef.scrollHeight, maxHeight);
		textareaRef.style.height = `${newHeight}px`;
	}

	$effect(() => {
		inputValue;
		resizeTextarea();
	});

	function handleQuestionClick(question: { id: string; text: string }) {
		selectedQuestionId = question.id;
		// Delegate to external handler - parent manages message state
		onSendMessage(question.text);
		scrollToBottom();
	}

	function getResponseForQuestion(question: string): string {
		if (question.includes('curious')) {
			return 'Great! What part of this topic are you most curious to understand right now?';
		} else if (question.includes('unclear')) {
			return 'Great! What feels unclear or puzzling to you about this topic at the moment?';
		} else if (question.includes('learned')) {
			return 'Great! What do you already know or think about this topic so far?';
		}
		return 'Great! Tell me more about what you\'d like to explore.';
	}

	async function sendMessage() {
		if (!inputValue.trim()) return;

		const message = inputValue.trim();
		inputValue = '';

		// Call the external handler - parent will manage message state
		onSendMessage(message);

		await tick();
		scrollToBottom();
	}

	function handleClaimApprove(claimId: string) {
		onClaimApprove(claimId);
	}

	function handleClaimSave(claimId: string, content: string) {
		onClaimEdit(claimId, content);
	}

	function handleClaimRemove(claimId: string) {
		onClaimRemove(claimId);
	}

	function handleAddClaim() {
		onAddClaim();
	}
</script>

<div class="w-full max-w-5xl mx-auto flex flex-col items-center gap-4 p-4">
	<!-- Main Container -->
	<div class="w-full bg-white rounded-[16px] border border-chat-primary-light shadow-lg overflow-hidden">
		<!-- Bot Header -->
		<div class="p-4 bg-white border-b border-chat-primary-light flex items-center gap-4">
			<div class="relative">
				<div class="w-12 h-12 bg-chat-primary rounded-full ring-4 ring-chat-primary-lighter flex items-center justify-center">
					<MessageCircle class="w-6 h-6 text-white" />
				</div>
				<div class="w-3 h-3 absolute bottom-0 right-0 bg-green-400 rounded-full border-2 border-white"></div>
			</div>
			<div class="flex-1 flex flex-col">
				<span class="text-chat-text text-lg font-semibold leading-6">{botName}</span>
				<span class="text-chat-primary text-sm font-normal leading-5">{botSubtitle}</span>
			</div>
		</div>

		{#if isMobile}
			<!-- Mobile: Tabbed View -->
			<div class="w-full">
				<div class="w-full grid grid-cols-2 bg-chat-bg border-b border-chat-border">
					<button
						class="py-3 text-sm font-medium transition-colors {activeTab === 'chat' ? 'bg-white text-chat-primary' : 'text-chat-text-muted hover:text-chat-text'}"
						onclick={() => activeTab = 'chat'}
					>
						Chat
					</button>
					<button
						class="relative py-3 text-sm font-medium transition-colors {activeTab === 'claims' ? 'bg-white text-chat-primary' : 'text-chat-text-muted hover:text-chat-text'}"
						onclick={() => { activeTab = 'claims'; hasUnseenClaims = false; }}
					>
						<span class="inline-flex items-center gap-1.5">
							Claims ({initialClaims.length})
							{#if hasUnseenClaims}
								<span class="relative flex h-2.5 w-2.5">
									<span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-chat-primary opacity-75"></span>
									<span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-chat-primary"></span>
								</span>
							{/if}
						</span>
					</button>
				</div>

				{#if activeTab === 'chat'}
					<div class="flex flex-col h-[60vh]">
						<!-- Chat Messages -->
						<ScrollArea.Root bind:ref={scrollAreaRef} class="flex-1 min-h-0">
							<div class="p-6 bg-chat-primary-lighter/40 min-h-full">
								<div class="text-center mb-4">
									<p class="text-xs text-chat-text-muted">
										{new Date().toISOString().slice(0, 10).replace(/-/g, '.')}
									</p>
								</div>

								<div class="space-y-4">
									{#each chatMessages as message, index (message.id)}
										<div class={message.isBot ? '' : 'flex justify-end'}>
											<div
												class="{message.isBot
													? 'bg-white rounded-br-[16px]'
													: 'bg-chat-primary-dark rounded-bl-[16px]'} w-fit max-w-[85%] rounded-tl-[16px] rounded-tr-[16px] px-3 py-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.15)]"
											>
												{#if message.isBot}
													<div>
														<div class="flex items-start gap-2">
															{#if index < 1}
																<Sparkles class="w-4 h-4 text-chat-primary mt-0.5 flex-shrink-0" />
															{/if}
															<span class="text-chat-text text-sm">{message.content}</span>
														</div>

														{#if index === 0}
															<div class="flex flex-col gap-3 mt-3">
																{#each initialQuestions as question (question.id)}
																	<button
																		onclick={() => handleQuestionClick(question)}
																		class="{selectedQuestionId === question.id
																			? 'bg-chat-primary border-chat-primary'
																			: question.variant === 'primary'
																				? 'bg-chat-primary border-chat-primary'
																				: 'bg-white border-chat-primary-light'} px-2.5 py-1.5 rounded-[16px] border flex items-start gap-2.5 w-fit"
																	>
																		<span
																			class="{selectedQuestionId === question.id ||
																			question.variant === 'primary'
																				? 'text-white'
																				: 'text-chat-primary'} text-xs font-normal leading-4"
																		>
																			{question.text}
																		</span>
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
								</div>

								<div class="mt-2">
									<p class="text-xs text-chat-text-muted">10:35AM</p>
								</div>
							</div>
						</ScrollArea.Root>

						<!-- Input Area -->
						<div class="p-4 bg-chat-primary-lighter/40 border-t border-chat-primary-light">
							<div class="flex items-end gap-2">
								<div class="flex-1 flex items-end gap-2 bg-white rounded-[12px] border shadow-md border-chat-primary-light">
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
										rows={1}
										class="self-center flex-1 px-4 py-3 bg-transparent text-sm text-chat-text placeholder:text-chat-text-muted outline-none resize-none overflow-y-auto leading-5 min-h-6"
										style="max-height: 200px;"
									></textarea>
									<button class="p-2.5 text-chat-primary hover:text-chat-primary-dark transition-colors" aria-label="Voice input">
										<Mic class="w-5 h-5" />
									</button>
								</div>
								<button
									onclick={sendMessage}
									class="p-3 bg-chat-primary-dark text-white rounded-full hover:bg-chat-primary transition-colors disabled:opacity-50"
									disabled={!inputValue.trim()}
									aria-label="Send message"
								>
									<SendHorizontal class="w-5 h-5" />
								</button>
							</div>
						</div>
					</div>
				{:else}
					<div class="h-[60vh] overflow-auto">
						<ExtractedClaims
							claims={initialClaims}
							onApprove={handleClaimApprove}
							onSave={handleClaimSave}
							onRemove={handleClaimRemove}
							onAdd={handleAddClaim}
						/>
					</div>
				{/if}
			</div>
		{:else}
			<!-- Desktop: Side by Side -->
			<div class="flex h-[700px]">
				<!-- Chat Panel -->
				<div class="flex-1 flex flex-col border-r border-chat-primary-light min-h-0">
					<ScrollArea.Root bind:ref={scrollAreaRef} class="flex-1 min-h-0">
						<div class="p-6 bg-chat-primary-lighter/40 min-h-full">
							<div class="text-center mb-4">
								<p class="text-xs text-chat-text-muted">
									{new Date().toISOString().slice(0, 10).replace(/-/g, '.')}
								</p>
							</div>

							<div class="space-y-4">
								{#each chatMessages as message, index (message.id)}
									<div class={message.isBot ? '' : 'flex justify-end'}>
										<div
											class="{message.isBot
												? 'bg-white rounded-br-[16px]'
												: 'bg-chat-primary-dark rounded-bl-[16px]'} w-fit max-w-md rounded-tl-[16px] rounded-tr-[16px] px-3 py-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.15)]"
										>
											{#if message.isBot}
												<div>
													<div class="flex items-start gap-2">
														{#if index < 1}
															<Sparkles class="w-4 h-4 text-chat-primary mt-0.5 flex-shrink-0" />
														{/if}
														<span class="text-chat-text text-sm">{message.content}</span>
													</div>

													{#if index === 0}
														<div class="flex flex-col gap-3 mt-3">
															{#each initialQuestions as question (question.id)}
																<button
																	onclick={() => handleQuestionClick(question)}
																	class="{selectedQuestionId === question.id
																		? 'bg-chat-primary border-chat-primary'
																		: question.variant === 'primary'
																			? 'bg-chat-primary border-chat-primary'
																			: 'bg-white border-chat-primary-light'} px-2.5 py-1.5 rounded-[16px] border flex items-start gap-2.5 w-fit"
																>
																	<span
																		class="{selectedQuestionId === question.id ||
																		question.variant === 'primary'
																			? 'text-white'
																			: 'text-chat-primary'} text-xs font-normal leading-4"
																	>
																		{question.text}
																	</span>
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
							</div>

							<div class="mt-2">
								<p class="text-xs text-chat-text-muted">10:35AM</p>
							</div>
						</div>
					</ScrollArea.Root>

					<!-- Input Area -->
					<div class="p-4 bg-chat-primary-lighter/40 border-t border-chat-primary-light">
						<div class="flex items-end gap-2">
							<div class="flex-1 flex items-end gap-2 bg-white rounded-[12px] border shadow-md border-chat-primary-light">
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
									rows={1}
									class="self-center flex-1 px-4 py-3 bg-transparent text-sm text-chat-text placeholder:text-chat-text-muted outline-none resize-none overflow-y-auto leading-5 min-h-6"
									style="max-height: 200px;"
								></textarea>
								<button class="p-2.5 text-chat-primary hover:text-chat-primary-dark transition-colors" aria-label="Voice input">
									<Mic class="w-5 h-5" />
								</button>
							</div>
							<button
								onclick={sendMessage}
								class="p-3 bg-chat-primary-dark text-white rounded-full hover:bg-chat-primary transition-colors disabled:opacity-50"
								disabled={!inputValue.trim()}
								aria-label="Send message"
							>
								<SendHorizontal class="w-5 h-5" />
							</button>
						</div>
					</div>
				</div>

				<!-- Claims Panel -->
				<ExtractedClaims
					claims={initialClaims}
					onApprove={handleClaimApprove}
					onSave={handleClaimSave}
					onRemove={handleClaimRemove}
					onAdd={handleAddClaim}
				/>
			</div>
		{/if}
	</div>

	<button class="mt-4 px-8 py-3 bg-chat-primary-dark text-white rounded-full font-medium hover:bg-chat-primary transition-colors">
		Continue
	</button>
</div>
