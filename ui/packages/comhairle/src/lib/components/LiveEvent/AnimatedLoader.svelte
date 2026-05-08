<script lang="ts">
	interface Props {
		size?: number;
	}
	let { size = 48 }: Props = $props();
	let cell = $derived(size / 3);
	let pad = $derived(size / 6);
	let x1 = $derived(pad);
	let y1 = $derived(pad);
	let x2 = $derived(pad + cell);
	let y2 = $derived(pad + cell);
	let path = $derived(`M${x1},${y1} L${x2},${y1} L${x2},${y2} L${x1},${y2} Z`);
</script>

<div
	class="loader-animation relative"
	style="width: {size}px; height: {size}px;"
	aria-hidden="true"
>
	<span
		class="trail tl bg-primary/40"
		style="width:{cell}px;height:{cell}px;left:{x1}px;top:{y1}px;"
	></span>
	<span
		class="trail tr bg-primary/40"
		style="width:{cell}px;height:{cell}px;left:{x2}px;top:{y1}px;"
	></span>
	<span
		class="trail br bg-primary/40"
		style="width:{cell}px;height:{cell}px;left:{x2}px;top:{y2}px;"
	></span>
	<span
		class="trail bl bg-primary/40"
		style="width:{cell}px;height:{cell}px;left:{x1}px;top:{y2}px;"
	></span>

	<span
		class="head bg-primary"
		style="width:{cell}px;height:{cell}px;offset-path: path('{path}');"
	></span>
</div>

<style>
	/*
	 * Two-lap cycle: head walks the square twice per full cycle.
	 * Lap 1 (0%–50%): head paints a light trail at each corner it leaves.
	 * Lap 2 (50%–100%): head eats the trail at each corner it passes.
	 * Each trail uses the same keyframes with a delay matching when the
	 * head first reaches that corner.
	 */
	.trail {
		position: absolute;
		opacity: 0;
		animation: paint-eat 5.6s linear infinite;
	}

	.tl {
		animation-delay: 0s;
	}
	.tr {
		animation-delay: 0.7s;
	}
	.br {
		animation-delay: 1.4s;
	}
	.bl {
		animation-delay: 2.1s;
	}

	@keyframes paint-eat {
		0% {
			opacity: 0;
		}
		3% {
			opacity: 1;
		}
		6% {
			opacity: 1;
		}
		50% {
			opacity: 1;
		}
		53% {
			opacity: 0.2;
		}
		56% {
			opacity: 0;
		}
		100% {
			opacity: 0;
		}
	}

	.head {
		position: absolute;
		top: 0;
		left: 0;
		offset-anchor: 0 0;
		offset-rotate: 0deg;
		animation: travel 2.8s linear infinite;
	}
	@keyframes travel {
		from {
			offset-distance: 0%;
		}
		to {
			offset-distance: 100%;
		}
	}
	@media (prefers-reduced-motion: reduce) {
		.trail,
		.head {
			animation: none;
		}
		.head {
			offset-distance: 0%;
		}
		.trail {
			opacity: 1;
		}
	}
</style>
