<script lang="ts">
	/**
	 * The first run through a conversation, once: the chrome's fixed places, circled one at a
	 * time (ADR-0026).
	 *
	 * Each place is found in the DOM by its `data-tour` name and measured, so the ring sits on
	 * the real control rather than on a guessed corner, and the control itself stays lit while
	 * everything around it goes dark. The scrim is the ring's own box shadow, which is how the
	 * hole in it stays exactly the size of the thing being pointed at.
	 */
	import { Button } from '$lib/components/ui/button';
	import * as m from '$lib/paraglide/messages';

	let { onDismiss }: { onDismiss: () => void } = $props();

	type Spot = { id: string; text: string };

	const ALL_SPOTS: Spot[] = [
		{ id: 'intro', text: m.step_tour_before_you_start() },
		{ id: 'brief', text: m.step_tour_brief() },
		{ id: 'back', text: m.step_tour_back() },
		{ id: 'forward', text: m.step_tour_forward() }
	];

	/** Breathing room between the ring and the control it circles. */
	const PAD = 10;

	function target(spot: Spot | undefined): HTMLElement | null {
		if (!spot) return null;
		return document.querySelector<HTMLElement>(`[data-tour="${spot.id}"]`);
	}

	let index = $state(0);
	let rect = $state<DOMRect | null>(null);
	let card = $state<HTMLElement | null>(null);

	/**
	 * Only the places that are on this screen: a step with no brief has no chip, and a tour
	 * that circles an empty corner is worse than one that says less. Read once, at the moment
	 * the tour opens, by which time the chrome it points at is mounted.
	 */
	let spots = $derived(
		typeof document === 'undefined' ? [] : ALL_SPOTS.filter((spot) => target(spot))
	);

	let current = $derived(spots[index]);
	let isLast = $derived(index >= spots.length - 1);

	function measure() {
		rect = target(spots[index])?.getBoundingClientRect() ?? null;
	}

	$effect(() => {
		void index;
		void spots;
		measure();
		// Focus the card rather than its button: the caption is the dialog's name, so moving
		// focus here is what reads the new place out.
		card?.focus();
	});

	function next() {
		if (isLast) onDismiss();
		else index += 1;
	}

	function onkeydown(event: KeyboardEvent) {
		if (event.key === 'Escape') onDismiss();
	}

	// A small trap, because the page behind is still in the tab order.
	function onCardKeydown(event: KeyboardEvent) {
		if (event.key !== 'Tab') return;
		const buttons = card?.querySelectorAll<HTMLElement>('button');
		if (!buttons?.length) return;
		const first = buttons[0];
		const last = buttons[buttons.length - 1];
		const active = document.activeElement;
		if (event.shiftKey && (active === first || active === card)) {
			event.preventDefault();
			last.focus();
		} else if (!event.shiftKey && active === last) {
			event.preventDefault();
			first.focus();
		}
	}

	let cardStyle = $derived.by(() => {
		if (!rect || typeof window === 'undefined') {
			return 'left:50%; top:50%; transform:translate(-50%,-50%);';
		}
		const gap = PAD + 12;
		const vertical =
			rect.top < window.innerHeight / 2
				? `top:${rect.bottom + gap}px;`
				: `bottom:${window.innerHeight - rect.top + gap}px;`;
		// Anchored to the near edge rather than centred on the control, so a caption for a
		// corner control never hangs off the side of the screen.
		const horizontal =
			rect.left + rect.width / 2 < window.innerWidth / 2
				? `left:${Math.max(16, rect.left - PAD)}px;`
				: `right:${Math.max(16, window.innerWidth - rect.right - PAD)}px;`;
		return vertical + horizontal;
	});

	const captionId = 'step-tour-caption';
</script>

<svelte:window {onkeydown} onresize={measure} />

{#if current}
	<!-- Catches everything aimed at the page underneath. The ring above it is
		pointer-events-none, so the circled control looks reachable but is not: the tour is
		four taps long and pressing Next mid-tour would navigate out from under it. -->
	<div class="fixed inset-0 z-40"></div>

	{#if rect}
		<div
			class="pointer-events-none fixed z-50 rounded-full ring-2 ring-white/90"
			style="left:{rect.left - PAD}px; top:{rect.top - PAD}px; width:{rect.width +
				PAD * 2}px; height:{rect.height +
				PAD * 2}px; box-shadow: 0 0 0 9999px rgb(0 0 0 / 0.6);"
			aria-hidden="true"
		></div>
	{:else}
		<div class="pointer-events-none fixed inset-0 z-50 bg-black/60" aria-hidden="true"></div>
	{/if}

	<div
		bind:this={card}
		tabindex="-1"
		role="dialog"
		aria-modal="true"
		aria-labelledby={captionId}
		class="bg-card text-card-foreground fixed z-50 flex w-[min(20rem,calc(100vw-2rem))] flex-col gap-4 rounded-2xl p-4 shadow-xl outline-none"
		style={cardStyle}
		onkeydown={onCardKeydown}
	>
		<p id={captionId} class="text-base leading-6">{current.text}</p>

		<div class="flex items-center gap-3">
			<span class="text-muted-foreground text-sm">
				{m.step_tour_position({ current: index + 1, total: spots.length })}
			</span>
			<button
				type="button"
				class="text-muted-foreground ml-auto text-sm underline underline-offset-4"
				onclick={onDismiss}
			>
				{m.step_tour_skip()}
			</button>
			<Button class="h-10 text-base" onclick={next}>
				{isLast ? m.step_tour_dismiss() : m.pager_next()}
			</Button>
		</div>
	</div>
{/if}
