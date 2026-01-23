<script lang="ts">
	import { onMount } from 'svelte';
	import ElicitationBotChat from './ElicitationBotChat.svelte';
	import { AgentClient } from '$lib/api/agentClient.svelte';
	import type { ElicitationMessage } from './types';
	import { Loader2 } from 'lucide-svelte';

	type Props = {
		conversationId: string;
		workflowId: string;
		workflowStepId: string;
		botId: string;
		userId: string;
		onDone?: () => void;
	};

	let { conversationId, workflowId, workflowStepId, botId, userId, onDone }: Props = $props();

	let isLoading = $state(true);
	let initError = $state<string | null>(null);
	let agentClient = $state<AgentClient | null>(null);
	let chatMessages = $state<ElicitationMessage[]>([]);

	onMount(async () => {
		try {
			agentClient = new AgentClient(botId, conversationId, workflowId, workflowStepId);
			const success = await agentClient.initialize();

			if (!success) {
				initError = agentClient.error || 'Failed to initialize chat session';
				return;
			}

			if (agentClient.session?.messages) {
				chatMessages = agentClient.session.messages
					.filter((msg) => msg.content && msg.content.trim() !== '')
					.map((msg, idx) => ({
						id: msg.id || `msg-${idx}`,
						content: msg.content,
						isBot: msg.role === 'assistant',
						timestamp: null
					}));
			}

			// If no messages, add a welcome message
			if (chatMessages.length === 0) {
				chatMessages = [
					{
						id: 'welcome',
						content:
							"Hello, I am here to help you shape your views and opinions. What is your view on {TOPIC}",
						isBot: true,
						timestamp: new Date()
					}
				];
			}
		} catch (e) {
			console.error('ElicitationBot init error:', e);
			initError = e instanceof Error ? e.message : 'Failed to initialize';
		} finally {
			isLoading = false;
		}
	});

	async function handleSendMessage(message: string) {
		if (!agentClient) return;

		// Add user message immediately
		const userMessage: ElicitationMessage = {
			id: `user-${Date.now()}`,
			content: message,
			isBot: false,
			timestamp: new Date()
		};
		chatMessages = [...chatMessages, userMessage];

		// Add a placeholder for the bot response
		const botPlaceholderId = `bot-${Date.now()}`;
		chatMessages = [
			...chatMessages,
			{
				id: botPlaceholderId,
				content: '...',
				isBot: true,
				timestamp: new Date()
			}
		];

		// Send and stream the response
		await agentClient.send(message);

		const finalContent = agentClient.currentAnswer || 
			(agentClient.error ? `Error: ${agentClient.error}` : 'No response received from agent.');
		
		chatMessages = chatMessages.map((msg) =>
			msg.id === botPlaceholderId ? { ...msg, content: finalContent } : msg
		);
		
		console.log('Agent response:', {
			answer: agentClient.currentAnswer,
			error: agentClient.error,
			session: agentClient.session?.id
		});
	}

	$effect(() => {
		if (agentClient?.isStreaming && agentClient.currentAnswer) {
			const lastBotMsgIndex = chatMessages.findLastIndex((msg) => msg.isBot);
			if (lastBotMsgIndex >= 0) {
				chatMessages = chatMessages.map((msg, idx) =>
					idx === lastBotMsgIndex ? { ...msg, content: agentClient!.currentAnswer } : msg
				);
			}
		}
	});
</script>

{#if isLoading}
	<div class="flex h-96 items-center justify-center">
		<Loader2 class="h-8 w-8 animate-spin text-chat-primary" />
		<span class="ml-2 text-chat-text-muted">Loading chat...</span>
	</div>
{:else if initError}
	<div class="flex h-96 flex-col items-center justify-center gap-4">
		<p class="text-destructive">{initError}</p>
		<button
			class="rounded-lg bg-chat-primary px-4 py-2 text-white hover:bg-chat-primary-dark"
			onclick={() => window.location.reload()}
		>
			Try Again
		</button>
	</div>
{:else}
	<ElicitationBotChat
		botName="Elicitation Bot"
		botSubtitle="Here to help you shape your views and opinions"
		messages={chatMessages}
		claims={[]}
		onSendMessage={handleSendMessage}
	/>
{/if}
