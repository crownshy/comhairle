<!--
  A practice area: drag tools into an ordered list of steps, reorder them, and see roughly
  how long the conversation takes. Nothing is saved.

  - The tool tray uses plain browser drag and drop, so the tray itself never moves while
    you drag a tool out of it. Every tool can also be added by clicking it.
  - The steps use svelte-dnd-action (as DraggableList does) for reordering, which also
    works with the keyboard.
  - `simple` gives a lighter, yellow version for the welcome page: up to four steps in a
    row, a time estimate and nothing else.
-->
<script lang="ts">
	import { flip } from 'svelte/animate';
	import { dndzone, type DndEvent } from 'svelte-dnd-action';
	import {
		ArrowDown,
		ArrowUp,
		Clock,
		GripVertical,
		Lightbulb,
		Plus,
		RotateCcw,
		Users,
		X
	} from 'lucide-svelte';
	import { GUIDE_NAV, TOOL_GUIDES } from '$lib/tool_guides';
	import { conversationTemplates } from '$lib/conversation_templates';
	import { minutesRange, TEMPLATE_STEP_TOOLS } from '$lib/tool_overview';
	import { cn } from '$lib/utils';
	import { toolIcon } from './toolIcons';
	import GuideRichText from './GuideRichText.svelte';

	type Props = { simple?: boolean; class?: string };
	let { simple = false, class: className = '' }: Props = $props();

	type Tool = { key: string; label: string };
	type Item = Tool & { id: string };

	const WORKSHOP: Tool = { key: 'workshop', label: 'In-person workshop' };
	let TOOLS: Tool[] = $derived([
		...GUIDE_NAV.map((t) => ({ key: t.key, label: t.navLabel })),
		...(simple ? [] : [WORKSHOP])
	]);
	const MAX_SIMPLE_STEPS = 4;
	// Example step names, like the ones the workspace gives new steps (see
	// conversation_templates.ts and workflow_templates.ts), so the cards read like real steps.
	const STEP_NAMES: Record<string, string> = {
		learn: 'Learn about the topic',
		polis: 'Tell us what you think',
		survey: 'Take a short survey',
		prioritization: 'Rate the proposals',
		thinking_space: 'What do you think?',
		online_group_conversation: 'Join the live discussion',
		lived_experience: 'Share your experience',
		workshop: 'Meet in person'
	};
	const FLIP_MS = 200;

	let counter = 0;
	const make = (tool: Tool): Item => ({ ...tool, id: `step-${counter++}` });

	let steps = $state<Item[]>([]);
	let dropHint = $state(false);
	let full = $derived(simple && steps.length >= MAX_SIMPLE_STEPS);
	let list: HTMLOListElement;

	function insert(tool: Tool, at = steps.length) {
		if (full) return;
		steps = [...steps.slice(0, at), make(tool), ...steps.slice(at)];
	}
	function remove(index: number) {
		steps = steps.filter((_, i) => i !== index);
	}
	function move(index: number, by: number) {
		const next = [...steps];
		[next[index], next[index + by]] = [next[index + by], next[index]];
		steps = next;
	}
	function stepsUpdate(e: CustomEvent<DndEvent<Item>>) {
		steps = e.detail.items;
	}

	// Dragging a tool from the tray. Done with pointer events rather than the browser's own
	// drag and drop, which Safari and Firefox won't start from a <button>, and which doesn't
	// work on touch screens. A small "ghost" chip follows the pointer while dragging.
	let ghost = $state<{ tool: Tool; x: number; y: number } | null>(null);
	let pressed: { tool: Tool; x: number; y: number } | null = null;
	let suppressClick = false;

	function overList(e: PointerEvent) {
		const r = list.getBoundingClientRect();
		return (
			e.clientX >= r.left &&
			e.clientX <= r.right &&
			e.clientY >= r.top &&
			e.clientY <= r.bottom
		);
	}
	function indexAt(y: number) {
		const cards = [...list.querySelectorAll<HTMLElement>('[data-step]')];
		const at = cards.findIndex((el) => {
			const r = el.getBoundingClientRect();
			return y < r.top + r.height / 2;
		});
		return at === -1 ? steps.length : at;
	}
	function trayPointerDown(e: PointerEvent, tool: Tool) {
		if (e.button !== 0 || full) return;
		pressed = { tool, x: e.clientX, y: e.clientY };
		window.addEventListener('pointermove', trayPointerMove);
		window.addEventListener('pointerup', trayPointerUp, { once: true });
		window.addEventListener('pointercancel', trayPointerUp, { once: true });
	}
	function trayPointerMove(e: PointerEvent) {
		if (!pressed) return;
		// Only start dragging after a small movement, so a plain click still adds the tool.
		if (!ghost && Math.hypot(e.clientX - pressed.x, e.clientY - pressed.y) < 6) return;
		ghost = { tool: pressed.tool, x: e.clientX, y: e.clientY };
		dropHint = overList(e);
	}
	function trayPointerUp(e: PointerEvent) {
		window.removeEventListener('pointermove', trayPointerMove);
		window.removeEventListener('pointerup', trayPointerUp);
		window.removeEventListener('pointercancel', trayPointerUp);
		if (ghost) {
			if (e.type === 'pointerup' && overList(e)) insert(ghost.tool, indexAt(e.clientY));
			// The click that follows a drag shouldn't add the tool a second time.
			suppressClick = true;
			setTimeout(() => (suppressClick = false));
		}
		ghost = null;
		pressed = null;
		dropHint = false;
	}
	function trayClick(tool: Tool) {
		if (!suppressClick) insert(tool);
	}

	/** Used by the template cards on the Engagement tools page. */
	export function loadTemplate(key: string) {
		const template = conversationTemplates.find((t) => t.key === key);
		if (!template) return;
		steps = template.displaySteps.map((step) => {
			const tool = TEMPLATE_STEP_TOOLS[step.label];
			return tool && TOOL_GUIDES[tool]
				? make({ key: tool, label: TOOL_GUIDES[tool].navLabel })
				: make(WORKSHOP);
		});
	}

	// Rough time for participants, from each tool's "participant time".
	let time = $derived.by(() => {
		let low = 0;
		let high = 0;
		let unknown = false;
		for (const step of steps) {
			const range = minutesRange(TOOL_GUIDES[step.key]?.atAGlance?.participantTime);
			if (range) {
				low += range[0];
				high += range[1];
			} else unknown = true;
		}
		return { low, high, unknown };
	});

	// Friendly nudges, not rules. The simple version keeps just the two about order.
	const LEARN_FIRST =
		'Most conversations open with a Learn step, so everyone starts with the same background.';
	const THINK_BEFORE_POLL =
		'Nice: a Thinking space before the poll helps people form their views first.';
	// Each "Choose from templates" template as a sequence of tool keys, e.g.
	// Informed-participants survey = learn, survey.
	const TEMPLATE_SEQUENCES = conversationTemplates
		.filter((t) => t.available)
		.map((t) => ({
			key: t.key,
			name: t.name,
			keys: t.displaySteps
				.map((step) => TEMPLATE_STEP_TOOLS[step.label] ?? WORKSHOP.key)
				.join(',')
		}));
	let matchingTemplate = $derived(
		TEMPLATE_SEQUENCES.find((t) => t.keys === steps.map((s) => s.key).join(','))
	);

	let tips = $derived.by(() => {
		const out: string[] = [];
		if (matchingTemplate)
			out.push(
				`Nice, that looks like one of our templates: [“${matchingTemplate.name}”](/admin/info/tools#template-${matchingTemplate.key}).`
			);
		if (steps.length && steps[0].key !== 'learn') out.push(LEARN_FIRST);
		if (steps.some((s, i) => i > 0 && steps[i - 1].key === s.key))
			out.push('Two of the same tool in a row: could they be one step?');
		if (steps.length > 5)
			out.push('That’s a long journey. Fewer steps usually means more people finish.');
		const polis = steps.findIndex((s) => s.key === 'polis');
		if (polis > 0 && steps.slice(0, polis).some((s) => s.key === 'thinking_space'))
			out.push(THINK_BEFORE_POLL);
		return simple
			? out.filter(
					(tip) =>
						tip === LEARN_FIRST ||
						tip === THINK_BEFORE_POLL ||
						tip.startsWith('Nice, that')
				)
			: out;
	});

	// Colours: primary for the full version, a playful yellow for the simple one.
	let tone = $derived(
		simple
			? {
					frame: 'border-amber-300 bg-amber-50 dark:border-amber-400/40 dark:bg-amber-400/10',
					well: 'bg-amber-100/70 dark:bg-amber-400/10',
					toolIcon: 'bg-amber-400 text-amber-950',
					drop: 'ring-amber-400'
				}
			: {
					frame: 'border-primary/30',
					well: 'bg-nav-background',
					toolIcon: 'bg-primary text-primary-foreground',
					drop: 'ring-primary'
				}
	);
</script>

{#snippet icon(key: string)}
	{@const Icon = key === 'workshop' ? Users : toolIcon(key)}
	<span
		class="flex size-7 shrink-0 items-center justify-center rounded-full {key === 'workshop'
			? 'bg-accent text-accent-foreground'
			: tone.toolIcon}"
	>
		<Icon class="size-4" aria-hidden="true" />
	</span>
{/snippet}

<div class={cn('d rounded-2xl  p-5 md:p-6', tone.frame, className)}>
	<!-- Simple version: tools on the left, the conversation built out to the right -->
	<div class={simple ? 'grid gap-6 md:grid-cols-[13rem_minmax(0,1fr)]' : ''}>
		<div>
			<!-- Tool tray: stays put while you drag from it -->
			<p class="text-muted-foreground text-sm font-medium">
				{simple
					? 'Drag a tool across, or click it'
					: 'Drag a tool into the steps below, or click it to add it'}
			</p>
			<ul
				class="mt-3 flex gap-2 {simple ? 'flex-col items-start' : 'flex-wrap'}"
				aria-label="Tools"
			>
				{#each TOOLS as tool (tool.key)}
					<li>
						<button
							type="button"
							onpointerdown={(e) => trayPointerDown(e, tool)}
							onclick={() => trayClick(tool)}
							disabled={full}
							aria-label={`Add ${tool.label} as the next step`}
							class="bg-card border-border hover:border-primary/50 flex cursor-grab touch-none items-center gap-2 rounded-full border py-1.5 pr-3 pl-1.5 text-sm font-medium shadow-xs transition-colors select-none active:cursor-grabbing disabled:cursor-not-allowed disabled:opacity-50"
						>
							{@render icon(tool.key)}
							{tool.label}
							<Plus class="text-muted-foreground size-3.5" aria-hidden="true" />
						</button>
					</li>
				{/each}
			</ul>
		</div>

		<div class="min-w-0">
			<!-- The conversation being built -->
			<div class="{simple ? '' : 'mt-6'} flex items-center justify-between gap-2">
				<h3 class="text-foreground font-semibold">Your conversation</h3>
				{#if steps.length}
					<button
						type="button"
						onclick={() => (steps = [])}
						class="text-muted-foreground hover:text-foreground inline-flex items-center gap-1 text-sm"
					>
						<RotateCcw class="size-3.5" aria-hidden="true" /> Start again
					</button>
				{/if}
			</div>
			<div class="relative mt-2">
				<ol
					bind:this={list}
					class="flex min-h-24 flex-col gap-2.5 rounded-xl p-3 transition-shadow {tone.well} {dropHint
						? `ring-2 ${tone.drop}`
						: ''}"
					aria-label="Your conversation steps"
					use:dndzone={{
						items: steps,
						flipDurationMs: FLIP_MS,
						dropTargetStyle: {}
					}}
					onconsider={stepsUpdate}
					onfinalize={stepsUpdate}
				>
					{#each steps as step, i (step.id)}
						<!-- Looks like a step card in Process design: grip, numbered badge,
						     step name with its tool underneath, arrows on hover. -->
						<li
							data-step
							class="group bg-card border-border hover:border-primary/50 flex cursor-grab items-center gap-3 rounded-xl border p-3 transition-all hover:shadow-md active:cursor-grabbing"
							animate:flip={{ duration: FLIP_MS }}
						>
							<GripVertical
								class="text-muted-foreground group-hover:text-foreground size-5 shrink-0 transition-colors"
								aria-hidden="true"
							/>
							<span
								class="bg-primary text-primary-foreground flex size-6 shrink-0 items-center justify-center rounded-lg text-sm font-bold"
								>{i + 1}</span
							>
							<span class="flex min-w-0 flex-1 flex-col">
								<span
									class="text-foreground group-hover:text-primary truncate font-medium transition-colors"
									>{STEP_NAMES[step.key] ?? step.label}</span
								>
								<span class="text-primary text-sm font-medium">{step.label}</span>
							</span>
							<span
								class="flex shrink-0 items-center gap-0.5 transition-opacity sm:opacity-0 sm:group-hover:opacity-100 sm:focus-within:opacity-100"
							>
								<button
									type="button"
									disabled={i === 0}
									onclick={() => move(i, -1)}
									aria-label={`Move ${step.label} up`}
									class="text-muted-foreground hover:text-foreground hover:bg-accent flex size-8 items-center justify-center rounded-md disabled:pointer-events-none disabled:opacity-30"
									><ArrowUp class="size-4" aria-hidden="true" /></button
								>
								<button
									type="button"
									disabled={i === steps.length - 1}
									onclick={() => move(i, 1)}
									aria-label={`Move ${step.label} down`}
									class="text-muted-foreground hover:text-foreground hover:bg-accent flex size-8 items-center justify-center rounded-md disabled:pointer-events-none disabled:opacity-30"
									><ArrowDown class="size-4" aria-hidden="true" /></button
								>
								<button
									type="button"
									onclick={() => remove(i)}
									aria-label={`Remove ${step.label}`}
									class="text-muted-foreground hover:text-destructive hover:bg-accent flex size-8 items-center justify-center rounded-md"
									><X class="size-4" aria-hidden="true" /></button
								>
							</span>
						</li>
					{/each}
				</ol>
				{#if !steps.length}
					<p
						class="text-muted-foreground pointer-events-none absolute inset-0 flex items-center justify-center gap-2 text-sm"
					>
						Drop tools here to add steps
					</p>
				{/if}
			</div>

			<!-- Feedback: nudges first, then a light-touch time estimate -->
			{#if steps.length}
				<div class="mt-4 flex flex-col gap-2 text-sm" aria-live="polite">
					{#each tips as tip (tip)}
						<p class="text-foreground flex items-start gap-2">
							<Lightbulb
								class="mt-0.5 size-4 shrink-0 {simple
									? 'text-amber-600'
									: 'text-primary'}"
								aria-hidden="true"
							/>
							<span><GuideRichText text={tip} /></span>
						</p>
					{/each}
					<p class="text-muted-foreground flex items-center gap-1.5 text-xs">
						<Clock class="size-3.5 shrink-0" aria-hidden="true" />
						About {time.low}–{time.high} min per participant{time.unknown
							? ', plus any in-person or live time'
							: ''}{#if full}. That’s plenty to start with!{/if}
					</p>
				</div>
			{/if}
		</div>
	</div>
	<p class="text-muted-foreground mt-4 text-xs">
		Just for practice, nothing here is saved.
		{#if simple}
			Learn more about each tool in
			<a
				href="/admin/info/tools"
				class="text-primary underline underline-offset-2 hover:no-underline"
				>Engagement tools</a
			>.
		{:else}
			To build a real one, see
			<a
				href="/admin/info/how-to/design-the-engagement-process/design-steps"
				class="text-primary underline underline-offset-2 hover:no-underline"
				>Design your steps</a
			>.
		{/if}
	</p>
</div>

{#if ghost}
	<!-- Follows the pointer while a tool is dragged from the tray -->
	<div
		class="bg-card border-primary pointer-events-none fixed z-50 flex -translate-x-1/2 -translate-y-1/2 items-center gap-2 rounded-full border-2 py-1.5 pr-3 pl-1.5 text-sm font-medium shadow-lg"
		style={`left:${ghost.x}px;top:${ghost.y}px`}
		aria-hidden="true"
	>
		{@render icon(ghost.tool.key)}
		{ghost.tool.label}
	</div>
{/if}
