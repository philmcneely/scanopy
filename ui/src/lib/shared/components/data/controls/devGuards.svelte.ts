import type { FieldConfig, TableDefaults } from '../types';
import { serverFilterViolations, type ServerFilterMode } from './filtering';
import { defaultOrderingViolations } from './sorting';

/**
 * Fail loudly on a filter the server does not apply.
 *
 * Call during component init. Dev/test only, so it never reaches a user.
 */
export function guardServerFilters<T>(
	get: () => { fields: FieldConfig<T>[]; server: ServerFilterMode }
) {
	// A filterable field the server ignores is always a bug: the client holds one page, so the
	// filter would narrow nothing while the user believes it applies. The rule is documented on
	// `FieldConfig.serverFiltered`; this makes it fail loudly instead of silently, at the one place
	// that sees every field's resolved config.
	$effect(() => {
		if (!import.meta.env.DEV) return;

		const { fields, server } = get();
		const offenders = serverFilterViolations(fields, server);
		if (offenders.length === 0) return;

		throw new Error(
			`DataControls: ${offenders.join(', ')} ${offenders.length === 1 ? 'is' : 'are'} ` +
				`filterable but not filtered by the server, so the filter would narrow nothing. ` +
				`Mark the field serverFiltered and give it the endpoint's queryParam, or drop its ` +
				`filterable flag.`
		);
	});
}

/**
 * Fail loudly on a default group or sort the list cannot apply.
 *
 * Call during component init. Dev/test only, so it never reaches a user.
 */
export function guardTableDefaults<T>(
	get: () => { fields: FieldConfig<T>[]; defaults: TableDefaults }
) {
	$effect(() => {
		if (!import.meta.env.DEV) return;

		const { fields, defaults } = get();
		const offenders = defaultOrderingViolations(fields, defaults);
		if (offenders.length === 0) return;

		throw new Error(
			`DataControls: default ${offenders.join(', ')} cannot apply. A default must name a ` +
				`groupable or sortable field whose column renders by default.`
		);
	});
}
