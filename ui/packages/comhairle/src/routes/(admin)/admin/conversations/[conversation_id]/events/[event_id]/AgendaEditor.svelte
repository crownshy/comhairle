<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import * as Select from '$lib/components/ui/select';
	import { Trash2, GripVertical } from 'lucide-svelte';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import type { AgendaItemData } from './agenda-types';

	interface Props {
		items: AgendaItemData[];
		onUpdate: (items: AgendaItemData[]) => void;
	}

	let { items = $bindable(), onUpdate }: Props = $props();

	const balanceOptions = [
		'Age',
		'Gender',
		'Understanding of AI',
		'Education',
		'Postcode',
		'In-call activities'
	];

	function createId(): string {
		return crypto.randomUUID();
	}

	function addStandardItem() {
		items = [...items, { id: createId(), type: 'standard', title: '' }];
		onUpdate(items);
	}

	function addBreakoutSession() {
		items = [
			...items,
			{
				id: createId(),
				type: 'breakout',
				title: '',
				duration: 10,
				groupSize: 4,
				prompts: [{ title: '', instructions: '' }],
				assignmentMode: 'random',
				balanceBy: []
			}
		];
		onUpdate(items);
	}

	function removeItem(index: number) {
		items = items.filter((_, i) => i !== index);
		onUpdate(items);
	}

	function addPrompt(itemIndex: number) {
		const item = items[itemIndex];
		if (item.prompts) {
			item.prompts = [...item.prompts, { title: '', instructions: '' }];
			items = [...items];
			onUpdate(items);
		}
	}

	function removePrompt(itemIndex: number, promptIndex: number) {
		const item = items[itemIndex];
		if (item.prompts && item.prompts.length > 1) {
			item.prompts = item.prompts.filter((_, i) => i !== promptIndex);
			items = [...items];
			onUpdate(items);
		}
	}

	function toggleBalance(itemIndex: number, value: string) {
		const item = items[itemIndex];
		if (!item.balanceBy) item.balanceBy = [];
		if (item.balanceBy.includes(value)) {
			item.balanceBy = item.balanceBy.filter((v) => v !== value);
		} else {
			item.balanceBy = [...item.balanceBy, value];
		}
		items = [...items];
		onUpdate(items);
	}

	// Drag state
	let dragIdx: number | null = $state(null);
	let dropIdx: number | null = $state(null);

	function handleDragStart(e: DragEvent, idx: number) {
		dragIdx = idx;
		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'move';
			e.dataTransfer.setData('text/plain', String(idx));
		}
	}

	function handleDragOver(e: DragEvent, idx: number) {
		e.preventDefault();
		dropIdx = idx;
	}

	function handleDragLeave(e: DragEvent, idx: number) {
		const related = e.relatedTarget as HTMLElement | null;
		const currentTarget = e.currentTarget as HTMLElement;
		if (!related || !currentTarget.contains(related)) {
			if (dropIdx === idx) dropIdx = null;
		}
	}

	function handleDrop(e: DragEvent, targetIdx: number) {
		e.preventDefault();
		if (dragIdx === null || dragIdx === targetIdx) {
			dragIdx = null;
			dropIdx = null;
			return;
		}
		const newItems = [...items];
		const [moved] = newItems.splice(dragIdx, 1);
		newItems.splice(targetIdx, 0, moved);
		items = newItems;
		onUpdate(items);
		dragIdx = null;
		dropIdx = null;
	}

	function handleDragEnd() {
		dragIdx = null;
		dropIdx = null;
	}
</script>

<div class="flex flex-col gap-7">
	<div class="flex flex-col gap-2">
		<h3 class="text-foreground text-2xl font-bold">Agenda</h3>
		<p class="text-muted-foreground text-base">Optimise topics and activities</p>
	</div>

	<!-- Agenda items -->
	{#each items as item, index (item.id)}
		<div
			class="flex items-start gap-4 transition-all {dropIdx === index
				? 'border-ring border-t-2'
				: ''}"
			role="group"
			ondragover={(e) => handleDragOver(e, index)}
			ondragleave={(e) => handleDragLeave(e, index)}
			ondrop={(e) => handleDrop(e, index)}
		>
			<div class="border-border flex flex-1 flex-col gap-6 rounded-lg border p-6">
				{#if item.type === 'standard'}
					<!-- Standard item -->
					<div class="text-foreground text-xl font-bold">Standard item</div>
					<div class="flex items-center gap-6">
						<Label class="w-16 shrink-0 font-bold">Title</Label>
						<Input
							bind:value={item.title}
							placeholder="Enter agenda item here"
							class="bg-muted max-w-60"
							oninput={() => onUpdate(items)}
						/>
					</div>
				{:else}
					<!-- Breakout session -->
					<div class="text-foreground text-xl font-bold">Breakout session</div>

					<!-- Time -->
					<div class="flex items-center gap-2">
						<div class="flex flex-col gap-1">
							<Label class="font-bold">Time</Label>
							<span class="text-muted-foreground text-sm">Session duration</span>
						</div>
						<Input
							type="number"
							bind:value={item.duration}
							class="bg-muted w-14"
							min={1}
							max={120}
							oninput={() => onUpdate(items)}
						/>
						<span class="text-sm">minutes</span>
					</div>

					<!-- Group size -->
					<div class="flex items-center gap-2">
						<div class="flex flex-col gap-1">
							<Label class="font-bold">Group size</Label>
							<span class="text-muted-foreground text-sm">People per room</span>
						</div>
						<Input
							type="number"
							bind:value={item.groupSize}
							class="bg-muted w-14"
							min={2}
							max={20}
							oninput={() => onUpdate(items)}
						/>
						<span class="text-sm">people</span>
					</div>

					<!-- Breakout prompts -->
					<div class="text-foreground text-lg font-bold">Breakout prompt</div>

					{#each item.prompts ?? [] as prompt, pIdx}
						<div class="border-border flex flex-col gap-6 rounded-lg border p-6">
							<div class="flex items-start gap-4">
								<div class="flex flex-col gap-1">
									<Label class="font-bold">Title</Label>
									<span class="text-muted-foreground text-sm"
										>What should each group discuss?</span
									>
								</div>
								<Input
									bind:value={prompt.title}
									placeholder="Enter prompt title here"
									class="bg-muted max-w-64"
									oninput={() => onUpdate(items)}
								/>
							</div>

							<div class="flex flex-col gap-2">
								<Label class="font-bold">Instructions</Label>
								<span class="text-muted-foreground text-sm"
									>What should participants do during this session?</span
								>
								<RichTextEditor
									value={prompt.instructions}
									placeholder="Enter instructions here"
									minHeight="120px"
									onChange={(json) => {
										prompt.instructions = json;
										onUpdate(items);
									}}
								/>
							</div>

							{#if (item.prompts?.length ?? 0) > 1}
								<button
									class="text-destructive hover:text-destructive/80 flex items-center gap-1 text-sm"
									onclick={() => removePrompt(index, pIdx)}
								>
									<Trash2 class="h-4 w-4" />
									Remove prompt
								</button>
							{/if}
						</div>
					{/each}

					<Button variant="default" class="w-fit" onclick={() => addPrompt(index)}>
						+ Add breakout prompt
					</Button>

					<!-- Group assignment -->
					<div class="flex flex-col gap-2">
						<Label class="font-bold">Group assignment</Label>
						<span class="text-muted-foreground text-sm"
							>How do we want to breakdown the groups?</span
						>
						<select
							bind:value={item.assignmentMode}
							class="border-input bg-muted/50 w-64 rounded-md border p-2.5 text-sm"
							onchange={() => onUpdate(items)}
						>
							<option value="random">Random</option>
							<option value="diversify">Diversify by participant info</option>
						</select>
					</div>

					<!-- Balance by (only if diversify) -->
					{#if item.assignmentMode === 'diversify'}
						<div class="flex flex-col gap-2">
							<Label class="font-bold">Balance group by</Label>
							<span class="text-muted-foreground text-sm"
								>How would you like to balance each group's composition?</span
							>
							<div class="flex flex-col">
								{#each balanceOptions as opt}
									<label class="flex cursor-pointer items-center gap-2.5 p-2.5">
										<input
											type="checkbox"
											checked={item.balanceBy?.includes(opt) ?? false}
											onchange={() => toggleBalance(index, opt)}
											class="border-border bg-muted/50 h-4 w-4 rounded-full border shadow-sm"
										/>
										<span class="text-base">{opt}</span>
									</label>
								{/each}
							</div>
						</div>
					{/if}
				{/if}
			</div>

			<!-- Side controls: drag handle + delete -->
			<div class="flex flex-col items-center gap-2 pt-6">
				<button
					class="text-muted-foreground hover:text-foreground cursor-grab active:cursor-grabbing"
					draggable="true"
					ondragstart={(e) => handleDragStart(e, index)}
					ondragend={handleDragEnd}
				>
					<GripVertical class="h-5 w-5" />
				</button>
				<button
					class="text-muted-foreground hover:text-destructive"
					onclick={() => removeItem(index)}
				>
					<Trash2 class="h-5 w-5" />
				</button>
			</div>
		</div>
	{/each}

	<!-- Add buttons -->
	<div class="flex gap-4">
		<Button variant="default" onclick={addStandardItem}>+ Add agenda item</Button>
		<Button variant="default" onclick={addBreakoutSession}>+ Add breakout session</Button>
	</div>
</div>
