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
		r?: number;
		spacing?: number;
		strokeWidth?: number;
		stroke?: string;
		hoveredTid?: number | null;
		selectedTid?: number | null;
		onmouseenter?: (e: MouseEvent, comment: ReportComment) => void;
		onmouseleave?: () => void;
		onclick?: (comment: ReportComment) => void;
	}

	let {
		r = 4,
		spacing = 1.5,
		strokeWidth = 0,
		stroke = '#fff',
		hoveredTid = null,
		selectedTid = null,
		onmouseenter,
		onmouseleave,
		onclick
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
						const candidateY = a.y + Math.sqrt(radius2 - (a.x - b.x) ** 2);
						if (candidateY < b.y && !intersects(b.x, candidateY)) b.y = candidateY;
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
		dodge($data as ReportComment[], { rds: r * 2 + spacing + strokeWidth, x: $xGet })
	);
</script>

<g class="bee-group">
	{#each circles as d (d.data.tid)}
		{@const isHovered = hoveredTid === d.data.tid}
		{@const isSelected = selectedTid === d.data.tid}
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<circle
			role="img"
			aria-label={d.data.text.trim()}
			cx={d.x}
			cy={$height - r - spacing - strokeWidth / 2 - d.y}
			{r}
			fill={isHovered || isSelected ? '#374151' : '#1f2937'}
			stroke={isHovered || isSelected ? '#feca3a' : stroke}
			stroke-width={isHovered || isSelected ? 2 : strokeWidth}
			opacity={hoveredTid != null && !isHovered ? 0.3 : 1}
			class="cursor-pointer transition-opacity duration-150"
			onmouseenter={(e) => onmouseenter?.(e, d.data)}
			onmouseleave={() => onmouseleave?.()}
			onclick={() => onclick?.(d.data)}
		/>
	{/each}
</g>
