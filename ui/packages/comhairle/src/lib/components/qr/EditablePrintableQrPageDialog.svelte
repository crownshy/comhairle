<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Button } from '$lib/components/ui/button';
	import RichTextEditor from '$lib/components/RichTextEditor/RichTextEditor.svelte';
	import { notifications } from '$lib/notifications.svelte';
	import QrCode from 'svelte-qrcode';

	type Props = {
		url: string;
		open?: boolean;
		title?: string;
		description?: string;
		initialContent?: string;
	};

	let {
		url,
		open = $bindable(false),
		title = 'Live recordings QR page',
		description = 'Edit the page text, then print or share the QR code.',
		initialContent = '## Live recordings\nScan this QR code to open the participant live recording page.'
	}: Props = $props();

	let content = $state(initialContent);
	let editorContainer = $state<HTMLDivElement | null>(null);
	let qrCodeContainer = $state<HTMLDivElement | null>(null);

	function getQrCodeSvgElement(): SVGSVGElement | null {
		return qrCodeContainer?.querySelector('svg') ?? null;
	}

	function getQrCodeImageElement(): HTMLImageElement | null {
		return qrCodeContainer?.querySelector('img') ?? null;
	}

	function qrCodeSvgMarkup(): string | null {
		const qrSvg = getQrCodeSvgElement();
		if (!qrSvg) return null;
		return new XMLSerializer().serializeToString(qrSvg);
	}

	function qrCodeImageUrl(): string | null {
		const qrImageSrc = getQrCodeImageElement()?.src;
		if (qrImageSrc && qrImageSrc.length > 0) return qrImageSrc;

		const svgMarkup = qrCodeSvgMarkup();
		if (!svgMarkup) return null;
		return `data:image/svg+xml;charset=utf-8,${encodeURIComponent(svgMarkup)}`;
	}

	function tiptapContentHtml(): string | null {
		const tiptapElement = editorContainer?.querySelector('.tiptap');
		if (!tiptapElement) return null;
		return tiptapElement.innerHTML;
	}

	async function qrCodePngBlob(): Promise<Blob | null> {
		const imageUrl = qrCodeImageUrl();
		if (!imageUrl) return null;

		const image = new Image();
		image.src = imageUrl;
		await image.decode();

		const width = image.naturalWidth || image.width;
		const height = image.naturalHeight || image.height;
		if (width <= 0 || height <= 0) return null;

		const canvas = document.createElement('canvas');
		canvas.width = width;
		canvas.height = height;
		const context = canvas.getContext('2d');
		if (!context) return null;

		context.imageSmoothingEnabled = false;
		context.fillStyle = '#ffffff';
		context.fillRect(0, 0, width, height);
		context.drawImage(image, 0, 0, width, height);

		return await new Promise<Blob | null>((resolve) => {
			canvas.toBlob((blob) => resolve(blob), 'image/png');
		});
	}

	async function copyQrCodeImage() {
		try {
			if (!window?.navigator?.clipboard || typeof ClipboardItem === 'undefined') {
				throw new Error('Clipboard image copy unavailable');
			}

			const pngBlob = await qrCodePngBlob();
			if (!pngBlob) throw new Error('QR code not ready');

			await window.navigator.clipboard.write([
				new ClipboardItem({
					'image/png': pngBlob
				})
			]);

			notifications.send({
				message: 'QR code image copied',
				priority: 'INFO'
			});
		} catch {
			notifications.send({
				message: 'Could not copy QR code image',
				priority: 'ERROR'
			});
		}
	}

	async function copyUrl() {
		try {
			if (!window?.navigator?.clipboard) {
				throw new Error('Clipboard unavailable');
			}
			await window.navigator.clipboard.writeText(url);
			notifications.send({
				message: 'Live recording link copied',
				priority: 'INFO'
			});
		} catch {
			notifications.send({
				message: 'Could not copy live recording link',
				priority: 'ERROR'
			});
		}
	}

	function printPage() {
		const imageUrl = qrCodeImageUrl();
		if (!imageUrl) {
			notifications.send({ message: 'QR code not ready yet', priority: 'ERROR' });
			return;
		}

		const editorHtml = tiptapContentHtml();
		if (!editorHtml) {
			notifications.send({ message: 'Editor content not ready yet', priority: 'ERROR' });
			return;
		}

		const popup = window.open('about:blank', '_blank');
		if (!popup) {
			notifications.send({
				message: 'Could not open print window',
				priority: 'ERROR'
			});
			return;
		}

		popup.document.write(`<!doctype html>
<html>
<head>
  <meta charset="utf-8" />
  <title>${title}</title>
  <style>
    :root { color-scheme: light; }
    body {
      margin: 0;
      padding: 24px;
      background: #ffffff;
      color: #111827;
      font-family: ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, Helvetica, Arial, sans-serif;
    }
    .qr-print-page {
      max-width: 760px;
      margin: 0 auto;
      border: 1px solid #e5e7eb;
      border-radius: 12px;
      padding: 24px;
    }
    .qr-print-code-wrap {
      margin: 20px auto 12px;
      width: fit-content;
      border: 1px solid #d1d5db;
      border-radius: 12px;
      background: #ffffff;
      padding: 16px;
    }
    .qr-print-code-wrap img {
      width: 260px;
      height: 260px;
      display: block;
    }
    .qr-print-url {
      text-align: center;
      word-break: break-all;
      font-size: 12px;
      color: #4b5563;
    }
    .tiptap h1 { font-size: 28px; margin: 0 0 12px; line-height: 1.2; }
    .tiptap h2 { font-size: 22px; margin: 0 0 10px; line-height: 1.3; }
    .tiptap h3 { font-size: 18px; margin: 0 0 8px; line-height: 1.3; }
    .tiptap p { margin: 0 0 10px; line-height: 1.6; }
    .tiptap ul, .tiptap ol { margin: 0 0 10px 24px; }
    .tiptap blockquote {
      margin: 0 0 12px;
      padding-left: 12px;
      border-left: 3px solid #d1d5db;
      color: #4b5563;
    }
    @media print {
      body { padding: 0; }
      .qr-print-page {
        border: none;
        border-radius: 0;
        max-width: none;
      }
    }
  </style>
</head>
<body onload="window.print(); window.close();">
  <div class="qr-print-page">
		<div class="tiptap">
			${editorHtml}
		</div>
		<div class="qr-print-code-wrap">
			<img src="${imageUrl}" alt="QR code" />
		</div>
		<p class="qr-print-url">${url}</p>
  </div>
</body>
</html>`);
		popup.document.close();
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content
		class="h-[90vh] w-[calc(100vw-1rem)] max-w-[calc(100vw-1rem)] overflow-x-hidden overflow-y-auto p-4 sm:h-auto sm:max-h-[90vh] sm:w-[min(1100px,calc(100vw-2rem))] sm:max-w-[min(1100px,calc(100vw-2rem))] sm:p-6"
	>
		<Dialog.Header>
			<Dialog.Title>{title}</Dialog.Title>
			<Dialog.Description>{description}</Dialog.Description>
		</Dialog.Header>

		<div class="min-w-0 space-y-4">
			<div class="min-w-0">
				<p class="text-muted-foreground mb-2 text-xs font-medium tracking-wide uppercase">
					Page content
				</p>
				<div bind:this={editorContainer} class="min-w-0 overflow-x-hidden">
					<RichTextEditor
						value={content}
						onChange={(nextValue) => (content = nextValue)}
						minHeight="140px"
						maxHeight="280px"
						width="100%"
					/>
				</div>
			</div>

			<div class="min-w-0">
				<p class="text-muted-foreground mb-2 text-xs font-medium tracking-wide uppercase">
					QR code
				</p>
				<div class="qr-print-page rounded-xl border p-4">
					<div
						class="qr-print-code-wrap qr-preview mx-auto w-fit max-w-full rounded-lg border bg-white p-3"
						bind:this={qrCodeContainer}
					>
						<QrCode value={url} size={220} />
					</div>
					<p
						class="text-muted-foreground qr-print-url mt-3 text-center text-xs break-all"
					>
						{url}
					</p>
				</div>
			</div>
		</div>

		<Dialog.Footer class="flex-wrap gap-2 sm:justify-center">
			<Button type="button" variant="outline" onclick={copyQrCodeImage}>Copy QR image</Button>
			<Button type="button" variant="outline" onclick={printPage}>Print page</Button>
			<Button type="button" onclick={copyUrl}>Copy URL</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<style>
	.qr-preview {
		display: flex;
		align-items: center;
		justify-content: center;
		width: fit-content;
	}

	:global(.qr-preview img.qrcode) {
		display: block;
		margin: 0 auto;
		width: min(220px, 100%);
		height: auto;
		aspect-ratio: 1 / 1;
		image-rendering: pixelated;
	}
</style>
