<script lang="ts">
	import { tick, untrack } from 'svelte';
	import { ArrowDown, ArrowUp, Check, Lock, Plus, Trash2, TriangleAlert } from 'lucide-svelte';
	import type { EventAgendaItem } from '@crownshy/api-client/api';
	import * as Alert from '$lib/components/ui/alert';
	import { Button, LoadingButton } from '$lib/components/ui/button';
	import Input from '$lib/components/ui/input/input.svelte';
	import Label from '$lib/components/ui/label/label.svelte';
	import type { AgendaEditOutcome } from '$lib/services/videoCallService.svelte';
	import { cn } from '$lib/utils';
	import { mapApiAgenda } from '$lib/utils/liveEventAgenda';
	import {
		fromAgendaDraft,
		agendaDraftProblem,
		moveAgendaDraft,
		newBreakoutDraft,
		newPlenaryDraft,
		toAgendaDraft,
		upcomingAgendaItems,
		type AgendaDraftItem,
		type AgendaDraftProblem
	} from '$lib/utils/eventAgenda';

	type Props = {
		agenda: EventAgendaItem[];
		currentStep: number;
		/** Sends the upcoming items, saying which step the host was editing from. */
		onSave: (editedFromStep: number, upcoming: EventAgendaItem[]) => Promise<AgendaEditOutcome>;
		onClose: () => void;
	};

	let { agenda, currentStep, onSave, onClose }: Props = $props();

	const ICON_BUTTON_CLASS = 'text-muted-foreground size-7';

	// Seeded once, not derived: an agenda update from another host must not wipe what this
	// host is typing. "Start over" reseeds on purpose.
	let editedFromStep = $state(untrack(() => currentStep));
	let drafts = $state<AgendaDraftItem[]>(untrack(() => draftsFrom(agenda, currentStep)));
	let saving = $state(false);
	let saveError = $state<string | null>(null);
	let serverSaidStepMoved = $state(false);
	// Problems only show after a save attempt, so a half-typed new item isn't red straight away.
	let showProblems = $state(false);
	let editorBody: HTMLElement | undefined = $state();

	let lockedItems = $derived(mapApiAgenda(agenda).slice(0, editedFromStep + 1));
	let stepMoved = $derived(currentStep !== editedFromStep || serverSaidStepMoved);
	let problemCount = $derived(drafts.filter((draft) => agendaDraftProblem(draft)).length);

	function draftsFrom(items: EventAgendaItem[], step: number): AgendaDraftItem[] {
		return upcomingAgendaItems(items, step).map(toAgendaDraft);
	}

	function startOver() {
		editedFromStep = currentStep;
		drafts = draftsFrom(agenda, currentStep);
		serverSaidStepMoved = false;
		saveError = null;
		showProblems = false;
	}

	function problemFor(draft: AgendaDraftItem): AgendaDraftProblem | null {
		return showProblems ? agendaDraftProblem(draft) : null;
	}

	async function pointAtFirstProblem() {
		showProblems = true;
		await tick();
		editorBody?.querySelector<HTMLElement>('[aria-invalid="true"]')?.focus();
	}

	function move(index: number, to: number) {
		drafts = moveAgendaDraft(drafts, index, to);
	}

	function remove(index: number) {
		drafts.splice(index, 1);
	}

	async function save() {
		if (saving || stepMoved) return;
		if (problemCount > 0) {
			await pointAtFirstProblem();
			return;
		}
		saving = true;
		saveError = null;
		const outcome = await onSave(editedFromStep, drafts.map(fromAgendaDraft));
		saving = false;

		if (outcome === 'saved') onClose();
		else if (outcome === 'step_moved') serverSaidStepMoved = true;
		else if (outcome === 'not_host')
			saveError = 'Only hosts on the call can change the agenda.';
		else saveError = "The agenda didn't save. Your changes are still here, try again.";
	}
</script>

<div class="flex h-full flex-col overflow-hidden">
	<div class="flex shrink-0 items-center justify-center px-5">
		<h2 class="text-muted-foreground text-center text-xl leading-7 font-semibold">
			Edit agenda
		</h2>
	</div>

	<div bind:this={editorBody} class="flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto p-4">
		<section class="flex flex-col gap-1.5" aria-labelledby="locked-agenda-heading">
			<h3 id="locked-agenda-heading" class="text-muted-foreground text-sm font-medium">
				Started or finished, can't be changed
			</h3>
			{#each lockedItems as item, index (item.id)}
				<div
					class="bg-muted-foreground/10 text-muted-foreground flex items-center gap-2 rounded-xl px-3 py-2"
				>
					{#if index < editedFromStep}
						<Check class="h-4 w-4 shrink-0" aria-label="Finished" />
					{:else}
						<Lock class="h-4 w-4 shrink-0" aria-label="Current" />
					{/if}
					<span class="line-clamp-1 text-base">{item.title}</span>
				</div>
			{/each}
		</section>

		{#if stepMoved}
			<Alert.Root variant="destructive">
				<TriangleAlert />
				<Alert.Title>The agenda moved on</Alert.Title>
				<Alert.Description>
					<p>
						Another item started while you were editing, so these changes can't be
						saved.
					</p>
					<Button variant="outline" size="sm" class="mt-2" onclick={startOver}
						>Start over</Button
					>
				</Alert.Description>
			</Alert.Root>
		{/if}

		<section class="flex flex-col gap-2" aria-labelledby="upcoming-agenda-heading">
			<h3 id="upcoming-agenda-heading" class="text-muted-foreground text-sm font-medium">
				Coming up
			</h3>

			{#each drafts as draft, index (draft.id)}
				{@const problem = problemFor(draft)}
				<div
					class={cn(
						'bg-background flex flex-col gap-3 rounded-xl border p-3',
						problem && 'border-destructive'
					)}
				>
					<div class="flex items-center gap-1">
						<span class="text-primary flex-1 text-sm font-medium">
							{draft.type === 'breakout' ? 'Breakout session' : 'Agenda item'}
						</span>
						<Button
							variant="ghost"
							size="icon"
							class={ICON_BUTTON_CLASS}
							aria-label="Move up"
							disabled={index === 0}
							onclick={() => move(index, index - 1)}
						>
							<ArrowUp class="h-4 w-4" />
						</Button>
						<Button
							variant="ghost"
							size="icon"
							class={ICON_BUTTON_CLASS}
							aria-label="Move down"
							disabled={index === drafts.length - 1}
							onclick={() => move(index, index + 1)}
						>
							<ArrowDown class="h-4 w-4" />
						</Button>
						<Button
							variant="ghost"
							size="icon"
							class={cn(ICON_BUTTON_CLASS, 'hover:text-destructive')}
							aria-label="Remove"
							onclick={() => remove(index)}
						>
							<Trash2 class="h-4 w-4" />
						</Button>
					</div>

					{#if draft.type === 'plenary'}
						<Label class="flex flex-col items-start gap-1.5 text-sm">
							Title
							<Input
								bind:value={draft.title}
								placeholder="What's happening"
								aria-invalid={problem?.field === 'title'}
							/>
						</Label>
						<Label class="flex w-1/2 flex-col items-start gap-1.5 pr-1.5 text-sm">
							Minutes
							<Input
								type="number"
								min={0}
								max={240}
								bind:value={draft.durationMinutes}
								aria-invalid={problem?.field === 'minutes'}
							/>
						</Label>
					{:else}
						<Label class="flex flex-col items-start gap-1.5 text-sm">
							Question for each room
							<Input
								bind:value={draft.prompt}
								placeholder="What should groups discuss?"
							/>
						</Label>
						<div class="flex gap-3">
							<Label class="flex flex-1 flex-col items-start gap-1.5 text-sm">
								Minutes
								<Input
									type="number"
									min={1}
									max={120}
									bind:value={draft.durationMinutes}
									aria-invalid={problem?.field === 'minutes'}
								/>
							</Label>
							<Label class="flex flex-1 flex-col items-start gap-1.5 text-sm">
								People per room
								<Input
									type="number"
									min={2}
									max={20}
									bind:value={draft.maxPerRoom}
									aria-invalid={problem?.field === 'people'}
								/>
							</Label>
						</div>
					{/if}
					{#if problem}
						<p class="text-destructive text-sm">{problem.message}</p>
					{/if}
				</div>
			{:else}
				<p class="text-muted-foreground text-base">Nothing after the current item yet.</p>
			{/each}

			<div class="flex gap-2">
				<Button
					variant="outline"
					class="flex-1"
					onclick={() => (drafts = [...drafts, newPlenaryDraft()])}
				>
					<Plus class="h-4 w-4" />
					Item
				</Button>
				<Button
					variant="outline"
					class="flex-1"
					onclick={() => (drafts = [...drafts, newBreakoutDraft()])}
				>
					<Plus class="h-4 w-4" />
					Breakout
				</Button>
			</div>
		</section>
	</div>

	<div class="flex shrink-0 flex-col gap-2 border-t p-4">
		{#if showProblems && problemCount > 0}
			<p role="alert" class="text-destructive text-sm">
				{problemCount === 1
					? 'One item needs fixing before you can save.'
					: `${problemCount} items need fixing before you can save.`}
			</p>
		{/if}
		{#if saveError}
			<p role="alert" class="text-destructive text-sm">{saveError}</p>
		{/if}
		<div class="flex gap-2">
			<Button variant="outline" class="h-10 flex-1" disabled={saving} onclick={onClose}>
				Cancel
			</Button>
			<LoadingButton
				variant="primaryDark"
				class="h-10 flex-1"
				loading={saving}
				disabled={stepMoved}
				onclick={save}
			>
				Save
			</LoadingButton>
		</div>
	</div>
</div>
