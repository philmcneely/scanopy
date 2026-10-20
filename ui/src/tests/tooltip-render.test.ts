import { describe, it, expect } from 'vitest';
import { renderTooltip, type TooltipContent } from '$lib/shared/actions/tooltip';
import { hostTooltip, serviceTooltip } from '$lib/features/topology/element-render-data';
import type { Host } from '$lib/features/hosts/types/base';
import type { Service } from '$lib/features/services/types/base';
import type { MarkValue } from '$lib/features/topology/element-marks';
import { createColorHelper } from '$lib/shared/utils/styling';

/**
 * The renderer builds the tooltip from strings that come from scanned devices and users. It must
 * set them as text, never parse them as markup. The UI tests run without a DOM, so this fake
 * records what the renderer builds and refuses `innerHTML`.
 */
class FakeElement {
	children: FakeElement[] = [];
	style: Record<string, string> = {};
	private text = '';
	constructor(readonly tagName: string) {}
	get textContent(): string {
		return this.text + this.children.map((c) => c.textContent).join('');
	}
	set textContent(value: string) {
		this.text = value;
	}
	set innerHTML(_: string) {
		throw new Error('the tooltip renderer must not parse markup');
	}
	appendChild(child: FakeElement) {
		this.children.push(child);
		return child;
	}
	/** Every element at or below this one. */
	all(): FakeElement[] {
		return [this, ...this.children.flatMap((c) => c.all())];
	}
	/** The leaves' own text, in order: what reads on screen, one entry per element. */
	texts(): string[] {
		return this.all()
			.filter((e) => e.children.length === 0 && e.text)
			.map((e) => e.text);
	}
}

const fakeDocument = {
	createElement: (tag: string) => new FakeElement(tag)
} as unknown as Pick<Document, 'createElement'>;

function render(content: string | TooltipContent): FakeElement {
	return renderTooltip(fakeDocument, content) as unknown as FakeElement;
}

describe('renderTooltip', () => {
	it('renders a plain string as one text block that keeps its line breaks', () => {
		const tip = render('Ubuntu 22.04\nKernel 6.5');
		expect(tip.children).toHaveLength(0);
		expect(tip.textContent).toBe('Ubuntu 22.04\nKernel 6.5');
		expect(tip.style.whiteSpace).toBe('pre-line');
	});

	it('renders the title, rows, headed list and note in order', () => {
		const tip = render({
			title: 'uptime-kuma',
			sections: [
				{ rows: [{ label: 'Manager', value: 'Proxmox' }] },
				{ heading: 'Services', items: ['Uptime Kuma', 'SSH'] },
				{ text: 'Reported by the daemon.' }
			]
		});
		expect(tip.texts()).toEqual([
			'uptime-kuma',
			'Manager',
			'Proxmox',
			'Services',
			'Uptime Kuma',
			'SSH',
			'Reported by the daemon.'
		]);
		expect(tip.all().filter((e) => e.tagName === 'li')).toHaveLength(2);
	});

	it('draws a dot in the row colour before the value', () => {
		const tip = render({
			sections: [{ rows: [{ label: 'Status', value: 'Running', dot: 'rgb(34, 197, 94)' }] }]
		});
		const dots = tip.all().filter((e) => e.style.background === 'rgb(34, 197, 94)');
		expect(dots).toHaveLength(1);
	});

	it('renders markup in a value as text', () => {
		const hostile = '<img src=x onerror=alert(1)>';
		const tip = render({
			title: hostile,
			sections: [
				{ heading: hostile, text: hostile, rows: [{ label: hostile, value: hostile }] },
				{ items: [hostile] }
			]
		});
		expect(tip.all().some((e) => e.tagName === 'img')).toBe(false);
		expect(tip.texts().every((t) => t === hostile)).toBe(true);
		expect(tip.texts()).toHaveLength(6);
	});
});

describe('icon card tooltip content', () => {
	const running: MarkValue = { id: 'Running', label: 'Running', color: 'Green' } as MarkValue;

	const guest = {
		id: 'h1',
		name: 'uptime-kuma',
		display_name: 'uptime-kuma',
		virtualization_metadata: {
			type: 'Proxmox',
			details: { guest_type: 'Lxc', state: 'Running' }
		}
	} as unknown as Host;
	const service = { id: 's1', name: 'Uptime Kuma' } as unknown as Service;

	it('titles a guest by its name, colours its run state, and lists its services apart', () => {
		const content = hostTooltip(guest, {
			deviceType: 'Server',
			services: [service],
			status: running
		});
		expect(content.title).toBe('uptime-kuma');
		const [facts, services] = content.sections;
		const status = facts.rows?.find((r) => r.value === 'Running');
		expect(status?.dot).toBe(createColorHelper('Green').rgb);
		expect(services.heading).toBeTruthy();
		expect(services.items).toEqual(['Uptime Kuma']);
	});

	it('drops facts with no value, and the status dot when the view paints no state', () => {
		const content = hostTooltip(guest, { address: null, services: [] });
		const rows = content.sections.flatMap((s) => s.rows ?? []);
		expect(rows.every((r) => r.value)).toBe(true);
		expect(rows.some((r) => r.dot)).toBe(false);
		// No services: no empty list under a heading.
		expect(content.sections.some((s) => s.items)).toBe(false);
	});

	it('gives a plain service with no host its name alone', () => {
		expect(serviceTooltip(service)).toEqual({ title: 'Uptime Kuma', sections: [] });
	});
});
