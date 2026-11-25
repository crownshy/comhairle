const GLOSSARY_TERM_SELECTOR = '.glossary-term';
const EDGE_MARGIN_PX = 8;
// Long enough to cross the gap between a term and its tooltip without the tooltip closing.
const HOVER_CLOSE_DELAY_MS = 150;

function closestTerm(target: EventTarget | null): HTMLElement | null {
	return target instanceof Element ? target.closest<HTMLElement>(GLOSSARY_TERM_SELECTOR) : null;
}

function isInside(element: Element | null, target: EventTarget | null): boolean {
	return !!element && target instanceof Node && element.contains(target);
}

/**
 * Drives the single shared glossary tooltip for every term inside `container`. The terms come
 * from `{@html}`, so a component can't mount per term; listeners are delegated instead.
 *
 * Follows WCAG 1.4.13: the tooltip opens on hover, focus or click, stays open while the pointer
 * is over the term or the tooltip itself, and Escape closes it without moving focus.
 *
 * @param container - the element the rendered rich text lives in
 * @param tooltip - the `role="tooltip"` element to fill and position; needs an `id`
 * @returns a cleanup function that removes every listener
 */
export function attachGlossaryTooltip(container: HTMLElement, tooltip: HTMLElement): () => void {
	let activeTerm: HTMLElement | null = null;
	let hoverCloseTimer: ReturnType<typeof setTimeout> | undefined;

	function cancelHoverClose() {
		clearTimeout(hoverCloseTimer);
		hoverCloseTimer = undefined;
	}

	function position(term: HTMLElement) {
		// Anchor at the term's top-centre; the CSS transform lifts and centres the box. Clamp
		// horizontally so a term near an edge doesn't push the tooltip off-screen.
		const rect = term.getBoundingClientRect();
		const halfWidth = tooltip.offsetWidth / 2;
		const centre = rect.left + rect.width / 2;
		const left = Math.min(
			Math.max(centre, halfWidth + EDGE_MARGIN_PX),
			window.innerWidth - halfWidth - EDGE_MARGIN_PX
		);
		tooltip.style.left = `${left}px`;
		tooltip.style.top = `${rect.top}px`;
	}

	function show(term: HTMLElement) {
		cancelHoverClose();
		const text = term.getAttribute('data-glossary-tooltip');
		if (!text) return;

		if (activeTerm && activeTerm !== term) activeTerm.removeAttribute('aria-describedby');
		activeTerm = term;
		tooltip.textContent = text;
		tooltip.setAttribute('data-visible', 'true');
		tooltip.setAttribute('aria-hidden', 'false');
		term.setAttribute('aria-describedby', tooltip.id);
		position(term);
	}

	function hide() {
		cancelHoverClose();
		tooltip.setAttribute('data-visible', 'false');
		tooltip.setAttribute('aria-hidden', 'true');
		activeTerm?.removeAttribute('aria-describedby');
		activeTerm = null;
	}

	function hideUnlessFocused() {
		if (activeTerm && activeTerm === document.activeElement) return;
		hide();
	}

	function scheduleHoverClose() {
		cancelHoverClose();
		hoverCloseTimer = setTimeout(hideUnlessFocused, HOVER_CLOSE_DELAY_MS);
	}

	function showForEvent(event: Event) {
		const term = closestTerm(event.target);
		if (term) show(term);
	}

	function onTermMouseOut(event: MouseEvent) {
		const term = closestTerm(event.target);
		if (!term || term !== activeTerm) return;
		if (isInside(term, event.relatedTarget) || isInside(tooltip, event.relatedTarget)) return;
		scheduleHoverClose();
	}

	function onTermFocusOut(event: FocusEvent) {
		const term = closestTerm(event.target);
		if (term && term === activeTerm) hide();
	}

	function onTooltipMouseLeave(event: MouseEvent) {
		if (isInside(activeTerm, event.relatedTarget)) return;
		scheduleHoverClose();
	}

	function onKeydown(event: KeyboardEvent) {
		if (event.key !== 'Escape' || !activeTerm) return;
		// Consume the key so a dialog the content sits in stays open; only the tooltip closes.
		event.stopPropagation();
		hide();
	}

	container.addEventListener('mouseover', showForEvent);
	container.addEventListener('focusin', showForEvent);
	container.addEventListener('click', showForEvent);
	container.addEventListener('mouseout', onTermMouseOut);
	container.addEventListener('focusout', onTermFocusOut);
	tooltip.addEventListener('mouseenter', cancelHoverClose);
	tooltip.addEventListener('mouseleave', onTooltipMouseLeave);
	window.addEventListener('keydown', onKeydown, true);
	// Capture so a scroll in any ancestor (the article scroll container) dismisses it.
	window.addEventListener('scroll', hide, true);
	window.addEventListener('resize', hide);

	return () => {
		cancelHoverClose();
		container.removeEventListener('mouseover', showForEvent);
		container.removeEventListener('focusin', showForEvent);
		container.removeEventListener('click', showForEvent);
		container.removeEventListener('mouseout', onTermMouseOut);
		container.removeEventListener('focusout', onTermFocusOut);
		tooltip.removeEventListener('mouseenter', cancelHoverClose);
		tooltip.removeEventListener('mouseleave', onTooltipMouseLeave);
		window.removeEventListener('keydown', onKeydown, true);
		window.removeEventListener('scroll', hide, true);
		window.removeEventListener('resize', hide);
	};
}
