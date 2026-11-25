type ScrollPosition = { scrollTop: number; clientHeight: number; scrollHeight: number };

// A section counts as current once its heading is this close to the top of the scroller.
const HEADING_ZONE_PX = 96;

/**
 * Index of the section being read, given each section's top offset inside the scroller.
 * The last section wins at the bottom, since a short last section can never reach the top.
 */
export function activeSectionIndex(sectionTops: number[], position: ScrollPosition): number {
	const atBottom = position.scrollTop + position.clientHeight >= position.scrollHeight - 1;
	if (atBottom) return Math.max(0, sectionTops.length - 1);
	const reached = sectionTops.findLastIndex((top) => top <= position.scrollTop + HEADING_ZONE_PX);
	return Math.max(0, reached);
}
