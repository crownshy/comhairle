/**
 * Takes a size in bytes and returns size to most appropriate unit to 2 decimal places.
 * Presumes files will have a max value of gb.
 */
export default function formatFileSize(sizeBytes: number) {
	if (sizeBytes < 1000) return `${sizeBytes}b`;
	if (sizeBytes < 1_000_000) return `${(sizeBytes / 1000).toFixed(2).toLocaleString()}kb`;
	if (sizeBytes < 1_000_000_000)
		return `${(sizeBytes / 1_000_000).toFixed(2).toLocaleString()}mb`;
	return `${(sizeBytes / 1_000_000_000).toFixed(2).toLocaleString()}gb`;
}
