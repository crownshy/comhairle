<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { Button } from '$lib/components/ui/button';
	import { Label } from '$lib/components/ui/label';
	import * as Select from '$lib/components/ui/select';
	import * as Card from '$lib/components/ui/card';
	import { Plus, ArrowUp, ArrowDown, Trash2 } from 'lucide-svelte';
	import type { PrioritisationStore } from './store.svelte';
	import ProposalEditorDialog from './ProposalEditorDialog.svelte';

	let { store }: { store: PrioritisationStore } = $props();

	let editingProposalId = $state<string | null>(null);

	const TIMER_OPTIONS: { value: string; label: string }[] = [
		{ value: 'null', label: 'Forever' },
		{ value: '300', label: '5 minutes' },
		{ value: '600', label: '10 minutes' },
		{ value: '900', label: '15 minutes' },
		{ value: '1800', label: '30 minutes' },
		{ value: '3600', label: '60 minutes' }
	];

	let timerValue = $derived(
		store.poll.settings.timerSeconds === null
			? 'null'
			: String(store.poll.settings.timerSeconds)
	);

	function setTimer(v: string) {
		store.setTimer(v === 'null' ? null : parseInt(v, 10));
	}
</script>

<div class="flex flex-col gap-6">
	<!-- Title / instruction -->
	<Card.Root>
		<Card.Header>
			<Card.Title>Create a poll</Card.Title>
			<Card.Description>
				Each proposal has its own questions. Add proposals below and click into one to edit
				its details and questions.
			</Card.Description>
		</Card.Header>
		<Card.Content class="flex flex-col gap-4">
			<div class="flex flex-col gap-1">
				<Label for="poll-title">Title</Label>
				<p class="text-muted-foreground text-xs">What is this poll called?</p>
				<Input
					id="poll-title"
					placeholder="Enter poll title"
					value={store.poll.title}
					oninput={(e) => store.setTitle((e.target as HTMLInputElement).value)}
				/>
			</div>

			<div class="flex flex-col gap-1">
				<Label for="poll-instruction">Instruction</Label>
				<p class="text-muted-foreground text-xs">
					How should participants answer this poll? Give them some instructions here.
				</p>
				<Textarea
					id="poll-instruction"
					placeholder="Enter instruction"
					value={store.poll.instruction}
					oninput={(e) => store.setInstruction((e.target as HTMLTextAreaElement).value)}
				/>
			</div>
		</Card.Content>
	</Card.Root>

	<!-- Proposals -->
	<Card.Root>
		<Card.Header>
			<Card.Title>Proposals</Card.Title>
			<Card.Description>
				Add the ideas or proposals participants will rank. Click a proposal to edit its
				content and questions.
			</Card.Description>
		</Card.Header>
		<Card.Content class="flex flex-col gap-3">
			{#each store.poll.proposals as p (p.id)}
				<div class="flex items-center gap-2 rounded-md border p-3">
					<span class="text-muted-foreground w-16 text-xs">Proposal {p.order}</span>
					<button
						class="flex-1 truncate text-left text-sm hover:underline"
						onclick={() => (editingProposalId = p.id)}
					>
						{p.title || 'Untitled proposal'}
					</button>
					<span class="text-muted-foreground text-xs">
						{p.questions.length} question{p.questions.length === 1 ? '' : 's'}
					</span>
					<Button
						variant="ghost"
						size="icon"
						onclick={() => store.reorderProposal(p.id, 'up')}
						disabled={p.order === 1}
						aria-label="Move up"
					>
						<ArrowUp class="size-4" />
					</Button>
					<Button
						variant="ghost"
						size="icon"
						onclick={() => store.reorderProposal(p.id, 'down')}
						disabled={p.order === store.poll.proposals.length}
						aria-label="Move down"
					>
						<ArrowDown class="size-4" />
					</Button>
					<Button
						variant="ghost"
						size="icon"
						onclick={() => store.removeProposal(p.id)}
						aria-label="Remove"
					>
						<Trash2 class="size-4" />
					</Button>
				</div>
			{/each}
			{#if store.poll.proposals.length < 2}
				<p class="text-muted-foreground text-xs">Add at least two proposals.</p>
			{/if}
			<Button
				variant="outline"
				onclick={() => {
					const p = store.addProposal();
					editingProposalId = p.id;
				}}
			>
				<Plus class="mr-1 size-4" /> Add proposal
			</Button>
		</Card.Content>
	</Card.Root>

	<!-- Settings -->
	<Card.Root>
		<Card.Header>
			<Card.Title>Settings</Card.Title>
		</Card.Header>
		<Card.Content class="flex flex-col gap-4">
			<div class="flex flex-col gap-1">
				<Label>Set timer</Label>
				<p class="text-muted-foreground text-xs">
					How long will participants have to answer the poll?
				</p>
				<Select.Root
					type="single"
					value={timerValue}
					onValueChange={(v: string) => setTimer(v)}
				>
					<Select.Trigger class="w-[220px]"
						>{TIMER_OPTIONS.find((o) => o.value === timerValue)?.label ?? 'Forever'}
					</Select.Trigger>
					<Select.Content>
						{#each TIMER_OPTIONS as o (o.value)}
							<Select.Item value={o.value}>{o.label}</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
			</div>
		</Card.Content>
	</Card.Root>
</div>

{#if editingProposalId}
	<ProposalEditorDialog
		{store}
		proposalId={editingProposalId}
		onClose={() => (editingProposalId = null)}
	/>
{/if}
