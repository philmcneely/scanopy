/**
 * Reloads the tab onto the server's frontend when the server has been upgraded under it.
 *
 * A tab loaded before an upgrade keeps running the old bundle against the new API. Every
 * response reports the server's version; when it differs from the version this bundle was built
 * as, the tab reloads. Forms holding user input live in modals, so with a modal open the reload
 * waits for it to close, and the modal shows a banner offering to reload now
 * (`ServerUpdatedBanner`).
 */

import { get, writable } from 'svelte/store';
import { VERSION } from '$lib/version';
import { modalState } from '$lib/shared/stores/modal-registry';

/**
 * The server version the last automatic reload was for. If the reloaded bundle still doesn't
 * match it (a CDN still serving the old bundle), reloading again would loop, so the banner asks
 * instead.
 */
const RELOADED_FOR_KEY = 'scanopy_reloaded_for_server_version';

/** Whether the server runs a different version than this bundle and the tab has not reloaded. */
export const serverUpdatePending = writable(false);

let waitingForModal: (() => void) | null = null;

export function checkServerVersion(serverVersion: string): void {
	if (serverVersion === VERSION) {
		clearReloadedFor();
		return;
	}
	if (get(serverUpdatePending)) return;
	serverUpdatePending.set(true);

	if (readReloadedFor() === serverVersion) return;
	if (get(modalState).name === null) {
		reloadFor(serverVersion);
		return;
	}
	// The first call, made synchronously by subscribe, sees the open modal and returns, so
	// `waitingForModal` is assigned by the time the modal closes.
	waitingForModal = modalState.subscribe((state) => {
		if (state.name !== null) return;
		waitingForModal?.();
		waitingForModal = null;
		reloadFor(serverVersion);
	});
}

/** Reload now, from the banner. */
export function reloadForServerUpdate(): void {
	window.location.reload();
}

function reloadFor(serverVersion: string): void {
	try {
		sessionStorage.setItem(RELOADED_FOR_KEY, serverVersion);
	} catch {
		// Without storage the loop guard is lost, not the reload.
	}
	window.location.reload();
}

function readReloadedFor(): string | null {
	try {
		return sessionStorage.getItem(RELOADED_FOR_KEY);
	} catch {
		return null;
	}
}

function clearReloadedFor(): void {
	try {
		sessionStorage.removeItem(RELOADED_FOR_KEY);
	} catch {
		// Nothing stored to clear.
	}
}
