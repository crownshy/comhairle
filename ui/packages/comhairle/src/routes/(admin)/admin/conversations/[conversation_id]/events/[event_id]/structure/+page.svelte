<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import type { AgendaItemData } from '../agenda-types';
	import type { EventAgendaItem } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { notifications } from '$lib/notifications.svelte';
	import { invalidate } from '$app/navigation';
	import { key } from '$lib/utils/invalidationKey';
	import { Trash2, GripVertical } from '@lucide/svelte';
	import { tryCatchAsync } from '$lib/utils/errorHandling';

	const { data } = $props();
	const { conversation, event } = $derived(data);

	// $derived.by for deep-reactivity as $derived doesn't have deep reactivity
	let items = $derived.by<AgendaItemData[]>(() => apiAgendaToEditor(data.event.agenda ?? []));
	let dirty = $state(false);
	let saving = $state(false);

	const balanceOptions = [
		'Age',
		'Gender',
		'Understanding of AI',
		'Education',
		'Postcode',
		'In-call activities'
	];

	function apiAgendaToEditor(items: EventAgendaItem[]): AgendaItemData[] {
		return items.map((item) => {
			if ('Basic' in item) {
				return {
					id: crypto.randomUUID(),
					type: 'standard',
					title: item.Basic.title
				};
			} else {
				return {
					id: crypto.randomUUID(),
					type: 'breakout',
					title: '',
					duration: item.BreakoutRoom.estimated_time,
					groupSize: item.BreakoutRoom.max_per_room ?? 4,
					prompts: [
						{
							title: item.BreakoutRoom.prompt,
							instructions: item.BreakoutRoom.instructions
						}
					],
					assignmentMode: 'random',
					balanceBy: []
				};
			}
		});
	}

	function editorAgendaToApi(items: AgendaItemData[]): EventAgendaItem[] {
		return items.map((item) => {
			if (item.type === 'standard') {
				return {
					Basic: {
						title: item.title || '',
						description: '',
						estimated_time: 0
					}
				};
			} else {
				const firstPrompt = item.prompts?.[0];
				return {
					BreakoutRoom: {
						prompt: firstPrompt?.title || '',
						instructions: firstPrompt?.instructions || '',
						estimated_time: item.duration ?? 10,
						time_limit: item.duration ? item.duration * 60 : null,
						max_per_room: item.groupSize ?? null
					}
				};
			}
		});
	}

	async function handleSaveAgenda() {
		saving = true;

		const updateEvent = await tryCatchAsync(() =>
			apiClient.UpdateEvent(
				{ agenda: editorAgendaToApi(items) },
				{
					params: {
						conversation_id: conversation.id,
						event_id: event.id
					}
				}
			)
		);

		saving = false;

		if (updateEvent.err !== null) {
			console.error(updateEvent.err);
			notifications.send({ message: 'Failed to save agenda', priority: 'ERROR' });
			return;
		}

		await invalidate(key('conversation/event'));
		dirty = false;
		notifications.send({ message: 'Agenda saved', priority: 'INFO' });
	}

	function createId(): string {
		return crypto.randomUUID();
	}

	function addStandardItem() {
		items.push({ id: createId(), type: 'standard', title: '' });
		dirty = true;
	}

	function addBreakoutSession() {
		items.push({
			id: createId(),
			type: 'breakout',
			title: '',
			duration: 10,
			groupSize: 4,
			prompts: [{ title: '', instructions: '' }],
			assignmentMode: 'random',
			balanceBy: []
		});
		dirty = true;
	}

	function removeItem(index: number) {
		items.splice(index, 1);
		dirty = true;
	}

	function addPrompt(itemIndex: number) {
		items[itemIndex]?.prompts?.push({ title: '', instructions: '' });
		dirty = true;
	}

	function removePrompt(itemIndex: number, promptIndex: number) {
		items[itemIndex]?.prompts?.splice(promptIndex, 1);
		dirty = true;
	}

	function toggleBalance(itemIndex: number, value: string) {
		const item = items[itemIndex];
		if (!item.balanceBy) item.balanceBy = [];
		if (item.balanceBy.includes(value)) {
			item.balanceBy = item.balanceBy.filter((v) => v !== value);
		} else {
			item.balanceBy.push(value);
		}
		dirty = true;
	}

	// Drag state
	let dragIndex: number | null = $state(null);
	let dropIndex: number | null = $state(null);

	function handleDragStart(e: DragEvent, index: number) {
		dragIndex = index;
		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'move';
			e.dataTransfer.setData('text/plain', String(index));
		}
	}

	function handleDragLeave(e: DragEvent, idx: number) {
		const related = e.relatedTarget as HTMLElement | null;
		const currentTarget = e.currentTarget as HTMLElement;
		if (!related || !currentTarget.contains(related)) {
			if (dropIndex === idx) dropIndex = null;
		}
	}

	function handleDrop(e: DragEvent, targetIndex: number) {
		e.preventDefault();
		if (dragIndex === null || dragIndex === targetIndex) {
			dragIndex = null;
			dropIndex = null;
			return;
		}
		const newItems = [...items];
		const [moved] = newItems.splice(dragIndex, 1);
		newItems.splice(targetIndex, 0, moved);
		items = newItems;
		dirty = true;
		dragIndex = null;
		dropIndex = null;
	}

	function handleDragEnd() {
		dragIndex = null;
		dropIndex = null;
	}
</script>

<div class="flex flex-col gap-10 py-6">
	<div class="flex flex-col gap-2">
		<h2 class="text-3xl font-bold">
			Event structure <span class="font-bold">(for facilitator)</span>
		</h2>
		<p class="text-muted-foreground text-base">Plan how your meeting will run</p>
	</div>

	<div class="flex flex-col gap-7">
		<div class="flex flex-col gap-2">
			<h3 class="text-foreground text-2xl font-bold">Agenda</h3>
			<p class="text-muted-foreground text-base">Optimise topics and activities</p>
		</div>

		<!-- Agenda items -->
		{#each items as item, index (item.id)}
			<div
				class="flex items-start gap-4 transition-all {dropIndex === index
					? 'border-ring border-t-2'
					: ''}"
				role="group"
				ondragover={(e) => {
					e.preventDefault();
					dropIndex = index;
				}}
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
								oninput={() => void (dirty = true)}
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
								oninput={() => void (dirty = true)}
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
								oninput={() => void (dirty = true)}
							/>
							<span class="text-sm">people</span>
						</div>

						<!-- Breakout prompts -->
						<div class="text-foreground text-lg font-bold">Breakout prompt</div>

						{#each item.prompts ?? [] as prompt, i (prompt.title)}
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
										oninput={() => void (dirty = true)}
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
											dirty = true;
										}}
									/>
								</div>

								{#if (item.prompts?.length ?? 0) > 1}
									<button
										class="text-destructive hover:text-destructive/80 flex items-center gap-1 text-sm"
										onclick={() => removePrompt(index, i)}
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
								onchange={() => {
									items = items.slice();
									dirty = true;
								}}
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
									{#each balanceOptions as opt (opt)}
										<label
											class="flex cursor-pointer items-center gap-2.5 p-2.5"
										>
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

	<div class="border-border flex justify-center border-t py-6">
		<Button
			variant="default"
			class="px-12"
			disabled={!dirty || saving}
			onclick={handleSaveAgenda}
		>
			{saving ? 'Saving...' : 'Save Agenda'}
		</Button>
	</div>
</div>
