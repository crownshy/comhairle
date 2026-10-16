import { BellRing, Flag, Mail, QrCode, Rocket, Users, type Icon } from 'lucide-svelte';
import type { ActivityKind } from './sampleMonitorData';

/**
 * How each kind of logged action looks on the signups chart: marker line, label, key
 * and the action cards in its panel.
 */
export type ActionKindStyle = {
	icon: typeof Icon;
	label: string;
	/** Solid fill, e.g. the swatch in the chart key. */
	dot: string;
	/** Light tile with matching text, e.g. the icon in the log. */
	tile: string;
	/** Chart marker line (CSS colour, set inline so chart styles can't override it) and its label. */
	lineColor: string;
	fill: string;
};

export const ACTION_KINDS: Record<ActivityKind, ActionKindStyle> = {
	launch: {
		icon: Rocket,
		label: 'Launch',
		dot: 'bg-green-600',
		tile: 'bg-green-50 text-green-800',
		lineColor: 'var(--color-green-600)',
		fill: 'fill-green-700'
	},
	email: {
		icon: Mail,
		label: 'Email invite',
		dot: 'bg-primary',
		tile: 'bg-accent text-primary',
		lineColor: 'var(--primary)',
		fill: 'fill-primary'
	},
	qr: {
		icon: QrCode,
		label: 'QR codes',
		dot: 'bg-violet-600',
		tile: 'bg-violet-50 text-violet-800',
		lineColor: 'var(--color-violet-600)',
		fill: 'fill-violet-700'
	},
	event: {
		icon: Users,
		label: 'Event',
		dot: 'bg-amber-500',
		tile: 'bg-amber-50 text-amber-900',
		lineColor: 'var(--color-amber-500)',
		fill: 'fill-amber-700'
	},
	reminder: {
		icon: BellRing,
		label: 'Reminder',
		dot: 'bg-sky-600',
		tile: 'bg-sky-50 text-sky-800',
		lineColor: 'var(--color-sky-600)',
		fill: 'fill-sky-700'
	},
	close: {
		icon: Flag,
		label: 'Close',
		dot: 'bg-foreground',
		tile: 'bg-muted text-foreground',
		lineColor: 'var(--foreground)',
		fill: 'fill-foreground'
	}
};

/** "Wed 30 Sept": how action dates are written in the chart panel. */
export function formatActionDay(date: Date): string {
	return date.toLocaleDateString('en-GB', {
		weekday: 'short',
		day: 'numeric',
		month: 'short',
		timeZone: 'UTC'
	});
}
