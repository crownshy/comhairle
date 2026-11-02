import type { GuideLink } from '$lib/components/SectionHeader.svelte';

/** The current page's Handbook guide. Its SectionHeader registers it; the help button reads it. */
class HelpStore {
	guide = $state<GuideLink | null>(null);

	setGuide(guide: GuideLink) {
		this.guide = guide;
	}

	/** Guards on href so a fast route change can't clear the next page's guide. */
	clearGuide(guide: GuideLink) {
		if (this.guide?.href === guide.href) this.guide = null;
	}
}

export const helpStore = new HelpStore();
