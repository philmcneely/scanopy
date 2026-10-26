import type { Component } from 'svelte';
import type { CircleQuestionMark } from 'lucide-svelte';

export type IconComponent =
	| typeof CircleQuestionMark
	| (($$payload: Record<string, unknown>, $$props: Record<string, unknown>) => unknown)
	// An icon of the app's own, such as `PaidCrown`: sized and classed like a lucide icon.
	| Component<{ size?: number; class?: string }>;
