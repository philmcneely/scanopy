/**
 * Whether the server is answering, and the refetch once it answers again.
 *
 * A request that never reaches the server (refused connection, a proxy answering for a server
 * that is down or restarting) fails every query that runs meanwhile. TanStack retries each a
 * couple of times and then leaves it in error until a focus or remount, so after a restart the
 * app keeps showing failures with the server back up. This module marks the server unreachable on
 * the first such failure, probes until it answers, and then invalidates every query once, which
 * refetches the active ones and marks the rest stale for their next mount.
 *
 * Invalidating on recovery, rather than taking the app offline through TanStack's
 * `onlineManager`, keeps failed requests failing as they do now: going offline would also pause
 * mutations and replay them when the server came back.
 */

/** How often to probe while the server is unreachable. */
export const PROBE_INTERVAL_MS = 5_000;

/**
 * The readiness route, rather than the liveness one: it answers only once the server can reach
 * its database, which is when the app's own requests start working again.
 */
const PROBE_PATH = '/api/health/ready';

let unreachable = false;
let probeTimer: ReturnType<typeof setTimeout> | null = null;
let probeUrl = PROBE_PATH;

/** Set the server origin the probe goes to. The API client calls this once with its base URL. */
export function setProbeOrigin(origin: string): void {
	probeUrl = `${origin}${PROBE_PATH}`;
}

export function isServerUnreachable(): boolean {
	return unreachable;
}

/**
 * Whether a response came from something other than the Scanopy server: a gateway error, or a
 * 5xx without the server's JSON error body, as a reverse proxy or the dev server's proxy sends
 * when the server behind it is down. A 5xx the server sent itself is a real error, not an outage.
 */
export function isUnreachableResponse(response: Response): boolean {
	if (response.status === 502 || response.status === 503 || response.status === 504) return true;
	if (response.status < 500) return false;
	return !(response.headers.get('content-type') ?? '').includes('application/json');
}

/**
 * Record a request that did not reach the server. Returns true for the first one of an outage,
 * so the caller tells the user once rather than once per failed request.
 */
export function reportUnreachable(): boolean {
	if (unreachable) return false;
	unreachable = true;
	scheduleProbe();
	return true;
}

/** Record a response from the server. The first one after an outage refetches every query. */
export function reportAnswered(): void {
	if (!unreachable) return;
	unreachable = false;
	if (probeTimer !== null) {
		clearTimeout(probeTimer);
		probeTimer = null;
	}
	// Imported lazily: query-client imports the API client, which imports this module.
	void import('./query-client').then(({ queryClient }) => queryClient.invalidateQueries());
}

function scheduleProbe(): void {
	if (probeTimer !== null) return;
	probeTimer = setTimeout(probe, PROBE_INTERVAL_MS);
}

/** Plain fetch, outside the API client, so a failed probe raises no toast and no report. */
async function probe(): Promise<void> {
	probeTimer = null;
	if (!unreachable) return;
	try {
		const response = await fetch(probeUrl, { credentials: 'include' });
		if (response.ok) {
			reportAnswered();
			return;
		}
	} catch {
		// Still unreachable.
	}
	if (unreachable) scheduleProbe();
}
