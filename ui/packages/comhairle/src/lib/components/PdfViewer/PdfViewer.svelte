<script lang="ts">
	import * as pdfjsLib from 'pdfjs-dist';
	import { TextLayer } from 'pdfjs-dist';
	import type { PDFDocumentProxy, PDFPageProxy } from 'pdfjs-dist';
	import 'pdfjs-dist/web/pdf_viewer.css';
	import { Button } from '$lib/components/ui/button';
	import * as Select from '$lib/components/ui/select';
	import { Spinner } from '$lib/components/ui/spinner';
	import { cn } from '$lib/utils';
	import ChevronLeftIcon from '@lucide/svelte/icons/chevron-left';
	import ChevronRightIcon from '@lucide/svelte/icons/chevron-right';
	import MinusIcon from '@lucide/svelte/icons/minus';
	import PlusIcon from '@lucide/svelte/icons/plus';
	import type { PdfHighlight } from './highlights';
	import { HttpStatus } from '$lib/utils/constants';

	pdfjsLib.GlobalWorkerOptions.workerSrc = new URL(
		'pdfjs-dist/build/pdf.worker.min.mjs',
		import.meta.url
	).toString();

	type RenderTask = ReturnType<PDFPageProxy['render']>;

	// Either a fit mode or an explicit scale (1 === 100% of the PDF's natural size).
	type Zoom = 'fit-width' | 'fit-page' | number;

	type Props = {
		src: string;
		/** Passage rectangles to shade, in PDF points (see {@link PdfHighlight}). */
		highlights?: PdfHighlight[];
		/**
		 * Page to scroll to when the document first renders (1-based). Defaults to
		 * the first highlighted page when `highlights` is set.
		 */
		initialPage?: number | null;
	};

	let { src, highlights = [], initialPage = null }: Props = $props();

	const highlightsForPage = (pageNumber: number) =>
		highlights.filter((h) => h.page === pageNumber);

	const MIN_ZOOM = 0.25;
	const MAX_ZOOM = 4;
	// Breathing room around the document in the scroll area at the fit zooms.
	const FIT_GUTTER = 24;
	// Backing-store width of a thumbnail, before devicePixelRatio.
	const THUMB_WIDTH = 116;
	// Open filling the available width, so the page is fully visible and centered
	// with no horizontal scroll at any dialog or screen size (the capped dialog
	// keeps this from overshooting on wide desktops).
	const DEFAULT_ZOOM: Zoom = 'fit-width';

	const ZOOM_PRESETS = [
		{ value: 'fit-width', label: 'Fit width' },
		{ value: 'fit-page', label: 'Fit page' },
		{ value: '0.5', label: '50%' },
		{ value: '0.75', label: '75%' },
		{ value: '1', label: '100%' },
		{ value: '1.25', label: '125%' },
		{ value: '1.5', label: '150%' }
	];

	let scrollContainer = $state<HTMLDivElement | null>(null);
	let canvases = $state<HTMLCanvasElement[]>([]);
	let textLayers = $state<HTMLDivElement[]>([]);
	let pageWrappers = $state<HTMLDivElement[]>([]);
	let thumbCanvases = $state<HTMLCanvasElement[]>([]);
	let thumbButtons = $state<HTMLButtonElement[]>([]);

	let pages = $state<PDFPageProxy[]>([]);
	let numPages = $state(0);
	let currentPage = $state(1);
	let zoom = $state<Zoom>(DEFAULT_ZOOM);
	let renderedScale = $state(1);
	let resizeTick = $state(0);
	let loading = $state(true);
	let error = $state<string | null>(null);
	// Set when the download failed because the underlying document is gone, which
	// happens to citations in older assistant answers after an admin re-syncs the
	// learn content (the old document is deleted and replaced with a new id). We show
	// a plain-language explanation instead of a raw HTTP status. See ADR-0010.
	let sourceUnavailable = $state(false);
	// True once the target page (and everything above it) has rendered, so its
	// wrapper has its real height and an auto-jump lands on the right place. Reset
	// when `src` changes. We gate on the target rather than the whole document so a
	// deep PDF reveals as soon as the cited page is ready.
	let revealReady = $state(false);
	// True once the auto-jump has run and the document is positioned on the passage.
	// The viewport stays behind a loading overlay until this flips, so the reader
	// never sees the open-then-jump. Reset when `src` changes.
	let revealed = $state(false);
	// Guards the one-shot auto-jump, keyed by `src` + target page.
	let jumpedFor = $state<string | null>(null);

	let doc: PDFDocumentProxy | null = null;
	let renderTasks: RenderTask[] = [];
	// Bumped on every render pass so stale async work can detect it was superseded.
	let renderGen = 0;

	const canPrev = $derived(currentPage > 1);
	const canNext = $derived(currentPage < numPages);
	// The page the viewer auto-jumps to on open (1-based): an explicit `initialPage`,
	// else the first highlighted page, else null when there's nothing to jump to
	// (e.g. reloaded history with no positions).
	const targetPage = $derived(
		initialPage ?? (highlights.length ? Math.min(...highlights.map((h) => h.page)) : null)
	);
	// Per-page CSS aspect-ratio, so a thumbnail holds its true page shape (A4,
	// Letter, etc.) even before its preview finishes rendering.
	const thumbAspects = $derived(
		pages.map((p) => {
			const v = p.getViewport({ scale: 1 });
			return `${v.width} / ${v.height}`;
		})
	);
	// The Select binds to a string; explicit scales stringify to their preset value.
	const zoomValue = $derived(String(zoom));
	const zoomLabel = $derived(
		zoom === 'fit-width'
			? 'Fit width'
			: zoom === 'fit-page'
				? 'Fit page'
				: `${Math.round(renderedScale * 100)}%`
	);

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
		if (gen !== renderGen) {
			div.innerHTML = '';
			return;
		}
	}

	// Resolve the current zoom setting to a concrete render scale for the container.
	function resolveScale(): number {
		if (!scrollContainer || pages.length === 0) return typeof zoom === 'number' ? zoom : 1;

		const base = pages[0].getViewport({ scale: 1 });
		const usableWidth = Math.max(scrollContainer.clientWidth - FIT_GUTTER * 2, 0);
		const usableHeight = Math.max(scrollContainer.clientHeight - FIT_GUTTER * 2, 0);

		let scale: number;
		if (zoom === 'fit-width') {
			scale = usableWidth / base.width;
		} else if (zoom === 'fit-page') {
			scale = Math.min(usableWidth / base.width, usableHeight / base.height);
		} else {
			scale = zoom;
		}
		return Math.min(Math.max(scale, MIN_ZOOM), MAX_ZOOM);
	}

	async function renderAll() {
		if (!doc || pages.length === 0 || !scrollContainer) return;
		if (canvases.filter(Boolean).length !== pages.length) return;

		const gen = ++renderGen;
		cancelRenderTasks();

		// The canvas backing store is sized at devicePixelRatio and scaled back
		// down via CSS, so pages stay sharp on HiDPI screens.
		const dpr = window.devicePixelRatio || 1;
		const scale = resolveScale();
		renderedScale = scale;

		// The page we can reveal after: the auto-jump target, or page 1 when there's
		// nothing to jump to. Rendering is sequential, so once this page is done every
		// page above it is sized too and the jump lands correctly.
		const revealTarget = Math.min(Math.max(targetPage ?? 1, 1), pages.length);

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

			// Reveal as soon as the target page is on screen, without waiting for the
			// rest of a deep document to finish rendering.
			if (gen === renderGen && !revealReady && i + 1 >= revealTarget) revealReady = true;
		}
	}

	function goToPage(pageNumber: number, behavior: 'smooth' | 'auto' = 'smooth') {
		pageWrappers[pageNumber - 1]?.scrollIntoView({ behavior, block: 'start' });
	}

	function setZoom(value: string) {
		zoom = value === 'fit-width' || value === 'fit-page' ? value : Number(value);
	}

	function zoomIn() {
		zoom = Math.min(renderedScale * 1.25, MAX_ZOOM);
	}

	function zoomOut() {
		zoom = Math.max(renderedScale / 1.25, MIN_ZOOM);
	}

	// Load the document and every page proxy when src changes.
	$effect(() => {
		const url = src;
		loading = true;
		error = null;
		sourceUnavailable = false;
		pages = [];
		numPages = 0;
		currentPage = 1;
		zoom = DEFAULT_ZOOM;
		revealReady = false;
		revealed = false;
		jumpedFor = null;

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
				// A missing document comes back as 404 (pdf.js MissingPDFException); the
				// RAGFlow-proxied "document not found" path can also surface as 500
				// (UnexpectedResponseException). Both mean the cited source no longer exists,
				// typically because the learn content was re-synced after this answer.
				const status =
					typeof (e as { status?: unknown })?.status === 'number'
						? (e as { status: number }).status
						: undefined;
				const name = e instanceof Error ? e.name : '';
				sourceUnavailable =
					name === 'MissingPDFException' ||
					name === 'UnexpectedResponseException' ||
					status === HttpStatus.NotFound ||
					status === HttpStatus.InternalServerError;
				error = e instanceof Error ? e.message : String(e);
				loading = false;
			});

		return () => {
			cancelled = true;
			renderGen++;
			cancelRenderTasks();
			task.destroy();
			doc?.destroy();
			doc = null;
		};
	});

	// Re-render the main pages when they load, zoom changes, or the container resizes.
	$effect(() => {
		void zoom;
		void resizeTick;
		if (pages.length === 0 || !scrollContainer) return;
		if (canvases.filter(Boolean).length !== pages.length) return;
		renderAll();
	});

	// Render the thumbnail rail once per document. Thumbnails are zoom-independent.
	$effect(() => {
		if (pages.length === 0) return;
		if (thumbCanvases.filter(Boolean).length !== pages.length) return;

		let cancelled = false;
		const dpr = window.devicePixelRatio || 1;

		(async () => {
			for (let i = 0; i < pages.length; i++) {
				if (cancelled) return;
				const page = pages[i];
				const canvas = thumbCanvases[i];
				const ctx = canvas?.getContext('2d');
				if (!canvas || !ctx) continue;
				// Already drawn. Can't use canvas.width as the sentinel: a fresh
				// canvas defaults to 300, so we track completion on the element.
				if (canvas.dataset.rendered === 'true') continue;

				const base = page.getViewport({ scale: 1 });
				const cssScale = THUMB_WIDTH / base.width;
				const viewport = page.getViewport({ scale: cssScale * dpr });

				// Backing store only; CSS sizes the display, so a thumbnail stays
				// crisp on HiDPI without re-rendering when the rail resizes.
				canvas.width = Math.floor(viewport.width);
				canvas.height = Math.floor(viewport.height);

				try {
					await page.render({ canvasContext: ctx, viewport }).promise;
					canvas.dataset.rendered = 'true';
				} catch {
					// Superseded by a document change; the next pass will redraw.
				}
			}
		})();

		return () => {
			cancelled = true;
		};
	});

	// Recompute fit scale on container resize.
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

	// Auto-scroll to the passage once the target page has rendered, then reveal the
	// viewport already positioned there. Runs once per src/target (guarded by
	// `jumpedFor`) so it doesn't fight the user's own scrolling on zoom or resize.
	// Instant jump, not smooth, so a deep target page doesn't animate through
	// everything above it (and it happens behind the overlay anyway).
	$effect(() => {
		if (!revealReady || !scrollContainer) return;
		const key = `${src}:${targetPage ?? 'none'}`;
		if (jumpedFor === key) return;
		jumpedFor = key;
		if (targetPage) goToPage(targetPage, 'auto');
		revealed = true;
	});

	// Track which page is most visible so the toolbar counter and rail stay accurate.
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

	// Keep the active thumbnail in view as the reader scrolls the document.
	$effect(() => {
		thumbButtons[currentPage - 1]?.scrollIntoView({ block: 'nearest', inline: 'nearest' });
	});
</script>

<div class="flex h-full w-full flex-col">
	<div
		class="border-border bg-background flex flex-wrap items-center justify-between gap-x-8 gap-y-2 border-b px-4 py-2"
	>
		<div class="flex items-center gap-2">
			<Button
				variant="outline"
				size="sm"
				class="max-sm:hidden"
				aria-label="Previous page"
				title="Previous page"
				disabled={!canPrev || !revealed}
				onclick={() => goToPage(currentPage - 1)}
			>
				<ChevronLeftIcon class="size-4" />
			</Button>
			<span class="text-foreground text-sm font-medium">Page {currentPage} of {numPages}</span
			>
			<Button
				variant="outline"
				size="sm"
				class="max-sm:hidden"
				aria-label="Next page"
				title="Next page"
				disabled={!canNext || !revealed}
				onclick={() => goToPage(currentPage + 1)}
			>
				<ChevronRightIcon class="size-4" />
			</Button>
		</div>

		<div class="flex items-center gap-2">
			<Button
				variant="outline"
				size="sm"
				aria-label="Zoom out"
				title="Zoom out"
				disabled={!revealed || renderedScale <= MIN_ZOOM}
				onclick={zoomOut}
			>
				<MinusIcon class="size-4" />
			</Button>
			<Select.Root type="single" value={zoomValue} onValueChange={setZoom}>
				<Select.Trigger size="sm" class="min-w-26 justify-between" aria-label="Zoom level">
					{zoomLabel}
				</Select.Trigger>
				<Select.Content>
					{#each ZOOM_PRESETS as preset (preset.value)}
						<Select.Item value={preset.value}>{preset.label}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
			<Button
				variant="outline"
				size="sm"
				aria-label="Zoom in"
				title="Zoom in"
				disabled={!revealed || renderedScale >= MAX_ZOOM}
				onclick={zoomIn}
			>
				<PlusIcon class="size-4" />
			</Button>
		</div>
	</div>

	<div class="relative flex min-h-0 flex-1">
		{#if !loading && !error && pages.length > 0}
			<div
				class="border-border bg-background flex w-45 shrink-0 flex-col gap-2 overflow-y-auto border-r p-3 max-sm:hidden"
			>
				<span class="text-muted-foreground px-1 text-sm font-medium">Pages</span>
				<div class="flex flex-col gap-3">
					{#each pages as page, i (page.pageNumber)}
						<button
							type="button"
							class={cn(
								'group flex shrink-0 cursor-pointer flex-col items-center gap-1 border-0 bg-transparent p-0',
								currentPage === i + 1 ? 'text-foreground' : 'text-muted-foreground'
							)}
							aria-label={`Go to page ${i + 1}`}
							aria-current={currentPage === i + 1 ? 'page' : undefined}
							bind:this={thumbButtons[i]}
							onclick={() => goToPage(i + 1)}
						>
							<span
								class={cn(
									'block w-29 overflow-hidden rounded-lg border-2 bg-white transition-colors',
									currentPage === i + 1
										? 'border-primary'
										: 'border-border group-hover:border-muted-foreground'
								)}
								style="aspect-ratio: {thumbAspects[i] ?? '1 / 1.414'};"
							>
								<canvas class="block h-full w-full" bind:this={thumbCanvases[i]}
								></canvas>
							</span>
							<span class="text-xs font-medium">{i + 1}</span>
						</button>
					{/each}
				</div>
			</div>
		{/if}

		<div
			bind:this={scrollContainer}
			class="bg-muted flex min-h-0 flex-1 flex-col gap-4 overflow-auto py-4 [scrollbar-gutter:stable]"
		>
			{#if sourceUnavailable}
				<p class="text-muted-foreground m-auto max-w-prose p-8 text-center text-base">
					This source is no longer available. The learning materials have been updated
					since this answer was written, so the passage it cited no longer exists. Ask
					your question again to get an answer with up-to-date sources.
				</p>
			{:else if error}
				<p class="text-destructive m-auto p-8 text-sm">Failed to load PDF: {error}</p>
			{:else}
				<!-- Pages mount and render while still hidden behind the overlay below, so
				the auto-jump lands before the reader ever sees them. -->
				{#each pages as page, i (page.pageNumber)}
					<div
						class="relative mx-auto shrink-0 bg-white shadow-[0_2px_10px_rgba(0,0,0,0.15)]"
						data-page={i + 1}
						bind:this={pageWrappers[i]}
					>
						<canvas class="block" bind:this={canvases[i]}></canvas>
						<div class="textLayer" bind:this={textLayers[i]}></div>
						{#each highlightsForPage(i + 1) as h, hi (hi)}
							<!-- Amber marker-pen tint; mix-blend-multiply keeps the text legible on the white page. -->
							<div
								class="pointer-events-none absolute rounded-[2px] bg-yellow-400/40 mix-blend-multiply"
								style="left: {h.left * renderedScale}px; top: {h.top *
									renderedScale}px; width: {h.width *
									renderedScale}px; height: {h.height * renderedScale}px;"
							></div>
						{/each}
					</div>
				{/each}
			{/if}
		</div>

		<!-- Covers the viewport (and rail) until the document has rendered and jumped to
		the cited passage, so the reader never watches it open-then-scroll. -->
		{#if !error && !revealed}
			<div
				class="bg-background absolute inset-0 z-10 flex items-center justify-center gap-3"
				role="status"
				aria-live="polite"
			>
				<Spinner class="text-muted-foreground size-5" />
				<span class="text-muted-foreground text-sm">Loading document…</span>
			</div>
		{/if}
	</div>
</div>
