<script lang="ts">
	import * as pdfjsLib from 'pdfjs-dist';
	import { TextLayer } from 'pdfjs-dist';
	import type { PDFDocumentProxy, PDFPageProxy } from 'pdfjs-dist';
	import 'pdfjs-dist/web/pdf_viewer.css';
	import { Button } from '$lib/components/ui/button';
	import ChevronLeftIcon from '@lucide/svelte/icons/chevron-left';
	import ChevronRightIcon from '@lucide/svelte/icons/chevron-right';
	import MinusIcon from '@lucide/svelte/icons/minus';
	import PlusIcon from '@lucide/svelte/icons/plus';

	pdfjsLib.GlobalWorkerOptions.workerSrc = new URL(
		'pdfjs-dist/build/pdf.worker.min.mjs',
		import.meta.url
	).toString();

	type RenderTask = ReturnType<PDFPageProxy['render']>;

	type Props = {
		src: string;
	};

	let { src }: Props = $props();

	const MIN_ZOOM = 0.25;
	const MAX_ZOOM = 4;
	// Breathing room left/right of the document at the default (fit-to-width) zoom.
	const FIT_GUTTER = 24;

	let scrollContainer = $state<HTMLDivElement | null>(null);
	let canvases = $state<HTMLCanvasElement[]>([]);
	let textLayers = $state<HTMLDivElement[]>([]);
	let pageWrappers = $state<HTMLDivElement[]>([]);

	let pages = $state<PDFPageProxy[]>([]);
	let numPages = $state(0);
	let currentPage = $state(1);
	let userZoom = $state(1);
	let renderedScale = $state(1);
	let resizeTick = $state(0);
	let loading = $state(true);
	let error = $state<string | null>(null);

	let doc: PDFDocumentProxy | null = null;
	let renderTasks: RenderTask[] = [];
	// Bumped on every render pass so stale async work can detect it was superseded.
	let renderGen = 0;

	const canPrev = $derived(currentPage > 1);
	const canNext = $derived(currentPage < numPages);
	const zoomPercent = $derived(Math.round(renderedScale * 100));

	function cancelRenderTasks() {
		for (const task of renderTasks) {
			try {
				task.cancel();
			} catch {
				// already settled
			}
		}
		renderTasks = [];
	}

	async function renderTextLayer(
		page: PDFPageProxy,
		div: HTMLDivElement,
		viewport: ReturnType<PDFPageProxy['getViewport']>,
		scale: number,
		gen: number
	) {
		div.innerHTML = '';
		div.style.width = `${Math.floor(viewport.width)}px`;
		div.style.height = `${Math.floor(viewport.height)}px`;
		// pdf.js positions text-layer spans relative to this CSS variable.
		div.style.setProperty('--scale-factor', String(scale));

		const textContent = await page.getTextContent();
		if (gen !== renderGen) return;

		const layer = new TextLayer({ textContentSource: textContent, container: div, viewport });
		await layer.render();
	}

	async function renderAll() {
		if (!doc || pages.length === 0 || !scrollContainer) return;
		if (canvases.filter(Boolean).length !== pages.length) return;

		const gen = ++renderGen;
		cancelRenderTasks();

		// The canvas backing store is sized at devicePixelRatio and scaled back
		// down via CSS, so pages stay sharp on HiDPI screens.
		const dpr = window.devicePixelRatio || 1;
		const baseWidth = pages[0].getViewport({ scale: 1 }).width;
		const containerWidth = scrollContainer.clientWidth;
		if (containerWidth <= 0 || baseWidth <= 0) return;

		// Fit-to-width: the document fills the container minus a gutter, userZoom on top.
		const usableWidth = Math.max(containerWidth - FIT_GUTTER * 2, 0);
		const scale = (usableWidth / baseWidth) * userZoom;
		renderedScale = scale;

		for (let i = 0; i < pages.length; i++) {
			if (gen !== renderGen) return;

			const page = pages[i];
			const canvas = canvases[i];
			const textDiv = textLayers[i];
			const ctx = canvas?.getContext('2d');
			if (!canvas || !ctx) continue;

			const viewport = page.getViewport({ scale });
			const cssWidth = Math.floor(viewport.width);
			const cssHeight = Math.floor(viewport.height);

			canvas.width = Math.floor(cssWidth * dpr);
			canvas.height = Math.floor(cssHeight * dpr);
			canvas.style.width = `${cssWidth}px`;
			canvas.style.height = `${cssHeight}px`;

			const task = page.render({
				canvasContext: ctx,
				viewport,
				transform: [dpr, 0, 0, dpr, 0, 0]
			});
			renderTasks.push(task);

			try {
				await task.promise;
			} catch {
				// RenderingCancelledException when a newer pass supersedes this one.
				if (gen !== renderGen) return;
				continue;
			}
			if (gen !== renderGen) return;

			if (textDiv) await renderTextLayer(page, textDiv, viewport, scale, gen);
		}
	}

	function goToPage(pageNumber: number) {
		pageWrappers[pageNumber - 1]?.scrollIntoView({ behavior: 'smooth', block: 'start' });
	}

	function zoomIn() {
		userZoom = Math.min(userZoom * 1.25, MAX_ZOOM);
	}

	function zoomOut() {
		userZoom = Math.max(userZoom / 1.25, MIN_ZOOM);
	}

	// Load the document and every page proxy when src changes.
	$effect(() => {
		const url = src;
		loading = true;
		error = null;
		pages = [];
		numPages = 0;
		currentPage = 1;
		userZoom = 1;

		let cancelled = false;
		const task = pdfjsLib.getDocument(url);

		task.promise
			.then(async (loaded) => {
				if (cancelled) {
					loaded.destroy();
					return;
				}
				doc = loaded;
				numPages = loaded.numPages;

				const loadedPages: PDFPageProxy[] = [];
				for (let i = 1; i <= loaded.numPages; i++) {
					const page = await loaded.getPage(i);
					if (cancelled) return;
					loadedPages.push(page);
				}
				pages = loadedPages;
				loading = false;
			})
			.catch((e) => {
				if (cancelled) return;
				error = e instanceof Error ? e.message : String(e);
				loading = false;
			});

		return () => {
			cancelled = true;
			renderGen++;
			cancelRenderTasks();
			doc?.destroy();
			doc = null;
		};
	});

	// Re-render when pages load, zoom changes, or the container resizes.
	$effect(() => {
		void userZoom;
		void resizeTick;
		if (pages.length === 0 || !scrollContainer) return;
		if (canvases.filter(Boolean).length !== pages.length) return;
		renderAll();
	});

	// Recompute fit-to-width scale on container resize.
	$effect(() => {
		if (!scrollContainer) return;
		let timer: ReturnType<typeof setTimeout>;
		const observer = new ResizeObserver(() => {
			clearTimeout(timer);
			timer = setTimeout(() => resizeTick++, 100);
		});
		observer.observe(scrollContainer);
		return () => {
			clearTimeout(timer);
			observer.disconnect();
		};
	});

	// Track which page is most visible so the toolbar counter stays accurate.
	$effect(() => {
		if (pages.length === 0 || !scrollContainer) return;
		if (pageWrappers.filter(Boolean).length !== pages.length) return;

		// Intersection ratio per page, indexed by page number.
		const ratios: number[] = [];
		const observer = new IntersectionObserver(
			(entries) => {
				for (const entry of entries) {
					const pageNumber = Number((entry.target as HTMLElement).dataset.page);
					ratios[pageNumber] = entry.intersectionRatio;
				}
				let best = currentPage;
				let bestRatio = -1;
				ratios.forEach((ratio, pageNumber) => {
					if (ratio > bestRatio) {
						bestRatio = ratio;
						best = pageNumber;
					}
				});
				currentPage = best;
			},
			{ root: scrollContainer, threshold: [0, 0.1, 0.25, 0.5, 0.75, 1] }
		);

		for (const wrapper of pageWrappers) {
			if (wrapper) observer.observe(wrapper);
		}
		return () => observer.disconnect();
	});
</script>

<div class="pdf-viewer">
	<div class="pdf-toolbar">
		<div class="pdf-toolbar-group">
			<Button
				variant="outline"
				size="sm"
				class="pdf-page-button"
				aria-label="Previous page"
				title="Previous page"
				disabled={!canPrev || loading}
				onclick={() => goToPage(currentPage - 1)}
			>
				<ChevronLeftIcon class="size-4" />
			</Button>
			<span class="pdf-toolbar-text">Page {currentPage} of {numPages}</span>
			<Button
				variant="outline"
				size="sm"
				class="pdf-page-button"
				aria-label="Next page"
				title="Next page"
				disabled={!canNext || loading}
				onclick={() => goToPage(currentPage + 1)}
			>
				<ChevronRightIcon class="size-4" />
			</Button>
		</div>

		<div class="pdf-toolbar-group">
			<Button
				variant="outline"
				size="sm"
				aria-label="Zoom out"
				title="Zoom out"
				disabled={userZoom <= MIN_ZOOM}
				onclick={zoomOut}
			>
				<MinusIcon class="size-4" />
			</Button>
			<span class="pdf-toolbar-text pdf-zoom">{zoomPercent}%</span>
			<Button
				variant="outline"
				size="sm"
				aria-label="Zoom in"
				title="Zoom in"
				disabled={userZoom >= MAX_ZOOM}
				onclick={zoomIn}
			>
				<PlusIcon class="size-4" />
			</Button>
		</div>
	</div>

	<div bind:this={scrollContainer} class="pdf-scroll">
		{#if error}
			<p class="pdf-message text-destructive">Failed to load PDF: {error}</p>
		{:else if loading}
			<p class="pdf-message text-muted-foreground">Loading document…</p>
		{:else}
			{#each pages as page, i (page.pageNumber)}
				<div class="pdf-page" data-page={i + 1} bind:this={pageWrappers[i]}>
					<canvas bind:this={canvases[i]}></canvas>
					<div class="textLayer" bind:this={textLayers[i]}></div>
				</div>
			{/each}
		{/if}
	</div>
</div>

<style>
	.pdf-viewer {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
	}

	.pdf-toolbar {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
		gap: 0.5rem 2rem;
		padding: 0.5rem 1rem;
		background-color: var(--background);
		border-bottom: 1px solid var(--border);
	}

	.pdf-toolbar-group {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.pdf-toolbar-text {
		font-size: 0.875rem;
		font-weight: 500;
		color: var(--foreground);
	}

	.pdf-zoom {
		min-width: 3.5rem;
		text-align: center;
	}

	.pdf-scroll {
		flex: 1;
		min-height: 0;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		padding: 1rem 0;
		overflow: auto;
		scrollbar-gutter: stable;
		background-color: var(--muted);
	}

	.pdf-page {
		position: relative;
		flex-shrink: 0;
		/* Auto margins center the page when it fits and collapse to 0 when it
		   overflows, so a zoomed-in document stays scrollable to both edges. */
		margin-inline: auto;
		background-color: white;
		box-shadow: 0 2px 10px rgba(0, 0, 0, 0.15);
	}

	.pdf-page canvas {
		display: block;
	}

	.pdf-message {
		margin: auto;
		padding: 2rem;
		font-size: 0.875rem;
	}

	/* Continuous scroll replaces page stepping on small screens. */
	@media (max-width: 640px) {
		:global(.pdf-page-button) {
			display: none;
		}
	}
</style>
