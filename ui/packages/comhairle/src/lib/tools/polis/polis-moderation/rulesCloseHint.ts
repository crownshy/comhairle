import { driver } from 'driver.js';
import 'driver.js/dist/driver.css';

/**
 * After the moderation rules dialog closes, spotlights its button with a short tip, so
 * moderators know where to open the rules again. Any click dismisses it.
 * Waits for the dialog's own close animation so the two don't overlap.
 */
export function showCloseHint(target: HTMLElement | null) {
	if (!target) return;
	setTimeout(() => {
		const tip = driver({
			overlayColor: 'black',
			overlayOpacity: 0.5,
			stagePadding: 6,
			stageRadius: 8,
			allowClose: true,
			animate: !window.matchMedia('(prefers-reduced-motion: reduce)').matches
		});
		tip.highlight({
			element: target,
			popover: {
				title: 'Moderation rules are here any time',
				description: 'Open them again whenever you want a refresher.',
				side: 'bottom',
				align: 'start',
				showButtons: ['next'],
				nextBtnText: 'Got it',
				onNextClick: () => tip.destroy()
			}
		});
	}, 150);
}
