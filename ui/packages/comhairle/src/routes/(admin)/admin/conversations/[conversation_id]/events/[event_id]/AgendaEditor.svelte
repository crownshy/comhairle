<script lang="ts">
	import { tick } from 'svelte';
	import { flip } from 'svelte/animate';
	import { dragHandle, dragHandleZone, type DndEvent } from 'svelte-dnd-action';
	import ArrowDown from '@lucide/svelte/icons/arrow-down';
	import ArrowUp from '@lucide/svelte/icons/arrow-up';
	import ChevronDown from '@lucide/svelte/icons/chevron-down';
	import Ellipsis from '@lucide/svelte/icons/ellipsis';
	import GripVertical from '@lucide/svelte/icons/grip-vertical';
	import Plus from '@lucide/svelte/icons/plus';
	import Trash2 from '@lucide/svelte/icons/trash-2';
	import * as Collapsible from '$lib/components/ui/collapsible';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import Badge from '$lib/components/ui/badge/badge.svelte';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import ContentRenderer from '$lib/components/RichTextEditor/ContentRenderer/ContentRenderer.svelte';
	import SaveStatusPill from '$lib/components/SaveStatusPill.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import { cn, formatTime } from '$lib/utils';
	import type { SaveStatus } from '$lib/utils/autosave.svelte';
	import { Second } from '$lib/utils/units';
	import {
		agendaDraftTitle,
		agendaSchedule,
		formatAgendaMinutes,
		moveAgendaDraft,
		newBreakoutDraft,
		newPlenaryDraft,
		type AgendaDraftItem,
		type BreakoutDraft
	} from '$lib/utils/eventAgenda';

	type Props = {
		items: AgendaDraftItem[];
		/** The event's start, as an ISO string. Item times run on from it. */
		startTime: string;
		/** Called after every change, so the page can save. */
		onChange: () => void;
		saveStatus: SaveStatus;
		editable?: boolean;
	};

	let { items = $bindable(), startTime, onChange, saveStatus, editable = true }: Props = $props();

	const FLIP_DURATION_MS = 200;
	const UNDO_DURATION_MS = 8 * Second;
	const FIELD_LABEL_CLASS = 'flex flex-col items-start gap-1.5 text-sm font-medium';
	// Reads as plain text in the row until hovered or focused.
	const INLINE_INPUT_CLASS =
		'hover:border-input h-9 border-transparent bg-transparent px-2 text-base shadow-none';

	/** The breakout whose room settings are showing. One at a time keeps the list short. */
	let openItemId = $state('');

	let schedule = $derived(agendaSchedule(items, new Date(startTime)));

	async function add(item: AgendaDraftItem) {
		items = [...items, item];
		onChange();
		await tick();
		document.getElementById(titleInputId(item))?.focus();
	}

	function titleInputId(item: AgendaDraftItem) {
		return `agenda-title-${item.id}`;
	}

	function move(index: number, to: number) {
		items = moveAgendaDraft(items, index, to);
		onChange();
	}

	function remove(index: number) {
		const removed = items[index];
		items = items.filter((_, i) => i !== index);
		onChange();
		notifications.send({
			message: `Removed "${agendaDraftTitle(removed) || 'Untitled item'}"`,
			priority: 'INFO',
			duration: UNDO_DURATION_MS,
			action: { label: 'Undo', onClick: () => restore(removed, index) }
		});
	}

	function restore(item: AgendaDraftItem, index: number) {
		items = items.toSpliced(index, 0, item);
		onChange();
	}

	function handleDragConsider(event: CustomEvent<DndEvent<AgendaDraftItem>>) {
		items = event.detail.items;
	}

	function handleDragFinalize(event: CustomEvent<DndEvent<AgendaDraftItem>>) {
		items = event.detail.items;
		onChange();
	}
</script>

<section class="flex flex-col gap-6" aria-labelledby="agenda-heading">
	<div class="flex flex-wrap items-start justify-between gap-4">
		<div class="flex flex-col gap-2">
			<h2 id="agenda-heading" class="text-3xl font-bold">Agenda</h2>
			<p class="text-muted-foreground text-base">
				The order of the meeting. Facilitators step through it during the call.
			</p>
		</div>
		<div class="flex items-center gap-3">
			{#if items.length > 0}
				<p class="text-muted-foreground text-base tabular-nums">
					{formatAgendaMinutes(schedule.totalMinutes)} · ends {formatTime(
						schedule.end.toISOString()
					)}
				</p>
			{/if}
			<SaveStatusPill status={saveStatus} />
		</div>
	</div>

	{#if items.length === 0}
		<div
			class="bg-card flex flex-col items-center gap-2 rounded-xl border px-6 py-10 text-center"
		>
			<p class="text-lg font-semibold">No agenda yet</p>
			<p class="text-muted-foreground max-w-md text-base">
				{editable
					? 'Add the first item. Breakout sessions split people into small rooms for a set time.'
					: 'Nobody has planned this meeting yet.'}
			</p>
		</div>
	{:else}
		<ol
			class="bg-card divide-y rounded-xl border"
			use:dragHandleZone={{
				items,
				flipDurationMs: FLIP_DURATION_MS,
				dragDisabled: !editable,
				dropTargetStyle: {}
			}}
			onconsider={handleDragConsider}
			onfinalize={handleDragFinalize}
		>
			{#each items as item, index (item.id)}
				<li animate:flip={{ duration: FLIP_DURATION_MS }} class="outline-none">
					{@render agendaRow(item, index)}
				</li>
			{/each}
		</ol>
	{/if}

	{#if editable}
		<div class="flex flex-wrap gap-3">
			<Button variant="outline" onclick={() => add(newPlenaryDraft())}>
				<Plus class="size-4" />
				Agenda item
			</Button>
			<Button variant="outline" onclick={() => add(newBreakoutDraft())}>
				<Plus class="size-4" />
				Breakout session
			</Button>
		</div>
	{/if}
</section>

{#snippet agendaRow(item: AgendaDraftItem, index: number)}
	{@const title = agendaDraftTitle(item) || 'Untitled item'}
	{@const position = index + 1}
	<Collapsible.Root
		open={openItemId === item.id}
		onOpenChange={(open) => (openItemId = open ? item.id : '')}
	>
		<div class="flex items-center gap-2 px-3 py-1.5">
			{#if editable}
				<span
					use:dragHandle
					aria-label="Drag to reorder {title}"
					class="text-muted-foreground hover:text-foreground cursor-grab rounded-md p-1 active:cursor-grabbing"
				>
					<GripVertical class="size-4" />
				</span>
			{/if}
			<span class="text-muted-foreground w-5 shrink-0 text-right tabular-nums"
				>{position}</span
			>
			<span class="text-muted-foreground min-w-20 shrink-0 whitespace-nowrap tabular-nums">
				{formatTime(schedule.starts[index].toISOString())}
			</span>
			{#if item.type === 'breakout'}
				<Badge variant="secondary" class="shrink-0">Breakout</Badge>
			{/if}
			<div class="min-w-0 flex-1">
				{#if !editable}
					<span
						class={cn(
							'truncate font-medium',
							!agendaDraftTitle(item) && 'text-muted-foreground'
						)}
					>
						{title}
					</span>
				{:else if item.type === 'plenary'}
					<Input
						id={titleInputId(item)}
						bind:value={item.title}
						aria-label="Title of item {position}"
						placeholder="Untitled item"
						class={cn(INLINE_INPUT_CLASS, 'font-medium')}
						oninput={onChange}
					/>
				{:else}
					<Input
						id={titleInputId(item)}
						bind:value={item.prompt}
						aria-label="Question for item {position}"
						placeholder="What should each room discuss?"
						class={cn(INLINE_INPUT_CLASS, 'font-medium')}
						oninput={onChange}
					/>
				{/if}
			</div>
			{#if editable}
				<label class="text-muted-foreground flex shrink-0 items-center gap-1">
					<Input
						type="number"
						bind:value={item.durationMinutes}
						aria-label="Minutes for item {position}"
						min={item.type === 'breakout' ? 1 : 0}
						max={240}
						class={cn(INLINE_INPUT_CLASS, 'w-16 text-right tabular-nums')}
						oninput={onChange}
					/>
					min
				</label>
			{:else}
				<span class="text-muted-foreground shrink-0 tabular-nums">
					{formatAgendaMinutes(item.durationMinutes ?? 0)}
				</span>
			{/if}
			{#if item.type === 'breakout'}
				<Collapsible.Trigger
					class={cn(
						buttonVariants({ variant: 'ghost', size: 'icon' }),
						'shrink-0 [&[data-state=open]>svg]:rotate-180'
					)}
					aria-label="Room settings for item {position}"
				>
					<ChevronDown class="size-4 transition-transform duration-200" />
				</Collapsible.Trigger>
			{:else}
				<span class="size-9 shrink-0" aria-hidden="true"></span>
			{/if}
			{#if editable}
				{@render rowMenu(index, title)}
			{/if}
		</div>
		{#if item.type === 'breakout'}
			<Collapsible.Content class="flex flex-col gap-5 px-3 pb-5 pl-12 text-base">
				{@render roomSettings(item)}
			</Collapsible.Content>
		{/if}
	</Collapsible.Root>
{/snippet}

{#snippet rowMenu(index: number, title: string)}
	<DropdownMenu.Root>
		<DropdownMenu.Trigger
			class={cn(buttonVariants({ variant: 'ghost', size: 'icon' }), 'shrink-0')}
			aria-label="Options for {title}"
		>
			<Ellipsis class="size-4" />
		</DropdownMenu.Trigger>
		<DropdownMenu.Content align="end" class="w-44">
			<DropdownMenu.Item disabled={index === 0} onclick={() => move(index, index - 1)}>
				<ArrowUp />
				Move up
			</DropdownMenu.Item>
			<DropdownMenu.Item
				disabled={index === items.length - 1}
				onclick={() => move(index, index + 1)}
			>
				<ArrowDown />
				Move down
			</DropdownMenu.Item>
			<DropdownMenu.Separator />
			<DropdownMenu.Item variant="destructive" onclick={() => remove(index)}>
				<Trash2 />
				Delete
			</DropdownMenu.Item>
		</DropdownMenu.Content>
	</DropdownMenu.Root>
{/snippet}

{#snippet roomSettings(item: BreakoutDraft)}
	<Label class={FIELD_LABEL_CLASS}>
		People per room
		<Input
			type="number"
			disabled={!editable}
			bind:value={item.maxPerRoom}
			min={2}
			max={20}
			class="w-24"
			oninput={onChange}
		/>
	</Label>
	<div class="flex flex-col gap-1.5">
		<span class="text-sm font-medium">Instructions</span>
		<span class="text-muted-foreground text-sm">What should people do in their room?</span>
		{#if editable}
			<RichTextEditor
				value={item.instructions}
				toolset="text"
				placeholder="For example, take turns to share one priority"
				minHeight="100px"
				maxHeight="320px"
				onChange={(json) => {
					item.instructions = json;
					onChange();
				}}
			/>
		{:else if item.instructions}
			<ContentRenderer content={item.instructions} />
		{:else}
			<p class="text-muted-foreground text-base">No instructions</p>
		{/if}
	</div>
{/snippet}
