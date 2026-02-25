<script lang="ts">
	import { tick } from 'svelte';
	import { Plus } from 'lucide-svelte';
	import * as ScrollArea from '$lib/components/ui/scroll-area';
	import type { ExtractedClaim } from './types';

	interface Props {
		claims?: ExtractedClaim[];
		onApprove?: (claimId: string) => void;
		onEdit?: (claimId: string, newContent: string) => void;
		onRemove?: (claimId: string) => void;
		onSave?: (claimId: string, content: string) => void;
		onAdd?: () => void;
	}

	let {
		claims = [],
		onApprove = () => {},
		onEdit = () => {},
		onRemove = () => {},
		onSave = () => {},
		onAdd = () => {}
	}: Props = $props();

	let editingClaims = $state<Record<string, string>>({});
	let scrollAreaRef: HTMLElement | null = $state(null);

	$effect(() => {
		const claimsLength = claims.length;
		if (claimsLength > 0 && scrollAreaRef) {
			tick().then(() => {
				const viewport = scrollAreaRef?.querySelector('[data-slot="scroll-area-viewport"]');
				if (viewport) {
					viewport.scrollTop = viewport.scrollHeight;
				}
			});
		}
	});

	async function handleAdd() {
		onAdd();
		await tick();
		if (scrollAreaRef) {
			const viewport = scrollAreaRef.querySelector('[data-slot="scroll-area-viewport"]');
			if (viewport) {
				viewport.scrollTop = viewport.scrollHeight;
			}
		}
	}

	function startEditing(claim: ExtractedClaim) {
		editingClaims[claim.id] = claim.content;
	}

	function handleSave(claimId: string) {
		const content = editingClaims[claimId];
		if (content !== undefined) {
			onSave(claimId, content);
			delete editingClaims[claimId];
			editingClaims = { ...editingClaims };
		}
	}

	function isEditing(claimId: string): boolean {
		return claimId in editingClaims;
	}
</script>

<div class="w-full lg:w-96 h-full p-6 bg-chat-primary-lighter/40 rounded-br-2xl border-r border-b border-chat-primary-light flex flex-col justify-start items-center gap-6">
	<div class="self-stretch flex-1 flex flex-col justify-between items-center min-h-0">
		<ScrollArea.Root bind:ref={scrollAreaRef} class="self-stretch flex-1 min-h-0">
			<div class="flex flex-col justify-start items-start gap-4">
			<div class="self-stretch justify-center text-chat-primary text-lg font-semibold leading-7 line-clamp-1 flex-shrink-0">
				Extracted claims
			</div>
			{#each claims as claim, index (claim.id)}
				<div class="self-stretch flex flex-col justify-start items-start gap-2.5">
					<div class="self-stretch flex flex-col justify-start items-start gap-2">
						<div class="self-stretch p-4 bg-white rounded-xl shadow-[0px_1px_2px_0px_rgba(0,0,0,0.15)] flex flex-col justify-start items-start gap-3 overflow-hidden">
							<div class="justify-start text-chat-primary text-sm font-medium leading-5">
								Opinion {index + 1}
							</div>
							
							<div class="self-stretch flex flex-col justify-start items-start gap-1">
								{#if claim.status === 'streaming'}
									<div class="self-stretch justify-start text-chat-text-muted text-base font-semibold leading-6">
										{claim.content}<span class="inline-block w-1.5 h-4 ml-0.5 bg-chat-primary animate-pulse"></span>
									</div>
								{:else if isEditing(claim.id) || claim.status === 'editing'}
									<div class="self-stretch h-10 px-3 py-1 bg-white rounded-lg shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] border border-chat-border inline-flex justify-start items-center gap-2 overflow-hidden">
										<input
											type="text"
											bind:value={editingClaims[claim.id]}
											class="flex-1 justify-start text-chat-text text-sm font-normal leading-5 bg-transparent outline-none"
											placeholder="Enter claim..."
										/>
									</div>
								{:else}
									<div class="self-stretch justify-start text-chat-text-muted text-base font-semibold leading-6">
										{claim.content}
									</div>
								{/if}
							</div>
							
							<div class="inline-flex justify-start items-center gap-2 flex-wrap content-center">
								{#if claim.status === 'streaming'}
									<div class="h-8 px-3 py-2 bg-chat-primary-lighter rounded-lg flex justify-center items-center gap-2">
										<span class="justify-center text-chat-primary text-xs font-medium leading-4">Streaming...</span>
									</div>
								{:else if isEditing(claim.id) || claim.status === 'editing'}
									<button
										onclick={() => handleSave(claim.id)}
										class="h-8 px-3 py-2 bg-white rounded-lg shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] border border-chat-border flex justify-center items-center gap-2 hover:bg-gray-50 transition-colors"
									>
										<span class="justify-center text-chat-text text-xs font-medium leading-4">Save</span>
									</button>
								{:else if claim.status === 'approved'}
									<div class="h-8 px-3 py-2 bg-chat-primary-dark rounded-lg shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] flex justify-center items-center gap-2">
										<span class="justify-center text-white text-xs font-medium leading-4">Approved</span>
									</div>
									<button
										onclick={() => startEditing(claim)}
										class="h-8 px-3 py-2 bg-white rounded-lg shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] border border-chat-border flex justify-center items-center gap-2 hover:bg-gray-50 transition-colors"
									>
										<span class="justify-center text-chat-text text-xs font-medium leading-4">Edit</span>
									</button>
									<button
										onclick={() => onRemove(claim.id)}
										class="h-8 px-3 py-2 bg-white rounded-lg shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] border border-chat-border flex justify-center items-center gap-2 hover:bg-red-50 transition-colors"
									>
										<span class="justify-center text-red-500 text-xs font-medium leading-4">Remove</span>
									</button>
								{:else}
									<button
										onclick={() => onApprove(claim.id)}
										class="h-8 px-3 py-2 bg-chat-primary-dark rounded-lg shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] flex justify-center items-center gap-2 hover:bg-chat-primary transition-colors"
									>
										<span class="justify-center text-white text-xs font-medium leading-4">Approve</span>
									</button>
									<button
										onclick={() => startEditing(claim)}
										class="h-8 px-3 py-2 bg-white rounded-lg shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] border border-chat-border flex justify-center items-center gap-2 hover:bg-gray-50 transition-colors"
									>
										<span class="justify-center text-chat-text text-xs font-medium leading-4">Edit</span>
									</button>
									<button
										onclick={() => onRemove(claim.id)}
										class="h-8 px-3 py-2 bg-white rounded-lg shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] border border-chat-border flex justify-center items-center gap-2 hover:bg-red-50 transition-colors"
									>
										<span class="justify-center text-red-500 text-xs font-medium leading-4">Remove</span>
									</button>
								{/if}
							</div>
						</div>
					</div>
				</div>
				{/each}
			</div>
		</ScrollArea.Root>
		
		<button
			onclick={handleAdd}
			class="w-12 h-12 mt-4 flex-shrink-0 bg-white rounded-full overflow-hidden shadow-md hover:shadow-lg transition-shadow flex items-center justify-center"
		>
			<Plus class="w-6 h-6 text-chat-neutral" />
		</button>
	</div>
</div>
