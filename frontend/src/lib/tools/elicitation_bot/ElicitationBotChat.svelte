<script lang="ts">
	import { tick } from 'svelte';
	import { SendHorizontal, Mic, Sparkles, MessageCircle } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import ExtractedClaims from './ExtractedClaims.svelte';
	import type { ElicitationMessage, ElicitationBotProps } from './types';

	let {
		botName = 'Elicitation bot',
		botSubtitle = 'Ask questions',
		messages = [],
		claims: initialClaims = [
			{ id: '1', content: 'What decisions will this influence', status: 'pending' as const },
			{ id: '2', content: 'What decisions will this influence', status: 'editing' as const },
			{ id: '3', content: 'What decisions will this influence', status: 'editing' as const }
		],
		topic,
		placeholder = 'Ask questions....',
		onSendMessage = (message: string) => console.log('Message sent:', message),
		onClaimApprove = (claimId: string) => console.log('Claim approved:', claimId),
		onClaimEdit = (claimId: string, newContent: string) =>
			console.log('Claim edited:', claimId, newContent),
		onClaimRemove = (claimId: string) => console.log('Claim removed:', claimId),
		onAddClaim = () => console.log('Add claim clicked'),
		onDone
	}: ElicitationBotProps = $props();

	const defaultOpeningMessage = {
		id: '1',
		content: `I am here to help you explore your understanding of ${topic}. What is your initial opinion of ${topic}?`,
		isBot: true,
		timestamp: new Date()
	};

	let inputValue = $state('');
	let scrollAreaRef: HTMLElement | null = $state(null);
	let textareaRef: HTMLTextAreaElement | null = $state(null);
	let [, ...messageHistory] = messages;
	let chatMessages = $state<ElicitationMessage[]>([
		defaultOpeningMessage,
		...(messageHistory ?? [])
	]);
	let isMobile = $state(false);
	let activeTab = $state('chat');
	let hasUnseenClaims = $state(false);
	let previousClaimsCount = $state(initialClaims.length);

	function formatTime(date: Date): string {
		return date.toLocaleTimeString('en-US', {
			hour: 'numeric',
			minute: '2-digit',
			hour12: true
		});
	}

	let currentTime = $state(formatTime(new Date()));

	$effect(() => {
		chatMessages;
		tick().then(() => scrollToBottom());
	});

	$effect(() => {
		if (initialClaims.length > previousClaimsCount && activeTab === 'chat' && isMobile) {
			hasUnseenClaims = true;
		}
		previousClaimsCount = initialClaims.length;
	});

	const initialQuestions = [
		{ id: '1', text: 'What does this bot do?', variant: 'default' as const }
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
		return "Great! Tell me more about what you'd like to explore.";
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

<div class="mx-auto flex w-full max-w-5xl flex-col items-center gap-4 p-4">
	<!-- Main Container -->
	<div
		class="border-chat-primary-light w-full overflow-hidden rounded-2xl border bg-white shadow-lg"
	>
		<!-- Bot Header -->
		<div class="border-chat-primary-light flex items-center gap-4 border-b bg-white p-4">
			<div class="relative">
				<div
					class="bg-chat-primary ring-chat-primary-lighter flex h-12 w-12 items-center justify-center rounded-full ring-4"
				>
					<MessageCircle class="h-6 w-6 text-white" />
				</div>
				<div
					class="absolute bottom-0 right-0 h-3 w-3 rounded-full border-2 border-white bg-green-400"
				></div>
			</div>
			<div class="flex flex-1 flex-col">
				<span class="text-chat-text text-lg font-semibold leading-6">{botName}</span>
				<span class="text-chat-primary text-sm font-normal leading-5">{botSubtitle}</span>
			</div>
		</div>

		{#if isMobile}
			<!-- Mobile: Tabbed View -->
			<div class="w-full">
				<div class="bg-chat-bg border-chat-border grid w-full grid-cols-2 border-b">
					<button
						class="py-3 text-sm font-medium transition-colors {activeTab === 'chat'
							? 'text-chat-primary bg-white'
							: 'text-chat-text-muted hover:text-chat-text'}"
						onclick={() => (activeTab = 'chat')}
					>
						Chat
					</button>
					<button
						class="relative py-3 text-sm font-medium transition-colors {activeTab ===
						'claims'
							? 'text-chat-primary bg-white'
							: 'text-chat-text-muted hover:text-chat-text'}"
						onclick={() => {
							activeTab = 'claims';
							hasUnseenClaims = false;
						}}
					>
						<span class="inline-flex items-center gap-1.5">
							Claims ({initialClaims.length})
							{#if hasUnseenClaims}
								<span class="relative flex h-2.5 w-2.5">
									<span
										class="bg-chat-primary absolute inline-flex h-full w-full animate-ping rounded-full opacity-75"
									></span>
									<span
										class="bg-chat-primary relative inline-flex h-2.5 w-2.5 rounded-full"
									></span>
								</span>
							{/if}
						</span>
					</button>
				</div>

				{#if activeTab === 'chat'}
					<div class="flex h-[60vh] flex-col">
						<!-- Chat Messages -->
						<ScrollArea.Root bind:ref={scrollAreaRef} class="min-h-0 flex-1">
							<div class="bg-chat-primary-lighter/40 min-h-full p-6">
								<div class="mb-4 text-center">
									<p class="text-chat-text-muted text-xs">
										{new Date().toISOString().slice(0, 10).replace(/-/g, '.')}
									</p>
								</div>

								<div class="space-y-4">
									{#each chatMessages as message, index (message.id)}
										<div class={message.isBot ? '' : 'flex justify-end'}>
											<div
												class="{message.isBot
													? 'rounded-br-[16px] bg-white'
													: 'bg-chat-primary-dark rounded-bl-[16px]'} w-fit max-w-[85%] rounded-tl-[16px] rounded-tr-[16px] px-3 py-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.15)]"
											>
												{#if message.isBot}
													<div>
														<div class="flex items-start gap-2">
															{#if index < 1}
																<Sparkles
																	class="text-chat-primary mt-0.5 h-4 w-4 flex-shrink-0"
																/>
															{/if}
															{#if message.content === '...'}
																<span
																	class="flex items-center gap-1"
																>
																	<span
																		class="bg-chat-primary h-2 w-2 animate-bounce rounded-full"
																	></span>
																	<span
																		class="bg-chat-primary h-2 w-2 animate-bounce rounded-full"
																		style="animation-delay: 0.1s"
																	></span>
																	<span
																		class="bg-chat-primary h-2 w-2 animate-bounce rounded-full"
																		style="animation-delay: 0.2s"
																	></span>
																</span>
															{:else}
																<span class="text-chat-text text-sm"
																	>{message.content}</span
																>
															{/if}
														</div>

														{#if index === 0}
															<div class="mt-3 flex flex-col gap-3">
																{#each initialQuestions as question (question.id)}
																	<button
																		onclick={() =>
																			handleQuestionClick(
																				question
																			)}
																		class="{selectedQuestionId ===
																		question.id
																			? 'bg-chat-primary border-chat-primary'
																			: question.variant ===
																				  'primary'
																				? 'bg-chat-primary border-chat-primary'
																				: 'border-chat-primary-light bg-white'} flex w-fit items-start gap-2.5 rounded-2xl border px-2.5 py-1.5"
																	>
																		<span
																			class="{selectedQuestionId ===
																				question.id ||
																			question.variant ===
																				'primary'
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
													<p class="text-sm text-white">
														{message.content}
													</p>
												{/if}
											</div>
										</div>
									{/each}
								</div>

								<div class="mt-2">
									<p class="text-chat-text-muted text-xs">{currentTime}</p>
								</div>
							</div>
						</ScrollArea.Root>

						<!-- Input Area -->
						<div
							class="bg-chat-primary-lighter/40 border-chat-primary-light border-t p-4"
						>
							<div class="flex items-end gap-2">
								<div
									class="border-chat-primary-light flex flex-1 items-end gap-2 rounded-xl border bg-white shadow-md"
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
										rows={1}
										class="text-chat-text placeholder:text-chat-text-muted min-h-6 flex-1 resize-none self-center overflow-y-auto bg-transparent px-4 py-3 text-sm leading-5 outline-none"
										style="max-height: 200px;"
									></textarea>
									<button
										class="text-chat-primary hover:text-chat-primary-dark p-2.5 transition-colors"
										aria-label="Voice input"
									>
										<Mic class="h-5 w-5" />
									</button>
								</div>
								<button
									onclick={sendMessage}
									class="bg-chat-primary-dark hover:bg-chat-primary rounded-full p-3 text-white transition-colors disabled:opacity-50"
									disabled={!inputValue.trim()}
									aria-label="Send message"
								>
									<SendHorizontal class="h-5 w-5" />
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
				<div class="border-chat-primary-light flex min-h-0 flex-1 flex-col border-r">
					<ScrollArea.Root bind:ref={scrollAreaRef} class="min-h-0 flex-1">
						<div class="bg-chat-primary-lighter/40 min-h-full p-6">
							<div class="mb-4 text-center">
								<p class="text-chat-text-muted text-xs">
									{new Date().toISOString().slice(0, 10).replace(/-/g, '.')}
								</p>
							</div>

							<div class="space-y-4">
								{#each chatMessages as message, index (message.id)}
									<div class={message.isBot ? '' : 'flex justify-end'}>
										<div
											class="{message.isBot
												? 'rounded-br-[16px] bg-white'
												: 'bg-chat-primary-dark rounded-bl-[16px]'} w-fit max-w-md rounded-tl-[16px] rounded-tr-[16px] px-3 py-2.5 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.15)]"
										>
											{#if message.isBot}
												<div>
													<div class="flex items-start gap-2">
														{#if index < 1}
															<Sparkles
																class="text-chat-primary mt-0.5 h-4 w-4 flex-shrink-0"
															/>
														{/if}
														{#if message.content === '...'}
															<span class="flex items-center gap-1">
																<span
																	class="bg-chat-primary h-2 w-2 animate-bounce rounded-full"
																></span>
																<span
																	class="bg-chat-primary h-2 w-2 animate-bounce rounded-full"
																	style="animation-delay: 0.1s"
																></span>
																<span
																	class="bg-chat-primary h-2 w-2 animate-bounce rounded-full"
																	style="animation-delay: 0.2s"
																></span>
															</span>
														{:else}
															<span class="text-chat-text text-sm"
																>{message.content}</span
															>
														{/if}
													</div>

													{#if index === 0}
														<div class="mt-3 flex flex-col gap-3">
															{#each initialQuestions as question (question.id)}
																<button
																	onclick={() =>
																		handleQuestionClick(
																			question
																		)}
																	class="{selectedQuestionId ===
																	question.id
																		? 'bg-chat-primary border-chat-primary'
																		: question.variant ===
																			  'primary'
																			? 'bg-chat-primary border-chat-primary'
																			: 'border-chat-primary-light bg-white'} flex w-fit items-start gap-2.5 rounded-2xl border px-2.5 py-1.5"
																>
																	<span
																		class="{selectedQuestionId ===
																			question.id ||
																		question.variant ===
																			'primary'
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
												<p class="text-sm text-white">{message.content}</p>
											{/if}
										</div>
									</div>
								{/each}
							</div>

							<div class="mt-2">
								<p class="text-chat-text-muted text-xs">{currentTime}</p>
							</div>
						</div>
					</ScrollArea.Root>

					<!-- Input Area -->
					<div class="bg-chat-primary-lighter/40 border-chat-primary-light border-t p-4">
						<div class="flex items-end gap-2">
							<div
								class="border-chat-primary-light flex flex-1 items-end gap-2 rounded-xl border bg-white shadow-md"
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
									rows={1}
									class="text-chat-text placeholder:text-chat-text-muted min-h-6 flex-1 resize-none self-center overflow-y-auto bg-transparent px-4 py-3 text-sm leading-5 outline-none"
									style="max-height: 200px;"
								></textarea>
								<button
									class="text-chat-primary hover:text-chat-primary-dark p-2.5 transition-colors"
									aria-label="Voice input"
								>
									<Mic class="h-5 w-5" />
								</button>
							</div>
							<button
								onclick={sendMessage}
								class="bg-chat-primary-dark hover:bg-chat-primary rounded-full p-3 text-white transition-colors disabled:opacity-50"
								disabled={!inputValue.trim()}
								aria-label="Send message"
							>
								<SendHorizontal class="h-5 w-5" />
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

	{#if onNext}
		<Button onclick={onDone} class="w-full mt-10">Continue</Button>
	{/if}
</div>
