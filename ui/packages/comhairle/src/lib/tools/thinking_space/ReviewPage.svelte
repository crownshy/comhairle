<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Card, CardContent } from '$lib/components/ui/card';
	import { Pencil } from 'lucide-svelte';
	import type { ParticipantClaim } from './types';

	type Props = {
		claims: ParticipantClaim[];
		onChange: (claims: ParticipantClaim[]) => void;
		onSubmit: () => void;
	};

	let { claims, onChange, onSubmit }: Props = $props();

	let editingId = $state<string | null>(null);
	let editDraft = $state('');

	function update(next: ParticipantClaim[]) {
		onChange(next);
	}

	function startEdit(claim: ParticipantClaim) {
		editingId = claim.id;
		editDraft = claim.content;
	}

	function saveEdit() {
		if (!editingId) return;
		update(claims.map((c) => (c.id === editingId ? { ...c, content: editDraft } : c)));
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
			These were captured from your reflections. Edit any that don't sound right before
			submitting.
		</p>
	</header>

	<div class="w-full space-y-3">
		{#each claims as claim (claim.id)}
			<Card>
				<CardContent class="space-y-3 px-4">
					<div class="flex items-start justify-between gap-3">
						<p
							class="text-muted-foreground text-xs font-medium tracking-wide uppercase"
						>
							{truncate(claim.sourceQuestionText, 80)}
						</p>
						{#if editingId !== claim.id}
							<Button
								size="sm"
								variant="outline"
								class="shrink-0"
								onclick={() => startEdit(claim)}
							>
								<Pencil class="size-3.5" />
								Edit
							</Button>
						{/if}
					</div>

					{#if editingId === claim.id}
						<Textarea bind:value={editDraft} rows={3} class="text-sm" />
						<div class="flex gap-2">
							<Button size="sm" onclick={saveEdit}>Save</Button>
							<Button size="sm" variant="ghost" onclick={cancelEdit}>Cancel</Button>
						</div>
					{:else}
						<p class="text-foreground text-base leading-relaxed">
							{claim.content}
						</p>
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
		<Button size="lg" class="w-full" disabled={claims.length === 0} onclick={onSubmit}>
			Submit my views
		</Button>
	</div>
</div>
