<script lang="ts">
	type Props = {
		src: string;
	};

	let { src }: Props = $props();

	let html = $state<string | null>(null);
	let loading = $state(true);
	let error = $state<string | null>(null);

	// mammoth ships a browser bundle (`mammoth.browser.js`) that avoids Node-only deps.
	// It has no TypeScript definitions, so we type the small surface we use ourselves.
	type Mammoth = {
		convertToHtml: (input: { arrayBuffer: ArrayBuffer }) => Promise<{ value: string }>;
	};

	$effect(() => {
		const url = src;
		loading = true;
		error = null;
		html = null;

		let cancelled = false;
		(async () => {
			try {
				const [{ default: mammoth }, response] = await Promise.all([
					// @ts-expect-error -- no types for the browser entry
					import('mammoth/mammoth.browser.js') as Promise<{ default: Mammoth }>,
					fetch(url)
				]);
				if (!response.ok) throw new Error(`HTTP ${response.status}`);
				const arrayBuffer = await response.arrayBuffer();
				if (cancelled) return;
				const result = await mammoth.convertToHtml({ arrayBuffer });
				if (cancelled) return;
				html = result.value;
				loading = false;
			} catch (e) {
				if (cancelled) return;
				error = e instanceof Error ? e.message : String(e);
				loading = false;
			}
		})();

		return () => {
			cancelled = true;
		};
	});
</script>

<div class="docx-viewer">
	<div class="docx-scroll">
		{#if error}
			<p class="docx-message text-destructive">
				Failed to load document: {error}. Legacy <code>.doc</code> files are not supported in-browser
				— please download instead.
			</p>
		{:else if loading}
			<p class="docx-message text-muted-foreground">Loading document…</p>
		{:else if html !== null}
			<article class="docx-page prose max-w-none">
				<!-- eslint-disable-next-line svelte/no-at-html-tags -->
				{@html html}
			</article>
		{/if}
	</div>
</div>

<style>
	.docx-viewer {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
	}

	.docx-scroll {
		flex: 1;
		min-height: 0;
		display: flex;
		flex-direction: column;
		padding: 1rem;
		overflow: auto;
		background-color: var(--muted);
	}

	.docx-page {
		margin-inline: auto;
		padding: 3rem 4rem;
		max-width: 56rem;
		width: 100%;
		background-color: white;
		color: #1a1a1a;
		box-shadow: 0 2px 10px rgba(0, 0, 0, 0.15);
		font-family:
			-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell,
			'Helvetica Neue', sans-serif;
		line-height: 1.6;
	}

	/* All children inherit the page's dark-on-white palette so the document
	 * looks identical in light and dark mode (the page itself is the surface). */
	.docx-page :global(*) {
		color: inherit;
		background-color: transparent;
	}

	.docx-page :global(p) {
		margin: 0 0 0.75em;
	}

	.docx-page :global(h1),
	.docx-page :global(h2),
	.docx-page :global(h3),
	.docx-page :global(h4),
	.docx-page :global(h5),
	.docx-page :global(h6) {
		margin: 1.25em 0 0.5em;
		font-weight: 600;
		line-height: 1.25;
		color: #111;
	}

	.docx-page :global(strong),
	.docx-page :global(b) {
		font-weight: 700;
		color: inherit;
	}

	.docx-page :global(em),
	.docx-page :global(i) {
		font-style: italic;
		color: inherit;
	}

	.docx-page :global(h1) {
		font-size: 1.75rem;
	}
	.docx-page :global(h2) {
		font-size: 1.5rem;
	}
	.docx-page :global(h3) {
		font-size: 1.25rem;
	}

	.docx-page :global(ul),
	.docx-page :global(ol) {
		margin: 0 0 0.75em 1.5em;
	}

	.docx-page :global(table) {
		border-collapse: collapse;
		margin: 0.75em 0;
	}

	.docx-page :global(td),
	.docx-page :global(th) {
		border: 1px solid #ccc;
		padding: 0.4em 0.6em;
	}

	.docx-page :global(img) {
		max-width: 100%;
		height: auto;
	}

	.docx-page :global(a) {
		color: var(--primary);
		text-decoration: underline;
	}

	.docx-message {
		margin: auto;
		padding: 2rem;
		font-size: 0.875rem;
		text-align: center;
	}

	@media (max-width: 640px) {
		.docx-page {
			padding: 1.5rem 1.25rem;
		}
	}
</style>
