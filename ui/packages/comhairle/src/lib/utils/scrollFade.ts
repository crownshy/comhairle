type ScrollMetrics = {
	scrollTop: number;
	scrollHeight: number;
	clientHeight: number;
};

const FADE_SIZE = '2rem';
/** Sub-pixel scroll positions can stop a pixel short of the true end. */
const EDGE_TOLERANCE_PX = 1;

/**
 * A CSS `mask-image` that fades whichever edge of a scroll container still has content past it,
 * or `none` when everything is in view.
 */
export function scrollFadeMask({ scrollTop, scrollHeight, clientHeight }: ScrollMetrics): string {
	const hasMoreAbove = scrollTop > EDGE_TOLERANCE_PX;
	const hasMoreBelow = scrollTop + clientHeight < scrollHeight - EDGE_TOLERANCE_PX;
	if (!hasMoreAbove && !hasMoreBelow) return 'none';

	const top = hasMoreAbove ? `transparent, black ${FADE_SIZE}` : 'black, black';
	const bottom = hasMoreBelow ? `black calc(100% - ${FADE_SIZE}), transparent` : 'black';
	return `linear-gradient(to bottom, ${top}, ${bottom})`;
}
