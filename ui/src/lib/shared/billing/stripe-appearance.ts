import type { Appearance } from '@stripe/stripe-js';

// Stripe Elements lives in an iframe, so it can't inherit the app's CSS.
// Mirror Scanopy's design tokens (read live from :root, so it tracks the
// active light/dark theme) into the Elements appearance API.
function cssVar(name: string): string {
	return getComputedStyle(document.documentElement).getPropertyValue(name).trim();
}

function parseColor(color: string): [number, number, number, number] | null {
	const hex = /^#([0-9a-f]{3}|[0-9a-f]{6})$/i.exec(color);
	if (hex) {
		const digits = hex[1].length === 3 ? hex[1].replace(/./g, '$&$&') : hex[1];
		const channel = (i: number) => parseInt(digits.slice(i, i + 2), 16);
		return [channel(0), channel(2), channel(4), 1];
	}
	const rgb = /^rgba?\(\s*([\d.]+)[\s,]+([\d.]+)[\s,]+([\d.]+)(?:[\s,/]+([\d.]+))?\s*\)$/i.exec(
		color
	);
	if (rgb) return [Number(rgb[1]), Number(rgb[2]), Number(rgb[3]), Number(rgb[4] ?? 1)];
	return null;
}

/**
 * `color` as an opaque `rgb()`, blended onto `backdrop` when it is
 * translucent. Stripe's `colorBackground` takes only opaque colours and drops
 * any other, and the dark theme's input colour is translucent. Returns
 * `color` unchanged when either can't be read.
 */
export function opaqueOn(color: string, backdrop: string): string {
	const front = parseColor(color);
	const back = parseColor(backdrop);
	if (!front || !back) return color;
	const [r, g, b, a] = front;
	const mix = (f: number, k: number) => Math.round(f * a + k * (1 - a));
	return `rgb(${mix(r, back[0])}, ${mix(g, back[1])}, ${mix(b, back[2])})`;
}

/** Elements appearance matching the app's current theme. */
export function buildStripeAppearance(): Appearance {
	const isDark = document.documentElement.classList.contains('dark');
	const accent = '#3b82f6'; // blue-500, matches btn-primary / focus ring
	const inputBg = cssVar('--color-bg-input');
	const inputBorder = cssVar('--color-border-input');
	const textPrimary = cssVar('--color-text-primary');
	return {
		theme: isDark ? 'night' : 'stripe',
		variables: {
			colorPrimary: accent,
			// The form sits on the modal's elevated surface.
			colorBackground: opaqueOn(inputBg, cssVar('--color-bg-elevated')),
			colorText: textPrimary,
			colorTextSecondary: cssVar('--color-text-secondary'),
			colorTextPlaceholder: cssVar('--color-text-muted'),
			colorDanger: '#ef4444', // red-500
			fontFamily: getComputedStyle(document.body).fontFamily,
			borderRadius: '6px'
		},
		rules: {
			'.Input': {
				backgroundColor: inputBg,
				borderColor: inputBorder,
				color: textPrimary
			},
			'.Input:focus': {
				borderColor: accent,
				boxShadow: '0 0 0 2px rgba(59, 130, 246, 0.5)'
			},
			'.Tab, .AccordionItem': {
				backgroundColor: cssVar('--color-bg-elevated'),
				borderColor: cssVar('--color-border')
			},
			'.Tab:hover, .AccordionItem:hover': {
				backgroundColor: inputBg
			},
			'.Label': {
				color: cssVar('--color-text-secondary')
			}
		}
	};
}
