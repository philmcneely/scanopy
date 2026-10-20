import { createRawSnippet, mount, unmount } from 'svelte';
import Popover from '$lib/shared/components/data/Popover.svelte';

/**
 * Svelte action that shows a tooltip on hover.
 * Shows `content` when given, otherwise the plain text of the element's data-tooltip attribute.
 * Rendered in a `Popover`, so it places, sizes and looks the same as every other popover.
 *
 * `anchorClosest` places the tooltip against the nearest ancestor matching that selector instead
 * of the node itself, so a tooltip on part of a card clears the whole card.
 */

/** One labelled fact: a muted label, an emphasized value, and the value's state colour if any. */
export interface TooltipRow {
	label: string;
	value: string;
	/** CSS colour of a dot drawn before the value: the same colour the app paints that state. */
	dot?: string;
}

/** A block of a structured tooltip, set off from the one above it by a rule. */
export interface TooltipSection {
	heading?: string;
	/** A muted sentence or two: a description, or why something is the way it is. */
	text?: string;
	rows?: TooltipRow[];
	items?: string[];
}

/** A tooltip with structure: an optional title, then its sections in order. */
export interface TooltipContent {
	title?: string;
	sections: TooltipSection[];
}

/** A tooltip's text as one name: plain text as it is, structured content by its title. */
export function tooltipLabel(content: string | TooltipContent | null | undefined): string {
	return typeof content === 'string' ? content : (content?.title ?? '');
}

export interface TooltipOptions {
	content?: TooltipContent | null;
	anchorClosest?: string;
}

/** The part of `Document` the renderer uses, so it renders the same into any document. */
type TooltipDocument = Pick<Document, 'createElement'>;

function el(
	doc: TooltipDocument,
	tag: string,
	style: Partial<CSSStyleDeclaration>,
	text?: string
): HTMLElement {
	const e = doc.createElement(tag);
	Object.assign(e.style, style);
	if (text !== undefined) e.textContent = text;
	return e;
}

/**
 * The tooltip's body. Every string, which may come from a scanned device or a user (a hostname, a
 * service or tag name), is set through `textContent`, so markup in it renders as text.
 */
export function renderTooltip(doc: TooltipDocument, content: string | TooltipContent): HTMLElement {
	if (typeof content === 'string') {
		// Keeps the line breaks of multi-line text.
		return el(doc, 'div', { whiteSpace: 'pre-line' }, content);
	}

	const root = el(doc, 'div', { display: 'flex', flexDirection: 'column', gap: '6px' });
	if (content.title) {
		root.appendChild(
			el(
				doc,
				'div',
				{ color: 'var(--color-text-primary)', fontWeight: '600', fontSize: '13px' },
				content.title
			)
		);
	}
	content.sections.forEach((section, i) => {
		const block = el(doc, 'div', { display: 'flex', flexDirection: 'column', gap: '4px' });
		if (i > 0 || content.title) {
			Object.assign(block.style, {
				borderTop: '1px solid var(--color-border)',
				paddingTop: '6px'
			});
		}
		if (section.heading) {
			block.appendChild(
				el(
					doc,
					'div',
					{
						color: 'var(--color-text-tertiary)',
						fontSize: '10px',
						fontWeight: '600',
						letterSpacing: '0.05em',
						textTransform: 'uppercase'
					},
					section.heading
				)
			);
		}
		if (section.text) {
			block.appendChild(el(doc, 'div', { color: 'var(--color-text-secondary)' }, section.text));
		}
		if (section.rows?.length) {
			const grid = el(doc, 'div', {
				display: 'grid',
				gridTemplateColumns: 'auto 1fr',
				columnGap: '12px',
				rowGap: '2px'
			});
			for (const row of section.rows) {
				grid.appendChild(el(doc, 'div', { color: 'var(--color-text-tertiary)' }, row.label));
				const value = el(doc, 'div', {
					color: 'var(--color-text-primary)',
					fontWeight: '500',
					display: 'flex',
					alignItems: 'center',
					gap: '5px',
					minWidth: '0'
				});
				if (row.dot) {
					value.appendChild(
						el(doc, 'span', {
							width: '8px',
							height: '8px',
							flexShrink: '0',
							borderRadius: '9999px',
							background: row.dot
						})
					);
				}
				value.appendChild(el(doc, 'span', { overflowWrap: 'anywhere' }, row.value));
				grid.appendChild(value);
			}
			block.appendChild(grid);
		}
		if (section.items?.length) {
			const list = el(doc, 'ul', {
				margin: '0',
				paddingLeft: '14px',
				listStyleType: 'disc',
				color: 'var(--color-text-primary)'
			});
			for (const item of section.items) list.appendChild(el(doc, 'li', {}, item));
			block.appendChild(list);
		}
		root.appendChild(block);
	});
	return root;
}

export function tooltip(node: HTMLElement, options: TooltipOptions = {}) {
	let host: HTMLDivElement | null = null;
	let popover: ReturnType<typeof mount> | null = null;

	function show() {
		const content = options.content ?? node.getAttribute('data-tooltip');
		if (!content) return;

		const anchor =
			(options.anchorClosest && node.closest<HTMLElement>(options.anchorClosest)) || node;
		const body = createRawSnippet(() => ({
			render: () => '<div class="text-secondary break-words text-xs leading-tight"></div>',
			setup(el: Element) {
				el.appendChild(renderTooltip(document, content));
			}
		}));

		host = document.createElement('div');
		document.body.appendChild(host);
		popover = mount(Popover, {
			target: host,
			props: {
				triggerElement: anchor,
				isOpen: true,
				role: 'tooltip',
				prefer: 'above',
				minWidth: '0',
				maxWidth: options.content ? '300px' : '250px',
				onClose: hide,
				children: body
			}
		});
	}

	function hide() {
		if (popover) unmount(popover);
		popover = null;
		host?.remove();
		host = null;
	}

	node.addEventListener('mouseenter', show);
	node.addEventListener('mouseleave', hide);

	return {
		update(next: TooltipOptions = {}) {
			options = next;
			// A tooltip open while its content changes redraws with the new content.
			if (popover) {
				hide();
				show();
			}
		},
		destroy() {
			node.removeEventListener('mouseenter', show);
			node.removeEventListener('mouseleave', hide);
			hide();
		}
	};
}
