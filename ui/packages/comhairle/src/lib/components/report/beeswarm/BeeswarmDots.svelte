<!--
	@component SVG Beeswarm dots — adapted from LayerCake's Beeswarm example.
	Uses the dodge algorithm from the LayerCake docs. Receives hover/select state from parent.
-->
<script lang="ts">
	import { getContext } from 'svelte';
	import { ascending } from 'd3-array';
	import type { ReportComment } from '$lib/types/report';

	const { data, xGet, height } = getContext('LayerCake');

	interface Props {
		radius?: number;
		spacing?: number;
		strokeWidth?: number;
		stroke?: string;
		hoveredTid?: number | null;
		selectedTid?: number | null;
		onmouseenter?: (e: MouseEvent, comment: ReportComment) => void;
		onmouseleave?: () => void;
		onclick?: (comment: ReportComment) => void;
		onfocus?: (e: FocusEvent, comment: ReportComment) => void;
		onblur?: () => void;
	}

	let {
		radius = 4,
		spacing = 1.5,
		strokeWidth = 0,
		stroke = 'var(--background)',
		hoveredTid = null,
		selectedTid = null,
		onmouseenter,
		onmouseleave,
		onclick,
		onfocus,
		onblur
	}: Props = $props();

	interface DodgedCircle {
		x: number;
		y: number;
		data: ReportComment;
		next: DodgedCircle | null;
	}

	function dodge(
		items: ReportComment[],
		opts: { rds: number; x: (d: ReportComment) => number }
	): DodgedCircle[] {
		const { rds, x } = opts;
		const radius2 = rds ** 2;
		const circles: DodgedCircle[] = items
			.map((d) => ({ x: x(d), y: 0, data: d, next: null }))
			.sort((a, b) => ascending(a.x, b.x));

		const epsilon = 1e-3;
		let head: DodgedCircle | null = null;
		let tail: DodgedCircle | null = null;

		function intersects(px: number, py: number): boolean {
			let a = head;
			while (a) {
				if (radius2 - epsilon > (a.x - px) ** 2 + (a.y - py) ** 2) {
					return true;
				}
				a = a.next;
			}
			return false;
		}

		for (const b of circles) {
			while (head && head.x < b.x - radius2) head = head.next;

			if (intersects(b.x, (b.y = 0))) {
				let a = head;
				b.y = Infinity;
				do {
					if (a) {
						const dy = Math.sqrt(radius2 - (a.x - b.x) ** 2);
						const candidatePos = a.y + dy;
						const candidateNeg = a.y - dy;
						if (
							Math.abs(candidatePos) < Math.abs(b.y) &&
							!intersects(b.x, candidatePos)
						)
							b.y = candidatePos;
						if (
							Math.abs(candidateNeg) < Math.abs(b.y) &&
							!intersects(b.x, candidateNeg)
						)
							b.y = candidateNeg;
						a = a.next;
					}
				} while (a);
			}

			b.next = null;
			if (head === null) {
				head = tail = b;
			} else {
				tail!.next = b;
				tail = b;
			}
		}

		return circles;
	}

	let circles = $derived(
		dodge($data as ReportComment[], { rds: radius * 2 + spacing + strokeWidth, x: $xGet })
	);
</script>

<g class="bee-group">
	{#each circles as d (d.data.tid)}
		{@const isHovered = hoveredTid === d.data.tid}
		{@const isSelected = selectedTid === d.data.tid}
		<circle
			role="button"
			aria-label={d.data.text.trim()}
			tabindex="0"
			cx={d.x}
			cy={$height / 2 - d.y}
			r={radius}
			fill={isHovered || isSelected ? 'var(--chart-1)' : 'var(--beeswarm-dot)'}
			stroke={isHovered || isSelected ? 'var(--primary)' : stroke}
			stroke-width={isHovered || isSelected ? 2 : strokeWidth}
			opacity={hoveredTid != null && !isHovered ? 0.3 : 1}
			class="cursor-pointer transition-opacity duration-150"
			onmouseenter={(e) => onmouseenter?.(e, d.data)}
			onmouseleave={() => onmouseleave?.()}
			onclick={() => onclick?.(d.data)}
			onfocus={(e) => onfocus?.(e, d.data)}
			onblur={() => onblur?.()}
			onkeydown={(e) => {
				if (e.key === 'Enter' || e.key === ' ') {
					e.preventDefault();
					onclick?.(d.data);
				}
			}}
		/>
	{/each}
</g>
