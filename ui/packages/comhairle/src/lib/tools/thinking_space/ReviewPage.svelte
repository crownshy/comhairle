<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Card, CardContent } from '$lib/components/ui/card';
	import { Check, Pencil, X, Undo2 } from 'lucide-svelte';
	import type { ParticipantClaim } from './types';

	type Props = {
		claims: ParticipantClaim[];
		onChange: (claims: ParticipantClaim[]) => void;
		onSubmit: () => void;
	};

	let { claims, onChange, onSubmit }: Props = $props();

	let editingId = $state<string | null>(null);
	let editDraft = $state('');

	let activeClaims = $derived(claims.filter((c) => c.status !== 'removed'));
	let allResolved = $derived(
		activeClaims.length > 0 && activeClaims.every((c) => c.status === 'approved')
	);
	let approvedCount = $derived(activeClaims.filter((c) => c.status === 'approved').length);

	function update(next: ParticipantClaim[]) {
		onChange(next);
	}

	function toggleApprove(id: string) {
		update(
			claims.map((c) =>
				c.id === id ? { ...c, status: c.status === 'approved' ? 'pending' : 'approved' } : c
			)
		);
	}

	function toggleRemove(id: string) {
		update(
			claims.map((c) =>
				c.id === id ? { ...c, status: c.status === 'removed' ? 'pending' : 'removed' } : c
			)
		);
		if (editingId === id) editingId = null;
	}

	function startEdit(claim: ParticipantClaim) {
		editingId = claim.id;
		editDraft = claim.content;
	}

	function saveEdit() {
		if (!editingId) return;
		update(
			claims.map((c) =>
				c.id === editingId ? { ...c, content: editDraft, status: 'pending' } : c
			)
		);
		editingId = null;
		editDraft = '';
	}

	function cancelEdit() {
		editingId = null;
		editDraft = '';
	}

	function truncate(text: string, n: number) {
		if (text.length <= n) return text;
		return text.slice(0, n - 1) + '…';
	}
</script>

<div class="mx-auto w-full max-w-4xl px-6 py-10">
	<header class="mb-8 text-center">
		<h2 class="text-foreground text-3xl font-semibold tracking-tight">Review your views</h2>
		<p class="text-muted-foreground mx-auto mt-2 max-w-md text-sm leading-relaxed">
			These were captured from your reflections. Approve, edit, or remove each one before
			submitting.
		</p>
		<p class="text-muted-foreground mt-3 text-xs">
			{approvedCount} of {activeClaims.length} approved
		</p>
	</header>

	<div class="w-full space-y-3">
		{#each claims as claim (claim.id)}
			<Card
				class="transition-colors {claim.status === 'approved'
					? 'border-primary/60'
					: claim.status === 'removed'
						? 'border-destructive/30 bg-destructive/5 opacity-60'
						: ''}"
			>
				<CardContent class="space-y-3 p-4">
					<p class="text-muted-foreground text-xs font-medium tracking-wide uppercase">
						{truncate(claim.sourceQuestionText, 80)}
					</p>

					{#if editingId === claim.id}
						<Textarea bind:value={editDraft} rows={3} class="text-sm" />
						<div class="flex gap-2">
							<Button size="sm" onclick={saveEdit}>Save</Button>
							<Button size="sm" variant="ghost" onclick={cancelEdit}>Cancel</Button>
						</div>
					{:else}
						<p
							class="text-foreground text-base leading-relaxed"
							class:line-through={claim.status === 'removed'}
						>
							{claim.content}
						</p>
						<div class="flex flex-wrap gap-2">
							<Button
								size="sm"
								variant={claim.status === 'approved' ? 'default' : 'outline'}
								onclick={() => toggleApprove(claim.id)}
								disabled={claim.status === 'removed'}
							>
								<Check class="size-3.5" />
								{claim.status === 'approved' ? 'Approved' : 'Approve'}
							</Button>
							<Button
								size="sm"
								variant="ghost"
								onclick={() => startEdit(claim)}
								disabled={claim.status === 'removed'}
							>
								<Pencil class="size-3.5" />
								Edit
							</Button>
							<Button
								size="sm"
								variant={claim.status === 'removed' ? 'secondary' : 'ghost'}
								onclick={() => toggleRemove(claim.id)}
								class={claim.status === 'removed'
									? ''
									: 'text-destructive hover:text-destructive'}
							>
								{#if claim.status === 'removed'}
									<Undo2 class="size-3.5" />
									Undo
								{:else}
									<X class="size-3.5" />
									Remove
								{/if}
							</Button>
						</div>
					{/if}
				</CardContent>
			</Card>
		{/each}
	</div>

	{#if claims.length === 0}
		<p class="text-muted-foreground py-8 text-center text-sm">
			No claims captured. Go back and answer some questions to see them here.
		</p>
	{/if}

	<div class="mt-8">
		<Button size="lg" class="w-full" disabled={!allResolved} onclick={onSubmit}>
			{allResolved ? 'Submit my views' : 'Approve or remove each view to continue'}
		</Button>
	</div>
</div>
