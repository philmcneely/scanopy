/**
 * Put `text` on the clipboard, through `navigator.clipboard` where the page is a secure context
 * and a selection copy elsewhere. Throws when the browser refuses both.
 */
export async function copyText(text: string): Promise<void> {
	if (window.isSecureContext && navigator.clipboard) {
		await navigator.clipboard.writeText(text);
	} else if (!copyViaSelection(text)) {
		throw new Error('the browser refused the copy');
	}
}

/**
 * Copy text by selecting it in an off-screen field. `navigator.clipboard` is gated to secure
 * contexts and Scanopy supports plain-HTTP self-hosts, so this is the path those pages use.
 */
export function copyViaSelection(text: string): boolean {
	const field = document.createElement('textarea');
	field.value = text;
	// Off-screen rather than `display: none`: a hidden field cannot be selected. Not `readonly`:
	// iOS Safari won't select text in a read-only field. 16px stops iOS zooming in on focus.
	field.style.position = 'fixed';
	field.style.opacity = '0';
	field.style.fontSize = '16px';
	document.body.appendChild(field);
	try {
		field.select();
		field.setSelectionRange(0, text.length);
		return document.execCommand('copy');
	} finally {
		field.remove();
	}
}
