/**
 * Svelte action that shows a fixed-position tooltip on hover.
 * Shows `content` when given, otherwise the plain text of the element's data-tooltip attribute.
 * Uses position:fixed and appends to document.body to escape overflow containers.
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
	let tip: HTMLDivElement | null = null;

	function show() {
		const content = options.content ?? node.getAttribute('data-tooltip');
		if (!content) return;

		tip = document.createElement('div');
		tip.appendChild(renderTooltip(document, content));
		Object.assign(tip.style, {
			position: 'fixed',
			zIndex: '9999',
			maxWidth: options.content ? '300px' : '250px',
			padding: '6px 10px',
			borderRadius: '6px',
			fontSize: '12px',
			lineHeight: '1.25',
			color: 'var(--color-text-secondary)',
			background: 'var(--color-bg-elevated)',
			border: '1px solid var(--color-border)',
			boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.3)',
			pointerEvents: 'none',
			wordWrap: 'break-word',
			transform: 'translateX(-50%) translateY(-100%)'
		});
		document.body.appendChild(tip);

		const anchor = (options.anchorClosest && node.closest(options.anchorClosest)) || node;
		const rect = anchor.getBoundingClientRect();
		const tipHeight = tip.offsetHeight;

		// Auto-detect: if tooltip would overflow above viewport, position below instead
		if (rect.top - tipHeight - 6 < 0) {
			tip.style.transform = 'translateX(-50%)';
			tip.style.top = `${rect.bottom + 6}px`;
		} else {
			tip.style.top = `${rect.top - 6}px`;
		}

		// The tooltip is centred on the node via translateX(-50%). Clamp that centre
		// so a wide tooltip on an edge (first/last) element stays fully in the
		// viewport instead of being clipped off the left/right edge.
		const margin = 6;
		const half = tip.offsetWidth / 2;
		const centre = rect.left + rect.width / 2;
		const clamped = Math.max(margin + half, Math.min(centre, window.innerWidth - margin - half));
		tip.style.left = `${clamped}px`;
	}

	function hide() {
		tip?.remove();
		tip = null;
	}

	node.addEventListener('mouseenter', show);
	node.addEventListener('mouseleave', hide);

	return {
		update(next: TooltipOptions = {}) {
			options = next;
			// A tooltip open while its content changes redraws with the new content.
			if (tip) {
				hide();
				show();
			}
		},
		destroy() {
			node.removeEventListener('mouseenter', show);
			node.removeEventListener('mouseleave', hide);
			tip?.remove();
		}
	};
}
