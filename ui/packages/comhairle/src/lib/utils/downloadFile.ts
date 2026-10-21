/** Waits this long before freeing the file, the same delay FileSaver.js uses. */
const REVOKE_DELAY_MS = 40_000;

/** Save a blob as a file in the browser. */
export function downloadBlob(filename: string, blob: Blob): void {
	const url = URL.createObjectURL(blob);
	const link = document.createElement('a');
	link.href = url;
	link.download = filename;
	document.body.appendChild(link);
	link.click();
	link.remove();
	// Revoking straight after click() can cancel the download before the browser reads the file.
	setTimeout(() => URL.revokeObjectURL(url), REVOKE_DELAY_MS);
}

/** Save plain text as a UTF-8 `.txt` file in the browser. */
export function downloadTextFile(filename: string, text: string): void {
	downloadBlob(filename, new Blob([text], { type: 'text/plain;charset=utf-8' }));
}
