import { test, expect, type Page } from '@playwright/test';
import { mkdirSync, writeFileSync } from 'node:fs';
import {
	enablePerfInstrumentation,
	readPipelinePerf,
	signIn,
	waitForStableLayout
} from '../tests-support/topology-harness';

/**
 * Layout quality for every view of every topology in the signed-in organization.
 *
 * Scores what is drawn: node positions come from the node store (`__scanopyTopologyLayout`) and
 * edge geometry from the rendered SVG paths, sampled in flow coordinates, so a crossing counted
 * here is a crossing on screen.
 *
 * Metrics, per view:
 *  - inner edges (both endpoints inside one root container): crossings between them, how many pass
 *    through a card that is not an endpoint, and their total and longest drawn length;
 *  - dependency order: share of inner dependency edges whose target reads after its source (a
 *    lower row, or the same row to the right);
 *  - edges between root containers: total and longest drawn length;
 *  - root containers: overlaps, bounding box, and the share of it no container covers;
 *  - ELK time per layout run.
 *
 * Prerequisites:
 *   1. `npm run dev` (Vite) plus a running backend, with the demo organization seeded.
 *   2. SESSION_ID from a logged-in session (`POST /api/auth/login`).
 *
 * Run:
 *   SESSION_ID=<session> npx playwright test tests/topology-layout-eval.ts --project=chromium
 *
 * Writes `tests/screenshots/layout-eval.json` and one screenshot per view.
 */

const VIEWS = ['L3Logical', 'Application', 'Workloads', 'L2Physical'] as const;

/** Dependency edge types, whose direction is the dependency's member order. */
const DEPENDENCY_EDGE_TYPES = new Set(['RequestPath', 'HubAndSpoke']);

/** Same-row tolerance for reading order, in flow pixels. */
const ROW_TOLERANCE = 10;

/** Distance between path samples, in flow pixels. */
const SAMPLE_STEP = 6;

/**
 * Regression bounds, keyed by `<site> <view>`. Each sits between the baseline recorded before the
 * layout fixes and the result after them, so the eval fails when a fix stops working.
 */
const THRESHOLDS: Record<string, Partial<Record<keyof ViewMetrics, { max?: number; min?: number }>>> =
	{};

interface SnapshotNode {
	id: string;
	parentId: string | null;
	type: string | undefined;
	x: number;
	y: number;
	width: number;
	height: number;
	header: string | null;
}

interface SnapshotEdge {
	id: string;
	source: string;
	target: string;
	edgeType: string | null;
	affectsLayout: boolean;
	label: string | null;
}

type Point = { x: number; y: number };

interface Box {
	x: number;
	y: number;
	width: number;
	height: number;
}

interface ViewMetrics {
	nodes: number;
	rootContainers: number;
	rootOverlaps: number;
	innerEdges: number;
	innerCrossings: number;
	innerBehindCards: number;
	innerLengthTotal: number;
	innerLengthMax: number;
	dependencyEdges: number;
	dependencyInOrderPct: number;
	crossEdges: number;
	crossLengthTotal: number;
	crossLengthMax: number;
	boundsWidth: number;
	boundsHeight: number;
	emptyAreaPct: number;
	elkMsPerRun: number;
}

/** Read the node store and the drawn path of every visible edge, in flow coordinates. */
async function readLayout(
	page: Page
): Promise<{ nodes: SnapshotNode[]; edges: SnapshotEdge[]; paths: Record<string, Point[]> }> {
	return page.evaluate((step) => {
		const read = (
			window as unknown as {
				__scanopyTopologyLayout?: () => { nodes: SnapshotNode[]; edges: SnapshotEdge[] };
			}
		).__scanopyTopologyLayout;
		if (!read) throw new Error('__scanopyTopologyLayout is not installed');
		const { nodes, edges } = read();

		const paths: Record<string, Point[]> = {};
		for (const group of document.querySelectorAll<SVGGElement>('.svelte-flow__edge')) {
			const id = group.dataset.id ?? group.getAttribute('data-id');
			const path = group.querySelector<SVGPathElement>('path.svelte-flow__edge-path');
			if (!id || !path) continue;
			const style = getComputedStyle(path);
			if (style.display === 'none' || style.visibility === 'hidden' || style.opacity === '0') {
				continue;
			}
			const length = path.getTotalLength();
			if (!(length > 0)) continue;
			const samples = Math.min(400, Math.max(2, Math.ceil(length / step)));
			const points: Point[] = [];
			for (let i = 0; i <= samples; i++) {
				const p = path.getPointAtLength((length * i) / samples);
				points.push({ x: p.x, y: p.y });
			}
			paths[id] = points;
		}
		return { nodes, edges, paths };
	}, SAMPLE_STEP);
}

function polylineLength(points: Point[]): number {
	let total = 0;
	for (let i = 1; i < points.length; i++) {
		total += Math.hypot(points[i].x - points[i - 1].x, points[i].y - points[i - 1].y);
	}
	return total;
}

function segmentsCross(a: Point, b: Point, c: Point, d: Point): boolean {
	const cross = (p: Point, q: Point, r: Point) => (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x);
	const d1 = cross(c, d, a);
	const d2 = cross(c, d, b);
	const d3 = cross(a, b, c);
	const d4 = cross(a, b, d);
	return d1 * d2 < 0 && d3 * d4 < 0;
}

function polylinesCross(a: Point[], b: Point[]): boolean {
	for (let i = 1; i < a.length; i++) {
		for (let j = 1; j < b.length; j++) {
			if (segmentsCross(a[i - 1], a[i], b[j - 1], b[j])) return true;
		}
	}
	return false;
}

function insideBox(p: Point, box: Box, inset: number): boolean {
	return (
		p.x > box.x + inset &&
		p.x < box.x + box.width - inset &&
		p.y > box.y + inset &&
		p.y < box.y + box.height - inset
	);
}

function boxesOverlap(a: Box, b: Box): boolean {
	return !(
		a.x + a.width <= b.x ||
		b.x + b.width <= a.x ||
		a.y + a.height <= b.y ||
		b.y + b.height <= a.y
	);
}

function computeMetrics(
	nodes: SnapshotNode[],
	edges: SnapshotEdge[],
	paths: Record<string, Point[]>,
	elkMsPerRun: number
): ViewMetrics {
	const byId = new Map(nodes.map((n) => [n.id, n]));

	const absolute = new Map<string, Box>();
	const absoluteOf = (id: string): Box | undefined => {
		const cached = absolute.get(id);
		if (cached) return cached;
		const node = byId.get(id);
		if (!node) return undefined;
		const parent = node.parentId ? absoluteOf(node.parentId) : undefined;
		const box = {
			x: node.x + (parent?.x ?? 0),
			y: node.y + (parent?.y ?? 0),
			width: node.width,
			height: node.height
		};
		absolute.set(id, box);
		return box;
	};
	const rootOf = (id: string): string => {
		let node = byId.get(id);
		while (node?.parentId && byId.has(node.parentId)) node = byId.get(node.parentId);
		return node?.id ?? id;
	};
	const ancestors = (id: string): Set<string> => {
		const out = new Set<string>();
		let node = byId.get(id);
		while (node?.parentId) {
			out.add(node.parentId);
			node = byId.get(node.parentId);
		}
		return out;
	};

	const drawn = edges.filter((e) => paths[e.id] && byId.has(e.source) && byId.has(e.target));
	const inner = drawn.filter((e) => rootOf(e.source) === rootOf(e.target));
	const cross = drawn.filter((e) => rootOf(e.source) !== rootOf(e.target));

	// Crossings between inner edges of the same root container that share no endpoint.
	let innerCrossings = 0;
	const innerByRoot = new Map<string, SnapshotEdge[]>();
	for (const e of inner) {
		const root = rootOf(e.source);
		if (!innerByRoot.has(root)) innerByRoot.set(root, []);
		innerByRoot.get(root)!.push(e);
	}
	for (const group of innerByRoot.values()) {
		for (let i = 0; i < group.length; i++) {
			for (let j = i + 1; j < group.length; j++) {
				const a = group[i];
				const b = group[j];
				if (
					a.source === b.source ||
					a.source === b.target ||
					a.target === b.source ||
					a.target === b.target
				) {
					continue;
				}
				if (polylinesCross(paths[a.id], paths[b.id])) innerCrossings++;
			}
		}
	}

	// Inner edges whose drawn path enters a card that is neither endpoint nor an endpoint's ancestor.
	const elements = nodes.filter((n) => n.type === 'Element' && n.width > 0 && n.height > 0);
	let innerBehindCards = 0;
	for (const e of inner) {
		const skip = new Set([e.source, e.target, ...ancestors(e.source), ...ancestors(e.target)]);
		const root = rootOf(e.source);
		const points = paths[e.id];
		const hit = elements.some((card) => {
			if (skip.has(card.id) || rootOf(card.id) !== root) return false;
			const box = absoluteOf(card.id)!;
			return points.some((p) => insideBox(p, box, 2));
		});
		if (hit) innerBehindCards++;
	}

	const innerLengths = inner.map((e) => polylineLength(paths[e.id]));
	const crossLengths = cross.map((e) => polylineLength(paths[e.id]));

	const dependencyEdges = inner.filter((e) => e.edgeType && DEPENDENCY_EDGE_TYPES.has(e.edgeType));
	const inOrder = dependencyEdges.filter((e) => {
		const s = absoluteOf(e.source)!;
		const t = absoluteOf(e.target)!;
		if (t.y > s.y + ROW_TOLERANCE) return true;
		return Math.abs(t.y - s.y) <= ROW_TOLERANCE && t.x > s.x;
	});

	const roots = nodes.filter((n) => !n.parentId && n.type === 'Container');
	const rootBoxes = roots.map((r) => absoluteOf(r.id)!);
	let rootOverlaps = 0;
	for (let i = 0; i < rootBoxes.length; i++) {
		for (let j = i + 1; j < rootBoxes.length; j++) {
			if (boxesOverlap(rootBoxes[i], rootBoxes[j])) rootOverlaps++;
		}
	}
	const minX = Math.min(...rootBoxes.map((b) => b.x));
	const minY = Math.min(...rootBoxes.map((b) => b.y));
	const maxX = Math.max(...rootBoxes.map((b) => b.x + b.width));
	const maxY = Math.max(...rootBoxes.map((b) => b.y + b.height));
	const boundsArea = rootBoxes.length ? (maxX - minX) * (maxY - minY) : 0;
	const coveredArea = rootBoxes.reduce((sum, b) => sum + b.width * b.height, 0);

	const sum = (xs: number[]) => xs.reduce((a, b) => a + b, 0);
	const max = (xs: number[]) => (xs.length ? Math.max(...xs) : 0);

	return {
		nodes: nodes.length,
		rootContainers: roots.length,
		rootOverlaps,
		innerEdges: inner.length,
		innerCrossings,
		innerBehindCards,
		innerLengthTotal: Math.round(sum(innerLengths)),
		innerLengthMax: Math.round(max(innerLengths)),
		dependencyEdges: dependencyEdges.length,
		dependencyInOrderPct: dependencyEdges.length
			? Math.round((100 * inOrder.length) / dependencyEdges.length)
			: 100,
		crossEdges: cross.length,
		crossLengthTotal: Math.round(sum(crossLengths)),
		crossLengthMax: Math.round(max(crossLengths)),
		boundsWidth: rootBoxes.length ? Math.round(maxX - minX) : 0,
		boundsHeight: rootBoxes.length ? Math.round(maxY - minY) : 0,
		emptyAreaPct: boundsArea > 0 ? Math.round(100 * (1 - coveredArea / boundsArea)) : 0,
		elkMsPerRun: Math.round(elkMsPerRun)
	};
}

/** The topology page picks its site from this key, not from the URL. */
const SELECTED_SITE_KEY = 'scanopy_topology_selected_site_id';

test('topology layout quality evaluation', async ({ context }) => {
	test.setTimeout(600_000);
	await signIn(context);

	const sites = (await (await context.request.get('/api/v1/sites')).json()).data as {
		id: string;
		name: string;
	}[];
	expect(sites.length, 'no sites visible to this session').toBeGreaterThan(0);

	mkdirSync('tests/screenshots', { recursive: true });
	const results: Record<string, ViewMetrics> = {};
	const failures: string[] = [];

	for (const { id: siteId, name: site } of sites) {
		// A page per site, so each one's init script selects only that site.
		const page = await context.newPage();
		await page.setViewportSize({ width: 1920, height: 1200 });
		await enablePerfInstrumentation(page);
		// Every node has to be in the DOM to be scored, so culling stays off.
		await page.addInitScript(
			([key, value]) => {
				(window as unknown as { __topoNoCull: boolean }).__topoNoCull = true;
				localStorage.setItem(key, value);
			},
			[SELECTED_SITE_KEY, siteId]
		);
		for (const view of VIEWS) {
			const name = `${site} ${view}`;
			await page.goto(`/?view=${view}#topology`);
			await page.waitForSelector('.svelte-flow__node', { timeout: 60_000 });
			await waitForStableLayout(page);

			const perf = await readPipelinePerf(page);
			const elkRuns = perf.counts['layout.elk-compute'] ?? 0;
			const elkMs = elkRuns ? (perf.durations['layout.elk-compute'] ?? 0) / elkRuns : 0;

			const { nodes, edges, paths } = await readLayout(page);
			const metrics = computeMetrics(nodes, edges, paths, elkMs);
			results[name] = metrics;

			const file = name.replace(/\s+/g, '-').toLowerCase();
			await page.screenshot({ path: `tests/screenshots/layout-${file}.png` });

			if (metrics.rootOverlaps > 0) failures.push(`${name}: ${metrics.rootOverlaps} root overlaps`);
			for (const [key, bound] of Object.entries(THRESHOLDS[name] ?? {})) {
				const value = metrics[key as keyof ViewMetrics];
				if (bound?.max !== undefined && value > bound.max) {
					failures.push(`${name}: ${key} ${value} > ${bound.max}`);
				}
				if (bound?.min !== undefined && value < bound.min) {
					failures.push(`${name}: ${key} ${value} < ${bound.min}`);
				}
			}
		}
		await page.close();
	}

	console.log('\n=== Topology layout evaluation ===');
	console.table(results);
	writeFileSync('tests/screenshots/layout-eval.json', JSON.stringify(results, null, 2));

	expect(failures, failures.join('\n')).toEqual([]);
});
