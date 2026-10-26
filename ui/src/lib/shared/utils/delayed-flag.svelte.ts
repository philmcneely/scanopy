/**
 * True once `active` has held for `ms`, false as soon as it stops. A spinner
 * gated on it stays hidden through a wait too short to need one.
 */
export function useDelayedFlag(active: () => boolean, ms: number): { readonly current: boolean } {
	let shown = $state(false);
	$effect(() => {
		if (!active()) {
			shown = false;
			return;
		}
		const timer = setTimeout(() => (shown = true), ms);
		return () => clearTimeout(timer);
	});
	return {
		get current() {
			return shown;
		}
	};
}
