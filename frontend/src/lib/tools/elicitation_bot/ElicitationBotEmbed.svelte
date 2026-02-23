<script lang="ts">
	import { onMount, untrack } from 'svelte';
	import ElicitationBotChat from './ElicitationBotChat.svelte';
	import { AgentClient, parseSessionHistory } from '$lib/api/agentClient.svelte';
	import type { ElicitationMessage, ExtractedClaim } from './types';
	import { Loader2 } from 'lucide-svelte';
	import { loadClaims, saveClaimModifications, type ClaimModification } from './claimStorage';

	type Props = {
		conversationId: string;
		workflowId: string;
		workflowStepId: string;
		userId: string;
		topic?: string;
		onDone?: () => void;
	};

	let { conversationId, workflowStepId, userId, topic = 'this topic', onDone }: Props = $props();

	let isLoading = $state(true);
	let initError = $state<string | null>(null);
	let agentClient = $state<AgentClient | null>(null);
	let chatMessages = $state<ElicitationMessage[]>([]);
	let claims = $state<ExtractedClaim[]>([]);
	let activeRequestId = $state<string | null>(null);
	let claimModifications = $state<ClaimModification | null>(null);

	onMount(async () => {
		try {
			const mods = loadClaims(workflowStepId, conversationId, userId);
			claimModifications = mods;
			claims = mods.addedClaims
				.filter((c) => !mods.removedClaimIds.has(c.id))
				.map((c) => ({
					...c,
					status: mods.approvedClaimIds.has(c.id) ? ('approved' as const) : c.status
				}));

			agentClient = new AgentClient(workflowStepId);
			const sessionHistory = await agentClient.getSessionHistory();

			if (sessionHistory) {
				const { messages: loadedMessages } = parseSessionHistory(sessionHistory, topic);
				chatMessages = loadedMessages;
			}

			agentClient.onClaimUpdate = (streamingClaim, extractedClaims) => {
				const newClaims: ExtractedClaim[] = [];

				for (const claim of extractedClaims) {
					if (!claimModifications?.addedClaims.some((c) => c.id === claim.id)) {
						const newClaim: ExtractedClaim = {
							id: claim.id,
							content: claim.content,
							status: 'pending'
						};
						newClaims.push(newClaim);
						claimModifications?.addedClaims.push(newClaim);
					}
				}

				if (newClaims.length > 0 && claimModifications) {
					saveClaimModifications(
						workflowStepId,
						conversationId,
						userId,
						claimModifications
					);
				}

				const mods = claimModifications;
				const displayClaims: ExtractedClaim[] = mods
					? mods.addedClaims
							.filter((c) => !mods.removedClaimIds.has(c.id))
							.map((c) => ({
								...c,
								status: mods.approvedClaimIds.has(c.id)
									? ('approved' as const)
									: c.status
							}))
					: [];

				if (streamingClaim && streamingClaim.content) {
					displayClaims.push({
						id: streamingClaim.id,
						content: streamingClaim.content,
						status: 'streaming'
					});
				}

				claims = displayClaims;
			};
		} catch (e) {
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

		if (activeRequestId !== botPlaceholderId) return;

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
		const mods = claimModifications;
		if (mods) {
			saveClaimModifications(workflowStepId, conversationId, userId, mods);
			claims = mods.addedClaims
				.filter((c) => !mods.removedClaimIds.has(c.id))
				.map((c) => ({
					...c,
					status: mods.approvedClaimIds.has(c.id) ? ('approved' as const) : c.status
				}));
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
		{topic}
		onSendMessage={handleSendMessage}
		onClaimApprove={handleClaimApprove}
		onClaimEdit={handleClaimEdit}
		onClaimRemove={handleClaimRemove}
		onAddClaim={handleAddClaim}
		onDone={onDone}
	/>
{/if}
