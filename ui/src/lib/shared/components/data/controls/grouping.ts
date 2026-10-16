import {
	getFieldKey,
	type FieldConfig,
	type GroupPosition,
	type GroupSlice,
	type TreeConfig
} from '../types';
import { getFieldValue, type FieldValue } from './fieldValues';

/**
 * Stands in for a null group key, which has no string form of its own.
 *
 * Prefixed with NUL because Postgres text values cannot contain one, so this
 * can never collide with a real group value. Built with `fromCharCode` rather
 * than a literal to keep a raw NUL byte out of the source file.
 */
export const UNGROUPED_KEY = `${String.fromCharCode(0)}ungrouped`;

/** A per-group total from the server, across every page. */
export interface ServerGroupCount {
	value?: string | null;
	count: number;
}

/** The translated headers for groups whose raw value has no readable form. */
export interface GroupLabels {
	/** Rows with no value for the grouped field. */
	ungrouped: string;
	/** A boolean field's `true` group. */
	yes: string;
	/** A boolean field's `false` group. */
	no: string;
}

/**
 * Bucket items by the grouped field, keyed by the header each group shows.
 *
 * A boolean field's groups read as the `yes`/`no` labels rather than the
 * stringified `true`/`false`. The header is only a display key: matching a group
 * to its server total goes through `serverGroupKey`, which reads the raw value.
 *
 * The rows arrive in the server's group order, so the buckets keep insertion
 * order: re-sorting here would desync the headers from the cumulative offsets
 * the server's totals index by.
 */
export function groupItems<T>(
	items: T[],
	fields: FieldConfig<T>[],
	groupFieldKey: string | null,
	labels: GroupLabels
): Map<string, T[]> {
	if (!groupFieldKey) return new Map();

	const field = fields.find((f) => getFieldKey(f) === groupFieldKey);
	if (!field) return new Map();

	const groups = new Map<string, T[]>();

	items.forEach((item) => {
		const value = field.getGroupLabel ? field.getGroupLabel(item) : getFieldValue(item, field);
		const groupKey = groupLabel(value, field, labels);

		if (!groups.has(groupKey)) {
			groups.set(groupKey, []);
		}
		groups.get(groupKey)!.push(item);
	});

	return groups;
}

/**
 * A row of a tree group. `depth` is how many sections (counting the group itself when its top row
 * heads it) the row sits inside: it is indented one step per level and draws one guide line per
 * level, so each section's line runs from just under its header down to its last row.
 */
export interface TreeRow<T> {
	type: 'row';
	item: T;
	depth: number;
}

/**
 * A row that has children, drawn as itself with a chevron that collapses them; its children
 * follow one level in. It is not a separate heading: a heading with the row repeated under it
 * read as the row nesting under itself (a VM shown as virtualized by itself). `key` is the path
 * from the group down to this row, so two rows with the same label never share collapse state.
 */
export interface TreeSection<T> {
	type: 'section';
	key: string;
	/** The row with children. */
	item: T;
	/** Rows below it in the loaded subtree, itself excluded. */
	count: number;
	/** Its own indent level; its children sit one level in. */
	depth: number;
	/** Its children only. */
	entries: TreeEntry<T>[];
}

/**
 * The head of a tree whose root row is on another page: names the root and stands in for its row,
 * so the page's rows of that tree nest under it as they would under the root. It is not a row of
 * the page, so it holds no item.
 */
export interface TreeContinuation<T> {
	type: 'continued';
	key: string;
	/** The root's name: the header its group would have had. */
	label: string;
	/** How much of the tree this page shows. */
	range: GroupSlice | null;
	/** The page's rows of the tree, one level in. */
	entries: TreeEntry<T>[];
}

export type TreeEntry<T> = TreeRow<T> | TreeSection<T> | TreeContinuation<T>;

/**
 * What a tree row's first cell shows besides its value: the section when the row has children
 * (its chevron and count), else a spacer; and, for a root in the merged group of trees, its own
 * page range in place of the count.
 */
export interface TreeCell<T> {
	section: TreeSection<T> | null;
	range: GroupSlice | null;
}

/** A group as the table draws it. */
export interface RenderGroup<T> {
	/** Collapse-state key, unique per group. */
	key: string;
	/** The header. */
	name: string;
	/** Every row of the group, in the order drawn. */
	items: T[];
	range: GroupSlice | null;
	/** The group's nested sections when the grouping is a tree, else null. */
	entries: TreeEntry<T>[] | null;
	/**
	 * For the merged group of trees (see `mergeRootedTrees`): each root's own page range, by the
	 * root row's key, shown on that row in place of its count. Null otherwise.
	 */
	rootRanges: Map<string, GroupSlice> | null;
}

/** Collapse key of the merged group of trees; NUL keeps it apart from every group label. */
export const ROOTED_TREES_KEY = `${String.fromCharCode(0)}rooted-trees`;

/**
 * Collect every group that is one tree into a single group headed `tree.rootsLabel()`, placed where
 * the first of them was.
 *
 * Each such group is otherwise headed by its own root's name, with that root's row repeated
 * under it, which reads as the root nesting under itself. Merged, the roots are ordinary rows
 * carrying their own chevrons, and the shared header sets them apart from the rows outside any
 * tree ("Virtualized" beside "Not Virtualized"). A root's own page range moves onto its row.
 *
 * A tree whose root row is on another page joins too, under a continuation entry that names the
 * root and carries the tree's page range, so its rows nest as they would under the root.
 */
export function mergeRootedTrees<T>(
	groups: RenderGroup<T>[],
	tree: TreeConfig<T>
): RenderGroup<T>[] {
	const rootOf = (group: RenderGroup<T>) =>
		group.entries !== null && group.items.length > 0 ? tree.rootKey(group.items[0]) : null;
	const isRooted = (group: RenderGroup<T>) => rootOf(group) !== null;
	const rooted = groups.filter(isRooted);
	if (rooted.length === 0) return groups;

	const rootRanges = new Map<string, GroupSlice>();
	const entries = rooted.flatMap((group): TreeEntry<T>[] => {
		const root = rootOf(group)!;
		if (group.items.some((item) => tree.key(item) === root)) {
			if (group.range) rootRanges.set(root, group.range);
			return group.entries!;
		}
		return [
			{
				type: 'continued',
				key: group.key + PATH_SEPARATOR + root,
				label: group.name,
				range: group.range,
				entries: group.entries!.map((entry) => deeper(entry))
			}
		];
	});
	const merged: RenderGroup<T> = {
		key: ROOTED_TREES_KEY,
		name: tree.rootsLabel(),
		items: rooted.flatMap((group) => group.items),
		range: null,
		entries,
		rootRanges
	};

	const at = groups.findIndex(isRooted);
	const rest = groups.filter((group) => !isRooted(group));
	return [...rest.slice(0, at), merged, ...rest.slice(at)];
}

/** An entry and everything under it, one level further in. */
function deeper<T>(entry: TreeEntry<T>): TreeEntry<T> {
	if (entry.type === 'row') return { ...entry, depth: entry.depth + 1 };
	if (entry.type === 'section') {
		return { ...entry, depth: entry.depth + 1, entries: entry.entries.map((e) => deeper(e)) };
	}
	return { ...entry, entries: entry.entries.map((e) => deeper(e)) };
}

interface TreeNode<T> {
	item: T;
	children: TreeNode<T>[];
}

/** Separates the keys of a section path; NUL cannot occur in a group label or an id. */
const PATH_SEPARATOR = String.fromCharCode(0);

/**
 * One group's rows as the nested sections the tree `tree` describes.
 *
 * Every row with children becomes a section: the row itself, with its children one level in. The
 * rows keep the order they arrived in, which the server made parent-first across every page, and
 * nesting comes from `tree.depth`. A row whose ancestors are on an earlier page starts at the top
 * of the group.
 *
 * `groupKey` prefixes every section key, so the same row under two groupings keeps two states.
 */
export function buildTreeSections<T>(
	items: T[],
	tree: TreeConfig<T>,
	groupKey: string
): TreeEntry<T>[] {
	return forestFromDepths(items, tree).map((node) => toEntry(node, 0, groupKey, tree));
}

/** Every row of a group's entries, in the order they are drawn. */
export function flattenTreeEntries<T>(entries: TreeEntry<T>[]): T[] {
	return entries.flatMap((entry) => {
		if (entry.type === 'row') return [entry.item];
		if (entry.type === 'section') return [entry.item, ...flattenTreeEntries(entry.entries)];
		return flattenTreeEntries(entry.entries);
	});
}

function toEntry<T>(
	node: TreeNode<T>,
	depth: number,
	parentPath: string,
	tree: TreeConfig<T>
): TreeEntry<T> {
	if (node.children.length === 0) return { type: 'row', item: node.item, depth };

	const key = parentPath + PATH_SEPARATOR + tree.key(node.item);
	return {
		type: 'section',
		key,
		item: node.item,
		count: descendantCount(node),
		depth,
		entries: node.children.map((child) => toEntry(child, depth + 1, key, tree))
	};
}

function descendantCount<T>(node: TreeNode<T>): number {
	return node.children.reduce((sum, child) => sum + 1 + descendantCount(child), 0);
}

function forestFromDepths<T>(items: T[], tree: TreeConfig<T>): TreeNode<T>[] {
	const roots: TreeNode<T>[] = [];
	// Open ancestors, deepest last. A row nests under the nearest one shallower than itself.
	const stack: { depth: number; node: TreeNode<T> }[] = [];
	for (const item of items) {
		const depth = tree.depth(item);
		while (stack.length > 0 && stack[stack.length - 1].depth >= depth) stack.pop();
		const node: TreeNode<T> = { item, children: [] };
		if (stack.length > 0) stack[stack.length - 1].node.children.push(node);
		else roots.push(node);
		stack.push({ depth, node });
	}
	return roots;
}

function groupLabel<T>(value: FieldValue, field: FieldConfig<T>, labels: GroupLabels): string {
	if (value === null || value === undefined) return labels.ungrouped;
	if (field.type === 'boolean') return value ? labels.yes : labels.no;
	return String(value);
}

/**
 * Where each group starts in the full ordered result set.
 *
 * The server returns groups in the same order it orders rows, so a running sum
 * of the counts gives every group's global offset — which is what turns "this
 * page holds rows 100-199" into "this is rows 1-40 of that group".
 */
export function computeGroupOffsets(counts: ServerGroupCount[] | null): Map<string, GroupPosition> {
	const offsets = new Map<string, GroupPosition>();
	let cursor = 0;

	for (const group of counts ?? []) {
		offsets.set(group.value ?? UNGROUPED_KEY, { start: cursor, count: group.count });
		cursor += group.count;
	}

	return offsets;
}

/**
 * The value the server grouped these rows under, which is not always what the
 * header displays — a site group reads as a name but groups by id.
 */
export function serverGroupKey<T>(
	groupItems: T[],
	fields: FieldConfig<T>[],
	groupFieldKey: string | null
): string {
	const field = fields.find((f) => getFieldKey(f) === groupFieldKey);
	if (!field || groupItems.length === 0) return UNGROUPED_KEY;

	const raw = field.getGroupValue
		? field.getGroupValue(groupItems[0])
		: getFieldValue(groupItems[0], field);

	return raw === null || raw === undefined ? UNGROUPED_KEY : String(raw);
}
