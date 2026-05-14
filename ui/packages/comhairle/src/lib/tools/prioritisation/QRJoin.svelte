<script lang="ts">
	import type { PrioritisationStore } from './store.svelte';
	import { Button } from '$lib/components/ui/button';

	let { store }: { store: PrioritisationStore } = $props();

	/**
	 * Tiny SVG QR-ish placeholder. We deliberately avoid pulling in a real QR
	 * library for the prototype — the URL is decorative (no real public join
	 * domain yet, see documentation/prioritisation-tool-deferred.md).
	 */
	const fakeUrl = $derived(`https://poll.example/${store.poll.joinCode}`);

	// Cheap deterministic pixel grid keyed off the join code so it visually
	// "looks like" a QR code without encoding anything meaningful.
	function gridFor(code: string): boolean[][] {
		const seed = [...code].reduce((a, c) => a * 31 + c.charCodeAt(0), 7);
		const rng = (i: number) => {
			const x = Math.sin(seed + i) * 10000;
			return x - Math.floor(x);
		};
		const N = 21;
		const grid: boolean[][] = [];
		for (let r = 0; r < N; r++) {
			const row: boolean[] = [];
			for (let c = 0; c < N; c++) row.push(rng(r * N + c) > 0.5);
			grid.push(row);
		}
		// finder squares
		const stamp = (r0: number, c0: number) => {
			for (let r = 0; r < 7; r++) {
				for (let c = 0; c < 7; c++) {
					const onEdge = r === 0 || r === 6 || c === 0 || c === 6;
					const innerSquare = r >= 2 && r <= 4 && c >= 2 && c <= 4;
					grid[r0 + r][c0 + c] = onEdge || innerSquare;
				}
			}
		};
		stamp(0, 0);
		stamp(0, 14);
		stamp(14, 0);
		return grid;
	}

	let grid = $derived(gridFor(store.poll.joinCode));

	function openProjector() {
		const w = window.open('', '_blank', 'width=900,height=900');
		if (!w) return;
		w.document.title = `Join: ${store.poll.title}`;
		w.document.body.innerHTML = `
			<style>
				html, body { margin:0; padding:0; height:100%; background:#fff;
					display:flex; align-items:center; justify-content:center;
					font-family: system-ui, sans-serif; }
				.wrap { text-align:center; padding:32px; }
				h1 { font-size:48px; margin:0 0 24px; }
				.code { font-size:96px; font-family:monospace; letter-spacing:8px; }
				.url { color:#666; margin-top:16px; font-size:20px; }
			</style>
			<div class="wrap">
				<h1>${store.poll.title || 'Join the poll'}</h1>
				${document.getElementById('prio-qr-svg')?.outerHTML ?? ''}
				<div class="code">${store.poll.joinCode}</div>
				<div class="url">${fakeUrl}</div>
			</div>
		`;
	}
</script>

<div class="flex flex-col items-center gap-4 p-6">
	<h2 class="text-xl font-semibold">{store.poll.title || 'Untitled poll'}</h2>
	<svg
		id="prio-qr-svg"
		viewBox="0 0 21 21"
		width="320"
		height="320"
		xmlns="http://www.w3.org/2000/svg"
		shape-rendering="crispEdges"
		class="rounded-md border bg-white"
	>
		{#each grid as row, r (r)}
			{#each row as cell, c (c)}
				{#if cell}
					<rect x={c} y={r} width="1" height="1" fill="black" />
				{/if}
			{/each}
		{/each}
	</svg>
	<div class="text-center">
		<div class="text-muted-foreground text-sm">Join code</div>
		<div class="font-mono text-5xl tracking-widest">{store.poll.joinCode}</div>
	</div>
	<div class="text-muted-foreground text-xs">{fakeUrl}</div>
	<Button variant="outline" onclick={openProjector}>Open in projector window</Button>
	<p class="text-muted-foreground max-w-md text-center text-xs">
		Note: the prototype's QR code is decorative. A real public join domain is deferred — see <code
			>documentation/prioritisation-tool-deferred.md</code
		>.
	</p>
</div>
