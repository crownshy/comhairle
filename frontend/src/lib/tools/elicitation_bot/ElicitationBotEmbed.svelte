<script lang="ts">
	import { onMount, untrack } from 'svelte';
	import ElicitationBotChat from './ElicitationBotChat.svelte';
	import { AgentClient } from '$lib/api/agentClient.svelte';
	import type { ElicitationMessage, ExtractedClaim } from './types';
	import { Loader2 } from 'lucide-svelte';
	import {
		loadClaimModifications,
		saveClaimModifications,
		mergeClaimsWithModifications,
		type ClaimModification
	} from './claimStorage';

	type Props = {
		conversationId: string;
		workflowId: string;
		workflowStepId: string;
		userId: string;
		topic?: string;
		onDone?: () => void;
	};

	let {
		conversationId,
		workflowId,
		workflowStepId,
		userId,
		topic = 'this topic',
		onDone
	}: Props = $props();

	let isLoading = $state(true);
	let initError = $state<string | null>(null);
	let agentClient = $state<AgentClient | null>(null);
	let chatMessages = $state<ElicitationMessage[]>([]);
	let claims = $state<ExtractedClaim[]>([]);
	let activeRequestId = $state<string | null>(null);
	let aiExtractedClaims = $state<ExtractedClaim[]>([]);
	let claimModifications = $state<ClaimModification | null>(null);

	onMount(async () => {
		try {
			claimModifications = loadClaimModifications(workflowStepId, conversationId);

			agentClient = new AgentClient(conversationId, workflowId, workflowStepId);

			agentClient.onClaimUpdate = (streamingClaim, extractedClaims) => {
				const finalizedClaims: ExtractedClaim[] = extractedClaims.map((claim) => ({
					id: claim.id,
					content: claim.content,
					status: 'pending' as const
				}));

				if (streamingClaim && streamingClaim.content) {
					finalizedClaims.push({
						id: streamingClaim.id,
						content: streamingClaim.content,
						status: 'streaming' as const
					});
				}

				aiExtractedClaims = finalizedClaims;
				if (claimModifications) {
					claims = mergeClaimsWithModifications(finalizedClaims, claimModifications);
				} else {
					claims = finalizedClaims;
				}
			};

			chatMessages = [
				{
					id: 'welcome',
					content: `Hello, I am here to help you shape your views and opinions. What is your view on ${topic}?`,
					isBot: true,
					timestamp: new Date()
				}
			];
		} catch (e) {
			console.error('ElicitationBot init error:', e);
			initError = e instanceof Error ? e.message : 'Failed to initialize';
		} finally {
			isLoading = false;
		}
	});

	async function handleSendMessage(message: string) {
		if (!agentClient) return;

		const userMessage: ElicitationMessage = {
			id: `user-${Date.now()}`,
			content: message,
			isBot: false,
			timestamp: new Date()
		};
		chatMessages = [...chatMessages, userMessage];

		// TODO: does this need to change?
		const botPlaceholderId = `bot-${Date.now()}`;
		activeRequestId = botPlaceholderId;

		chatMessages = [
			...chatMessages,
			{
				id: botPlaceholderId,
				content: '...',
				isBot: true,
				timestamp: new Date()
			}
		];

		await agentClient.send(message);

		if (activeRequestId !== botPlaceholderId) {
			return;
		}

		agentClient.finalizeStreamingClaim();

		const finalContent =
			agentClient.currentAnswer ||
			(agentClient.error
				? `Error: ${agentClient.error}`
				: 'No response received from agent.');

		chatMessages = chatMessages.map((msg) =>
			msg.id === botPlaceholderId ? { ...msg, content: finalContent } : msg
		);

		activeRequestId = null;
	}

	$effect(() => {
		const currentAnswer = agentClient?.currentAnswer;
		const reqId = activeRequestId;

		if (currentAnswer && reqId) {
			untrack(() => {
				chatMessages = chatMessages.map((msg) =>
					msg.id === reqId ? { ...msg, content: agentClient!.currentAnswer } : msg
				);
			});
		}
	});

	function persistModifications() {
		if (claimModifications) {
			saveClaimModifications(workflowStepId, conversationId, claimModifications);
			claims = mergeClaimsWithModifications(aiExtractedClaims, claimModifications);
		}
	}

	function handleClaimApprove(claimId: string) {
		if (!claimModifications) return;
		claimModifications.approvedClaimIds.add(claimId);
		persistModifications();
	}

	function handleClaimEdit(claimId: string, newContent: string) {
		if (!claimModifications) return;

		const addedClaimIndex = claimModifications.addedClaims.findIndex((c) => c.id === claimId);
		if (addedClaimIndex !== -1) {
			claimModifications.addedClaims[addedClaimIndex].content = newContent;
			claimModifications.addedClaims[addedClaimIndex].status = 'pending';
		} else {
			claimModifications.editedClaims[claimId] = newContent;
		}
		persistModifications();
	}

	function handleClaimRemove(claimId: string) {
		if (!claimModifications) return;

		const addedClaimIndex = claimModifications.addedClaims.findIndex((c) => c.id === claimId);
		if (addedClaimIndex !== -1) {
			claimModifications.addedClaims.splice(addedClaimIndex, 1);
		} else {
			claimModifications.removedClaimIds.add(claimId);
		}
		persistModifications();
	}

	function handleAddClaim() {
		if (!claimModifications) return;

		const newClaim: ExtractedClaim = {
			id: `user-claim-${Date.now()}`,
			content: '',
			status: 'editing'
		};
		claimModifications.addedClaims.push(newClaim);
		persistModifications();
	}
</script>

{#if isLoading}
	<div class="flex h-96 items-center justify-center">
		<Loader2 class="text-chat-primary h-8 w-8 animate-spin" />
		<span class="text-chat-text-muted ml-2">Loading chat...</span>
	</div>
{:else if initError}
	<div class="flex h-96 flex-col items-center justify-center gap-4">
		<p class="text-destructive">{initError}</p>
		<button
			class="bg-chat-primary hover:bg-chat-primary-dark rounded-lg px-4 py-2 text-white"
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
		{claims}
		onSendMessage={handleSendMessage}
		onClaimApprove={handleClaimApprove}
		onClaimEdit={handleClaimEdit}
		onClaimRemove={handleClaimRemove}
		onAddClaim={handleAddClaim}
	/>
{/if}
