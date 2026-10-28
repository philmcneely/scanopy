import type { ElkNode } from 'elkjs';

/**
 * Order a container's children so each edge's source comes before its target.
 *
 * A topological sort over the edges whose endpoints are both in `ids`, one connected group at a
 * time so a chain stays contiguous. Ties go to the child that appears first in `edges`: dependency
 * edges arrive in the dependency's member order, so a hub's spokes keep the order they were listed
 * in. On a cycle the earliest remaining child is taken as though its incoming edges were absent.
 * Groups go in order of their first edge; children no edge touches follow, in their order in `ids`.
 * The result depends only on the inputs, so the same data always orders the same.
 */
export function dependencyOrder(
	ids: readonly string[],
	edges: readonly { source: string; target: string }[]
): string[] {
	const members = new Set(ids);
	const firstSeen = new Map<string, number>();
	const outgoing = new Map<string, string[]>();
	const neighbours = new Map<string, string[]>();
	const inDegree = new Map<string, number>();
	const seen = new Set<string>();
	const link = (from: string, to: string, map: Map<string, string[]>) => {
		if (!map.has(from)) map.set(from, []);
		map.get(from)!.push(to);
	};
	for (const { source, target } of edges) {
		if (source === target || !members.has(source) || !members.has(target)) continue;
		const key = `${source}\u0000${target}`;
		if (seen.has(key)) continue;
		seen.add(key);
		for (const id of [source, target]) if (!firstSeen.has(id)) firstSeen.set(id, firstSeen.size);
		link(source, target, outgoing);
		link(source, target, neighbours);
		link(target, source, neighbours);
		inDegree.set(target, (inDegree.get(target) ?? 0) + 1);
	}

	const byFirstSeen = [...firstSeen.keys()];
	const placed = new Set<string>();
	const order: string[] = [];
	for (const start of byFirstSeen) {
		if (placed.has(start)) continue;
		// The connected group `start` belongs to, in first-seen order.
		const group = new Set([start]);
		const queue = [start];
		while (queue.length) {
			for (const next of neighbours.get(queue.pop()!) ?? []) {
				if (!group.has(next)) {
					group.add(next);
					queue.push(next);
				}
			}
		}
		const candidates = byFirstSeen.filter((id) => group.has(id));
		for (let left = candidates.length; left > 0; left--) {
			const next =
				candidates.find((id) => !placed.has(id) && (inDegree.get(id) ?? 0) === 0) ??
				candidates.find((id) => !placed.has(id))!;
			placed.add(next);
			order.push(next);
			for (const target of outgoing.get(next) ?? []) {
				inDegree.set(target, (inDegree.get(target) ?? 0) - 1);
			}
		}
	}
	return [...order, ...ids.filter((id) => !firstSeen.has(id))];
}

/**
 * The connected group each of `ids` belongs to over `edges`, named by its first member in `ids`.
 * A child no edge touches is a group of its own.
 */
export function connectedGroups(
	ids: readonly string[],
	edges: readonly { source: string; target: string }[]
): Map<string, string> {
	const parent = new Map(ids.map((id) => [id, id]));
	const find = (id: string): string => {
		let root = id;
		while (parent.get(root) !== root) root = parent.get(root)!;
		parent.set(id, root);
		return root;
	};
	for (const { source, target } of edges) {
		if (!parent.has(source) || !parent.has(target)) continue;
		const a = find(source);
		const b = find(target);
		if (a === b) continue;
		// The member earlier in `ids` names the group.
		if (ids.indexOf(a) < ids.indexOf(b)) parent.set(b, a);
		else parent.set(a, b);
	}
	return new Map(ids.map((id) => [id, find(id)]));
}

/**
 * Put a container's children in `rank` order and keep them there through box packing.
 *
 * ELK's box algorithm places children by `elk.priority`, highest first, and only then by size, so a
 * descending priority per position fixes the reading order: left to right, top to bottom.
 * Children missing from `rank` go last.
 */
export function pinChildOrder(children: ElkNode[], rank: Map<string, number>): void {
	const rankOf = (child: ElkNode) => rank.get(child.id) ?? Number.MAX_SAFE_INTEGER;
	children.sort((a, b) => rankOf(a) - rankOf(b));
	children.forEach((child, index) => {
		child.layoutOptions = {
			...child.layoutOptions,
			'elk.priority': String(children.length - index)
		};
	});
}
