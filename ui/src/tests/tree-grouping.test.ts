import { describe, it, expect } from 'vitest';
import {
	buildTreeSections,
	flattenTreeEntries,
	isSingleRootTree,
	mergeRootedTrees,
	ROOTED_TREES_KEY,
	type RenderGroup,
	type TreeEntry
} from '$lib/shared/components/data/controls/grouping';
import type { TreeConfig } from '$lib/shared/components/data/types';
import { subnetNesting } from '$lib/features/subnets/nesting';
import type { SubnetResponse } from '$lib/features/subnets/types/base';

interface Range {
	id: string;
	parent: string | null;
	depth: number;
}

const tree: TreeConfig<Range> = {
	key: (r) => r.id,
	parentKey: (r) => r.parent,
	depth: (r) => r.depth,
	rootsLabel: () => 'Nested'
};

// In the server's order: each range directly before the ranges inside it.
const wide: Range = { id: 'wide', parent: null, depth: 0 };
const sibling: Range = { id: 'sibling', parent: 'wide', depth: 1 };
const mid: Range = { id: 'mid', parent: 'wide', depth: 1 };
const leaf: Range = { id: 'leaf', parent: 'mid', depth: 2 };

/** A group's entries as `depth|id` for a leaf row and an object for a row with children. */
function shape(entries: TreeEntry<Range>[]): unknown[] {
	return entries.map((entry) =>
		entry.type === 'row'
			? `${entry.depth}|${entry.item.id}`
			: {
					section: `${entry.depth}|${entry.item.id}`,
					count: entry.count,
					depth: entry.depth,
					entries: shape(entry.entries)
				}
	);
}

describe('buildTreeSections', () => {
	it('nests three levels deep, each parent its own row with its children under it', () => {
		const entries = buildTreeSections([wide, sibling, mid, leaf], tree, 'g');
		// No parent appears inside its own section: wide is the row, its children follow one in.
		expect(shape(entries)).toEqual([
			{
				section: '0|wide',
				count: 3,
				depth: 0,
				entries: ['1|sibling', { section: '1|mid', count: 1, depth: 1, entries: ['2|leaf'] }]
			}
		]);
		expect(flattenTreeEntries(entries).map((r) => r.id)).toEqual([
			'wide',
			'sibling',
			'mid',
			'leaf'
		]);
	});

	it('starts rows whose ancestors are on an earlier page at the top', () => {
		const other: Range = { id: 'other', parent: 'wide', depth: 1 };
		expect(shape(buildTreeSections([leaf, other], tree, 'g'))).toEqual(['0|leaf', '0|other']);
		expect(shape(buildTreeSections([sibling, mid, leaf], tree, 'g'))).toEqual([
			'0|sibling',
			{ section: '0|mid', count: 1, depth: 0, entries: ['1|leaf'] }
		]);
	});

	it('keys sections by path, so same-labelled rows in different places stay apart', () => {
		const twinA: Range = { id: 'twin-a', parent: null, depth: 0 };
		const twinB: Range = { id: 'twin-b', parent: null, depth: 0 };
		const kidA: Range = { id: 'kid-a', parent: 'twin-a', depth: 1 };
		const kidB: Range = { id: 'kid-b', parent: 'twin-b', depth: 1 };
		const entries = buildTreeSections([twinA, kidA, twinB, kidB], tree, 'g');
		const keys = entries.map((entry) => (entry.type === 'section' ? entry.key : null));
		expect(keys[0]).not.toBeNull();
		expect(new Set(keys).size).toBe(2);
		expect(
			buildTreeSections([twinA, kidA, twinB, kidB], tree, 'other').map((entry) =>
				entry.type === 'section' ? entry.key : null
			)
		).not.toEqual(keys);
	});
});

describe('isSingleRootTree', () => {
	it('picks out a group that is one tree under a real root', () => {
		const entries = buildTreeSections([wide, sibling, mid, leaf], tree, 'g');
		expect(isSingleRootTree(entries, tree)).toBe(true);
	});

	it('leaves out a group whose top row has a parent on another page', () => {
		const entries = buildTreeSections([mid, leaf], tree, 'g');
		expect(isSingleRootTree(entries, tree)).toBe(false);
	});

	it('leaves out a group of several top-level rows', () => {
		const other: Range = { id: 'other', parent: null, depth: 0 };
		const entries = buildTreeSections([wide, mid, other], tree, 'g');
		expect(isSingleRootTree(entries, tree)).toBe(false);
	});
});

describe('mergeRootedTrees', () => {
	const group = (key: string, rows: Range[], range: RenderGroup<Range>['range'] = null) => {
		const entries = buildTreeSections(rows, tree, key);
		return { key, name: key, items: rows, range, entries, rootRanges: null };
	};
	const other: Range = { id: 'other', parent: null, depth: 0 };
	const otherKid: Range = { id: 'other-kid', parent: 'other', depth: 1 };
	const loose: Range = { id: 'loose', parent: null, depth: 0 };

	it('collects every rooted tree under one header, where the first one was', () => {
		const range = { start: 1, end: 3, total: 3 };
		const groups = [
			group('Ungrouped', [loose]),
			group('wide', [wide, mid, leaf], range),
			group('other', [other, otherKid])
		];
		const merged = mergeRootedTrees(groups, tree);
		expect(merged.map((g) => g.name)).toEqual(['Ungrouped', 'Nested']);
		const [, trees] = merged;
		expect(trees.key).toBe(ROOTED_TREES_KEY);
		// Roots are ordinary rows of the merged group, not headers of their own.
		expect(trees.entries!.map((entry) => entry.item.id)).toEqual(['wide', 'other']);
		expect(trees.items.map((r) => r.id)).toEqual(['wide', 'mid', 'leaf', 'other', 'other-kid']);
		expect(trees.rootRanges!.get('wide')).toEqual(range);
		expect(trees.rootRanges!.has('other')).toBe(false);
	});

	it('keeps a group whose root is not shown under its own header', () => {
		const groups = [group('wide', [mid, leaf]), group('other', [other, otherKid])];
		expect(mergeRootedTrees(groups, tree).map((g) => g.name)).toEqual(['wide', 'Nested']);
	});

	it('leaves groups alone when none is a rooted tree', () => {
		const groups = [group('Ungrouped', [loose])];
		expect(mergeRootedTrees(groups, tree)).toBe(groups);
	});
});

describe('subnetNesting', () => {
	function subnet(id: string, cidr: string, parent: string | null): SubnetResponse {
		return {
			id,
			cidr,
			name: id,
			parent_subnet_id: parent,
			used_addresses: 0,
			usable_addresses: 0
		} as SubnetResponse;
	}

	const subnets = [
		subnet('wide', '10.10.0.0/16', null),
		subnet('mid', '10.10.16.0/20', 'wide'),
		subnet('leaf', '10.10.16.0/24', 'mid'),
		subnet('lone', '192.168.1.0/24', null),
		subnet('orphan', '172.16.1.0/24', 'not-listed')
	];
	const nesting = subnetNesting(subnets);
	const find = (id: string) => subnets.find((s) => s.id === id)!;

	it('walks to the widest range above a subnet', () => {
		expect(nesting.rootOf(find('leaf')).id).toBe('wide');
		expect(nesting.rootOf(find('wide')).id).toBe('wide');
		expect(nesting.rootOf(find('lone')).id).toBe('lone');
	});

	it('derives children from the parent ids', () => {
		expect(nesting.childrenOf('wide').map((s) => s.id)).toEqual(['mid']);
		expect(nesting.childrenOf('leaf')).toEqual([]);
	});

	it('treats a parent missing from the list as no parent', () => {
		expect(nesting.parentOf(find('orphan'))).toBeNull();
		expect(nesting.rootOf(find('orphan')).id).toBe('orphan');
	});
});
