/**
 * Whether Settings shows the License tab. SettingsModal filters its tab list with
 * this, and the billing lock in routes/+page.svelte uses it to pick the tab the
 * Settings modal opens on for a licensed org, so the two can't disagree.
 *
 * An owner on a cloud plan sees it too: the tab previews the license key and
 * points them at the self-hosted plans.
 *
 * - `isOwner`: license keys and the card behind them are owner-only.
 * - `isDemoOrg`: a demo org never holds a license.
 * - `isCloudDeployment`: only Scanopy Cloud sells licenses through Stripe; a
 *   self-hosted server has nothing to show.
 * - `signingAvailable`: no signing key means the mint endpoints fail and no
 *   self-hosted plan can be bought, so the tab would only produce an error toast.
 *
 * Takes booleans so it can be tested without the metadata store.
 */
export function licenseTabVisible(s: {
	isOwner: boolean;
	isDemoOrg: boolean;
	isCloudDeployment: boolean;
	signingAvailable: boolean;
}): boolean {
	return s.isOwner && !s.isDemoOrg && s.isCloudDeployment && s.signingAvailable;
}
