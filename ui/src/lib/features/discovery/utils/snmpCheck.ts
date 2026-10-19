/**
 * The net-snmp commands that repeat a read a walk warning reports, so an operator can check the
 * warning against the device from the daemon's host.
 *
 * Everything that varies comes off the wire or the fixture: the address and the credential's
 * options (secrets left as placeholders) from the warning, and each group's first column, page
 * size, timeout and retries from `snmp-walk-groups.json`, which the daemon's own constants fill.
 */

import snmpWalkGroups from '$lib/data/snmp-walk-groups.json';
import { discovery_snmpOptionsPlaceholder } from '$lib/paraglide/messages';
import type { DiscoveryWarning } from './warnings';

interface WalkParameters {
	first_column: string;
	max_repetitions: number;
	timeout_seconds: number;
	retries: number;
}

function parametersOf(groupId: string): WalkParameters | null {
	const entry = snmpWalkGroups.find((g) => g.id === groupId);
	return (entry?.metadata as WalkParameters | undefined) ?? null;
}

/** The groups a warning's read failed on: one per cause for a whole-device line, since the reads
 *  that ended the same way repeat each other. */
function groupsToCheck(warning: DiscoveryWarning): string[] {
	switch (warning.code) {
		case 'SnmpWalkFailed':
		case 'SnmpWalkBridgeMibAbsent':
			return [warning.group];
		case 'SnmpCollectedNothing': {
			const firstPerCause = new Map<string, string>();
			for (const failure of warning.failures ?? []) {
				if (!firstPerCause.has(failure.cause)) firstPerCause.set(failure.cause, failure.group);
			}
			return [...firstPerCause.values()];
		}
		default:
			return [];
	}
}

/**
 * The commands for one warning: a GETBULK page the size the daemon asks for, a single-row
 * GETBULK, and GETNEXT, against the first column of each group. Which of the three answers is
 * what tells a device that cannot serve GETBULK from one that serves nothing.
 */
export function snmpCheckCommands(warning: DiscoveryWarning): string[] {
	if (!('net_snmp_options' in warning)) return [];
	const options = warning.net_snmp_options ?? discovery_snmpOptionsPlaceholder();

	return groupsToCheck(warning).flatMap((groupId) => {
		const walk = parametersOf(groupId);
		if (!walk) return [];
		const common = `${options} -t ${walk.timeout_seconds} -r ${walk.retries}`;
		const target = `${warning.address} ${walk.first_column}`;
		return [
			`snmpbulkwalk ${common} -Cr${walk.max_repetitions} ${target}`,
			`snmpbulkwalk ${common} -Cr1 ${target}`,
			`snmpwalk ${common} ${target}`
		];
	});
}
