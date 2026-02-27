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

<div
	class="bg-chat-primary-lighter/40 border-chat-primary-light flex h-full w-full flex-col items-center justify-start gap-6 rounded-br-2xl border-r border-b p-6 lg:w-96"
>
	<div class="flex min-h-0 flex-1 flex-col items-center justify-between self-stretch">
		<ScrollArea.Root bind:ref={scrollAreaRef} class="min-h-0 flex-1 self-stretch">
			<div class="flex flex-col items-start justify-start gap-4">
				<div
					class="text-chat-primary line-clamp-1 flex-shrink-0 justify-center self-stretch text-lg leading-7 font-semibold"
				>
					Extracted claims
				</div>
				{#each claims as claim, index (claim.id)}
					<div class="flex flex-col items-start justify-start gap-2.5 self-stretch">
						<div class="flex flex-col items-start justify-start gap-2 self-stretch">
							<div
								class="bg-chat-bubble flex flex-col items-start justify-start gap-3 self-stretch overflow-hidden rounded-xl p-4 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.15)]"
							>
								<div
									class="text-chat-primary justify-start text-sm leading-5 font-medium"
								>
									Opinion {index + 1}
								</div>

								<div
									class="flex flex-col items-start justify-start gap-1 self-stretch"
								>
									{#if claim.status === 'streaming'}
										<div
											class="text-chat-text-muted justify-start self-stretch text-base leading-6 font-semibold"
										>
											{claim.content}<span
												class="bg-chat-primary ml-0.5 inline-block h-4 w-1.5 animate-pulse"
											></span>
										</div>
									{:else if isEditing(claim.id) || claim.status === 'editing'}
										<div
											class="bg-chat-bubble border-chat-border inline-flex h-10 items-center justify-start gap-2 self-stretch overflow-hidden rounded-lg border px-3 py-1 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)]"
										>
											<input
												type="text"
												bind:value={editingClaims[claim.id]}
												class="text-chat-text flex-1 justify-start bg-transparent text-sm leading-5 font-normal outline-none"
												placeholder="Enter claim..."
											/>
										</div>
									{:else}
										<div
											class="text-chat-text-muted justify-start self-stretch text-base leading-6 font-semibold"
										>
											{claim.content}
										</div>
									{/if}
								</div>

								<div
									class="inline-flex flex-wrap content-center items-center justify-start gap-2"
								>
									{#if claim.status === 'streaming'}
										<div
											class="bg-chat-primary-lighter flex h-8 items-center justify-center gap-2 rounded-lg px-3 py-2"
										>
											<span
												class="text-chat-primary justify-center text-xs leading-4 font-medium"
												>Streaming...</span
											>
										</div>
									{:else if isEditing(claim.id) || claim.status === 'editing'}
										<button
											onclick={() => handleSave(claim.id)}
											class="bg-chat-bubble border-chat-border hover:bg-chat-bg flex h-8 items-center justify-center gap-2 rounded-lg border px-3 py-2 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] transition-colors"
										>
											<span
												class="text-chat-text justify-center text-xs leading-4 font-medium"
												>Save</span
											>
										</button>
									{:else if claim.status === 'approved'}
										<div
											class="bg-chat-primary-dark flex h-8 items-center justify-center gap-2 rounded-lg px-3 py-2 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)]"
										>
											<span
												class="justify-center text-xs leading-4 font-medium text-white"
												>Approved</span
											>
										</div>
										<button
											onclick={() => startEditing(claim)}
											class="bg-chat-bubble border-chat-border hover:bg-chat-bg flex h-8 items-center justify-center gap-2 rounded-lg border px-3 py-2 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] transition-colors"
										>
											<span
												class="text-chat-text justify-center text-xs leading-4 font-medium"
												>Edit</span
											>
										</button>
										<button
											onclick={() => onRemove(claim.id)}
											class="bg-chat-bubble border-chat-border hover:bg-destructive/10 flex h-8 items-center justify-center gap-2 rounded-lg border px-3 py-2 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] transition-colors"
										>
											<span
												class="justify-center text-xs leading-4 font-medium text-red-500"
												>Remove</span
											>
										</button>
									{:else}
										<button
											onclick={() => onApprove(claim.id)}
											class="bg-chat-primary-dark hover:bg-chat-primary flex h-8 items-center justify-center gap-2 rounded-lg px-3 py-2 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] transition-colors"
										>
											<span
												class="justify-center text-xs leading-4 font-medium text-white"
												>Approve</span
											>
										</button>
										<button
											onclick={() => startEditing(claim)}
											class="bg-chat-bubble border-chat-border hover:bg-chat-bg flex h-8 items-center justify-center gap-2 rounded-lg border px-3 py-2 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] transition-colors"
										>
											<span
												class="text-chat-text justify-center text-xs leading-4 font-medium"
												>Edit</span
											>
										</button>
										<button
											onclick={() => onRemove(claim.id)}
											class="bg-chat-bubble border-chat-border hover:bg-destructive/10 flex h-8 items-center justify-center gap-2 rounded-lg border px-3 py-2 shadow-[0px_1px_2px_0px_rgba(0,0,0,0.05)] transition-colors"
										>
											<span
												class="justify-center text-xs leading-4 font-medium text-red-500"
												>Remove</span
											>
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
			class="bg-chat-bubble mt-4 flex h-12 w-12 flex-shrink-0 items-center justify-center overflow-hidden rounded-full shadow-md transition-shadow hover:shadow-lg"
		>
			<Plus class="text-chat-neutral h-6 w-6" />
		</button>
	</div>
</div>
