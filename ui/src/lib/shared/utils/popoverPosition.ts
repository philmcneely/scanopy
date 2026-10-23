/**
 * Where a floating panel goes against its trigger, and how tall it may grow. Every popover, select
 * dropdown and tooltip places itself through `fitToViewport`, so they all keep the same distance
 * from the trigger and from the edges of the screen.
 */

/** The closest a popover comes to any edge of the viewport. */
export const VIEWPORT_MARGIN_PX = 16;
/** The space between a popover and its trigger. */
export const TRIGGER_GAP_PX = 4;
/** A popover is never capped shorter than this, even when neither side has the room. */
export const MIN_POPOVER_HEIGHT_PX = 160;

export interface PopoverPlacement {
	/** Set when the popover opens below the trigger. */
	top: number | null;
	/** Set when it opens above, measured from the viewport's bottom edge, so it grows upward. */
	bottom: number | null;
	left: number;
	maxHeight: number;
}

export function fitToViewport({
	trigger,
	contentHeight,
	width,
	viewport,
	align = 'center',
	prefer = 'below'
}: {
	trigger: Pick<DOMRect, 'top' | 'bottom' | 'left' | 'width'>;
	/** The popover's height with no cap applied. */
	contentHeight: number;
	/** The popover's rendered width. */
	width: number;
	viewport: { width: number; height: number };
	/** `start` lines the popover's left edge up with the trigger's; `center` centres it. */
	align?: 'start' | 'center';
	/** The side to open on when the content fits there or that side has the more room. */
	prefer?: 'below' | 'above';
}): PopoverPlacement {
	const spaceBelow = viewport.height - trigger.bottom - TRIGGER_GAP_PX - VIEWPORT_MARGIN_PX;
	const spaceAbove = trigger.top - TRIGGER_GAP_PX - VIEWPORT_MARGIN_PX;
	const [preferred, other] =
		prefer === 'below' ? [spaceBelow, spaceAbove] : [spaceAbove, spaceBelow];
	const onPreferred = contentHeight <= preferred || preferred >= other;
	const below = (prefer === 'below') === onPreferred;

	const start = align === 'start' ? trigger.left : trigger.left + trigger.width / 2 - width / 2;
	const left = Math.max(
		VIEWPORT_MARGIN_PX,
		Math.min(start, viewport.width - width - VIEWPORT_MARGIN_PX)
	);

	return below
		? {
				top: trigger.bottom + TRIGGER_GAP_PX,
				bottom: null,
				left,
				maxHeight: Math.max(spaceBelow, MIN_POPOVER_HEIGHT_PX)
			}
		: {
				top: null,
				bottom: viewport.height - trigger.top + TRIGGER_GAP_PX,
				left,
				maxHeight: Math.max(spaceAbove, MIN_POPOVER_HEIGHT_PX)
			};
}
