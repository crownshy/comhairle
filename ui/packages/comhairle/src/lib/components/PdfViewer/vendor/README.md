# Svelte PDF Viewer

A comprehensive PDF viewer component for Svelte 5 with interactive features like text selection, annotations, and flexible display modes.

## Features

- 📄 **PDF Rendering**: High-quality PDF rendering using PDF.js
- 🖱️ **Text Selection**: Native browser text selection with coordinate tracking
- 🔍 **Zoom Controls**: Mouse wheel zoom, keyboard shortcuts, and programmatic zoom
- 📑 **Display Modes**: Single page and continuous scrolling modes
- 📐 **Fit Modes**: Fit to width, page, auto-fit, or custom scaling
- ⌨️ **Keyboard Navigation**: Arrow keys, page up/down, home/end shortcuts
- 🎨 **Theming**: Light and dark theme support
- 📱 **Responsive**: Adapts to container size changes
- ✏️ **Annotations**: Drawing annotations with customizable colors and thickness
- 🔧 **TypeScript**: Full TypeScript support with comprehensive type definitions

## Installation

```bash
npm install svelte-pdf
```

## Quick Start

```svelte
<script>
	import { PDFViewer } from 'svelte-pdf';

	let pdfFile = $state();
	let currentPage = $state(1);
	let scale = $state(1.0);

	function handleTextSelection(selection) {
		console.log('Selected:', selection.text);
	}
</script>

<input type="file" accept=".pdf" on:change={(e) => (pdfFile = e.target.files[0])} />

{#if pdfFile}
	<PDFViewer
		src={pdfFile}
		bind:page={currentPage}
		bind:scale
		showNavigation={true}
		enableTextSelection={true}
		onTextSelect={handleTextSelection}
	/>
{/if}
```

## API Reference

### Props

| Prop                  | Type          | Default    | Description                                                 |
| --------------------- | ------------- | ---------- | ----------------------------------------------------------- |
| `src`                 | `PDFSource`   | -          | **Required.** PDF source (File, ArrayBuffer, or URL string) |
| `page`                | `number`      | `1`        | Current page number (bindable)                              |
| `scale`               | `number`      | `1.0`      | Current zoom scale (bindable)                               |
| `fitMode`             | `FitMode`     | `'none'`   | How to fit PDF in container                                 |
| `displayMode`         | `DisplayMode` | `'single'` | Page display mode                                           |
| `showNavigation`      | `boolean`     | `true`     | Show navigation controls                                    |
| `enableTextSelection` | `boolean`     | `true`     | Enable text selection                                       |
| `theme`               | `Theme`       | `'light'`  | UI theme                                                    |
| `containerClass`      | `string`      | `''`       | Additional CSS class for container                          |
| `canvasClass`         | `string`      | `''`       | Additional CSS class for canvas                             |
| `width`               | `number`      | -          | Fixed width (overrides fit modes)                           |
| `height`              | `number`      | -          | Fixed height (overrides fit modes)                          |
| `maxScale`            | `number`      | `3.0`      | Maximum zoom scale                                          |
| `minScale`            | `number`      | `0.1`      | Minimum zoom scale                                          |

### Events

| Event           | Type                                                         | Description                 |
| --------------- | ------------------------------------------------------------ | --------------------------- |
| `onLoad`        | `(event: { numPages: number; fingerprint: string }) => void` | Fired when PDF loads        |
| `onError`       | `(event: { error: string }) => void`                         | Fired on error              |
| `onPageChange`  | `(event: { page: number; total: number }) => void`           | Fired when page changes     |
| `onScaleChange` | `(event: { scale: number }) => void`                         | Fired when zoom changes     |
| `onTextSelect`  | `(selection: TextSelection) => void`                         | Fired when text is selected |

### Event Callbacks and Action Options

Configure event callbacks and interactive behaviors:

```svelte
<PDFViewer
	src={pdfFile}
	onTextSelect={handleTextSelection}
	onLoad={handleLoad}
	onError={handleError}
	onPageChange={handlePageChange}
	onScaleChange={handleScaleChange}
	zoomOptions={{
		onZoom: handleZoom,
		minScale: 0.5,
		maxScale: 5.0,
		step: 0.1
	}}
	navigationOptions={{
		onPageChange: handlePageChange,
		enableKeyboard: true,
		enableMouseWheel: false
	}}
	annotationOptions={{
		onAnnotate: handleAnnotation,
		color: '#ff0000',
		thickness: 2
	}}
/>
```

## Types

### PDFSource

```typescript
type PDFSource = string | File | ArrayBuffer;
```

### DisplayMode

```typescript
type DisplayMode = 'single' | 'continuous';
```

### FitMode

```typescript
type FitMode = 'none' | 'width' | 'page' | 'auto';
```

### Theme

```typescript
type Theme = 'light' | 'dark';
```

### TextSelection

```typescript
interface TextSelection {
	text: string;
	pageNumber: number;
	x: number;
	y: number;
	width: number;
	height: number;
}
```

## Usage Examples

### Basic PDF Viewer

```svelte
<script>
	import { PDFViewer } from 'svelte-pdf';

	let pdfUrl = $state('https://example.com/document.pdf');
</script>

<PDFViewer src={pdfUrl} />
```

### Advanced Configuration

```svelte
<script>
	import { PDFViewer } from 'svelte-pdf';

	let pdfFile = $state();
	let currentPage = $state(1);
	let scale = $state(1.0);
	let displayMode = $state('continuous');
	let theme = $state('dark');

	function handleTextSelection(selection) {
		console.log('Selected:', selection.text);
	}

	function handlePageChange(event) {
		console.log(`Page ${event.page} of ${event.total}`);
	}
</script>

<PDFViewer
	src={pdfFile}
	bind:page={currentPage}
	bind:scale
	{displayMode}
	{theme}
	fitMode="width"
	showNavigation={true}
	enableTextSelection={true}
	onTextSelect={handleTextSelection}
	onPageChange={handlePageChange}
	containerClass="my-pdf-viewer"
	maxScale={5.0}
	minScale={0.25}
/>
```

### Text Selection Handling

```svelte
<script>
	import { PDFViewer } from 'svelte-pdf';

	let selectedTexts = $state([]);

	function handleTextSelection(selection) {
		selectedTexts = [
			...selectedTexts,
			{
				id: Date.now(),
				text: selection.text,
				page: selection.pageNumber,
				coordinates: {
					x: selection.x,
					y: selection.y,
					width: selection.width,
					height: selection.height
				}
			}
		];
	}
</script>

<PDFViewer src={pdfFile} onTextSelect={handleTextSelection} />

{#if selectedTexts.length > 0}
	<div class="selections">
		<h3>Selected Text:</h3>
		{#each selectedTexts as selection (selection.id)}
			<div class="selection">
				<strong>Page {selection.page}:</strong>
				{selection.text}
			</div>
		{/each}
	</div>
{/if}
```

### Programmatic Control

```svelte
<script>
	import { PDFViewer } from 'svelte-pdf';

	let pdfViewer = $state();
	let currentPage = $state(1);
	let scale = $state(1.0);
	let totalPages = $state(0);

	function goToPage(pageNum) {
		currentPage = Math.max(1, Math.min(pageNum, totalPages));
	}

	function zoomIn() {
		scale = Math.min(scale * 1.2, 3.0);
	}

	function zoomOut() {
		scale = Math.max(scale / 1.2, 0.1);
	}

	function handleLoad(event) {
		totalPages = event.numPages;
	}
</script>

<div class="controls">
	<button onclick={() => goToPage(currentPage - 1)}>Previous</button>
	<span>Page {currentPage} of {totalPages}</span>
	<button onclick={() => goToPage(currentPage + 1)}>Next</button>

	<button onclick={zoomOut}>Zoom Out</button>
	<span>{Math.round(scale * 100)}%</span>
	<button onclick={zoomIn}>Zoom In</button>
</div>

<PDFViewer
	bind:this={pdfViewer}
	src={pdfFile}
	bind:page={currentPage}
	bind:scale
	onLoad={handleLoad}
	showNavigation={false}
/>
```

## Keyboard Shortcuts

| Key                         | Action                |
| --------------------------- | --------------------- |
| `←` / `Page Up`             | Previous page         |
| `→` / `Page Down` / `Space` | Next page             |
| `Home`                      | First page            |
| `End`                       | Last page             |
| `Ctrl/Cmd + +`              | Zoom in               |
| `Ctrl/Cmd + -`              | Zoom out              |
| `Ctrl/Cmd + 0`              | Reset zoom            |
| `Ctrl/Cmd + Scroll`         | Zoom with mouse wheel |

## Styling

The component provides CSS custom properties for theming:

```css
.pdf-viewer {
	--nav-bg: rgba(255, 255, 255, 0.1);
	--border-color: #ddd;
	--button-bg: #fff;
	--button-color: #333;
	--button-hover-bg: #f0f0f0;
}

.pdf-viewer.dark {
	--nav-bg: rgba(0, 0, 0, 0.1);
	--border-color: #555;
	--button-bg: #333;
	--button-color: #fff;
	--button-hover-bg: #444;
}
```

## Requirements

- Svelte 5+
- Modern browser with Canvas and PDF.js support
- PDF.js is included as a dependency

## Browser Support

- Chrome/Edge 88+
- Firefox 85+
- Safari 14+

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please read our contributing guidelines and submit pull requests to our repository.

## Troubleshooting

### PDF not loading

- Ensure the PDF source is accessible (check CORS for URLs)
- Verify the file is a valid PDF
- Check browser console for error messages

### Text selection not working

- Ensure `enableTextSelection` is `true`
- Text selection requires PDF text content (not scanned images)
- Some PDFs may have text layers that aren't selectable

### Performance with large PDFs

- Use `displayMode="single"` for better performance with large documents
- Monitor memory usage with continuous mode on documents with many pages
