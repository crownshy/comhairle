<script lang="ts">
	import { SvelteSet } from 'svelte/reactivity';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Tabs from '$lib/components/ui/tabs';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Plus } from 'lucide-svelte';
	import type { BreakoutCreateMode, DiversityCriterion, HybridRule } from './types';

	interface Props {
		open: boolean;
		onOpenChange: (open: boolean) => void;
		onPreview: (config: BreakoutConfig) => void;
		onCreate: (config: BreakoutConfig) => void;
	}

	export type BreakoutConfig =
		| { mode: 'manual'; rooms: number; minPerRoom: number; maxPerRoom: number }
		| {
				mode: 'automatic';
				rooms: number;
				maxPerRoom: number;
				diversifyBy: DiversityCriterion[];
		  }
		| { mode: 'hybrid'; rooms: number; maxPerRoom: number; rules: HybridRule[] };

	let { open, onOpenChange, onPreview, onCreate }: Props = $props();

	let activeMode: BreakoutCreateMode = $state('manual');

	// Manual state
	let manualRooms = $state(3);
	let manualMinPerRoom = $state(3);
	let manualMaxPerRoom = $state(5);

	// Automatic state
	let autoRooms = $state(3);
	let autoMaxPerRoom = $state(6);
	let diversityCriteria = $state<SvelteSet<DiversityCriterion>>(new SvelteSet());

	const criteriaOptions: { value: DiversityCriterion; label: string }[] = [
		{ value: 'age', label: 'Age' },
		{ value: 'gender', label: 'Gender' },
		{ value: 'understanding_of_ai', label: 'Understanding of AI' },
		{ value: 'education', label: 'Education' },
		{ value: 'postcode', label: 'Postcode' },
		{ value: 'in_call_activities', label: 'In-call activities' }
	];

	// Hybrid state
	let hybridRooms = $state(3);
	let hybridMaxPerRoom = $state(6);
	let hybridRules = $state<HybridRule[]>([{ id: '1', participants: [] }]);
	let ruleInputs = $state<Record<string, string>>({ '1': '' });

	function addHybridRule() {
		const id = String(Date.now());
		hybridRules = [...hybridRules, { id, participants: [] }];
		ruleInputs = { ...ruleInputs, [id]: '' };
	}

	function removeHybridRule(id: string) {
		hybridRules = hybridRules.filter((r) => r.id !== id);
		const { [id]: _, ...rest } = ruleInputs;
		ruleInputs = rest;
	}

	function parseParticipants(id: string) {
		const text = ruleInputs[id] ?? '';
		return text
			.split(',')
			.map((s) => s.trim())
			.filter(Boolean);
	}

	function toggleCriterion(criterion: DiversityCriterion) {
		if (diversityCriteria.has(criterion)) {
			diversityCriteria.delete(criterion);
		} else {
			diversityCriteria.add(criterion);
		}
	}

	function buildConfig(): BreakoutConfig {
		if (activeMode === 'manual') {
			return {
				mode: 'manual',
				rooms: manualRooms,
				minPerRoom: manualMinPerRoom,
				maxPerRoom: manualMaxPerRoom
			};
		} else if (activeMode === 'automatic') {
			return {
				mode: 'automatic',
				rooms: autoRooms,
				maxPerRoom: autoMaxPerRoom,
				diversifyBy: [...diversityCriteria]
			};
		} else {
			return {
				mode: 'hybrid',
				rooms: hybridRooms,
				maxPerRoom: hybridMaxPerRoom,
				rules: hybridRules.map((r) => ({
					...r,
					participants: parseParticipants(r.id)
				}))
			};
		}
	}

	function handlePreview() {
		onPreview(buildConfig());
	}

	function handleCreate() {
		onCreate(buildConfig());
		onOpenChange(false);
	}
</script>

<Dialog.Root {open} {onOpenChange}>
	<Dialog.Content class="max-w-lg">
		<Dialog.Header>
			<Dialog.Title>Create Breakout Rooms</Dialog.Title>
			<Dialog.Description>
				Choose how to assign participants to breakout rooms.
			</Dialog.Description>
		</Dialog.Header>

		<Tabs.Root bind:value={activeMode}>
			<Tabs.List class="w-full">
				<Tabs.Trigger value="manual" class="flex-1">Manual</Tabs.Trigger>
				<Tabs.Trigger value="automatic" class="flex-1">Automatic</Tabs.Trigger>
				<Tabs.Trigger value="hybrid" class="flex-1">Hybrid</Tabs.Trigger>
			</Tabs.List>

			<Tabs.Content value="manual" class="mt-4 space-y-4">
				<div class="grid grid-cols-2 gap-4">
					<div class="space-y-1.5">
						<label class="text-sm font-medium" for="manual-rooms">Rooms</label>
						<input
							id="manual-rooms"
							type="number"
							min="1"
							max="50"
							bind:value={manualRooms}
							class="border-border bg-background w-full rounded-lg border px-3 py-2 text-sm"
						/>
					</div>
					<div class="space-y-1.5">
						<label class="text-sm font-medium" for="manual-range">People per room</label
						>
						<div class="flex items-center gap-2">
							<input
								id="manual-range"
								type="number"
								min="1"
								max="50"
								bind:value={manualMinPerRoom}
								class="border-border bg-background w-20 rounded-lg border px-3 py-2 text-sm"
							/>
							<span class="text-muted-foreground">~</span>
							<input
								type="number"
								min="1"
								max="50"
								bind:value={manualMaxPerRoom}
								class="border-border bg-background w-20 rounded-lg border px-3 py-2 text-sm"
							/>
						</div>
					</div>
				</div>
			</Tabs.Content>

			<Tabs.Content value="automatic" class="mt-4 space-y-4">
				<div class="grid grid-cols-2 gap-4">
					<div class="space-y-1.5">
						<label class="text-sm font-medium" for="auto-rooms">Rooms</label>
						<input
							id="auto-rooms"
							type="number"
							min="1"
							max="50"
							bind:value={autoRooms}
							class="border-border bg-background w-full rounded-lg border px-3 py-2 text-sm"
						/>
					</div>
					<div class="space-y-1.5">
						<label class="text-sm font-medium" for="auto-max">Max per room</label>
						<input
							id="auto-max"
							type="number"
							min="1"
							max="50"
							bind:value={autoMaxPerRoom}
							class="border-border bg-background w-full rounded-lg border px-3 py-2 text-sm"
						/>
					</div>
				</div>

				<div class="space-y-2">
					<p class="text-sm font-medium">Diversify by:</p>
					<div class="grid grid-cols-2 gap-2">
						{#each criteriaOptions as criterion (criterion.value)}
							<label
								class="border-border hover:bg-muted/50 flex cursor-pointer items-center gap-2 rounded-lg border px-3 py-2 text-sm"
							>
								<Checkbox
									checked={diversityCriteria.has(criterion.value)}
									onCheckedChange={() => toggleCriterion(criterion.value)}
								/>
								{criterion.label}
							</label>
						{/each}
					</div>
				</div>
			</Tabs.Content>

			<Tabs.Content value="hybrid" class="mt-4 space-y-4">
				<div class="grid grid-cols-2 gap-4">
					<div class="space-y-1.5">
						<label class="text-sm font-medium" for="hybrid-rooms">Rooms</label>
						<input
							id="hybrid-rooms"
							type="number"
							min="1"
							max="50"
							bind:value={hybridRooms}
							class="border-border bg-background w-full rounded-lg border px-3 py-2 text-sm"
						/>
					</div>
					<div class="space-y-1.5">
						<label class="text-sm font-medium" for="hybrid-max">Max per room</label>
						<input
							id="hybrid-max"
							type="number"
							min="1"
							max="50"
							bind:value={hybridMaxPerRoom}
							class="border-border bg-background w-full rounded-lg border px-3 py-2 text-sm"
						/>
					</div>
				</div>

				<div class="space-y-3">
					<p class="text-sm font-medium">Assign these participants in different rooms:</p>

					{#each hybridRules as rule (rule.id)}
						<div class="flex items-start gap-2">
							<input
								type="text"
								placeholder="Alice, Bob, Carol, Delan"
								bind:value={ruleInputs[rule.id]}
								class="border-border bg-background flex-1 rounded-lg border px-3 py-2 text-sm"
							/>
							{#if hybridRules.length > 1}
								<button
									class="text-muted-foreground hover:text-destructive mt-2 text-xs"
									onclick={() => removeHybridRule(rule.id)}
								>
									Remove
								</button>
							{/if}
						</div>
					{/each}

					<button
						class="text-primary flex items-center gap-1 text-sm hover:underline"
						onclick={addHybridRule}
					>
						<Plus class="h-3 w-3" />
						Add one more rule
					</button>
				</div>

				<p class="text-muted-foreground text-sm">Randomly assign the rest</p>
			</Tabs.Content>
		</Tabs.Root>

		<Dialog.Footer class="mt-4">
			<Button variant="outline" onclick={handlePreview}>Preview</Button>
			<Button onclick={handleCreate}>Create</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
