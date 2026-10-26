import { ArrowUpCircle } from 'lucide-svelte';
import type { TagProps } from '$lib/shared/components/data/types';
import { common_upgrade, common_upgradePlan } from '$lib/paraglide/messages';

/**
 * The badge on something the plan lacks (an export format, a credential type, snapshots), in the
 * upgrade button's colours. `title` names what is missing; it names no plan, since the lowest one
 * differs between cloud and self-hosted.
 */
export function upgradeTagProps(title: string = common_upgradePlan()): TagProps {
	return { label: common_upgrade(), icon: ArrowUpCircle, upgrade: true, title };
}
