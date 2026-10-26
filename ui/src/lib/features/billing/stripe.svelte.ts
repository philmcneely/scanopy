// `/pure`: importing this module injects nothing. Stripe.js loads only when
// `preloadStripe` first runs (see StripeCardForm for why that matters).
import { loadStripe } from '@stripe/stripe-js/pure';
import type { Stripe } from '@stripe/stripe-js';
import { useConfigQuery } from '$lib/shared/stores/config-query';

let stripePromise: Promise<Stripe | null> | null = null;
let loadedKey: string | null = null;

/**
 * Stripe.js for `publishableKey`, loaded once and shared. The payment dialog
 * awaits the same promise the surfaces that offer it started, so by the time
 * someone clicks Add Payment Method the script is already there. A failed
 * load is forgotten, so the next caller retries.
 */
export function preloadStripe(
	publishableKey: string | null | undefined
): Promise<Stripe | null> | null {
	if (!publishableKey) return null;
	if (stripePromise == null || loadedKey !== publishableKey) {
		loadedKey = publishableKey;
		stripePromise = loadStripe(publishableKey).catch((error: unknown) => {
			stripePromise = null;
			throw error;
		});
	}
	return stripePromise;
}

/**
 * Start loading Stripe.js once a component that can open the payment dialog
 * is showing it (`showing`, default always) and the server config names the
 * publishable key. Components that stay mounted while hidden pass `showing`,
 * so a page that offers no card never fetches the script.
 */
export function usePreloadStripe(showing: () => boolean = () => true): void {
	const configQuery = useConfigQuery();
	$effect(() => {
		if (!showing()) return;
		void preloadStripe(configQuery.data?.stripe_publishable_key)?.catch(() => {
			// StripeCardForm reports a failed load when it needs Stripe.
		});
	});
}
