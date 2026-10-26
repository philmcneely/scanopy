// `/pure`: importing this module injects nothing. Stripe.js loads only when
// `preloadStripe` first runs (see StripeCardForm for why that matters).
import { loadStripe } from '@stripe/stripe-js/pure';
import type { Stripe } from '@stripe/stripe-js';
import { useConfigQuery } from '$lib/shared/stores/config-query';
import { useCurrentUserQuery } from '$lib/features/auth/queries';
import { useOrganizationQuery } from '$lib/features/organizations/queries';
import { createSetupIntent } from '$lib/features/billing/queries';

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

// One SetupIntent per org, made ahead of the payment dialog. An unconfirmed
// intent can be reused, so it is kept across opens until a card is saved.
let setupIntent: { organizationId: string; clientSecret: Promise<string> } | null = null;

/**
 * The client secret for `organizationId`'s payment form, created now unless
 * one is already in flight or ready. A failure is forgotten, so the next
 * caller retries.
 */
export function takeSetupIntent(organizationId: string): Promise<string> {
	if (setupIntent == null || setupIntent.organizationId !== organizationId) {
		const clientSecret = createSetupIntent().catch((error: unknown) => {
			if (setupIntent?.clientSecret === clientSecret) setupIntent = null;
			throw error;
		});
		setupIntent = { organizationId, clientSecret };
	}
	return setupIntent.clientSecret;
}

/** Drop the cached SetupIntent once a card has been saved with it. */
export function clearSetupIntent(): void {
	setupIntent = null;
}

/**
 * Get the payment dialog ready while a component that opens it is showing
 * it (`showing`, default always): Stripe.js once the config names the
 * publishable key, and, for an owner (the only role that can add a card),
 * the SetupIntent the form mounts on. Components that stay mounted while
 * hidden pass `showing`, so a page that offers no card fetches neither.
 */
export function usePreloadStripe(showing: () => boolean = () => true): void {
	const configQuery = useConfigQuery();
	const currentUserQuery = useCurrentUserQuery();
	const organizationQuery = useOrganizationQuery();
	$effect(() => {
		if (!showing()) return;
		const publishableKey = configQuery.data?.stripe_publishable_key;
		void preloadStripe(publishableKey)?.catch(() => {
			// StripeCardForm reports a failed load when it needs Stripe.
		});
		const organizationId = organizationQuery.data?.id;
		if (publishableKey && organizationId && currentUserQuery.data?.permissions === 'Owner') {
			void takeSetupIntent(organizationId).catch(() => {
				// The API client toasts it; the dialog retries when it opens.
			});
		}
	});
}
